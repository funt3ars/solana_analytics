use serde::{Serialize, Deserialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};
use validator::Validate;

#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct GovernanceVote {
    pub id: Uuid,
    #[validate(length(min = 1))]
    pub voter: String,
    #[validate(length(min = 1))]
    pub proposal_id: String,
    #[validate(length(min = 1))]
    pub vote: String,
    pub timestamp: DateTime<Utc>,
    #[validate(length(min = 1))]
    pub dao_name: String,
    pub created_at: DateTime<Utc>,
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_create_and_validate_governance_vote() {
        let vote = GovernanceVote {
            id: Uuid::new_v4(),
            voter: "voter1".to_string(),
            proposal_id: "proposal1".to_string(),
            vote: "yes".to_string(),
            timestamp: Utc::now(),
            dao_name: "dao1".to_string(),
            created_at: Utc::now(),
        };
        assert!(vote.validate().is_ok());
    }
} 