use uptop_core::common::{db_types::CassandraCacheSession, result::AppResult};
use crate::infrastructure::persistence::latest_message_repository::LatestMessageRepo;
use crate::infrastructure::persistence::topic_message_repository::TopicMessageRepo;

pub(crate) mod topic_repository;
pub(crate) mod latest_message_repository;
pub(crate) mod topic_message_repository;
pub(crate) mod topic_user_repository;
pub(crate) mod user_topic_repository;
pub(crate) mod notification_repository;

#[derive(Debug)]
pub struct MessageRepositories {
    pub topic: topic_repository::TopicRepo,
    pub topic_message: TopicMessageRepo,
    pub latest_message: LatestMessageRepo,
}

impl MessageRepositories {
    pub fn new(session: CassandraCacheSession) -> Self {
        Self {
            topic: topic_repository::TopicRepo::new(session.clone()),
            topic_message: TopicMessageRepo::new(session.clone()),
            latest_message: LatestMessageRepo::new(session),
        }
    }

    pub async fn auto_mod_identification_migrate(&self) -> AppResult<()> {
        self.topic.migrate_topic_table().await?;
        self.topic_message.migrate_topic_message_table().await?;
        self.latest_message.migrate_latest_message_table().await?;
        Ok(())
    }
}
