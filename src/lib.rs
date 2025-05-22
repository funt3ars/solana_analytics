//! Solana RPC Client with Database Integration
//! 
//! A robust Solana RPC client with connection pooling, retry logic, health monitoring,
//! and PostgreSQL database integration for analytics.
//! 
//! # Features
//! 
//! - **RPC Client**: Robust Solana RPC client with connection pooling and retry logic
//! - **Health Monitoring**: Endpoint health tracking and automatic failover
//! - **Database Integration**: PostgreSQL integration for analytics data
//! - **Rate Limiting**: Configurable rate limiting per endpoint
//! - **Metrics**: Prometheus metrics for monitoring
//! 
//! # Example
//! 
//! ```rust,no_run
//! use solana_rpc_client::{
//!     SolanaRpcClient,
//!     RpcConfig,
//!     EndpointConfig,
//!     Result,
//! };
//! use url::Url;
//! use std::time::Duration;
//! 
//! #[tokio::main]
//! async fn main() -> Result<()> {
//!     let config = RpcConfig {
//!         endpoints: vec![EndpointConfig {
//!             url: Url::parse("https://api.mainnet-beta.solana.com").unwrap(),
//!             weight: 1,
//!             requests_per_second: Some(100),
//!         }],
//!         pool_size: 10,
//!         keep_alive: Duration::from_secs(30),
//!         max_retries: 5,
//!         base_delay: Duration::from_millis(100),
//!         max_delay: Duration::from_secs(10),
//!         requests_per_second: 100,
//!         cache_ttl: Duration::from_secs(60),
//!         timeout: Duration::from_secs(30),
//!     };
//! 
//!     let client = SolanaRpcClient::new(config)?;
//!     let slot = client.get_slot().await?;
//!     println!("Current slot: {}", slot);
//! 
//!     Ok(())
//! }
//! ```
//! 
//! # Modules
//! 
//! - [`config`]: Configuration types and loading
//! - [`db`]: Database layer with migrations
//! - [`error`]: Error types and handling
//! - [`health`]: Health monitoring for RPC endpoints
//! - [`models`]: Data models for analytics
//! - [`rpc`]: RPC client implementation
//! - [`utils`]: Common utility functions

pub mod config;
pub mod db;
pub mod error;
pub mod health;
pub mod models;
pub mod rpc;
pub mod utils;

// Re-export commonly used types
pub use config::{EndpointConfig, RpcConfig};
pub use error::{RpcClientError, Result};
pub use rpc::SolanaRpcClient;
pub use db::Database;

// Re-export model types
pub use models::{
    Transaction,
    TokenAccount,
    PriceHistory,
    ProtocolInteraction,
    GovernanceVote,
};

// Re-export health types
pub use health::{HealthMonitor, EndpointHealth};

// Re-export utility functions
pub use utils::{
    pubkey_from_str,
    format_timestamp,
    get_last_n_days,
    format_sol_amount,
};

#[cfg(test)]
mod tests {
    use crate::db::{Database, DatabaseConfig};
    use std::time::Duration;

    #[tokio::test]
    async fn test_library_integration() {
        // Configure test database
        let config = DatabaseConfig {
            host: "localhost".to_string(),
            port: 5432,
            username: "postgres".to_string(),
            password: "postgres".to_string(),
            database: "solana_analytics_test".to_string(),
            max_connections: 5,
            connection_timeout: Duration::from_secs(5),
        };

        // Initialize database
        let db = Database::new(config).await;
        assert!(db.is_ok(), "Failed to initialize database: {:?}", db);
        let db = db.unwrap();

        // Run migrations first
        let result = db.run_migrations().await;
        assert!(result.is_ok(), "Failed to run migrations: {:?}", result);

        // Get a client and start transaction for verification
        let mut client = db.get_client().await.unwrap();
        let transaction = client.transaction().await.unwrap();

        // Verify migrations table exists
        let row = transaction.query_one(
            "SELECT COUNT(*) FROM migrations",
            &[],
        ).await.unwrap();
        let count: i64 = row.get(0);
        assert!(count > 0, "No migrations were applied");

        // Verify all required tables exist
        let tables = vec![
            "transactions",
            "token_accounts",
            "price_history",
            "protocol_interactions",
            "governance_votes",
        ];

        for table in tables {
            let row = transaction.query_one(
                "SELECT EXISTS (SELECT FROM information_schema.tables WHERE table_name = $1)",
                &[&table],
            ).await.unwrap();
            let exists: bool = row.get(0);
            assert!(exists, "Table {} does not exist", table);
        }

        // Commit transaction
        transaction.commit().await.unwrap();
    }
}
