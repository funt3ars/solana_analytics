use async_trait::async_trait;
use std::fmt::Debug;
use crate::core::error::Result;
use std::time::Duration;
use serde::{Serialize, Deserialize};
use validator::Validate;

/// Generic configuration trait for clients
pub trait Config: Send + Sync + Debug {
    /// Get the maximum number of concurrent requests
    fn max_concurrent_requests(&self) -> u32;
    
    /// Get the timeout duration
    fn timeout(&self) -> Duration;
    
    /// Get the retry configuration
    fn retry_config(&self) -> &RetryConfig;
}

/// Retry configuration
#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct RetryConfig {
    /// Maximum number of retry attempts
    pub max_retries: u32,
    /// Delay between retries in milliseconds
    pub retry_delay_ms: u64,
}

/// Repository trait for database operations
#[async_trait]
pub trait Repository<T: Send + Sync + Debug> {
    /// Create a new record
    async fn create(&self, item: T) -> Result<T>;
    
    /// Find a record by ID
    async fn find_by_id(&self, id: &str) -> Result<Option<T>>;
    
    /// Update a record
    async fn update(&self, item: T) -> Result<T>;
    
    /// Delete a record
    async fn delete(&self, id: &str) -> Result<()>;
    
    /// List all records with optional filtering
    async fn list<'a>(&self, filter: Option<&'a str>) -> Result<Vec<T>>;
}

/// Client trait for RPC operations
#[async_trait]
pub trait Client: Send + Sync + Debug {
    /// Get the client's configuration
    fn config(&self) -> &dyn Config;
    
    /// Check if the client is healthy
    async fn is_healthy(&self) -> Result<bool>;
    
    /// Get the current endpoint being used
    fn current_endpoint(&self) -> &str;
    
    /// Get metrics about the client's performance
    async fn get_metrics(&self) -> Result<ClientMetrics>;
}

/// Health check trait for monitoring
#[async_trait]
pub trait HealthCheck: Send + Sync + Debug {
    /// Check if the service is healthy
    async fn check_health(&self) -> Result<HealthStatus>;
    
    /// Get detailed health information
    async fn get_health_details(&self) -> Result<HealthDetails>;
}

/// Cache trait for caching operations
#[async_trait]
pub trait Cache<K: Send + Sync + Debug, V: Send + Sync + Debug>: Send + Sync + Debug {
    /// Get a value from the cache
    async fn get(&self, key: &K) -> Result<Option<V>>;
    
    /// Set a value in the cache
    async fn set(&self, key: K, value: V, ttl: Option<Duration>) -> Result<()>;
    
    /// Remove a value from the cache
    async fn remove(&self, key: &K) -> Result<()>;
    
    /// Clear the entire cache
    async fn clear(&self) -> Result<()>;
}

/// Client metrics for monitoring
#[derive(Debug, Clone)]
pub struct ClientMetrics {
    /// Number of successful requests
    pub successful_requests: u64,
    /// Number of failed requests
    pub failed_requests: u64,
    /// Average response time in milliseconds
    pub avg_response_time_ms: f64,
    /// Current requests per second
    pub current_rps: f64,
    /// Total bytes transferred
    pub bytes_transferred: u64,
}

/// Health status for monitoring
#[derive(Debug, Clone)]
pub struct HealthStatus {
    /// Whether the service is healthy
    pub is_healthy: bool,
    /// Last check timestamp
    pub last_check: chrono::DateTime<chrono::Utc>,
    /// Error message if unhealthy
    pub error: Option<String>,
}

/// Detailed health information
#[derive(Debug, Clone)]
pub struct HealthDetails {
    /// Overall health status
    pub status: HealthStatus,
    /// Component-specific health information
    pub components: Vec<ComponentHealth>,
    /// System metrics
    pub metrics: SystemMetrics,
}

/// Component-specific health information
#[derive(Debug, Clone)]
pub struct ComponentHealth {
    /// Component name
    pub name: String,
    /// Component status
    pub status: HealthStatus,
    /// Component-specific metrics
    pub metrics: Option<serde_json::Value>,
}

/// System metrics
#[derive(Debug, Clone)]
pub struct SystemMetrics {
    /// CPU usage percentage
    pub cpu_usage: f64,
    /// Memory usage in bytes
    pub memory_usage: u64,
    /// Disk usage in bytes
    pub disk_usage: u64,
    /// Network usage in bytes
    pub network_usage: u64,
}

#[cfg(test)]
mod tests {
    use super::*;
    use mockall::mock;

    mock! {
        #[derive(Debug)]
        TestRepository {}
        #[async_trait]
        impl Repository<String> for TestRepository {
            async fn create(&self, item: String) -> Result<String>;
            async fn find_by_id(&self, id: &str) -> Result<Option<String>>;
            async fn update(&self, item: String) -> Result<String>;
            async fn delete(&self, id: &str) -> Result<()>;
            async fn list<'a>(&self, filter: Option<&'a str>) -> Result<Vec<String>>;
        }
    }

    mock! {
        #[derive(Debug)]
        TestClient {}
        #[async_trait]
        impl Client for TestClient {
            fn config(&self) -> &dyn Config;
            async fn is_healthy(&self) -> Result<bool>;
            fn current_endpoint(&self) -> &str;
            async fn get_metrics(&self) -> Result<ClientMetrics>;
        }
    }

    mock! {
        #[derive(Debug)]
        TestHealthCheck {}
        #[async_trait]
        impl HealthCheck for TestHealthCheck {
            async fn check_health(&self) -> Result<HealthStatus>;
            async fn get_health_details(&self) -> Result<HealthDetails>;
        }
    }

    mock! {
        #[derive(Debug)]
        TestCache {}
        #[async_trait]
        impl Cache<String, String> for TestCache {
            async fn get(&self, key: &String) -> Result<Option<String>>;
            async fn set(&self, key: String, value: String, ttl: Option<Duration>) -> Result<()>;
            async fn remove(&self, key: &String) -> Result<()>;
            async fn clear(&self) -> Result<()>;
        }
    }

    #[tokio::test]
    async fn test_repository() {
        let mut mock = MockTestRepository::new();
        mock.expect_create()
            .returning(|item| Ok(item));
        
        let result = mock.create("test".to_string()).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_client() {
        let mut mock = MockTestClient::new();
        mock.expect_is_healthy()
            .returning(|| Ok(true));
        
        let result = mock.is_healthy().await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_health_check() {
        let mut mock = MockTestHealthCheck::new();
        mock.expect_check_health()
            .returning(|| Ok(HealthStatus {
                is_healthy: true,
                last_check: chrono::Utc::now(),
                error: None,
            }));
        
        let result = mock.check_health().await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_cache() {
        let mut mock = MockTestCache::new();
        mock.expect_set()
            .returning(|_, _, _| Ok(()));
        
        let result = mock.set("key".to_string(), "value".to_string(), None).await;
        assert!(result.is_ok());
    }
} 