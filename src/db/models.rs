use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use tokio_postgres::Row;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct Transaction {
    pub id: Uuid,
    pub signature: String,
    pub block_time: DateTime<Utc>,
    pub slot: i64,
    pub status: String,
    pub created_at: DateTime<Utc>,
}

impl From<Row> for Transaction {
    fn from(row: Row) -> Self {
        Self {
            id: row.get("id"),
            signature: row.get("signature"),
            block_time: row.get("block_time"),
            slot: row.get("slot"),
            status: row.get("status"),
            created_at: row.get("created_at"),
        }
    }
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

impl From<Row> for TokenAccount {
    fn from(row: Row) -> Self {
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

#[derive(Debug, Serialize, Deserialize)]
pub struct PriceHistory {
    pub id: Uuid,
    pub token_mint: String,
    pub price_usd: f64,
    pub timestamp: DateTime<Utc>,
    pub source: String,
    pub created_at: DateTime<Utc>,
}

impl From<Row> for PriceHistory {
    fn from(row: Row) -> Self {
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

impl From<Row> for ProtocolInteraction {
    fn from(row: Row) -> Self {
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

impl From<Row> for GovernanceVote {
    fn from(row: Row) -> Self {
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
    use uuid::Uuid;

    #[test]
    fn test_transaction_serialization() {
        let transaction = Transaction {
            id: Uuid::new_v4(),
            signature: "test_sig".to_string(),
            slot: 123,
            block_time: Utc::now(),
            status: "success".to_string(),
            created_at: Utc::now(),
        };

        let serialized = serde_json::to_string(&transaction).unwrap();
        let deserialized: Transaction = serde_json::from_str(&serialized).unwrap();
        
        assert_eq!(transaction.signature, deserialized.signature);
        assert_eq!(transaction.slot, deserialized.slot);
        assert_eq!(transaction.status, deserialized.status);
    }

    #[test]
    fn test_token_account_serialization() {
        let token_account = TokenAccount {
            pubkey: "test_pubkey".to_string(),
            mint: "test_mint".to_string(),
            owner: "test_owner".to_string(),
            amount: 1000,
            updated_at: Utc::now(),
            created_at: Utc::now(),
        };

        let serialized = serde_json::to_string(&token_account).unwrap();
        let deserialized: TokenAccount = serde_json::from_str(&serialized).unwrap();
        
        assert_eq!(token_account.pubkey, deserialized.pubkey);
        assert_eq!(token_account.mint, deserialized.mint);
        assert_eq!(token_account.owner, deserialized.owner);
        assert_eq!(token_account.amount, deserialized.amount);
    }

    #[test]
    fn test_price_history_serialization() {
        let price_history = PriceHistory {
            id: Uuid::new_v4(),
            token_mint: "test_mint".to_string(),
            price_usd: 1.23,
            timestamp: Utc::now(),
            source: "test_source".to_string(),
            created_at: Utc::now(),
        };

        let serialized = serde_json::to_string(&price_history).unwrap();
        let deserialized: PriceHistory = serde_json::from_str(&serialized).unwrap();
        
        assert_eq!(price_history.token_mint, deserialized.token_mint);
        assert_eq!(price_history.price_usd, deserialized.price_usd);
        assert_eq!(price_history.source, deserialized.source);
    }

    #[test]
    fn test_protocol_interaction_serialization() {
        let interaction = ProtocolInteraction {
            id: Uuid::new_v4(),
            wallet: "test_wallet".to_string(),
            protocol: "test_protocol".to_string(),
            interaction_type: "test_type".to_string(),
            amount: 100.0,
            timestamp: Utc::now(),
            created_at: Utc::now(),
        };

        let serialized = serde_json::to_string(&interaction).unwrap();
        let deserialized: ProtocolInteraction = serde_json::from_str(&serialized).unwrap();
        
        assert_eq!(interaction.wallet, deserialized.wallet);
        assert_eq!(interaction.protocol, deserialized.protocol);
        assert_eq!(interaction.interaction_type, deserialized.interaction_type);
        assert_eq!(interaction.amount, deserialized.amount);
    }

    #[test]
    fn test_governance_vote_serialization() {
        let vote = GovernanceVote {
            id: Uuid::new_v4(),
            voter: "test_voter".to_string(),
            proposal_id: "test_proposal".to_string(),
            vote: "yes".to_string(),
            timestamp: Utc::now(),
            dao_name: "test_dao".to_string(),
            created_at: Utc::now(),
        };

        let serialized = serde_json::to_string(&vote).unwrap();
        let deserialized: GovernanceVote = serde_json::from_str(&serialized).unwrap();
        
        assert_eq!(vote.voter, deserialized.voter);
        assert_eq!(vote.proposal_id, deserialized.proposal_id);
        assert_eq!(vote.vote, deserialized.vote);
        assert_eq!(vote.dao_name, deserialized.dao_name);
    }
} 