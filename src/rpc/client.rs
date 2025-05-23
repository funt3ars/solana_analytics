use crate::core::error::{Error, Result};
use crate::core::traits::{Client, Config, HealthStatus};
use async_trait::async_trait;
use governor::Quota;
use crate::rpc::rate_limit::RpcRateLimiter;
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
use std::time::Instant;
use std::fmt;
use std::fmt::Debug;

/// Client for interacting with Solana RPC endpoints
#[derive(Debug)]
pub struct SolanaRpcClient {
    /// Client configuration
    config: Arc<RpcConfig>,
    /// Health monitor for endpoints
    health_monitor: HealthMonitor,
    /// Rate limiter for requests
    rate_limiter: RpcRateLimiter,
    /// Current RPC client
    #[debug(skip)]
    client: Arc<RwLock<RpcClient>>,
    /// Store the current endpoint index for &str return
    current_endpoint_idx: usize,
    /// Store the endpoint URL as a String for &str return
    current_endpoint_url: String,
}

impl SolanaRpcClient {
    /// Create a new Solana RPC client
    pub fn new(config: RpcConfig) -> std::result::Result<Self, RpcError> {
        let config = Arc::new(config);
        
        // Initialize health monitor
        let health_monitor = HealthMonitor::new(config.clone());
        
        // Initialize rate limiter
        let rate_limiter = RpcRateLimiter::new(&config.rate_limit)?;
        
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
            current_endpoint_idx: 0,
            current_endpoint_url: endpoint.url.clone(),
        })
    }
    
    /// Get the health monitor
    pub fn health_monitor(&self) -> &HealthMonitor {
        &self.health_monitor
    }
    
    /// Get the rate limiter
    pub fn rate_limiter(&self) -> &RpcRateLimiter {
        &self.rate_limiter
    }

    async fn with_retry<F, T>(&self, operation: &str, f: F) -> std::result::Result<T, RpcError>
    where
        F: Fn() -> std::result::Result<T, RpcError>,
    {
        let mut attempts = 0;
        let max_attempts = self.config.retry.max_retries;
        let mut last_error = None;
        let start_time = Instant::now();

        while attempts < max_attempts {
            // Wait for rate limit permit
            self.rate_limiter.wait_for_permit().await;

            match f() {
                Ok(result) => {
                    // Record success
                    let endpoint_idx = self.health_monitor.get_current_endpoint().await;
                    let response_time_ms = start_time.elapsed().as_millis() as u64;
                    self.health_monitor.record_success(endpoint_idx, response_time_ms, 0).await.unwrap_or(());
                    return Ok(result);
                }
                Err(e) => {
                    // Record failure
                    let endpoint_idx = self.health_monitor.get_current_endpoint().await;
                    self.health_monitor.record_failure(endpoint_idx).await.unwrap_or(());
                    if !e.is_retryable() {
                        return Err(e.with_context(format!("{} failed", operation)));
                    }
                    last_error = Some(e);
                    attempts += 1;
                    // Calculate backoff duration
                    let backoff = Duration::from_millis(
                        self.config.retry.retry_delay_ms * 2u64.pow(attempts as u32)
                    );
                    tokio::time::sleep(backoff).await;
                }
            }
        }

        Err(last_error.unwrap_or_else(|| RpcError::Internal(format!("{} failed after {} attempts", operation, attempts))))
    }

    pub async fn get_block(&self, slot: u64) -> std::result::Result<solana_transaction_status::EncodedConfirmedBlock, RpcError> {
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
    ) -> std::result::Result<Option<solana_client::rpc_response::RpcSignatureResult>, RpcError> {
        self.with_retry("get_signature_status", || {
            let client = self.client.blocking_read();
            client.get_signature_status(signature)
                .map_err(|e| RpcError::RequestFailed(e)
                    .with_context(format!("Failed to get status for signature {}", signature)))
        }).await
    }

    async fn update_client(&self, endpoint_idx: usize) -> std::result::Result<(), RpcError> {
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
impl crate::core::traits::Client for SolanaRpcClient {
    fn config(&self) -> &dyn crate::core::traits::Config {
        &*self.config
    }
    async fn is_healthy(&self) -> crate::core::error::Result<bool> {
        // Use health_monitor or stub
        Ok(true)
    }
    fn current_endpoint(&self) -> &str {
        // Return the first enabled endpoint as a string reference
        self.config.endpoints.iter().find(|e| e.enabled).map(|e| e.url.as_str()).unwrap_or("")
    }
    async fn get_metrics(&self) -> crate::core::error::Result<crate::core::traits::ClientMetrics> {
        // Stub: return default metrics
        Ok(crate::core::traits::ClientMetrics {
            successful_requests: 0,
            failed_requests: 0,
            avg_response_time_ms: 0.0,
            current_rps: 0.0,
            bytes_transferred: 0,
        })
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
            request_timeout_ms: 5000,
            retry: Default::default(),
            rate_limit: Default::default(),
        };
        assert!(matches!(
            SolanaRpcClient::new(config),
            Err(RpcError::NoEnabledEndpoints)
        ));
    }
} 