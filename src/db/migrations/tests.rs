use crate::db::{Database, DatabaseConfig};
use std::time::Duration;

async fn recreate_test_database(
    host: &str,
    port: u16,
    username: &str,
    password: &str,
    database: &str,
) -> Result<(), tokio_postgres::Error> {
    // Connect to postgres database
    let mut config = tokio_postgres::Config::new();
    config
        .host(host)
        .port(port)
        .user(username)
        .password(password)
        .dbname("postgres");

    let (client, connection) = config.connect(tokio_postgres::NoTls).await?;
    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("Connection error: {}", e);
        }
    });

    // Terminate existing connections
    client.execute(
        &format!("SELECT pg_terminate_backend(pid) FROM pg_stat_activity WHERE datname = '{}'", database),
        &[],
    ).await?;

    // Drop database if exists
    client.execute(
        &format!("DROP DATABASE IF EXISTS {}", database),
        &[],
    ).await?;

    // Create fresh database
    client.execute(
        &format!("CREATE DATABASE {}", database),
        &[],
    ).await?;

    Ok(())
}

#[tokio::test]
async fn test_migrations() {
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

    // Recreate test database
    let result = recreate_test_database(
        &config.host,
        config.port,
        &config.username,
        &config.password,
        &config.database,
    ).await;
    assert!(result.is_ok(), "Failed to recreate test database: {:?}", result);

    // Initialize database
    let db = Database::new(config).await.unwrap();
    
    // Run migrations
    let result = db.run_migrations().await;
    assert!(result.is_ok(), "Failed to run migrations: {:?}", result);

    // Verify migrations table exists
    let client = db.get_client().await.unwrap();
    let row = client.query_one(
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
        let row = client.query_one(
            &format!("SELECT EXISTS (SELECT FROM information_schema.tables WHERE table_name = $1)"),
            &[&table],
        ).await.unwrap();
        let exists: bool = row.get(0);
        assert!(exists, "Table {} does not exist", table);
    }
} 