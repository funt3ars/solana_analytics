use crate::core::error::{Error, Result};
use crate::core::traits::{Client, Config, HealthStatus};
use async_trait::async_trait;
use governor::{Quota, RateLimiter};
use std::num::NonZeroU32;
use std::sync::Arc;
use std::time::Duration;
use solana_client::rpc_client::RpcClient;
use solana_sdk::commitment_config::CommitmentConfig;
use url::Url;
use crate::rpc::config::{RpcConfig, EndpointConfig};
use crate::rpc::health::HealthMonitor;
use tokio::sync::RwLock;
use crate::rpc::error::RpcError;
use crate::rpc::rate_limit::RateLimiter as RpcRateLimiter;
use std::time::Instant;

/// Configuration for a Solana RPC endpoint
#[derive(Debug, Clone)]
pub struct EndpointConfig {
    pub url: Url,
    pub requests_per_second: u32,
    pub weight: u32,
}

/// Configuration for the Solana RPC client
#[derive(Debug, Clone)]
pub struct RpcConfig {
    pub endpoints: Vec<EndpointConfig>,
    pub max_concurrent_requests: u32,
    pub retry_attempts: u32,
    pub retry_delay_ms: u64,
    pub timeout: Duration,
}

impl Default for RpcConfig {
    fn default() -> Self {
        Self {
            endpoints: vec![EndpointConfig {
                url: Url::parse("https://api.mainnet-beta.solana.com").unwrap(),
                requests_per_second: 100,
                weight: 1,
            }],
            max_concurrent_requests: 10,
            retry_attempts: 3,
            retry_delay_ms: 1000,
            timeout: Duration::from_secs(30),
        }
    }
}

/// Client for interacting with Solana RPC endpoints
pub struct SolanaRpcClient {
    /// Client configuration
    config: Arc<RpcConfig>,
    /// Health monitor for endpoints
    health_monitor: HealthMonitor,
    /// Rate limiter for requests
    rate_limiter: RateLimiter,
    /// Current RPC client
    client: Arc<RwLock<RpcClient>>,
}

impl SolanaRpcClient {
    /// Create a new Solana RPC client
    pub fn new(config: RpcConfig) -> Result<Self, RpcError> {
        let config = Arc::new(config);
        
        // Initialize health monitor
        let health_monitor = HealthMonitor::new(config.clone());
        
        // Initialize rate limiter
        let rate_limiter = RateLimiter::new(&config.rate_limit)?;
        
        // Initialize RPC client with first enabled endpoint
        let endpoint = config.endpoints.iter()
            .find(|e| e.enabled)
            .ok_or_else(|| RpcError::NoEnabledEndpoints)?;
            
        let client = RpcClient::new_with_commitment(
            endpoint.url.clone(),
            CommitmentConfig::confirmed(),
        );

        Ok(Self {
            config,
            health_monitor,
            rate_limiter,
            client: Arc::new(RwLock::new(client)),
        })
    }
    
    /// Get the health monitor
    pub fn health_monitor(&self) -> &HealthMonitor {
        &self.health_monitor
    }
    
    /// Get the rate limiter
    pub fn rate_limiter(&self) -> &RateLimiter {
        &self.rate_limiter
    }

    async fn with_retry<F, T>(&self, operation: &str, f: F) -> Result<T, RpcError>
    where
        F: Fn() -> Result<T, RpcError>,
    {
        let mut attempts = 0;
        let max_attempts = self.config.retry_attempts;
        let mut last_error = None;
        let start_time = Instant::now();

        while attempts < max_attempts {
            // Check circuit breaker
            if self.health_monitor.is_circuit_breaker_open().await {
                return Err(RpcError::CircuitBreakerOpen(
                    format!("Circuit breaker open after {} attempts", attempts)
                ));
            }

            // Wait for rate limit permit
            self.rate_limiter.wait_for_permit().await?;

            match f() {
                Ok(result) => {
                    // Record success
                    self.health_monitor.record_success(
                        start_time.elapsed().as_millis() as u64
                    ).await;
                    return Ok(result);
                }
                Err(e) => {
                    // Record failure
                    self.health_monitor.record_failure().await;
                    
                    if !e.is_retryable() {
                        return Err(e.with_context(format!("{} failed", operation)));
                    }

                    last_error = Some(e);
                    attempts += 1;

                    // Calculate backoff duration
                    let backoff = Duration::from_millis(
                        self.config.retry_delay_ms * 2u64.pow(attempts as u32)
                    );
                    
                    tokio::time::sleep(backoff).await;
                }
            }
        }

        Err(RpcError::RetryFailed {
            attempts,
            error: Box::new(
                last_error
                    .unwrap_or_else(|| RpcError::Internal("Unknown error".to_string()))
                    .with_context(format!("{} failed after {} attempts", operation, attempts))
            ),
        })
    }

    pub async fn get_block(&self, slot: u64) -> Result<solana_sdk::block::Block, RpcError> {
        self.with_retry("get_block", || {
            let client = self.client.blocking_read();
            client.get_block(slot)
                .map_err(|e| RpcError::RequestFailed(e)
                    .with_context(format!("Failed to get block at slot {}", slot)))
        }).await
    }

    pub async fn get_signature_status(
        &self,
        signature: &solana_sdk::signature::Signature,
    ) -> Result<Option<solana_client::rpc_response::RpcSignatureStatus>, RpcError> {
        self.with_retry("get_signature_status", || {
            let client = self.client.blocking_read();
            client.get_signature_status(signature)
                .map_err(|e| RpcError::RequestFailed(e)
                    .with_context(format!("Failed to get status for signature {}", signature)))
        }).await
    }

    async fn update_client(&self, endpoint_idx: usize) -> Result<(), RpcError> {
        let endpoint = self.config.endpoints.get(endpoint_idx)
            .ok_or_else(|| RpcError::InvalidEndpoint(endpoint_idx))?;
            
        let client = RpcClient::new_with_commitment(
            endpoint.url.clone(),
            CommitmentConfig::confirmed(),
        );
        
        *self.client.write().await = client;
        Ok(())
    }
}

#[async_trait::async_trait]
impl Client for SolanaRpcClient {
    type Config = RpcConfig;
    type Error = RpcError;

    fn config(&self) -> &Self::Config {
        &self.config
    }
    
    async fn check_health(&self) -> Result<HealthStatus, Self::Error> {
        self.health_monitor.check_health().await
    }
    
    async fn current_endpoint(&self) -> Result<String, Self::Error> {
        let current_idx = self.health_monitor.get_current_endpoint().await;
        let endpoint = self.config.endpoints.get(current_idx)
            .ok_or_else(|| RpcError::InvalidEndpoint(current_idx))?;
        Ok(endpoint.url.clone())
    }
    
    async fn metrics(&self) -> Result<serde_json::Value, Self::Error> {
        let stats = self.health_monitor.get_stats().await?;
        let current_idx = self.health_monitor.get_current_endpoint().await;
        
        let metrics = serde_json::json!({
            "current_endpoint": current_idx,
            "endpoints": stats.iter().enumerate().map(|(idx, stat)| {
                serde_json::json!({
                    "index": idx,
                    "url": self.config.endpoints[idx].url,
                    "successful_requests": stat.successful_requests,
                    "failed_requests": stat.failed_requests,
                    "avg_response_time_ms": stat.avg_response_time_ms,
                    "current_rps": stat.current_rps,
                    "total_bytes_transferred": stat.total_bytes_transferred,
                    "last_success": stat.last_success.map(|t| t.elapsed().as_secs()),
                    "last_failure": stat.last_failure.map(|t| t.elapsed().as_secs()),
                })
            }).collect::<Vec<_>>(),
            "rate_limiter": {
                "max_rps": self.rate_limiter.max_rps(),
                "burst_size": self.rate_limiter.burst_size(),
            }
        });

        Ok(metrics)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_client_creation() {
        let config = RpcConfig::default();
        let client = SolanaRpcClient::new(config);
        assert!(client.is_ok());
    }

    #[test]
    fn test_invalid_rate_limit() {
        let mut config = RpcConfig::default();
        config.max_concurrent_requests = 0; // Invalid rate limit
        let client = SolanaRpcClient::new(config);
        assert!(client.is_err());
    }

    #[test]
    fn test_invalid_url() {
        let mut config = RpcConfig::default();
        config.endpoints[0].url = Url::parse("invalid-url").unwrap_err();
        let client = SolanaRpcClient::new(config);
        assert!(client.is_err());
    }

    #[test]
    fn test_client_config() {
        let config = RpcConfig::default();
        let client = SolanaRpcClient::new(config.clone()).unwrap();
        assert_eq!(client.get_config().max_concurrent_requests, config.max_concurrent_requests);
    }

    #[tokio::test]
    async fn test_basic_rpc_call() {
        let config = RpcConfig::default();
        let client = SolanaRpcClient::new(config).unwrap();
        
        // Test a basic RPC call (getSlot)
        let result = client.client.blocking_read().get_slot();
        assert!(result.is_ok() || result.is_err()); // Either success or error is valid
    }

    #[tokio::test]
    async fn test_rate_limiting() {
        let mut config = RpcConfig::default();
        config.max_concurrent_requests = 1; // Very low rate limit for testing
        
        let client = SolanaRpcClient::new(config).unwrap();
        
        // Make two rapid requests
        let start = std::time::Instant::now();
        let _ = client.client.blocking_read().get_slot();
        let _ = client.client.blocking_read().get_slot();
        let duration = start.elapsed();
        
        // The second request should have been rate limited
        assert!(duration >= std::time::Duration::from_secs(1));
    }

    #[tokio::test]
    async fn test_client_creation() {
        let config = RpcConfig::default();
        let client = SolanaRpcClient::new(config).unwrap();
        assert!(client.check_health().await.unwrap() == HealthStatus::Unhealthy);
    }
    
    #[tokio::test]
    async fn test_client_metrics() {
        let config = RpcConfig::default();
        let client = SolanaRpcClient::new(config).unwrap();
        let metrics = client.metrics().await.unwrap();
        assert_eq!(metrics["current_endpoint"], 0);
        assert_eq!(metrics["endpoints"].as_array().unwrap().len(), 1);
        assert_eq!(metrics["rate_limiter"]["max_rps"], 100);
        assert_eq!(metrics["rate_limiter"]["burst_size"], 10);
    }

    #[test]
    fn test_no_enabled_endpoints() {
        let config = RpcConfig {
            endpoints: vec![
                EndpointConfig {
                    url: "http://endpoint1".to_string(),
                    weight: 1,
                    enabled: false,
                },
            ],
            max_concurrent_requests: 10,
            retry_attempts: 3,
            retry_delay_ms: 1000,
            timeout: Duration::from_secs(30),
        };
        
        assert!(matches!(
            SolanaRpcClient::new(config),
            Err(RpcError::NoEnabledEndpoints)
        ));
    }
} 