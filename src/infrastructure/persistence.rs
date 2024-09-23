use uptop_core::common::{db_types::CassandraCacheSession, result::AppResult};

pub(crate) mod topic_repository;

#[derive(Debug)]
pub struct IDRepositories {
    pub topic: topic_repository::TopicRepo,
}

impl IDRepositories {
    pub fn new(session: CassandraCacheSession) -> Self {
        Self {
            topic: topic_repository::TopicRepo::new(session),
        }
    }

    pub async fn auto_mod_identification_migrate(&self) -> AppResult<()> {
        self.topic.migrate_topic_table().await?;
        Ok(())
    }
}
