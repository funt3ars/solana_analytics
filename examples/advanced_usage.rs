use solana_rpc_client::prelude::*;
use solana_sdk::pubkey::Pubkey;
use solana_sdk::signature::Signature;
use solana_sdk::transaction::Transaction;
use tracing_subscriber;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize logging
    tracing_subscriber::fmt::init();

    // Configure the client with multiple endpoints
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

    // Get cluster nodes
    let nodes = client.rpc_client().get_cluster_nodes()?;
    println!("Cluster Nodes:");
    for node in nodes {
        println!("  Node: {}", node.pubkey);
        println!("    Gossip: {}", node.gossip);
        println!("    Tpu: {}", node.tpu);
        println!("    RPC: {}", node.rpc);
        println!("    Version: {}", node.version);
    }

    // Get recent blockhash
    let (recent_blockhash, _) = client.rpc_client().get_latest_blockhash()?;
    println!("\nRecent Blockhash: {}", recent_blockhash);

    // Get transaction
    let signature = Signature::default(); // Replace with actual signature
    match client.rpc_client().get_transaction(&signature, solana_transaction_status::UiTransactionEncoding::Base64)? {
        Some(tx) => {
            println!("\nTransaction Details:");
            println!("  Slot: {}", tx.slot);
            println!("  Block Time: {}", tx.block_time);
            println!("  Status: {:?}", tx.transaction.meta);
        }
        None => println!("Transaction not found"),
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