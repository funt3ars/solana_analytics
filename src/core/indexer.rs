use crate::core::config::Config;
use async_trait::async_trait;
use tracing::info;

/// Trait for protocol-specific parsers
#[async_trait]
pub trait ProtocolParser: Send + Sync {
    async fn parse(&self, data: &[u8]) -> anyhow::Result<()>;
}

/// Core indexer struct
pub struct CoreIndexer {
    pub config: Config,
}

impl CoreIndexer {
    pub fn new(config: Config) -> Self {
        info!("Initializing CoreIndexer");
        Self { config }
    }

    // Placeholder for starting the indexer
    pub async fn run(&self) -> anyhow::Result<()> {
        info!("Running CoreIndexer");
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::config::Config;

    #[tokio::test]
    async fn test_core_indexer_instantiation() {
        let config = Config::default();
        let indexer = CoreIndexer::new(config);
        assert!(indexer.run().await.is_ok());
    }
} 