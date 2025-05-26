use thiserror::Error;
use solana_client::client_error::ClientError;

#[derive(Error, Debug)]
pub enum RpcError {
    #[error("Invalid configuration: {0}")]
    InvalidConfig(String),

    #[error("No enabled endpoints found")]
    NoEnabledEndpoints,

    #[error("Invalid endpoint index: {0}")]
    InvalidEndpoint(usize),

    #[error("Request failed: {0}")]
    RequestFailed(#[from] ClientError),

    #[error("Rate limit exceeded")]
    RateLimitExceeded,

    #[error("Request timeout")]
    Timeout,

    #[error("Retry limit exceeded")]
    RetryLimitExceeded,

    #[error("Health check failed")]
    HealthCheckFailed,

    #[error("Internal error: {0}")]
    Internal(String),

    #[error("Connection error: {0}")]
    ConnectionError(String),

    #[error("Circuit breaker open: {0}")]
    CircuitBreakerOpen(String),

    #[error("All endpoints failed: {0}")]
    AllEndpointsFailed(String),
}

impl RpcError {
    pub fn is_retryable(&self) -> bool {
        matches!(
            self,
            RpcError::RequestFailed(_) |
            RpcError::RateLimitExceeded |
            RpcError::Timeout |
            RpcError::ConnectionError(_)
        )
    }

    pub fn is_circuit_breaker(&self) -> bool {
        matches!(self, RpcError::CircuitBreakerOpen(_))
    }

    pub fn with_context(self, context: String) -> Self {
        match self {
            Self::RequestFailed(e) => Self::Internal(format!("{}: {}", context, e)),
            Self::Internal(msg) => Self::Internal(format!("{}: {}", context, msg)),
            _ => self,
        }
    }

    pub fn is_timeout(&self) -> bool {
        matches!(self, RpcError::Timeout)
    }

    pub fn is_rate_limit(&self) -> bool {
        matches!(self, RpcError::RateLimitExceeded)
    }

    pub fn is_connection_error(&self) -> bool {
        matches!(self, RpcError::ConnectionError(_))
    }
}

impl From<std::io::Error> for RpcError {
    fn from(err: std::io::Error) -> Self {
        RpcError::ConnectionError(err.to_string())
    }
}

impl From<reqwest::Error> for RpcError {
    fn from(err: reqwest::Error) -> Self {
        if err.is_timeout() {
            RpcError::Timeout
        } else if err.is_connect() {
            RpcError::ConnectionError(err.to_string())
        } else {
            RpcError::RequestFailed(ClientError::from(err))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use solana_client::client_error::ClientErrorKind;

    #[test]
    fn test_retryable_errors() {
        let retryable_errors = vec![
            RpcError::RequestFailed(ClientError::from(ClientErrorKind::Custom("test".to_string()))),
            RpcError::RateLimitExceeded,
            RpcError::Timeout,
            RpcError::ConnectionError("test".to_string()),
        ];

        for error in retryable_errors {
            assert!(error.is_retryable(), "Error should be retryable: {:?}", error);
        }
    }

    #[test]
    fn test_non_retryable_errors() {
        let non_retryable_errors = vec![
            RpcError::InvalidConfig("test".to_string()),
            RpcError::NoEnabledEndpoints,
            RpcError::InvalidEndpoint(1),
            RpcError::HealthCheckFailed,
        ];

        for error in non_retryable_errors {
            assert!(!error.is_retryable(), "Error should not be retryable: {:?}", error);
        }
    }

    #[test]
    fn test_circuit_breaker_error() {
        let error = RpcError::CircuitBreakerOpen("test".to_string());
        assert!(error.is_circuit_breaker());
        assert!(!error.is_retryable());
    }

    // #[test]
    // fn test_error_with_context() {
    //     let error = RpcError::RequestFailed(ClientError::IoError(std::io::Error::new(
    //         std::io::ErrorKind::Other,
    //         "test error",
    //     )));
    //     let error = error.with_context("test context".to_string());
    //     assert!(matches!(error, RpcError::Internal(_)));
    // }

    #[test]
    fn test_error_display() {
        let error = RpcError::InvalidConfig("test config".to_string());
        assert_eq!(error.to_string(), "Invalid configuration: test config");

        let error = RpcError::NoEnabledEndpoints;
        assert_eq!(error.to_string(), "No enabled endpoints found");

        let error = RpcError::InvalidEndpoint(1);
        assert_eq!(error.to_string(), "Invalid endpoint index: 1");
    }
} 