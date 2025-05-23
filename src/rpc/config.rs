use crate::core::traits::{Config, RetryConfig};
use serde::{Deserialize, Serialize};
use std::time::Duration;
use url::Url;
use validator::Validate;

/// Configuration for a single RPC endpoint
#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct EndpointConfig {
    /// The URL of the RPC endpoint
    pub url: String,
    
    /// The weight of this endpoint for load balancing
    pub weight: u32,
    
    /// Whether this endpoint is enabled
    pub enabled: bool,
}

/// Configuration for the RPC client
#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct RpcConfig {
    /// List of RPC endpoints
    #[validate(length(min = 1))]
    pub endpoints: Vec<EndpointConfig>,
    
    /// Maximum number of concurrent requests
    #[validate(range(min = 1))]
    pub max_concurrent_requests: u32,
    
    /// Request timeout in milliseconds
    pub request_timeout_ms: u64,
    
    /// Retry configuration
    pub retry: RetryConfig,
    
    /// Rate limiting configuration
    pub rate_limit: RateLimitConfig,
}

/// Rate limiting configuration
#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct RateLimitConfig {
    /// Maximum requests per second
    #[validate(range(min = 1))]
    pub max_rps: u32,
    
    /// Burst size for rate limiting
    #[validate(range(min = 1))]
    pub burst_size: u32,
}

impl Config for RpcConfig {
    fn max_concurrent_requests(&self) -> u32 {
        self.max_concurrent_requests
    }
    
    fn timeout(&self) -> Duration {
        Duration::from_millis(self.request_timeout_ms)
    }
    
    fn retry_config(&self) -> &crate::core::traits::RetryConfig {
        &self.retry
    }
}

impl Default for RpcConfig {
    fn default() -> Self {
        Self {
            endpoints: vec![EndpointConfig {
                url: "http://localhost:8899".to_string(),
                weight: 1,
                enabled: true,
            }],
            max_concurrent_requests: 10,
            request_timeout_ms: 5000,
            retry: RetryConfig {
                max_retries: 3,
                retry_delay_ms: 100,
            },
            rate_limit: RateLimitConfig {
                max_rps: 100,
                burst_size: 10,
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_default_config() {
        let config = RpcConfig::default();
        assert!(config.validate().is_ok());
        assert_eq!(config.max_concurrent_requests, 10);
        assert_eq!(config.request_timeout_ms, 5000);
        assert_eq!(config.retry.max_retries, 3);
        assert_eq!(config.retry.retry_delay_ms, 100);
        assert_eq!(config.rate_limit.max_rps, 100);
        assert_eq!(config.rate_limit.burst_size, 10);
    }
    
    #[test]
    fn test_invalid_config() {
        let mut config = RpcConfig::default();
        config.max_concurrent_requests = 0;
        assert!(config.validate().is_err());
    }
} 