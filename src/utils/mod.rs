//! Utility functions and types for the Solana analytics system

use chrono::{DateTime, Duration, Utc};
use solana_sdk::pubkey::Pubkey;
use std::str::FromStr;

/// Converts a base58 encoded string to a Solana public key
pub fn pubkey_from_str(s: &str) -> Option<Pubkey> {
    Pubkey::from_str(s).ok()
}

/// Formats a timestamp as a human-readable string
pub fn format_timestamp(timestamp: DateTime<Utc>) -> String {
    timestamp.format("%Y-%m-%d %H:%M:%S UTC").to_string()
}

/// Gets a time range for the last N days
pub fn get_last_n_days(n: i64) -> (DateTime<Utc>, DateTime<Utc>) {
    let end = Utc::now();
    let start = end - Duration::days(n);
    (start, end)
}

/// Formats a lamports amount as SOL
pub fn format_sol_amount(lamports: i64) -> String {
    format!("{:.9} SOL", lamports as f64 / 1e9)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pubkey_from_str() {
        let valid_pubkey = "11111111111111111111111111111111";
        assert!(pubkey_from_str(valid_pubkey).is_some());

        let invalid_pubkey = "invalid";
        assert!(pubkey_from_str(invalid_pubkey).is_none());
    }

    #[test]
    fn test_format_timestamp() {
        let timestamp = Utc::now();
        let formatted = format_timestamp(timestamp);
        assert!(formatted.contains("UTC"));
    }

    #[test]
    fn test_get_last_n_days() {
        let (start, end) = get_last_n_days(7);
        let duration = end - start;
        assert_eq!(duration.num_days(), 7);
    }

    #[test]
    fn test_format_sol_amount() {
        assert_eq!(format_sol_amount(1_000_000_000), "1.000000000 SOL");
        assert_eq!(format_sol_amount(1_500_000_000), "1.500000000 SOL");
    }
} 