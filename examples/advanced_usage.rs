use solana_rpc_client::{SolanaRpcClient, RpcConfig, EndpointConfig};
use solana_sdk::pubkey::Pubkey;
use solana_sdk::signature::Keypair;
use solana_sdk::signer::Signer;
use url::Url;
use tracing_subscriber;
use std::time::Duration;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize logging
    tracing_subscriber::fmt::init();

    // Configure the client with caching enabled
    let config = RpcConfig {
        endpoints: vec![
            EndpointConfig {
                url: Url::parse("https://api.mainnet-beta.solana.com")?,
                weight: 1,
                requests_per_second: Some(100),
            },
        ],
        pool_size: 10,
        keep_alive: Duration::from_secs(30),
        max_retries: 5,
        base_delay: Duration::from_millis(100),
        max_delay: Duration::from_secs(10),
        requests_per_second: 100,
        cache_ttl: Duration::from_secs(60),
        timeout: Duration::from_secs(30),
    };

    // Create the client
    let client = SolanaRpcClient::new(config)?;

    // Generate some test pubkeys
    let pubkeys: Vec<Pubkey> = (0..5)
        .map(|_| Keypair::new().pubkey())
        .collect();

    println!("Testing batch account fetching...");
    let start = std::time::Instant::now();
    
    // Fetch multiple accounts in a single request
    let accounts = client.get_multiple_accounts(&pubkeys).await?;
    println!("Fetched {} accounts in {:?}", accounts.len(), start.elapsed());

    // Test caching behavior
    println!("\nTesting caching behavior...");
    
    // First request (cache miss)
    let start = std::time::Instant::now();
    let slot1 = client.get_slot().await?;
    println!("First request took: {:?}", start.elapsed());
    
    // Second request (should be cached)
    let start = std::time::Instant::now();
    let slot2 = client.get_slot().await?;
    println!("Second request took: {:?}", start.elapsed());
    
    assert_eq!(slot1, slot2, "Cached values should match");

    // Test error handling
    println!("\nTesting error handling...");
    
    // Test with invalid pubkey
    let invalid_pubkey = Pubkey::new_unique();
    match client.get_account_info(&invalid_pubkey, None).await {
        Ok(None) => println!("Account not found (expected)"),
        Ok(Some(_)) => println!("Unexpected: Account found"),
        Err(e) => println!("Error: {}", e),
    }

    // Test timeout handling
    println!("\nTesting timeout handling...");
    
    let config = RpcConfig {
        endpoints: vec![
            EndpointConfig {
                url: Url::parse("https://api.mainnet-beta.solana.com")?,
                weight: 1,
                requests_per_second: Some(100),
            },
        ],
        timeout: Duration::from_millis(1), // Very short timeout
        ..Default::default()
    };

    let client = SolanaRpcClient::new(config)?;
    match client.get_slot().await {
        Ok(_) => println!("Unexpected: Request succeeded"),
        Err(e) => println!("Expected: Request timed out: {}", e),
    }

    Ok(())
} 