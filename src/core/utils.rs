//! Utility functions and types for the Solana analytics system

use chrono::{DateTime, Duration, Utc};
use std::str::FromStr;

#[cfg(feature = "phase1")]
use solana_sdk::pubkey::Pubkey;

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

#[cfg(feature = "phase1")]
/// Convert a string to a Pubkey
pub fn pubkey_from_str(s: &str) -> Option<Pubkey> {
    Pubkey::from_str(s).ok()
}

#[cfg(feature = "phase1")]
/// Convert a Pubkey to a string
pub fn pubkey_to_string(pubkey: &Pubkey) -> String {
    pubkey.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[cfg(feature = "phase1")]
    #[test]
    fn test_pubkey_conversion() {
        let valid_pubkey = "11111111111111111111111111111111";
        let invalid_pubkey = "invalid";

        assert!(pubkey_from_str(valid_pubkey).is_some());
        assert!(pubkey_from_str(invalid_pubkey).is_none());
    }

    #[cfg(feature = "phase1")]
    #[test]
    fn test_pubkey_to_string() {
        let pubkey = Pubkey::new_unique();
        let string = pubkey_to_string(&pubkey);
        let back_to_pubkey = pubkey_from_str(&string).unwrap();
        assert_eq!(pubkey, back_to_pubkey);
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