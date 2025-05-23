use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;

/// Represents a Solana transaction with its metadata and instructions
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Transaction {
    /// The transaction signature (base58 encoded)
    pub signature: String,
    /// The slot number when the transaction was processed
    pub slot: i64,
    /// The block time when the transaction was processed
    pub block_time: DateTime<Utc>,
    /// The transaction fee in lamports
    pub fee: i64,
    /// The transaction status (e.g., "success", "failed")
    pub status: String,
    /// The transaction instructions in JSON format (as a JSON string)
    pub instructions_json: String,
    /// When this record was created
    pub created_at: DateTime<Utc>,
}

impl Transaction {
    /// Creates a new transaction record
    pub fn new(
        signature: String,
        slot: i64,
        block_time: DateTime<Utc>,
        fee: i64,
        status: String,
        instructions_json: String,
    ) -> Self {
        Self {
            signature,
            slot,
            block_time,
            fee,
            status,
            instructions_json,
            created_at: Utc::now(),
        }
    }

    /// Helper to get instructions_json as serde_json::Value
    pub fn instructions_json_value(&self) -> serde_json::Result<serde_json::Value> {
        serde_json::from_str(&self.instructions_json)
    }

    /// Converts a database row into a Transaction
    pub fn from_row(row: &tokio_postgres::Row) -> Self {
        Self {
            signature: row.get("signature"),
            slot: row.get("slot"),
            block_time: row.get("block_time"),
            fee: row.get("fee"),
            status: row.get("status"),
            instructions_json: row.get::<_, String>("instructions_json"),
            created_at: row.get("created_at"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_transaction_creation() {
        let transaction = Transaction::new(
            "test_sig".to_string(),
            123,
            Utc::now(),
            1000,
            "success".to_string(),
            json!({}).to_string(),
        );

        assert_eq!(transaction.signature, "test_sig");
        assert_eq!(transaction.slot, 123);
        assert_eq!(transaction.fee, 1000);
        assert_eq!(transaction.status, "success");
    }

    #[test]
    fn test_transaction_serialization() {
        let transaction = Transaction {
            signature: "test_sig".to_string(),
            slot: 123,
            block_time: Utc::now(),
            fee: 1000,
            status: "success".to_string(),
            instructions_json: json!({}).to_string(),
            created_at: Utc::now(),
        };

        let serialized = serde_json::to_string(&transaction).unwrap();
        let deserialized: Transaction = serde_json::from_str(&serialized).unwrap();
        assert_eq!(transaction.signature, deserialized.signature);
    }
} 