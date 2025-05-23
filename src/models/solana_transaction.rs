use serde::{Serialize, Deserialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};
use validator::Validate;

#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct SolanaTransaction {
    pub id: Uuid,
    #[validate(length(min = 1))]
    pub signature: String,
    pub slot: u64,
    pub block_time: DateTime<Utc>,
    pub fee: u64,
    #[validate(length(min = 1))]
    pub status: String,
    pub instructions_json: serde_json::Value,
    pub created_at: DateTime<Utc>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_create_and_validate_transaction() {
        let tx = SolanaTransaction {
            id: Uuid::new_v4(),
            signature: "test_signature".to_string(),
            slot: 123,
            block_time: Utc::now(),
            fee: 5000,
            status: "confirmed".to_string(),
            instructions_json: json!({"program":"spl-token"}),
            created_at: Utc::now(),
        };
        assert!(tx.validate().is_ok());
    }
} 