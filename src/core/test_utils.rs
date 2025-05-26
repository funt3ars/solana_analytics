use std::sync::Once;
use tracing_subscriber::{EnvFilter, fmt::format::FmtSpan};

use crate::core::config::Config;
use crate::core::traits::{ClientMetrics, HealthStatus, HealthDetails, SystemMetrics};

use solana_sdk::{
    pubkey::Pubkey,
    signature::Signature,
    transaction::Transaction,
};
/// Initialize test logging
pub fn init_test_logging() {
    static INIT: Once = Once::new();
    INIT.call_once(|| {
        tracing_subscriber::fmt()
            .with_env_filter(EnvFilter::from_default_env())
            .with_span_events(FmtSpan::CLOSE)
            .with_test_writer()
            .init();
    });
}

/// Create a test configuration
pub fn test_config() -> Config {
    Config::default()
}

/// Create a test transaction
pub fn test_transaction() -> Transaction {
    Transaction::default()
}

/// Create a test pubkey
pub fn test_pubkey() -> Pubkey {
    Pubkey::new_unique()
}

/// Create a test signature
pub fn test_signature() -> Signature {
    Signature::default()
}

/// Create test client metrics
pub fn test_client_metrics() -> ClientMetrics {
    ClientMetrics {
        
        successful_requests: 100,
        failed_requests: 0,
        avg_response_time_ms: 50.0,
        current_rps: 10.0,
        bytes_transferred: 1000,
    }
}

/// Create test health status
pub fn test_health_status() -> HealthStatus {
    HealthStatus::Healthy
}

/// Create test health details
pub fn test_health_details() -> HealthDetails {
    HealthDetails {
        status: test_health_status(),
        components: Vec::new(),
        metrics: test_system_metrics(),
    }
}

pub fn test_system_metrics() -> SystemMetrics {
    SystemMetrics {
        cpu_usage: 0.0,
        memory_usage: 0,
        disk_usage: 0,
        network_usage: 0,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore] // Avoids global logger collision in parallel test runs
    fn test_test_utils() {
        init_test_logging();
        let config = test_config();
        assert_eq!(config.max_concurrent_requests, 10);
        assert_eq!(config.retry_config.max_retries, 5);
    }

    #[test]
    fn test_transaction_creation() {
        let transaction = test_transaction();
        assert!(transaction.signatures.is_empty());
    }

    #[test]
    fn test_pubkey_creation() {
        let pubkey = test_pubkey();
        assert_ne!(pubkey, Pubkey::default());
    }

    #[test]
    fn test_signature_creation() {
        let signature = test_signature();
        assert_eq!(signature, Signature::default());
    }
} 
