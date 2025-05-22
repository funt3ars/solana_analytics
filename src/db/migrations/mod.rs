use deadpool_postgres::{Pool};
use crate::db::DatabaseError;

pub trait Migration {
    fn name(&self) -> &str;
    fn sql(&self) -> &str;
    fn cleanup(&self) -> &str {
        ""
    }
}

pub fn get_migrations() -> Vec<Box<dyn Migration>> {
    vec![
        Box::new(TransactionsMigration),
        Box::new(TokenAccountsMigration),
        Box::new(PriceHistoryMigration),
        Box::new(ProtocolInteractionsMigration),
        Box::new(GovernanceVotesMigration),
    ]
}

pub struct TransactionsMigration;

impl Migration for TransactionsMigration {
    fn name(&self) -> &str {
        "create_transactions_table"
    }

    fn sql(&self) -> &str {
        r#"
        CREATE TABLE IF NOT EXISTS transactions (
            signature VARCHAR(88) PRIMARY KEY,
            slot BIGINT NOT NULL,
            block_time TIMESTAMP WITH TIME ZONE NOT NULL,
            fee BIGINT NOT NULL,
            status VARCHAR(20) NOT NULL,
            instructions_json JSONB NOT NULL,
            created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
        )
        "#
    }

    fn cleanup(&self) -> &str {
        "DROP TABLE IF EXISTS transactions CASCADE"
    }
}

pub struct TokenAccountsMigration;

impl Migration for TokenAccountsMigration {
    fn name(&self) -> &str {
        "create_token_accounts_table"
    }

    fn sql(&self) -> &str {
        r#"
        CREATE TABLE IF NOT EXISTS token_accounts (
            pubkey VARCHAR(44) PRIMARY KEY,
            mint VARCHAR(44) NOT NULL,
            owner VARCHAR(44) NOT NULL,
            amount BIGINT NOT NULL,
            updated_at TIMESTAMP WITH TIME ZONE NOT NULL,
            created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
        )
        "#
    }

    fn cleanup(&self) -> &str {
        "DROP TABLE IF EXISTS token_accounts CASCADE"
    }
}

pub struct PriceHistoryMigration;

impl Migration for PriceHistoryMigration {
    fn name(&self) -> &str {
        "create_price_history_table"
    }

    fn sql(&self) -> &str {
        r#"
        CREATE TABLE IF NOT EXISTS price_history (
            id UUID PRIMARY KEY,
            token_mint VARCHAR(44) NOT NULL,
            price_usd DOUBLE PRECISION NOT NULL,
            timestamp TIMESTAMP WITH TIME ZONE NOT NULL,
            source VARCHAR(50) NOT NULL,
            created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
        )
        "#
    }

    fn cleanup(&self) -> &str {
        "DROP TABLE IF EXISTS price_history CASCADE"
    }
}

pub struct ProtocolInteractionsMigration;

impl Migration for ProtocolInteractionsMigration {
    fn name(&self) -> &str {
        "create_protocol_interactions_table"
    }

    fn sql(&self) -> &str {
        r#"
        CREATE TABLE IF NOT EXISTS protocol_interactions (
            id UUID PRIMARY KEY,
            wallet VARCHAR(44) NOT NULL,
            protocol VARCHAR(50) NOT NULL,
            interaction_type VARCHAR(50) NOT NULL,
            amount DOUBLE PRECISION NOT NULL,
            timestamp TIMESTAMP WITH TIME ZONE NOT NULL,
            created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
        )
        "#
    }

    fn cleanup(&self) -> &str {
        "DROP TABLE IF EXISTS protocol_interactions CASCADE"
    }
}

pub struct GovernanceVotesMigration;

impl Migration for GovernanceVotesMigration {
    fn name(&self) -> &str {
        "create_governance_votes_table"
    }

    fn sql(&self) -> &str {
        r#"
        CREATE TABLE IF NOT EXISTS governance_votes (
            id UUID PRIMARY KEY,
            voter VARCHAR(44) NOT NULL,
            proposal_id VARCHAR(50) NOT NULL,
            vote VARCHAR(20) NOT NULL,
            timestamp TIMESTAMP WITH TIME ZONE NOT NULL,
            dao_name VARCHAR(50) NOT NULL,
            created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
        )
        "#
    }

    fn cleanup(&self) -> &str {
        "DROP TABLE IF EXISTS governance_votes CASCADE"
    }
}

pub async fn run_migrations(pool: &Pool) -> Result<(), DatabaseError> {
    let mut client = pool.get().await.map_err(DatabaseError::ConnectionError)?;
    
    // Create migrations table if it doesn't exist
    client.execute(
        "CREATE TABLE IF NOT EXISTS migrations (
            id SERIAL PRIMARY KEY,
            name VARCHAR(255) NOT NULL UNIQUE,
            applied_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
        )",
        &[],
    ).await.map_err(DatabaseError::QueryError)?;

    // Get list of applied migrations
    let applied = client
        .query("SELECT name FROM migrations ORDER BY id", &[])
        .await
        .map_err(DatabaseError::QueryError)?
        .iter()
        .map(|row| row.get::<_, String>("name"))
        .collect::<Vec<String>>();

    // Run migrations
    for migration in get_migrations() {
        let name = migration.name();
        
        if !applied.contains(&name.to_string()) {
            // Start transaction
            let transaction = client.transaction().await.map_err(DatabaseError::QueryError)?;

            // Clean up existing tables if they exist
            if !migration.cleanup().is_empty() {
                transaction.execute(migration.cleanup(), &[])
                    .await
                    .map_err(DatabaseError::QueryError)?;
            }

            // Execute migration
            transaction.execute(migration.sql(), &[])
                .await
                .map_err(|e| DatabaseError::MigrationError(format!("Failed to run migration {}: {}", name, e)))?;

            // Create indexes
            match name {
                "create_transactions_table" => {
                    transaction.execute(
                        "CREATE INDEX IF NOT EXISTS idx_transactions_slot ON transactions(slot)",
                        &[],
                    ).await.map_err(DatabaseError::QueryError)?;
                    
                    transaction.execute(
                        "CREATE INDEX IF NOT EXISTS idx_transactions_block_time ON transactions(block_time)",
                        &[],
                    ).await.map_err(DatabaseError::QueryError)?;
                },
                "create_token_accounts_table" => {
                    transaction.execute(
                        "CREATE INDEX IF NOT EXISTS idx_token_accounts_mint ON token_accounts(mint)",
                        &[],
                    ).await.map_err(DatabaseError::QueryError)?;
                    
                    transaction.execute(
                        "CREATE INDEX IF NOT EXISTS idx_token_accounts_owner ON token_accounts(owner)",
                        &[],
                    ).await.map_err(DatabaseError::QueryError)?;
                },
                "create_price_history_table" => {
                    transaction.execute(
                        "CREATE INDEX IF NOT EXISTS idx_price_history_token_mint ON price_history(token_mint)",
                        &[],
                    ).await.map_err(DatabaseError::QueryError)?;
                    
                    transaction.execute(
                        "CREATE INDEX IF NOT EXISTS idx_price_history_timestamp ON price_history(timestamp)",
                        &[],
                    ).await.map_err(DatabaseError::QueryError)?;
                },
                "create_protocol_interactions_table" => {
                    transaction.execute(
                        "CREATE INDEX IF NOT EXISTS idx_protocol_interactions_wallet ON protocol_interactions(wallet)",
                        &[],
                    ).await.map_err(DatabaseError::QueryError)?;
                    
                    transaction.execute(
                        "CREATE INDEX IF NOT EXISTS idx_protocol_interactions_protocol ON protocol_interactions(protocol)",
                        &[],
                    ).await.map_err(DatabaseError::QueryError)?;
                    
                    transaction.execute(
                        "CREATE INDEX IF NOT EXISTS idx_protocol_interactions_timestamp ON protocol_interactions(timestamp)",
                        &[],
                    ).await.map_err(DatabaseError::QueryError)?;
                },
                "create_governance_votes_table" => {
                    transaction.execute(
                        "CREATE INDEX IF NOT EXISTS idx_governance_votes_voter ON governance_votes(voter)",
                        &[],
                    ).await.map_err(DatabaseError::QueryError)?;
                    
                    transaction.execute(
                        "CREATE INDEX IF NOT EXISTS idx_governance_votes_proposal_id ON governance_votes(proposal_id)",
                        &[],
                    ).await.map_err(DatabaseError::QueryError)?;
                    
                    transaction.execute(
                        "CREATE INDEX IF NOT EXISTS idx_governance_votes_timestamp ON governance_votes(timestamp)",
                        &[],
                    ).await.map_err(DatabaseError::QueryError)?;
                },
                _ => {}
            }

            // Record migration
            transaction.execute(
                "INSERT INTO migrations (name) VALUES ($1)",
                &[&name],
            ).await.map_err(DatabaseError::QueryError)?;

            // Commit transaction
            transaction.commit().await.map_err(DatabaseError::QueryError)?;
        }
    }

    Ok(())
}

pub async fn create_database_if_not_exists(
    host: &str,
    port: u16,
    username: &str,
    password: &str,
    database: &str,
) -> Result<(), DatabaseError> {
    let mut config = tokio_postgres::Config::new();
    config
        .host(host)
        .port(port)
        .user(username)
        .password(password)
        .dbname("postgres");

    let (client, connection) = config.connect(tokio_postgres::NoTls)
        .await
        .map_err(DatabaseError::QueryError)?;

    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("Connection error: {}", e);
        }
    });

    // Check if database exists
    let exists = client
        .query_opt(
            "SELECT 1 FROM pg_database WHERE datname = $1",
            &[&database],
        )
        .await
        .map_err(DatabaseError::QueryError)?
        .is_some();

    if !exists {
        // Create database
        client.execute(
            &format!("CREATE DATABASE {}", database),
            &[],
        ).await.map_err(DatabaseError::QueryError)?;
    }

    Ok(())
}

#[cfg(test)]
mod tests; 