use super::*;
use wiremock::{
    MockServer, Mock, ResponseTemplate,
    matchers::{method, path, body_json},
};
use solana_sdk::signature::{Keypair, Signer};
use solana_sdk::pubkey::Pubkey;
use std::time::Duration;

#[tokio::test]
async fn test_get_account_info() {
    let mock_server = MockServer::start().await;
    
    Mock::given(method("POST"))
        .and(path("/"))
        .and(body_json(serde_json::json!({
            "jsonrpc": "2.0",
            "id": 1,
            "method": "getAccountInfo",
            "params": [
                "11111111111111111111111111111111",
                {"encoding": "base64"}
            ]
        })))
        .respond_with(ResponseTemplate::new(200)
            .set_body_json(serde_json::json!({
                "jsonrpc": "2.0",
                "result": {
                    "context": {"slot": 1},
                    "value": {
                        "data": ["", "base64"],
                        "executable": false,
                        "lamports": 100,
                        "owner": "11111111111111111111111111111111",
                        "rentEpoch": 0
                    }
                },
                "id": 1
            })))
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
    let keypair = Keypair::new();
    let pubkey = keypair.pubkey();
    
    let result = client.get_account_info(&pubkey, None).await.unwrap();
    assert!(result.is_some());
}

#[tokio::test]
async fn test_retry_logic() {
    let mock_server = MockServer::start().await;
    
    // First request fails, second succeeds
    Mock::given(method("POST"))
        .and(path("/"))
        .respond_with(ResponseTemplate::new(500))
        .expect(1)
        .mount(&mock_server)
        .await;

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

    let config = RpcConfig {
        endpoints: vec![EndpointConfig {
            url: mock_server.uri().parse().unwrap(),
            weight: 1,
            requests_per_second: Some(100),
        }],
        max_retries: 3,
        base_delay: Duration::from_millis(100),
        ..Default::default()
    };

    let client = SolanaRpcClient::new(config).unwrap();
    let result = client.get_slot().await.unwrap();
    assert_eq!(result, 1);
}

#[tokio::test]
async fn test_rate_limiting() {
    let mock_server = MockServer::start().await;
    
    Mock::given(method("POST"))
        .and(path("/"))
        .respond_with(ResponseTemplate::new(200)
            .set_body_json(serde_json::json!({
                "jsonrpc": "2.0",
                "result": 1,
                "id": 1
            })))
        .expect(5)
        .mount(&mock_server)
        .await;

    let config = RpcConfig {
        endpoints: vec![EndpointConfig {
            url: mock_server.uri().parse().unwrap(),
            weight: 1,
            requests_per_second: Some(10),
        }],
        requests_per_second: 10,
        ..Default::default()
    };

    let client = SolanaRpcClient::new(config).unwrap();
    
    // Make multiple requests in quick succession
    let mut handles = vec![];
    for _ in 0..5 {
        let client = client.clone();
        handles.push(tokio::spawn(async move {
            client.get_slot().await.unwrap()
        }));
    }

    let results = futures::future::join_all(handles).await;
    for result in results {
        assert_eq!(result.unwrap(), 1);
    }
}

#[tokio::test]
async fn test_timeout_handling() {
    let mock_server = MockServer::start().await;
    
    Mock::given(method("POST"))
        .and(path("/"))
        .respond_with(ResponseTemplate::new(200)
            .set_delay(Duration::from_secs(2)))
        .mount(&mock_server)
        .await;

    let config = RpcConfig {
        endpoints: vec![EndpointConfig {
            url: mock_server.uri().parse().unwrap(),
            weight: 1,
            requests_per_second: Some(100),
        }],
        timeout: Duration::from_secs(1),
        ..Default::default()
    };

    let client = SolanaRpcClient::new(config).unwrap();
    let result = client.get_slot().await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_multiple_accounts() {
    let mock_server = MockServer::start().await;
    
    Mock::given(method("POST"))
        .and(path("/"))
        .and(body_json(serde_json::json!({
            "jsonrpc": "2.0",
            "id": 1,
            "method": "getMultipleAccounts",
            "params": [
                ["11111111111111111111111111111111", "22222222222222222222222222222222"],
                {"encoding": "base64"}
            ]
        })))
        .respond_with(ResponseTemplate::new(200)
            .set_body_json(serde_json::json!({
                "jsonrpc": "2.0",
                "result": {
                    "context": {"slot": 1},
                    "value": [
                        {
                            "data": ["", "base64"],
                            "executable": false,
                            "lamports": 100,
                            "owner": "11111111111111111111111111111111",
                            "rentEpoch": 0
                        },
                        {
                            "data": ["", "base64"],
                            "executable": false,
                            "lamports": 200,
                            "owner": "22222222222222222222222222222222",
                            "rentEpoch": 0
                        }
                    ]
                },
                "id": 1
            })))
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
    let pubkeys = vec![
        Pubkey::new_unique(),
        Pubkey::new_unique(),
    ];
    
    let result = client.get_multiple_accounts(&pubkeys).await.unwrap();
    assert_eq!(result.len(), 2);
} 