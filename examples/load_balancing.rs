use solana_rpc_client::{SolanaRpcClient, RpcConfig, EndpointConfig};
use url::Url;
use tracing_subscriber;
use std::time::Duration;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize logging
    tracing_subscriber::fmt::init();

    // Configure multiple endpoints with different weights
    let config = RpcConfig {
        endpoints: vec![
            EndpointConfig {
                url: Url::parse("https://api.mainnet-beta.solana.com")?,
                weight: 2,
                requests_per_second: Some(100),
            },
            EndpointConfig {
                url: Url::parse("https://solana-api.projectserum.com")?,
                weight: 1,
                requests_per_second: Some(50),
            },
        ],
        pool_size: 10,
        keep_alive: Duration::from_secs(30),
        max_retries: 5,
        base_delay: Duration::from_millis(100),
        max_delay: Duration::from_secs(10),
        requests_per_second: 150,
        cache_ttl: Duration::from_secs(60),
        timeout: Duration::from_secs(30),
    };

    // Create the client
    let client = SolanaRpcClient::new(config)?;

    // Test load balancing
    println!("Testing load balancing...");
    let start = std::time::Instant::now();
    
    // Make multiple requests to test load balancing
    let mut handles = vec![];
    for _ in 0..10 {
        let client = client.clone();
        handles.push(tokio::spawn(async move {
            client.get_slot().await
        }));
    }

    let results = futures::future::join_all(handles).await;
    println!("Made 10 concurrent requests in {:?}", start.elapsed());
    
    // Verify all requests succeeded
    for result in results {
        assert!(result.is_ok(), "All requests should succeed");
    }

    // Test failover
    println!("\nTesting failover...");
    
    // Create a client with one failing endpoint
    let config = RpcConfig {
        endpoints: vec![
            EndpointConfig {
                url: Url::parse("https://invalid-endpoint.solana.com")?,
                weight: 1,
                requests_per_second: Some(100),
            },
            EndpointConfig {
                url: Url::parse("https://api.mainnet-beta.solana.com")?,
                weight: 1,
                requests_per_second: Some(100),
            },
        ],
        ..Default::default()
    };

    let client = SolanaRpcClient::new(config)?;
    
    // Request should succeed despite one failing endpoint
    match client.get_slot().await {
        Ok(_) => println!("Successfully failed over to working endpoint"),
        Err(e) => println!("Failed to failover: {}", e),
    }

    Ok(())
} 