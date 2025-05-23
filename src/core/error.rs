use thiserror::Error;
use solana_client::client_error::ClientError;
use std::time::Duration;

/// Custom error type for the Solana RPC client
#[derive(Error, Debug)]
pub enum Error {
    /// RPC request failed
    #[error("RPC error: {0}")]
    Rpc(#[from] ClientError),

    /// Configuration error
    #[error("Configuration error: {0}")]
    Config(String),

    /// Validation error
    #[error("Validation error: {0}")]
    Validation(String),

    /// Rate limit exceeded
    #[error("Rate limit exceeded after {0:?}")]
    RateLimit(Duration),

    /// Timeout error
    #[error("Timeout after {0:?}")]
    Timeout(Duration),

    /// All endpoints failed
    #[error("All endpoints failed: {0}")]
    AllEndpointsFailed(String),

    /// Invalid URL
    #[error("Invalid URL: {0}")]
    InvalidUrl(String),

    /// IO error
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    /// Serialization error
    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),
}

impl Error {
    /// Create a new configuration error
    pub fn config(msg: impl Into<String>) -> Self {
        Error::Config(msg.into())
    }

    /// Create a new validation error
    pub fn validation(msg: impl Into<String>) -> Self {
        Error::Validation(msg.into())
    }

    /// Create a new rate limit error
    pub fn rate_limit(duration: Duration) -> Self {
        Error::RateLimit(duration)
    }

    /// Create a new timeout error
    pub fn timeout(duration: Duration) -> Self {
        Error::Timeout(duration)
    }

    /// Create a new all endpoints failed error
    pub fn all_endpoints_failed(msg: impl Into<String>) -> Self {
        Error::AllEndpointsFailed(msg.into())
    }

    /// Create a new invalid URL error
    pub fn invalid_url(url: impl Into<String>) -> Self {
        Error::InvalidUrl(url.into())
    }

    /// Check if the error is retryable
    pub fn is_retryable(&self) -> bool {
        matches!(self,
            Error::Rpc(_) |
            Error::Timeout(_) |
            Error::RateLimit(_)
        )
    }
}

/// Custom result type for the Solana RPC client
pub type Result<T> = std::result::Result<T, Error>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_creation() {
        let err = Error::config("test config error");
        assert!(matches!(err, Error::Config(_)));

        let err = Error::validation("test validation error");
        assert!(matches!(err, Error::Validation(_)));

        let err = Error::rate_limit(Duration::from_secs(1));
        assert!(matches!(err, Error::RateLimit(_)));

        let err = Error::timeout(Duration::from_secs(1));
        assert!(matches!(err, Error::Timeout(_)));

        let err = Error::all_endpoints_failed("test endpoints failed");
        assert!(matches!(err, Error::AllEndpointsFailed(_)));

        let err = Error::invalid_url("invalid-url");
        assert!(matches!(err, Error::InvalidUrl(_)));
    }

    #[test]
    fn test_retryable_errors() {
        let err = Error::Rpc(ClientError::Custom("test error".to_string()));
        assert!(err.is_retryable());

        let err = Error::Timeout(Duration::from_secs(1));
        assert!(err.is_retryable());

        let err = Error::RateLimit(Duration::from_secs(1));
        assert!(err.is_retryable());

        let err = Error::Config("test".to_string());
        assert!(!err.is_retryable());
    }
} 