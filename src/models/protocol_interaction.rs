use serde::{Serialize, Deserialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};
use validator::Validate;

#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct ProtocolInteraction {
    pub id: Uuid,
    #[validate(length(min = 1))]
    pub wallet: String,
    #[validate(length(min = 1))]
    pub protocol: String,
    #[validate(length(min = 1))]
    pub interaction_type: String,
    #[validate(range(min = 0.0))]
    pub amount: f64,
    pub timestamp: DateTime<Utc>,
    pub created_at: DateTime<Utc>,
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_create_and_validate_protocol_interaction() {
        let interaction = ProtocolInteraction {
            id: Uuid::new_v4(),
            wallet: "wallet1".to_string(),
            protocol: "protocol1".to_string(),
            interaction_type: "swap".to_string(),
            amount: 42.0,
            timestamp: Utc::now(),
            created_at: Utc::now(),
        };
        assert!(interaction.validate().is_ok());
    }
} 