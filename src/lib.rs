//! Solana RPC Client Library
//! 
//! This library provides a robust RPC client for interacting with Solana nodes.
//! It includes features for:
//! - Rate limiting
//! - Health monitoring
//! - Error handling
//! - Retry mechanisms

pub mod core;
pub mod rpc;
pub mod models;
pub mod db;

// Re-export commonly used types
pub use core::error::{Error, Result};
pub use core::config::{Config, EndpointConfig, RetryConfig};
pub use core::health::{HealthMonitor, EndpointHealth};
pub use rpc::client::SolanaRpcClient;

// Create a prelude module for easy imports
pub mod prelude {
    pub use crate::core::error::{Error, Result};
    pub use crate::core::config::{Config, EndpointConfig, RetryConfig};
    pub use crate::core::health::{HealthMonitor, EndpointHealth};
    pub use crate::rpc::client::SolanaRpcClient;
    pub use crate::models::*;
    pub use crate::db::*;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_prelude() {
        // Ensure prelude exports are available
        let _: Result<()> = Ok(());
        let _: Config = Config::default();
        let _: HealthMonitor = HealthMonitor::new(vec![]);
    }
}
