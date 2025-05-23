use std::sync::Once;
use tokio_postgres::NoTls;
use deadpool_postgres::{Pool, Manager};
use solana_rpc_client::core::test_utils::init_test_logging;

static INIT: Once = Once::new();

/// Initialize test environment
pub fn init() {
    INIT.call_once(|| {
        init_test_logging();
    });
}

/// Create a test database pool
pub async fn create_test_pool() -> Pool {
    let mut cfg = tokio_postgres::Config::new();
    cfg.host("localhost")
        .port(5432)
        .user("postgres")
        .password("postgres")
        .dbname("solana_test");

    let mgr = Manager::new(cfg, NoTls);
    Pool::new(mgr, 16)
}

/// Create test tables
pub async fn create_test_tables(pool: &Pool) -> Result<(), tokio_postgres::Error> {
    let client = pool.get().await?;
    
    // Create transactions table
    client.batch_execute("
        CREATE TABLE IF NOT EXISTS transactions (
            signature TEXT PRIMARY KEY,
            slot BIGINT NOT NULL,
            block_time TIMESTAMP WITH TIME ZONE NOT NULL,
            fee BIGINT NOT NULL,
            status TEXT NOT NULL,
            instructions_json JSONB NOT NULL,
            created_at TIMESTAMP WITH TIME ZONE NOT NULL
        );
    ").await?;

    // Create token accounts table
    client.batch_execute("
        CREATE TABLE IF NOT EXISTS token_accounts (
            address TEXT PRIMARY KEY,
            owner TEXT NOT NULL,
            mint TEXT NOT NULL,
            amount BIGINT NOT NULL,
            created_at TIMESTAMP WITH TIME ZONE NOT NULL,
            updated_at TIMESTAMP WITH TIME ZONE NOT NULL
        );
    ").await?;

    Ok(())
}

/// Drop test tables
pub async fn drop_test_tables(pool: &Pool) -> Result<(), tokio_postgres::Error> {
    let client = pool.get().await?;
    
    client.batch_execute("
        DROP TABLE IF EXISTS transactions;
        DROP TABLE IF EXISTS token_accounts;
    ").await?;

    Ok(())
}

/// Create test data
pub async fn create_test_data(pool: &Pool) -> Result<(), tokio_postgres::Error> {
    let client = pool.get().await?;
    
    // Insert test transaction
    client.execute("
        INSERT INTO transactions (signature, slot, block_time, fee, status, instructions_json, created_at)
        VALUES ($1, $2, $3, $4, $5, $6, $7)
    ", &[
        &"test_signature",
        &1i64,
        &chrono::Utc::now(),
        &5000i64,
        &"success",
        &serde_json::json!({}),
        &chrono::Utc::now(),
    ]).await?;

    // Insert test token account
    client.execute("
        INSERT INTO token_accounts (address, owner, mint, amount, created_at, updated_at)
        VALUES ($1, $2, $3, $4, $5, $6)
    ", &[
        &"test_address",
        &"test_owner",
        &"test_mint",
        &1000i64,
        &chrono::Utc::now(),
        &chrono::Utc::now(),
    ]).await?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_fixtures() {
        init();
        let pool = create_test_pool().await;
        assert!(create_test_tables(&pool).await.is_ok());
        assert!(create_test_data(&pool).await.is_ok());
        assert!(drop_test_tables(&pool).await.is_ok());
    }
} 