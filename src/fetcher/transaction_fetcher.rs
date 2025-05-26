use solana_sdk::pubkey::Pubkey;
use crate::models::transaction::Transaction;
use async_trait::async_trait;
use crate::rpc::client::RpcClientTrait;
use mockall::predicate::*;
use tracing;
use chrono::Utc;

/// Tracks progress of transaction fetching
pub struct FetchProgress {
    pub fetched: usize,
    pub last_signature: Option<String>,
    pub done: bool,
}

/// Errors that can occur during transaction fetching
#[derive(thiserror::Error, Debug)]
pub enum FetchError {
    #[error("RPC error: {0}")]
    Rpc(String),
    #[error("Rate limited")]
    RateLimited,
    #[error("Network error: {0}")]
    Network(String),
    #[error("Other: {0}")]
    Other(String),
}

/// Fetches transactions for a given address, with pagination, checkpointing, and progress tracking
pub struct TransactionFetcher<C: RpcClientTrait> {
    pub rpc_client: C, // Must implement RpcClientTrait
    pub address: Pubkey,
    pub batch_size: usize,
    pub checkpoint: Option<String>, // Last fetched signature
}

#[async_trait]
pub trait FetchTransactions {
    async fn fetch_next_batch(&mut self) -> Result<Vec<Transaction>, FetchError>;
    async fn fetch_all(&mut self) -> Result<Vec<Transaction>, FetchError>;
    fn set_checkpoint(&mut self, signature: Option<String>);
    fn get_checkpoint(&self) -> Option<String>;
    fn progress(&self) -> FetchProgress;
}

#[async_trait]
impl<C: RpcClientTrait> FetchTransactions for TransactionFetcher<C> {
    async fn fetch_next_batch(&mut self) -> Result<Vec<Transaction>, FetchError> {
        // Fetch signatures for the address, paginated by checkpoint
        let sigs = self
            .rpc_client
            .get_signatures_for_address(&self.address, self.checkpoint.clone(), self.batch_size)
            .await
            .map_err(|e| FetchError::Rpc(e.to_string()))?;
        if sigs.is_empty() {
            return Ok(vec![]);
        }
        // Update checkpoint to the last signature
        self.checkpoint = sigs.last().cloned();
        // Fetch full transactions for each signature
        let mut txs = Vec::with_capacity(sigs.len());
        for sig in sigs {
            match self.rpc_client.get_transaction(&sig).await {
                Ok(tx) => txs.push(tx),
                Err(e) => {
                    tracing::warn!("Failed to fetch transaction for {}: {}", sig, e);
                }
            }
        }
        Ok(txs)
    }

    async fn fetch_all(&mut self) -> Result<Vec<Transaction>, FetchError> {
        // Not implemented yet
        unimplemented!("fetch_all");
    }

    fn set_checkpoint(&mut self, signature: Option<String>) {
        self.checkpoint = signature;
    }

    fn get_checkpoint(&self) -> Option<String> {
        self.checkpoint.clone()
    }

    fn progress(&self) -> FetchProgress {
        FetchProgress {
            fetched: 0, // Not tracked yet
            last_signature: self.checkpoint.clone(),
            done: false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use solana_sdk::pubkey::Pubkey;
    use mockall::predicate::*;
    use crate::rpc::client::MockRpcClientTrait;
    use crate::models::transaction::Transaction;

    #[tokio::test]
    async fn test_fetch_signatures_pagination() {
        // Set up the mock to return two pages of signatures
        let mut mock = MockRpcClientTrait::new();
        let address = Pubkey::new_unique();
        let page1 = vec!["sig1".to_string(), "sig2".to_string()];
        let page2 = vec!["sig3".to_string()];

        mock.expect_get_signatures_for_address()
            .with(eq(address), eq(None), eq(2))
            .times(1)
            .returning(move |_, _, _| Ok(page1.clone()));
        mock.expect_get_signatures_for_address()
            .with(eq(address), eq(Some("sig2".to_string())), eq(2))
            .times(1)
            .returning(move |_, _, _| Ok(page2.clone()));
        mock.expect_get_signatures_for_address()
            .with(eq(address), eq(Some("sig3".to_string())), eq(2))
            .times(1)
            .returning(move |_, _, _| Ok(vec![]));

        // Mock transaction fetches for all expected signatures
        mock.expect_get_transaction()
            .withf(|sig| sig == "sig1")
            .returning(|_| Ok(Transaction { signature: "sig1".to_string(), ..Default::default() }));
        mock.expect_get_transaction()
            .withf(|sig| sig == "sig2")
            .returning(|_| Ok(Transaction { signature: "sig2".to_string(), ..Default::default() }));
        mock.expect_get_transaction()
            .withf(|sig| sig == "sig3")
            .returning(|_| Ok(Transaction { signature: "sig3".to_string(), ..Default::default() }));

        let mut fetcher = TransactionFetcher {
            rpc_client: mock,
            address,
            batch_size: 2,
            checkpoint: None,
        };

        // First batch
        let batch1 = fetcher.fetch_next_batch().await.unwrap();
        let sigs1: Vec<_> = batch1.iter().map(|tx| tx.signature.clone()).collect();
        assert_eq!(sigs1, vec!["sig1", "sig2"]);
        assert_eq!(fetcher.get_checkpoint(), Some("sig2".to_string()));

        // Second batch
        let batch2 = fetcher.fetch_next_batch().await.unwrap();
        let sigs2: Vec<_> = batch2.iter().map(|tx| tx.signature.clone()).collect();
        assert_eq!(sigs2, vec!["sig3"]);
        assert_eq!(fetcher.get_checkpoint(), Some("sig3".to_string()));

        // Third batch (should be empty)
        let batch3 = fetcher.fetch_next_batch().await.unwrap();
        assert!(batch3.is_empty());
    }

    #[tokio::test]
    async fn test_fetch_full_transactions_by_signature() {
        let mut mock = MockRpcClientTrait::new();
        let address = Pubkey::new_unique();
        let sigs = vec!["sig1".to_string(), "sig2".to_string()];

        // Mock signature fetch
        mock.expect_get_signatures_for_address()
            .return_once(move |_, _, _| Ok(sigs.clone()));

        // Mock transaction fetches
        let tx1 = Transaction {
            signature: "sig1".to_string(),
            slot: 1,
            block_time: Utc::now(),
            fee: 5000,
            status: "success".to_string(),
            instructions_json: "{}".to_string(),
            created_at: Utc::now(),
        };
        let tx2 = Transaction {
            signature: "sig2".to_string(),
            slot: 2,
            block_time: Utc::now(),
            fee: 6000,
            status: "success".to_string(),
            instructions_json: "{}".to_string(),
            created_at: Utc::now(),
        };

        let tx1_clone = tx1.clone();
        let tx2_clone = tx2.clone();
        mock.expect_get_transaction()
            .withf(|sig| sig == "sig1")
            .return_once(move |_| Ok(tx1_clone));
        mock.expect_get_transaction()
            .withf(|sig| sig == "sig2")
            .return_once(move |_| Ok(tx2_clone));

        let mut fetcher = TransactionFetcher {
            rpc_client: mock,
            address,
            batch_size: 2,
            checkpoint: None,
        };

        let txs = fetcher.fetch_next_batch().await.unwrap();
        assert_eq!(txs.len(), 2);
        assert_eq!(txs[0].signature, "sig1");
        assert_eq!(txs[1].signature, "sig2");
    }
} 