use solana_rpc_client::{SolanaRpcClient, RpcConfig, EndpointConfig};
use solana_sdk::pubkey::Pubkey;
use url::Url;
use tracing_subscriber;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize logging
    tracing_subscriber::fmt::init();

    // Configure the client
    let config = RpcConfig {
        endpoints: vec![
            EndpointConfig {
                url: Url::parse("https://api.mainnet-beta.solana.com")?,
                weight: 1,
                requests_per_second: Some(100),
            },
        ],
        pool_size: 10,
        keep_alive: std::time::Duration::from_secs(30),
        max_retries: 5,
        base_delay: std::time::Duration::from_millis(100),
        max_delay: std::time::Duration::from_secs(10),
        requests_per_second: 100,
        cache_ttl: std::time::Duration::from_secs(60),
        timeout: std::time::Duration::from_secs(30),
    };

    // Create the client
    let client = SolanaRpcClient::new(config)?;

    // Get current slot
    let slot = client.get_slot().await?;
    println!("Current slot: {}", slot);

    // Get account info for a known address
    let pubkey = Pubkey::new_unique();
    match client.get_account_info(&pubkey, None).await? {
        Some(account) => {
            println!("Account found:");
            println!("  Lamports: {}", account.account.lamports);
            println!("  Owner: {}", account.account.owner);
            println!("  Executable: {}", account.account.executable);
            println!("  Rent Epoch: {}", account.account.rent_epoch);
        }
        None => println!("Account not found"),
    }

    // Get transaction signatures for an address
    let signatures = client.get_signatures_for_address(&pubkey, None).await?;
    println!("Found {} signatures", signatures.len());
    for sig in signatures.iter().take(5) {
        println!("  Signature: {}", sig.signature);
        println!("  Slot: {}", sig.slot);
        println!("  Status: {:?}", sig.err);
    }

    // Get health stats
    let health_stats = client.get_health_stats().await;
    println!("\nEndpoint Health Stats:");
    for stat in health_stats {
        println!("Endpoint: {}", stat.url);
        println!("  Health Score: {:.2}", stat.health_score());
        println!("  Success Rate: {}/{}", stat.success_count, stat.success_count + stat.error_count);
        println!("  Average Response Time: {:?}", stat.response_time);
        println!("  Is Healthy: {}", stat.is_healthy);
        if let Some(error) = stat.last_error {
            println!("  Last Error: {}", error);
        }
    }

    Ok(())
} 