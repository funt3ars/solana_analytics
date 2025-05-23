use serde::{Serialize, Deserialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};
use validator::Validate;

#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct TokenAccount {
    pub id: Uuid,
    #[validate(length(min = 1))]
    pub pubkey: String,
    #[validate(length(min = 1))]
    pub mint: String,
    #[validate(length(min = 1))]
    pub owner: String,
    pub amount: u64,
    pub updated_at: DateTime<Utc>,
    pub created_at: DateTime<Utc>,
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_create_and_validate_token_account() {
        let acc = TokenAccount {
            id: Uuid::new_v4(),
            pubkey: "pubkey1".to_string(),
            mint: "mint1".to_string(),
            owner: "owner1".to_string(),
            amount: 1000,
            updated_at: Utc::now(),
            created_at: Utc::now(),
        };
        assert!(acc.validate().is_ok());
    }
} 