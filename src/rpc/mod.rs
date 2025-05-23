use crate::core::error::Error;
use crate::core::traits::{HealthCheck, HealthStatus};

pub mod client;
pub mod config;
pub mod error;
pub mod health;
pub mod rate_limit;

pub type RpcClientError = Error;

pub use client::SolanaRpcClient;
pub use config::{EndpointConfig, RpcConfig, RateLimitConfig};
pub use error::RpcError;
pub use health::{HealthMonitor, EndpointStats};
pub use rate_limit::RpcRateLimiter;

pub type Result<T> = std::result::Result<T, RpcClientError>;

#[cfg(test)]
mod tests {
    use super::*;
    use url::Url;

    #[test]
    fn test_rpc_config_default() {
        let config = RpcConfig::default();
        assert_eq!(config.retry.max_retries, 3);
        assert_eq!(config.retry.retry_delay_ms, 1000);
        assert_eq!(config.endpoints[0].url, "http://localhost:8899");
        assert_eq!(config.endpoints[0].weight, 1);
    }

    #[tokio::test]
    async fn test_rpc_client_creation() {
        let config = RpcConfig {
            endpoints: vec![
                EndpointConfig {
                    url: "http://endpoint1".to_string(),
                    weight: 1,
                    enabled: true,
                },
            ],
            max_concurrent_requests: 10,
            request_timeout_ms: 5000,
            retry: Default::default(),
            rate_limit: Default::default(),
        };

        let client = SolanaRpcClient::new(config).unwrap();
        assert_eq!(client.check_health().await.unwrap(), HealthStatus::Unhealthy(None));
    }
    
    #[tokio::test]
    async fn test_rate_limiting() {
        let config = RpcConfig {
            endpoints: vec![
                EndpointConfig {
                    url: "http://endpoint1".to_string(),
                    weight: 1,
                    enabled: true,
                },
            ],
            max_concurrent_requests: 10,
            request_timeout_ms: 5000,
            retry: Default::default(),
            rate_limit: RateLimitConfig {
                max_rps: 2,
                burst_size: 1,
            },
        };

        let client = SolanaRpcClient::new(config).unwrap();
        let rate_limiter = client.rate_limiter();

        // First request should succeed immediately
        rate_limiter.wait_for_permit().await;

        // Second request should be rate limited
        let start = std::time::Instant::now();
        rate_limiter.wait_for_permit().await;
        let duration = start.elapsed();

        // Should have waited at least 450ms (1 second / 2 requests)
        assert!(duration >= std::time::Duration::from_millis(450));
    }
    
    #[tokio::test]
    async fn test_health_monitoring() {
        let config = RpcConfig {
            endpoints: vec![
                EndpointConfig {
                    url: "http://endpoint1".to_string(),
                    weight: 1,
                    enabled: true,
                },
                EndpointConfig {
                    url: "http://endpoint2".to_string(),
                    weight: 1,
                    enabled: true,
                },
            ],
            max_concurrent_requests: 10,
            request_timeout_ms: 5000,
            retry: Default::default(),
            rate_limit: Default::default(),
        };

        let client = SolanaRpcClient::new(config).unwrap();
        let health_monitor = client.health_monitor();

        // Initially unhealthy
        assert_eq!(health_monitor.check_health().await.unwrap(), HealthStatus::Unhealthy(None));

        // Record success
        health_monitor.record_success(0, 100, 1000).await.unwrap();
        assert_eq!(health_monitor.check_health().await.unwrap(), HealthStatus::Healthy);

        // Record failure
        health_monitor.record_failure(0).await.unwrap();
        assert_eq!(health_monitor.check_health().await.unwrap(), HealthStatus::Healthy); // Still healthy due to recent success

        // Switch endpoint
        health_monitor.switch_endpoint().await.unwrap();
        assert_eq!(health_monitor.get_current_endpoint().await, 1);
    }
} 