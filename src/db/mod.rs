use std::time::Duration;
use deadpool_postgres::{Pool, Manager};
use tokio_postgres::Error as PostgresError;
use thiserror::Error;

pub mod migrations;
pub mod models;

#[derive(Debug, Error)]
pub enum DatabaseError {
    #[error("Connection error: {0}")]
    ConnectionError(#[from] deadpool_postgres::PoolError),
    #[error("Build error: {0}")]
    BuildError(#[from] deadpool_postgres::BuildError),
    #[error("Query error: {0}")]
    QueryError(#[from] PostgresError),
    #[error("Migration error: {0}")]
    MigrationError(String),
}

pub type Result<T> = std::result::Result<T, DatabaseError>;

#[derive(Debug, Clone)]
pub struct DatabaseConfig {
    pub host: String,
    pub port: u16,
    pub username: String,
    pub password: String,
    pub database: String,
    pub max_connections: u32,
    pub connection_timeout: Duration,
}

impl Default for DatabaseConfig {
    fn default() -> Self {
        Self {
            host: "localhost".to_string(),
            port: 5432,
            username: "postgres".to_string(),
            password: "postgres".to_string(),
            database: "solana_analytics".to_string(),
            max_connections: 10,
            connection_timeout: Duration::from_secs(30),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Database {
    pool: Pool,
}

impl Database {
    pub async fn new(config: DatabaseConfig) -> Result<Self> {
        let mut pg_config = tokio_postgres::Config::new();
        pg_config
            .host(&config.host)
            .port(config.port)
            .user(&config.username)
            .password(&config.password)
            .dbname(&config.database);

        let mgr = Manager::new(pg_config, tokio_postgres::NoTls);
        let pool = Pool::builder(mgr)
            .max_size(config.max_connections as usize)
            .build()
            .map_err(DatabaseError::BuildError)?;

        Ok(Self { pool })
    }

    pub async fn get_client(&self) -> Result<deadpool_postgres::Client> {
        self.pool.get().await.map_err(DatabaseError::ConnectionError)
    }

    pub async fn run_migrations(&self) -> Result<()> {
        let client = self.get_client().await?;
        
        // Create migrations table if it doesn't exist
        client.execute(
            "CREATE TABLE IF NOT EXISTS migrations (
                id SERIAL PRIMARY KEY,
                name VARCHAR(255) NOT NULL,
                applied_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
            )",
            &[],
        ).await.map_err(DatabaseError::QueryError)?;

        // Run migrations
        for migration in crate::db::migrations::get_migrations() {
            let name = migration.name();
            
            // Check if migration has been applied
            let row = client.query_opt(
                "SELECT id FROM migrations WHERE name = $1",
                &[&name],
            ).await.map_err(DatabaseError::QueryError)?;

            if row.is_none() {
                // Run migration
                client.execute(migration.sql(), &[])
                    .await
                    .map_err(|e| DatabaseError::MigrationError(format!("Failed to run migration {}: {}", name, e)))?;

                // Record migration
                client.execute(
                    "INSERT INTO migrations (name) VALUES ($1)",
                    &[&name],
                ).await.map_err(DatabaseError::QueryError)?;
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_database_connection() {
        let config = DatabaseConfig::default();
        let db = Database::new(config).await;
        assert!(db.is_ok());
    }
} 