use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct Transaction {
    pub signature: String,
    pub slot: i64,
    pub block_time: DateTime<Utc>,
    pub fee: i64,
    pub status: String,
    pub instructions_json: serde_json::Value,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TokenAccount {
    pub pubkey: String,
    pub mint: String,
    pub owner: String,
    pub amount: i64,
    pub updated_at: DateTime<Utc>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PriceHistory {
    pub id: Uuid,
    pub token_mint: String,
    pub price_usd: f64,
    pub timestamp: DateTime<Utc>,
    pub source: String,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProtocolInteraction {
    pub id: Uuid,
    pub wallet: String,
    pub protocol: String,
    pub interaction_type: String,
    pub amount: f64,
    pub timestamp: DateTime<Utc>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GovernanceVote {
    pub id: Uuid,
    pub voter: String,
    pub proposal_id: String,
    pub vote: String,
    pub timestamp: DateTime<Utc>,
    pub dao_name: String,
    pub created_at: DateTime<Utc>,
}

impl Transaction {
    pub fn from_row(row: &tokio_postgres::Row) -> Self {
        Self {
            signature: row.get("signature"),
            slot: row.get("slot"),
            block_time: row.get("block_time"),
            fee: row.get("fee"),
            status: row.get("status"),
            instructions_json: row.get("instructions_json"),
            created_at: row.get("created_at"),
        }
    }
}

impl TokenAccount {
    pub fn from_row(row: &tokio_postgres::Row) -> Self {
        Self {
            pubkey: row.get("pubkey"),
            mint: row.get("mint"),
            owner: row.get("owner"),
            amount: row.get("amount"),
            updated_at: row.get("updated_at"),
            created_at: row.get("created_at"),
        }
    }
}

impl PriceHistory {
    pub fn from_row(row: &tokio_postgres::Row) -> Self {
        Self {
            id: row.get("id"),
            token_mint: row.get("token_mint"),
            price_usd: row.get("price_usd"),
            timestamp: row.get("timestamp"),
            source: row.get("source"),
            created_at: row.get("created_at"),
        }
    }
}

impl ProtocolInteraction {
    pub fn from_row(row: &tokio_postgres::Row) -> Self {
        Self {
            id: row.get("id"),
            wallet: row.get("wallet"),
            protocol: row.get("protocol"),
            interaction_type: row.get("interaction_type"),
            amount: row.get("amount"),
            timestamp: row.get("timestamp"),
            created_at: row.get("created_at"),
        }
    }
}

impl GovernanceVote {
    pub fn from_row(row: &tokio_postgres::Row) -> Self {
        Self {
            id: row.get("id"),
            voter: row.get("voter"),
            proposal_id: row.get("proposal_id"),
            vote: row.get("vote"),
            timestamp: row.get("timestamp"),
            dao_name: row.get("dao_name"),
            created_at: row.get("created_at"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Utc;

    #[test]
    fn test_transaction_serialization() {
        let transaction = Transaction {
            signature: "test_sig".to_string(),
            slot: 123,
            block_time: Utc::now(),
            fee: 1000,
            status: "success".to_string(),
            instructions_json: serde_json::json!({}),
            created_at: Utc::now(),
        };

        let serialized = serde_json::to_string(&transaction).unwrap();
        let deserialized: Transaction = serde_json::from_str(&serialized).unwrap();
        assert_eq!(transaction.signature, deserialized.signature);
    }
} 