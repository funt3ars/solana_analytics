use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Represents a token price history record
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PriceHistory {
    /// Unique identifier for this price record
    pub id: Uuid,
    /// The token mint public key (base58 encoded)
    pub token_mint: String,
    /// The price in USD
    pub price_usd: f64,
    /// When this price was recorded
    pub timestamp: DateTime<Utc>,
    /// The source of the price data (e.g., "coingecko", "binance")
    pub source: String,
    /// When this record was created
    pub created_at: DateTime<Utc>,
}

impl PriceHistory {
    /// Creates a new price history record
    pub fn new(
        token_mint: String,
        price_usd: f64,
        timestamp: DateTime<Utc>,
        source: String,
    ) -> Self {
        Self {
            id: Uuid::new_v4(),
            token_mint,
            price_usd,
            timestamp,
            source,
            created_at: Utc::now(),
        }
    }

    /// Updates the price
    pub fn update_price(&mut self, new_price: f64) {
        self.price_usd = new_price;
        self.timestamp = Utc::now();
    }

    /// Converts a database row into a PriceHistory
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_price_history_creation() {
        let now = Utc::now();
        let price = PriceHistory::new(
            "test_mint".to_string(),
            1.23,
            now,
            "coingecko".to_string(),
        );

        assert_eq!(price.token_mint, "test_mint");
        assert_eq!(price.price_usd, 1.23);
        assert_eq!(price.timestamp, now);
        assert_eq!(price.source, "coingecko");
    }

    #[test]
    fn test_price_history_update() {
        let now = Utc::now();
        let mut price = PriceHistory::new(
            "test_mint".to_string(),
            1.23,
            now,
            "coingecko".to_string(),
        );

        price.update_price(2.34);
        assert_eq!(price.price_usd, 2.34);
        assert!(price.timestamp > now);
    }

    #[test]
    fn test_price_history_serialization() {
        let now = Utc::now();
        let price = PriceHistory {
            id: Uuid::new_v4(),
            token_mint: "test_mint".to_string(),
            price_usd: 1.23,
            timestamp: now,
            source: "coingecko".to_string(),
            created_at: now,
        };

        let serialized = serde_json::to_string(&price).unwrap();
        let deserialized: PriceHistory = serde_json::from_str(&serialized).unwrap();
        assert_eq!(price.token_mint, deserialized.token_mint);
        assert_eq!(price.price_usd, deserialized.price_usd);
        assert_eq!(price.source, deserialized.source);
    }
} 