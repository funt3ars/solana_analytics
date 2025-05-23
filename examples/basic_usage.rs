use solana_rpc_client::prelude::*;
use solana_sdk::pubkey::Pubkey;
use tracing_subscriber;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize logging
    tracing_subscriber::fmt::init();

    // Configure the client
    let config = RpcConfig {
        endpoint: EndpointConfig {
            url: "https://api.mainnet-beta.solana.com".to_string(),
            requests_per_second: 100,
        },
        retry_attempts: 5,
        retry_delay_ms: 1000,
    };

    // Create the client
    let client = Client::new(config)?;

    // Get current slot
    let slot = client.rpc_client().get_slot()?;
    println!("Current slot: {}", slot);

    // Get account info for a known address
    let pubkey = Pubkey::new_unique();
    match client.rpc_client().get_account(&pubkey)? {
        Some(account) => {
            println!("Account found:");
            println!("  Lamports: {}", account.lamports);
            println!("  Owner: {}", account.owner);
            println!("  Executable: {}", account.executable);
            println!("  Rent Epoch: {}", account.rent_epoch);
        }
        None => println!("Account not found"),
    }

    // Get health stats
    let health_stats = client.get_health_stats().await;
    println!("\nEndpoint Health Stats:");
    for stat in health_stats {
        println!("Endpoint: {}", stat.url);
        println!("  Status: {}", stat.status);
        println!("  Latency: {:?}", stat.latency);
        println!("  Last Check: {}", stat.last_check);
    }

    Ok(())
} 