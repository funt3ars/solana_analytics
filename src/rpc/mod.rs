use std::sync::Arc;
use std::time::{Duration, Instant};
use std::num::NonZeroU32;
use reqwest::Client;
use solana_client::rpc_config::{RpcAccountInfoConfig, RpcSignaturesForAddressConfig};
use solana_client::rpc_response::{
    Response as RpcResponse,
    RpcKeyedAccount,
    RpcConfirmedTransactionStatusWithSignature,
};
use solana_sdk::pubkey::Pubkey;
use solana_sdk::signature::Signature;
use tokio_retry::{
    strategy::{jitter, ExponentialBackoff},
    Retry,
};
use tracing::instrument;
use url::Url;
use governor::{
    Quota, RateLimiter,
    state::{InMemoryState, NotKeyed},
    clock::QuantaClock,
};
use serde_json::Value;

use crate::config::RpcConfig;
use crate::error::RpcClientError;
use crate::health::HealthMonitor;

pub type Result<T> = std::result::Result<T, RpcClientError>;

/// A robust Solana RPC client with connection pooling, retry logic, and health monitoring.
#[derive(Clone)]
pub struct SolanaRpcClient {
    config: Arc<RpcConfig>,
    client: Client,
    health_monitor: HealthMonitor,
    rate_limiter: Arc<RateLimiter<NotKeyed, InMemoryState, QuantaClock>>,
}

impl SolanaRpcClient {
    /// Creates a new Solana RPC client with the specified configuration.
    pub fn new(config: RpcConfig) -> Result<Self> {
        let client = Client::builder()
            .pool_idle_timeout(config.keep_alive)
            .pool_max_idle_per_host(config.pool_size)
            .build()
            .map_err(RpcClientError::RequestError)?;

        let endpoints: Vec<Url> = config.endpoints.iter().map(|e| e.url.clone()).collect();
        let health_monitor = HealthMonitor::new(endpoints);

        let quota = Quota::with_period(Duration::from_secs(1))
            .unwrap()
            .allow_burst(NonZeroU32::new(config.requests_per_second).unwrap());
        let rate_limiter = Arc::new(RateLimiter::direct(quota));

        Ok(Self {
            config: Arc::new(config),
            client,
            health_monitor,
            rate_limiter,
        })
    }

    /// Makes an RPC request with retry logic and health monitoring.
    #[instrument(skip(self))]
    async fn make_request<T: serde::de::DeserializeOwned>(
        &self,
        endpoint: &Url,
        method: &str,
        params: Value,
    ) -> Result<T> {
        let start = Instant::now();
        
        tokio::time::timeout(self.config.timeout, self.rate_limiter.until_ready())
            .await
            .map_err(|_| RpcClientError::Timeout(self.config.timeout))?;

        let request = self.client
            .post(endpoint.clone())
            .json(&serde_json::json!({
                "jsonrpc": "2.0",
                "id": 1,
                "method": method,
                "params": params,
            }))
            .timeout(self.config.timeout);

        let retry_strategy = ExponentialBackoff::from_millis(self.config.base_delay.as_millis() as u64)
            .max_delay(self.config.max_delay)
            .map(jitter);

        let response = Retry::spawn(retry_strategy, || async {
            match request.try_clone() {
                Some(req) => {
                    let response = req.send().await?;
                    let response_time = start.elapsed();
                    
                    if response.status().is_success() {
                        self.health_monitor.record_success(endpoint, response_time).await;
                        Ok(response)
                    } else {
                        let error = format!("HTTP error: {}", response.status());
                        self.health_monitor.record_error(endpoint, error.clone()).await;
                        Err(RpcClientError::RequestError(response.error_for_status().unwrap_err()))
                    }
                }
                None => {
                    let error = "Request clone failed".to_string();
                    self.health_monitor.record_error(endpoint, error.clone()).await;
                    Err(RpcClientError::RequestCloneFailed(error))
                }
            }
        })
        .await?;

        let response: RpcResponse<T> = response.json().await?;
        Ok(response.value)
    }

    /// Gets account information for a specified public key.
    #[instrument(skip(self))]
    pub async fn get_account_info(
        &self,
        pubkey: &Pubkey,
        config: Option<RpcAccountInfoConfig>,
    ) -> Result<Option<RpcKeyedAccount>> {
        let endpoint = self.health_monitor.get_healthiest_endpoint().await
            .ok_or_else(|| RpcClientError::AllEndpointsFailed("No healthy endpoints available".to_string()))?;

        let params = serde_json::json!([
            pubkey.to_string(),
            config.unwrap_or_default(),
        ]);

        self.make_request(&endpoint, "getAccountInfo", params).await
    }

    /// Gets signatures for a specified address.
    #[instrument(skip(self))]
    pub async fn get_signatures_for_address(
        &self,
        address: &Pubkey,
        config: Option<RpcSignaturesForAddressConfig>,
    ) -> Result<Vec<RpcConfirmedTransactionStatusWithSignature>> {
        let endpoint = self.health_monitor.get_healthiest_endpoint().await
            .ok_or_else(|| RpcClientError::AllEndpointsFailed("No healthy endpoints available".to_string()))?;

        let params = serde_json::json!([
            address.to_string(),
            config.unwrap_or_default(),
        ]);

        self.make_request(&endpoint, "getSignaturesForAddress", params).await
    }

    /// Gets transaction information for a specified signature.
    #[instrument(skip(self))]
    pub async fn get_transaction(
        &self,
        signature: &Signature,
    ) -> Result<Option<Value>> {
        let endpoint = self.health_monitor.get_healthiest_endpoint().await
            .ok_or_else(|| RpcClientError::AllEndpointsFailed("No healthy endpoints available".to_string()))?;

        let params = serde_json::json!([
            signature.to_string(),
            {"encoding": "json", "maxSupportedTransactionVersion": 0},
        ]);

        self.make_request(&endpoint, "getTransaction", params).await
    }

    /// Gets account information for multiple public keys.
    #[instrument(skip(self))]
    pub async fn get_multiple_accounts(
        &self,
        pubkeys: &[Pubkey],
    ) -> Result<Vec<Option<RpcKeyedAccount>>> {
        let endpoint = self.health_monitor.get_healthiest_endpoint().await
            .ok_or_else(|| RpcClientError::AllEndpointsFailed("No healthy endpoints available".to_string()))?;

        let params = serde_json::json!([
            pubkeys.iter().map(|p| p.to_string()).collect::<Vec<_>>(),
            {"encoding": "base64"},
        ]);

        self.make_request(&endpoint, "getMultipleAccounts", params).await
    }

    /// Gets the current slot.
    #[instrument(skip(self))]
    pub async fn get_slot(&self) -> Result<u64> {
        let endpoint = self.health_monitor.get_healthiest_endpoint().await
            .ok_or_else(|| RpcClientError::AllEndpointsFailed("No healthy endpoints available".to_string()))?;

        self.make_request(&endpoint, "getSlot", serde_json::json!([])).await
    }

    /// Gets health statistics for all endpoints.
    pub async fn get_health_stats(&self) -> Vec<crate::health::EndpointHealth> {
        self.health_monitor.get_health_stats().await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::EndpointConfig;

    #[tokio::test]
    async fn test_rpc_client_creation() {
        let config = RpcConfig {
            endpoints: vec![EndpointConfig {
                url: url::Url::parse("http://localhost:8899").unwrap(),
                weight: 1,
                requests_per_second: Some(100),
            }],
            pool_size: 10,
            keep_alive: Duration::from_secs(30),
            max_retries: 5,
            base_delay: Duration::from_millis(100),
            max_delay: Duration::from_secs(10),
            requests_per_second: 100,
            cache_ttl: Duration::from_secs(60),
            timeout: Duration::from_secs(30),
        };

        let client = SolanaRpcClient::new(config);
        assert!(client.is_ok());
    }
} 