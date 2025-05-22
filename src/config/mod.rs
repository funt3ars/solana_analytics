//! Configuration types and loading for the Solana analytics system

use serde::Deserialize;
use std::time::Duration;
use url::Url;

#[derive(Debug, Clone, Deserialize)]
pub struct RpcConfig {
    pub endpoints: Vec<EndpointConfig>,
    pub pool_size: usize,
    pub keep_alive: Duration,
    pub max_retries: u32,
    pub base_delay: Duration,
    pub max_delay: Duration,
    pub requests_per_second: u32,
    pub cache_ttl: Duration,
    pub timeout: Duration,
}

#[derive(Debug, Clone, Deserialize)]
pub struct EndpointConfig {
    pub url: Url,
    pub weight: u32,
    pub requests_per_second: Option<u32>,
}

impl Default for RpcConfig {
    fn default() -> Self {
        Self {
            endpoints: vec![],
            pool_size: 10,
            keep_alive: Duration::from_secs(30),
            max_retries: 5,
            base_delay: Duration::from_millis(100),
            max_delay: Duration::from_secs(10),
            requests_per_second: 100,
            cache_ttl: Duration::from_secs(60),
            timeout: Duration::from_secs(30),
        }
    }
}

impl RpcConfig {
    pub fn from_env() -> Result<Self, config::ConfigError> {
        let config = config::Config::builder()
            .add_source(config::Environment::with_prefix("SOLANA_RPC"))
            .build()?;

        config.try_deserialize()
    }

    pub fn from_file(path: &str) -> Result<Self, config::ConfigError> {
        let config = config::Config::builder()
            .add_source(config::File::with_name(path))
            .build()?;

        config.try_deserialize()
    }
} 