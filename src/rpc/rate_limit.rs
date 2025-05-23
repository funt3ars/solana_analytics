use crate::core::error::{Error, Result};
use crate::rpc::config::RateLimitConfig;
use crate::rpc::error::RpcError;
use governor::{Quota, RateLimiter};
use nonzero_ext::nonzero;
use std::num::NonZeroU32;
use std::sync::Arc;
use std::time::Duration;

/// Rate limiter for RPC requests
#[derive(Debug, Clone)]
pub struct RateLimiter {
    /// The underlying rate limiter
    limiter: Arc<RateLimiter<governor::state::NotKeyed, governor::state::InMemoryState, governor::clock::DefaultClock>>,
    /// Maximum requests per second
    max_rps: u32,
    /// Burst size
    burst_size: u32,
}

impl RateLimiter {
    /// Create a new rate limiter
    pub fn new(config: &RateLimitConfig) -> Result<Self, RpcError> {
        let max_rps = NonZeroU32::new(config.max_rps)
            .ok_or_else(|| RpcError::InvalidConfig("max_rps must be greater than 0".to_string()))?;
        
        let burst_size = NonZeroU32::new(config.burst_size)
            .ok_or_else(|| RpcError::InvalidConfig("burst_size must be greater than 0".to_string()))?;

        let quota = Quota::with_period(Duration::from_secs(1))
            .ok_or_else(|| RpcError::InvalidConfig("Invalid rate limit period".to_string()))?
            .allow_burst(burst_size)
            .allow_n(max_rps);

        let limiter = Arc::new(RateLimiter::direct(quota));

        Ok(Self {
            limiter,
            max_rps: config.max_rps,
            burst_size: config.burst_size,
        })
    }
    
    /// Wait for a permit to make a request
    pub async fn wait_for_permit(&self) -> Result<(), RpcError> {
        self.limiter.until_ready().await;
        Ok(())
    }
    
    /// Get the maximum requests per second
    pub fn max_rps(&self) -> u32 {
        self.max_rps
    }
    
    /// Get the burst size
    pub fn burst_size(&self) -> u32 {
        self.burst_size
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Instant;
    
    #[tokio::test]
    async fn test_rate_limiter_creation() {
        let config = RateLimitConfig {
            max_rps: 100,
            burst_size: 10,
        };
        let limiter = RateLimiter::new(&config).unwrap();
        assert_eq!(limiter.max_rps(), 100);
        assert_eq!(limiter.burst_size(), 10);
    }
    
    #[tokio::test]
    async fn test_rate_limiting() {
        let config = RateLimitConfig {
            max_rps: 2,
            burst_size: 1,
        };
        let limiter = RateLimiter::new(&config).unwrap();
        
        // First request should succeed immediately
        limiter.wait_for_permit().await.unwrap();
        
        // Second request should be rate limited
        let start = Instant::now();
        limiter.wait_for_permit().await.unwrap();
        let duration = start.elapsed();
        
        // Should have waited at least 500ms (1 second / 2 requests)
        assert!(duration >= Duration::from_millis(450));
    }
    
    #[test]
    fn test_invalid_config() {
        let config = RateLimitConfig {
            max_rps: 0,
            burst_size: 10,
        };
        assert!(RateLimiter::new(&config).is_err());

        let config = RateLimitConfig {
            max_rps: 100,
            burst_size: 0,
        };
        assert!(RateLimiter::new(&config).is_err());
    }
} 