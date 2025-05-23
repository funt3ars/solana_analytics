use serde::{Serialize, Deserialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};
use validator::Validate;

#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct PriceData {
    pub id: Uuid,
    #[validate(length(min = 1))]
    pub token_mint: String,
    #[validate(range(min = 0.0))]
    pub price_usd: f64,
    pub timestamp: DateTime<Utc>,
    #[validate(length(min = 1))]
    pub source: String,
    pub created_at: DateTime<Utc>,
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_create_and_validate_price_data() {
        let price = PriceData {
            id: Uuid::new_v4(),
            token_mint: "mint1".to_string(),
            price_usd: 1.23,
            timestamp: Utc::now(),
            source: "oracle1".to_string(),
            created_at: Utc::now(),
        };
        assert!(price.validate().is_ok());
    }
} 