use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Represents a protocol interaction record
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProtocolInteraction {
    /// Unique identifier for this interaction
    pub id: Uuid,
    /// The wallet address (base58 encoded)
    pub wallet: String,
    /// The protocol name
    pub protocol: String,
    /// The type of interaction (e.g., "swap", "stake", "borrow")
    pub interaction_type: String,
    /// The amount involved in the interaction
    pub amount: f64,
    /// When this interaction occurred
    pub timestamp: DateTime<Utc>,
    /// When this record was created
    pub created_at: DateTime<Utc>,
}

impl ProtocolInteraction {
    /// Creates a new protocol interaction record
    pub fn new(
        wallet: String,
        protocol: String,
        interaction_type: String,
        amount: f64,
        timestamp: DateTime<Utc>,
    ) -> Self {
        Self {
            id: Uuid::new_v4(),
            wallet,
            protocol,
            interaction_type,
            amount,
            timestamp,
            created_at: Utc::now(),
        }
    }

    /// Updates the interaction amount
    pub fn update_amount(&mut self, new_amount: f64) {
        self.amount = new_amount;
        self.timestamp = Utc::now();
    }

    /// Converts a database row into a ProtocolInteraction
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_protocol_interaction_creation() {
        let now = Utc::now();
        let interaction = ProtocolInteraction::new(
            "test_wallet".to_string(),
            "test_protocol".to_string(),
            "swap".to_string(),
            100.0,
            now,
        );

        assert_eq!(interaction.wallet, "test_wallet");
        assert_eq!(interaction.protocol, "test_protocol");
        assert_eq!(interaction.interaction_type, "swap");
        assert_eq!(interaction.amount, 100.0);
        assert_eq!(interaction.timestamp, now);
    }

    #[test]
    fn test_protocol_interaction_update() {
        let now = Utc::now();
        let mut interaction = ProtocolInteraction::new(
            "test_wallet".to_string(),
            "test_protocol".to_string(),
            "swap".to_string(),
            100.0,
            now,
        );

        interaction.update_amount(200.0);
        assert_eq!(interaction.amount, 200.0);
        assert!(interaction.timestamp > now);
    }

    #[test]
    fn test_protocol_interaction_serialization() {
        let now = Utc::now();
        let interaction = ProtocolInteraction {
            id: Uuid::new_v4(),
            wallet: "test_wallet".to_string(),
            protocol: "test_protocol".to_string(),
            interaction_type: "swap".to_string(),
            amount: 100.0,
            timestamp: now,
            created_at: now,
        };

        let serialized = serde_json::to_string(&interaction).unwrap();
        let deserialized: ProtocolInteraction = serde_json::from_str(&serialized).unwrap();
        assert_eq!(interaction.wallet, deserialized.wallet);
        assert_eq!(interaction.protocol, deserialized.protocol);
        assert_eq!(interaction.interaction_type, deserialized.interaction_type);
        assert_eq!(interaction.amount, deserialized.amount);
    }
} 