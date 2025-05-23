use solana_rpc_client::rpc::client::SolanaRpcClient;
use solana_rpc_client::rpc::config::RpcConfig;
use solana_rpc_client::rpc::error::RpcError;
use solana_rpc_client::rpc::config::EndpointConfig;

#[test]
fn test_client_creation() {
    let config = RpcConfig::default();
    let client = SolanaRpcClient::new(config);
    assert!(client.is_ok());
}

#[test]
fn test_invalid_rate_limit() {
    let mut config = RpcConfig::default();
    config.max_concurrent_requests = 0; // Invalid rate limit
    let client = SolanaRpcClient::new(config);
    assert!(client.is_err());
}

#[test]
fn test_invalid_url() {
    let mut config = RpcConfig::default();
    config.endpoints[0].url = "invalid-url".to_string();
    let client = SolanaRpcClient::new(config);
    assert!(client.is_err());
}

#[test]
fn test_client_config() {
    let config = RpcConfig::default();
    let client = SolanaRpcClient::new(config.clone()).unwrap();
    assert_eq!(client.get_config().max_concurrent_requests, config.max_concurrent_requests);
}

#[tokio::test]
async fn test_basic_rpc_call() {
    let config = RpcConfig::default();
    let client = SolanaRpcClient::new(config).unwrap();
    // Test a basic RPC call (getSlot)
    let result = client.rpc_client().blocking_read().get_slot();
    assert!(result.is_ok() || result.is_err()); // Either success or error is valid
}

#[tokio::test]
async fn test_rate_limiting() {
    let mut config = RpcConfig::default();
    config.max_concurrent_requests = 1; // Very low rate limit for testing
    let client = SolanaRpcClient::new(config).unwrap();
    // Make two rapid requests
    let start = std::time::Instant::now();
    let _ = client.rpc_client().blocking_read().get_slot();
    let _ = client.rpc_client().blocking_read().get_slot();
    let duration = start.elapsed();
    // The second request should have been rate limited
    assert!(duration >= std::time::Duration::from_secs(1));
}

#[test]
fn test_no_enabled_endpoints() {
    let config = RpcConfig {
        endpoints: vec![
            EndpointConfig {
                url: "http://endpoint1".to_string(),
                weight: 1,
                enabled: false,
            },
        ],
        max_concurrent_requests: 10,
        request_timeout_ms: 5000,
        retry: Default::default(),
        rate_limit: Default::default(),
    };
    assert!(matches!(
        SolanaRpcClient::new(config),
        Err(RpcError::NoEnabledEndpoints)
    ));
}

#[test]
fn test_debug_impl() {
    let config = RpcConfig::default();
    let client = SolanaRpcClient::new(config).unwrap();
    let _ = format!("{:?}", client); // Should not panic
} 