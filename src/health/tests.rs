use super::*;
use wiremock::{
    MockServer, Mock, ResponseTemplate,
    matchers::{method, path},
};
use std::time::Duration;

#[tokio::test]
async fn test_health_monitoring() {
    let mock_server = MockServer::start().await;
    
    // First request succeeds, second fails
    Mock::given(method("POST"))
        .and(path("/"))
        .respond_with(ResponseTemplate::new(200)
            .set_body_json(serde_json::json!({
                "jsonrpc": "2.0",
                "result": 1,
                "id": 1
            })))
        .expect(1)
        .mount(&mock_server)
        .await;

    Mock::given(method("POST"))
        .and(path("/"))
        .respond_with(ResponseTemplate::new(500))
        .expect(1)
        .mount(&mock_server)
        .await;

    let config = RpcConfig {
        endpoints: vec![EndpointConfig {
            url: mock_server.uri().parse().unwrap(),
            weight: 1,
            requests_per_second: Some(100),
        }],
        ..Default::default()
    };

    let client = SolanaRpcClient::new(config).unwrap();
    
    // First request should succeed
    let result = client.get_slot().await.unwrap();
    assert_eq!(result, 1);

    // Second request should fail
    let result = client.get_slot().await;
    assert!(result.is_err());

    // Check health stats
    let stats = client.get_health_stats().await;
    assert_eq!(stats.len(), 1);
    assert_eq!(stats[0].success_count, 1);
    assert_eq!(stats[0].error_count, 1);
}

#[tokio::test]
async fn test_endpoint_failover() {
    let mock_server1 = MockServer::start().await;
    let mock_server2 = MockServer::start().await;
    
    // First endpoint fails
    Mock::given(method("POST"))
        .and(path("/"))
        .respond_with(ResponseTemplate::new(500))
        .expect(1)
        .mount(&mock_server1)
        .await;

    // Second endpoint succeeds
    Mock::given(method("POST"))
        .and(path("/"))
        .respond_with(ResponseTemplate::new(200)
            .set_body_json(serde_json::json!({
                "jsonrpc": "2.0",
                "result": 1,
                "id": 1
            })))
        .expect(1)
        .mount(&mock_server2)
        .await;

    let config = RpcConfig {
        endpoints: vec![
            EndpointConfig {
                url: mock_server1.uri().parse().unwrap(),
                weight: 1,
                requests_per_second: Some(100),
            },
            EndpointConfig {
                url: mock_server2.uri().parse().unwrap(),
                weight: 1,
                requests_per_second: Some(100),
            },
        ],
        ..Default::default()
    };

    let client = SolanaRpcClient::new(config).unwrap();
    
    // Request should succeed using the second endpoint
    let result = client.get_slot().await.unwrap();
    assert_eq!(result, 1);

    // Check health stats
    let stats = client.get_health_stats().await;
    assert_eq!(stats.len(), 2);
    assert!(!stats[0].is_healthy);
    assert!(stats[1].is_healthy);
} 