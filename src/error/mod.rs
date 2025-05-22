//! Error types and handling for the Solana analytics system

use thiserror::Error;
use solana_client::client_error::ClientError;
use std::time::Duration;

#[derive(Error, Debug)]
pub enum RpcClientError {
    #[error("RPC request failed: {0}")]
    RequestError(#[from] reqwest::Error),

    #[error("Solana client error: {0}")]
    SolanaError(#[from] ClientError),

    #[error("Rate limit exceeded after {0:?}")]
    RateLimitExceeded(Duration),

    #[error("All endpoints failed: {0}")]
    AllEndpointsFailed(String),

    #[error("Invalid configuration: {0}")]
    ConfigError(String),

    #[error("Timeout after {0:?}")]
    Timeout(Duration),

    #[error("Serialization error: {0}")]
    SerializationError(#[from] serde_json::Error),

    #[error("Cache error: {0}")]
    CacheError(String),

    #[error("Health check failed: {0}")]
    HealthCheckError(String),

    #[error("Request clone failed: {0}")]
    RequestCloneFailed(String),

    #[error("Endpoint unhealthy: {0}")]
    UnhealthyEndpoint(String),
}

impl RpcClientError {
    pub fn is_retryable(&self) -> bool {
        matches!(self,
            RpcClientError::RequestError(_) |
            RpcClientError::Timeout(_) |
            RpcClientError::RateLimitExceeded(_)
        )
    }
}

pub type Result<T> = std::result::Result<T, RpcClientError>; 