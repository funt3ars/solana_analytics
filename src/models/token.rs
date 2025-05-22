use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// Represents a Solana token account with its metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TokenAccount {
    /// The token account public key (base58 encoded)
    pub pubkey: String,
    /// The token mint public key (base58 encoded)
    pub mint: String,
    /// The owner's public key (base58 encoded)
    pub owner: String,
    /// The token balance in raw units
    pub amount: i64,
    /// When this account was last updated
    pub updated_at: DateTime<Utc>,
    /// When this record was created
    pub created_at: DateTime<Utc>,
}

impl TokenAccount {
    /// Creates a new token account record
    pub fn new(
        pubkey: String,
        mint: String,
        owner: String,
        amount: i64,
        updated_at: DateTime<Utc>,
    ) -> Self {
        Self {
            pubkey,
            mint,
            owner,
            amount,
            updated_at,
            created_at: Utc::now(),
        }
    }

    /// Updates the token account balance
    pub fn update_balance(&mut self, new_amount: i64) {
        self.amount = new_amount;
        self.updated_at = Utc::now();
    }

    /// Converts a database row into a TokenAccount
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_token_account_creation() {
        let now = Utc::now();
        let account = TokenAccount::new(
            "test_pubkey".to_string(),
            "test_mint".to_string(),
            "test_owner".to_string(),
            1000,
            now,
        );

        assert_eq!(account.pubkey, "test_pubkey");
        assert_eq!(account.mint, "test_mint");
        assert_eq!(account.owner, "test_owner");
        assert_eq!(account.amount, 1000);
        assert_eq!(account.updated_at, now);
    }

    #[test]
    fn test_token_account_balance_update() {
        let now = Utc::now();
        let mut account = TokenAccount::new(
            "test_pubkey".to_string(),
            "test_mint".to_string(),
            "test_owner".to_string(),
            1000,
            now,
        );

        account.update_balance(2000);
        assert_eq!(account.amount, 2000);
        assert!(account.updated_at > now);
    }

    #[test]
    fn test_token_account_serialization() {
        let now = Utc::now();
        let account = TokenAccount {
            pubkey: "test_pubkey".to_string(),
            mint: "test_mint".to_string(),
            owner: "test_owner".to_string(),
            amount: 1000,
            updated_at: now,
            created_at: now,
        };

        let serialized = serde_json::to_string(&account).unwrap();
        let deserialized: TokenAccount = serde_json::from_str(&serialized).unwrap();
        assert_eq!(account.pubkey, deserialized.pubkey);
        assert_eq!(account.mint, deserialized.mint);
        assert_eq!(account.owner, deserialized.owner);
        assert_eq!(account.amount, deserialized.amount);
    }
} 