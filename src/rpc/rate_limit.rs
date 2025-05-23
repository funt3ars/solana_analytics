use crate::rpc::config::RateLimitConfig;
use crate::rpc::error::RpcError;
use governor::{Quota, RateLimiter as GovRateLimiter, state::NotKeyed, state::InMemoryState, clock::DefaultClock};
use std::num::NonZeroU32;
use std::sync::Arc;
use std::time::Duration;

/// Rate limiter for RPC requests
#[derive(Debug, Clone)]
pub struct RpcRateLimiter {
    /// The underlying rate limiter
    limiter: Arc<GovRateLimiter<NotKeyed, InMemoryState, DefaultClock>>,
    /// Maximum requests per second
    max_rps: u32,
    /// Burst size
    burst_size: u32,
}

impl RpcRateLimiter {
    /// Create a new rate limiter
    pub fn new(config: &RateLimitConfig) -> std::result::Result<Self, RpcError> {
        let max_rps = NonZeroU32::new(config.max_rps)
            .ok_or_else(|| RpcError::InvalidConfig("max_rps must be greater than 0".to_string()))?;
        
        let burst_size = NonZeroU32::new(config.burst_size)
            .ok_or_else(|| RpcError::InvalidConfig("burst_size must be greater than 0".to_string()))?;

        let quota = Quota::per_second(max_rps).allow_burst(burst_size);

        let limiter = Arc::new(GovRateLimiter::direct(quota));

        Ok(Self {
            limiter,
            max_rps: config.max_rps,
            burst_size: config.burst_size,
        })
    }
    
    /// Wait for a permit to make a request
    pub async fn wait_for_permit(&self) {
        self.limiter.until_ready().await;
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
    use std::time::Duration;
    use std::time::Instant;
    
    #[tokio::test]
    async fn test_rate_limiter_creation() {
        let config = RateLimitConfig {
            max_rps: 100,
            burst_size: 10,
        };
        let limiter = RpcRateLimiter::new(&config).unwrap();
        assert_eq!(limiter.max_rps(), 100);
        assert_eq!(limiter.burst_size(), 10);
    }
    
    #[tokio::test]
    async fn test_rate_limiting() {
        let config = RateLimitConfig {
            max_rps: 2,
            burst_size: 1,
        };
        let limiter = RpcRateLimiter::new(&config).unwrap();
        
        // First request should succeed immediately
        limiter.wait_for_permit().await;
        
        // Second request should be rate limited
        let start = Instant::now();
        limiter.wait_for_permit().await;
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
        assert!(RpcRateLimiter::new(&config).is_err());

        let config = RateLimitConfig {
            max_rps: 100,
            burst_size: 0,
        };
        assert!(RpcRateLimiter::new(&config).is_err());
    }
} 