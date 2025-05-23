use crate::core::traits::HealthStatus;
use crate::rpc::config::RpcConfig;
use crate::rpc::error::RpcError;
use std::sync::Arc;
use tokio::sync::RwLock;
use tokio::time::{Duration, Instant};
use chrono;

/// Statistics for an endpoint
#[derive(Debug, Clone)]
pub struct EndpointStats {
    /// Number of successful requests
    pub successful_requests: u64,
    /// Number of failed requests
    pub failed_requests: u64,
    /// Average response time in milliseconds
    pub avg_response_time_ms: f64,
    /// Current requests per second
    pub current_rps: f64,
    /// Total bytes transferred
    pub total_bytes_transferred: u64,
    /// Last successful request timestamp
    pub last_success: Option<Instant>,
    /// Last failed request timestamp
    pub last_failure: Option<Instant>,
}

impl Default for EndpointStats {
    fn default() -> Self {
        Self {
            successful_requests: 0,
            failed_requests: 0,
            avg_response_time_ms: 0.0,
            current_rps: 0.0,
            total_bytes_transferred: 0,
            last_success: None,
            last_failure: None,
        }
    }
}

/// Health monitor for RPC endpoints
#[derive(Debug)]
pub struct HealthMonitor {
    /// Client configuration
    config: Arc<RpcConfig>,
    /// Statistics for each endpoint
    stats: Arc<RwLock<Vec<EndpointStats>>>,
    /// Current endpoint index
    current_endpoint: Arc<RwLock<usize>>,
}

impl HealthMonitor {
    /// Create a new health monitor
    pub fn new(config: Arc<RpcConfig>) -> Self {
        let stats = vec![EndpointStats::default(); config.endpoints.len()];
        Self {
            config,
            stats: Arc::new(RwLock::new(stats)),
            current_endpoint: Arc::new(RwLock::new(0)),
        }
    }
    
    /// Check if the current endpoint is healthy
    pub async fn check_health(&self) -> Result<HealthStatus, RpcError> {
        let stats = self.stats.read().await;
        let current_idx = *self.current_endpoint.read().await;
        
        let endpoint_stats = &stats[current_idx];
        let now = Instant::now();

        // Check if we have recent successful requests
        let is_healthy = endpoint_stats.last_success
            .map(|last| now.duration_since(last) < Duration::from_secs(30))
            .unwrap_or(false);

        if is_healthy {
            Ok(HealthStatus {
                is_healthy: true,
                last_check: chrono::Utc::now(),
                error: None,
            })
        } else {
            // Try to find a healthy endpoint
            for (idx, stats) in stats.iter().enumerate() {
                if stats.last_success
                    .map(|last| now.duration_since(last) < Duration::from_secs(30))
                    .unwrap_or(false)
                {
                    *self.current_endpoint.write().await = idx;
                    return Ok(HealthStatus {
                        is_healthy: true,
                        last_check: chrono::Utc::now(),
                        error: None,
                    });
                }
            }
            Ok(HealthStatus {
                is_healthy: false,
                last_check: chrono::Utc::now(),
                error: Some("Unhealthy".to_string()),
            })
        }
    }
    
    /// Get statistics for all endpoints
    pub async fn get_stats(&self) -> Result<Vec<EndpointStats>, RpcError> {
        Ok(self.stats.read().await.clone())
    }
    
    /// Record a successful request
    pub async fn record_success(&self, endpoint_idx: usize, response_time_ms: u64, bytes_transferred: u64) -> Result<(), RpcError> {
        let mut stats = self.stats.write().await;
        if endpoint_idx >= stats.len() {
            return Err(RpcError::InvalidEndpoint(endpoint_idx));
        }

        let stats = &mut stats[endpoint_idx];
        stats.successful_requests += 1;
        stats.total_bytes_transferred += bytes_transferred;
        stats.last_success = Some(Instant::now());

        // Update average response time
        let total_requests = stats.successful_requests + stats.failed_requests;
        stats.avg_response_time_ms = (stats.avg_response_time_ms * (total_requests - 1) as f64 + response_time_ms as f64) / total_requests as f64;

        Ok(())
    }
    
    /// Record a failed request
    pub async fn record_failure(&self, endpoint_idx: usize) -> Result<(), RpcError> {
        let mut stats = self.stats.write().await;
        if endpoint_idx >= stats.len() {
            return Err(RpcError::InvalidEndpoint(endpoint_idx));
        }

        let stats = &mut stats[endpoint_idx];
        stats.failed_requests += 1;
        stats.last_failure = Some(Instant::now());

        Ok(())
    }
    
    /// Get the current endpoint index
    pub async fn get_current_endpoint(&self) -> usize {
        *self.current_endpoint.read().await
    }
    
    /// Switch to the next endpoint
    pub async fn switch_endpoint(&self) -> Result<(), RpcError> {
        let mut current = self.current_endpoint.write().await;
        *current = (*current + 1) % self.config.endpoints.len();
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::rpc::config::{EndpointConfig, RpcConfig};

    fn create_test_config() -> RpcConfig {
        RpcConfig {
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
        }
    }

    #[tokio::test]
    async fn test_health_monitor_creation() {
        let config = Arc::new(create_test_config());
        let monitor = HealthMonitor::new(config);
        assert_eq!(monitor.get_current_endpoint().await, 0);
    }

    #[tokio::test]
    async fn test_health_check() {
        let config = Arc::new(create_test_config());
        let monitor = HealthMonitor::new(config);

        // Initially unhealthy
        assert_eq!(monitor.check_health().await.unwrap(), HealthStatus::Unhealthy);

        // Record success
        monitor.record_success(0, 100, 1000).await.unwrap();
        assert_eq!(monitor.check_health().await.unwrap(), HealthStatus::Healthy);

        // Record failure
        monitor.record_failure(0).await.unwrap();
        assert_eq!(monitor.check_health().await.unwrap(), HealthStatus::Healthy); // Still healthy due to recent success

        // Switch endpoint
        monitor.switch_endpoint().await.unwrap();
        assert_eq!(monitor.get_current_endpoint().await, 1);
    }

    #[tokio::test]
    async fn test_stats_recording() {
        let config = Arc::new(create_test_config());
        let monitor = HealthMonitor::new(config);

        // Record some stats
        monitor.record_success(0, 100, 1000).await.unwrap();
        monitor.record_success(0, 200, 2000).await.unwrap();
        monitor.record_failure(0).await.unwrap();

        let stats = monitor.get_stats().await.unwrap();
        assert_eq!(stats[0].successful_requests, 2);
        assert_eq!(stats[0].failed_requests, 1);
        assert_eq!(stats[0].total_bytes_transferred, 3000);
        assert!(stats[0].avg_response_time_ms > 0.0);
    }

    #[tokio::test]
    async fn test_invalid_endpoint() {
        let config = Arc::new(create_test_config());
        let monitor = HealthMonitor::new(config);

        // Try to record stats for non-existent endpoint
        assert!(monitor.record_success(2, 100, 1000).await.is_err());
        assert!(monitor.record_failure(2).await.is_err());
    }
} 