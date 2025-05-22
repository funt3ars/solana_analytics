//! Data models for the Solana analytics system

pub mod transaction;
pub mod token;
pub mod price;
pub mod protocol;
pub mod governance;

// Re-export commonly used types
pub use transaction::Transaction;
pub use token::TokenAccount;
pub use price::PriceHistory;
pub use protocol::ProtocolInteraction;
pub use governance::GovernanceVote; 