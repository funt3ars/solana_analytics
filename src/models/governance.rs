use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Represents a governance vote record
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GovernanceVote {
    /// Unique identifier for this vote
    pub id: Uuid,
    /// The voter's wallet address (base58 encoded)
    pub voter: String,
    /// The proposal identifier
    pub proposal_id: String,
    /// The vote cast (e.g., "yes", "no", "abstain")
    pub vote: String,
    /// When this vote was cast
    pub timestamp: DateTime<Utc>,
    /// The name of the DAO
    pub dao_name: String,
    /// When this record was created
    pub created_at: DateTime<Utc>,
}

impl GovernanceVote {
    /// Creates a new governance vote record
    pub fn new(
        voter: String,
        proposal_id: String,
        vote: String,
        timestamp: DateTime<Utc>,
        dao_name: String,
    ) -> Self {
        Self {
            id: Uuid::new_v4(),
            voter,
            proposal_id,
            vote,
            timestamp,
            dao_name,
            created_at: Utc::now(),
        }
    }

    /// Updates the vote
    pub fn update_vote(&mut self, new_vote: String) {
        self.vote = new_vote;
        self.timestamp = Utc::now();
    }

    /// Converts a database row into a GovernanceVote
    pub fn from_row(row: &tokio_postgres::Row) -> Self {
        Self {
            id: row.get("id"),
            voter: row.get("voter"),
            proposal_id: row.get("proposal_id"),
            vote: row.get("vote"),
            timestamp: row.get("timestamp"),
            dao_name: row.get("dao_name"),
            created_at: row.get("created_at"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_governance_vote_creation() {
        let now = Utc::now();
        let vote = GovernanceVote::new(
            "test_voter".to_string(),
            "test_proposal".to_string(),
            "yes".to_string(),
            now,
            "test_dao".to_string(),
        );

        assert_eq!(vote.voter, "test_voter");
        assert_eq!(vote.proposal_id, "test_proposal");
        assert_eq!(vote.vote, "yes");
        assert_eq!(vote.timestamp, now);
        assert_eq!(vote.dao_name, "test_dao");
    }

    #[test]
    fn test_governance_vote_update() {
        let now = Utc::now();
        let mut vote = GovernanceVote::new(
            "test_voter".to_string(),
            "test_proposal".to_string(),
            "yes".to_string(),
            now,
            "test_dao".to_string(),
        );

        vote.update_vote("no".to_string());
        assert_eq!(vote.vote, "no");
        assert!(vote.timestamp > now);
    }

    #[test]
    fn test_governance_vote_serialization() {
        let now = Utc::now();
        let vote = GovernanceVote {
            id: Uuid::new_v4(),
            voter: "test_voter".to_string(),
            proposal_id: "test_proposal".to_string(),
            vote: "yes".to_string(),
            timestamp: now,
            dao_name: "test_dao".to_string(),
            created_at: now,
        };

        let serialized = serde_json::to_string(&vote).unwrap();
        let deserialized: GovernanceVote = serde_json::from_str(&serialized).unwrap();
        assert_eq!(vote.voter, deserialized.voter);
        assert_eq!(vote.proposal_id, deserialized.proposal_id);
        assert_eq!(vote.vote, deserialized.vote);
        assert_eq!(vote.dao_name, deserialized.dao_name);
    }
} 