use serde::{Deserialize, Serialize};
use std::time::Duration;
use url::Url;
use validator::{Validate, ValidationError};

/// Configuration for an RPC endpoint
#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct EndpointConfig {
    /// RPC endpoint URL
    #[validate(url)]
    #[validate(custom(function = "validate_url"))]
    pub url: String,
    
    /// Maximum requests per second
    #[validate(range(min = 1))]
    pub requests_per_second: u32,
    
    /// Request timeout in milliseconds
    #[validate(range(min = 100, max = 30000))]
    pub timeout_ms: u64,

    /// Weight for load balancing
    pub weight: u32,
}

fn validate_url(url: &str) -> Result<(), ValidationError> {
    match Url::parse(url) {
        Ok(parsed) => {
            if parsed.scheme() != "http" && parsed.scheme() != "https" {
                return Err(ValidationError::new("invalid_scheme"));
            }
            Ok(())
        }
        Err(_) => Err(ValidationError::new("invalid_url")),
    }
}

impl EndpointConfig {
    /// Create a new endpoint configuration
    pub fn new(url: String, requests_per_second: u32, timeout_ms: u64) -> Self {
        Self {
            url,
            requests_per_second,
            timeout_ms,
            weight: 1,
        }
    }
}

/// Global configuration for the RPC client
#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct Config {
    /// List of RPC endpoints
    #[validate(length(min = 1))]
    pub endpoints: Vec<EndpointConfig>,
    
    /// Maximum number of concurrent requests
    #[validate(range(min = 1, max = 1000))]
    pub max_concurrent_requests: usize,
    
    /// Retry configuration
    #[validate(nested)]
    pub retry_config: RetryConfig,

    /// Connection pool size
    pub pool_size: usize,

    /// Keep-alive duration
    pub keep_alive: Duration,

    /// Cache TTL
    pub cache_ttl: Duration,

    /// Global timeout
    pub timeout: Duration,
}

/// Configuration for retry behavior
#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct RetryConfig {
    /// Maximum number of retries
    #[validate(range(min = 0, max = 10))]
    pub max_retries: u32,
    
    /// Initial backoff delay in milliseconds
    #[validate(range(min = 100, max = 5000))]
    pub initial_delay_ms: u64,
    
    /// Maximum backoff delay in milliseconds
    #[validate(range(min = 1000, max = 30000))]
    pub max_delay_ms: u64,
    
    /// Backoff multiplier
    #[validate(range(min = 1.0, max = 5.0))]
    pub backoff_multiplier: f64,
}

impl Config {
    /// Create a new configuration
    pub fn new(
        endpoints: Vec<EndpointConfig>,
        max_concurrent_requests: usize,
        retry_config: RetryConfig,
    ) -> Self {
        Self {
            endpoints,
            max_concurrent_requests,
            retry_config,
            pool_size: 10,
            keep_alive: Duration::from_secs(30),
            cache_ttl: Duration::from_secs(60),
            timeout: Duration::from_secs(30),
        }
    }

    /// Load configuration from a file
    pub fn from_file(path: &str) -> Result<Self, crate::Error> {
        let contents = std::fs::read_to_string(path)
            .map_err(|e| crate::Error::config(format!("Failed to read config file: {}", e)))?;
        
        let config = serde_json::from_str(&contents)
            .map_err(|e| crate::Error::config(format!("Failed to parse config file: {}", e)))?;
        
        Ok(config)
    }

    /// Save configuration to a file
    pub fn save_to_file(&self, path: &str) -> Result<(), crate::Error> {
        let contents = serde_json::to_string_pretty(self)
            .map_err(|e| crate::Error::config(format!("Failed to serialize config: {}", e)))?;
        
        std::fs::write(path, contents)
            .map_err(|e| crate::Error::config(format!("Failed to write config file: {}", e)))?;
        
        Ok(())
    }

    /// Load configuration from environment variables
    pub fn from_env() -> Result<Self, config::ConfigError> {
        let config = config::Config::builder()
            .add_source(config::Environment::with_prefix("SOLANA_RPC"))
            .build()?;

        config.try_deserialize()
    }
}

impl RetryConfig {
    /// Create a new retry configuration
    pub fn new(
        max_retries: u32,
        initial_delay_ms: u64,
        max_delay_ms: u64,
        backoff_multiplier: f64,
    ) -> Self {
        Self {
            max_retries,
            initial_delay_ms,
            max_delay_ms,
            backoff_multiplier,
        }
    }
}

impl Default for RetryConfig {
    fn default() -> Self {
        Self {
            max_retries: 5,
            initial_delay_ms: 100,
            max_delay_ms: 10000,
            backoff_multiplier: 2.0,
        }
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            endpoints: vec![EndpointConfig::new(
                "https://api.mainnet-beta.solana.com".to_string(),
                100,
                10000,
            )],
            max_concurrent_requests: 10,
            retry_config: RetryConfig::default(),
            pool_size: 10,
            keep_alive: Duration::from_secs(30),
            cache_ttl: Duration::from_secs(60),
            timeout: Duration::from_secs(30),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use validator::Validate;

    #[test]
    fn test_config_default() {
        let config = Config::default();
        assert_eq!(config.max_concurrent_requests, 10);
        assert_eq!(config.retry_config.max_retries, 5);
        assert_eq!(config.pool_size, 10);
    }

    #[test]
    fn test_config_validation() {
        let mut config = Config::default();
        assert!(config.validate().is_ok());

        config.max_concurrent_requests = 0;
        assert!(config.validate().is_err());

        config.max_concurrent_requests = 1001;
        assert!(config.validate().is_err());

        config.retry_config.max_retries = 11;
        assert!(config.validate().is_err());
    }

    #[test]
    fn test_endpoint_config_validation() {
        let config = EndpointConfig {
            url: "https://api.mainnet-beta.solana.com".to_string(),
            requests_per_second: 100,
            timeout_ms: 10000,
            weight: 1,
        };
        assert!(config.validate().is_ok());

        let config = EndpointConfig {
            url: "ftp://example.com".to_string(),
            requests_per_second: 0,
            timeout_ms: 0,
            weight: 1,
        };
        assert!(config.validate().is_err());
    }

    #[test]
    fn test_config_serialization() {
        let config = Config::default();
        let serialized = serde_json::to_string(&config).unwrap();
        let deserialized: Config = serde_json::from_str(&serialized).unwrap();
        assert_eq!(config.max_concurrent_requests, deserialized.max_concurrent_requests);
    }
} 