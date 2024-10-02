use super::entity::UserTopic;
use std::future::Future;
use uptop_core::common::result::AppResult;
use crate::application::user_topic::request::{RequestGetTopicsByUsername, RequestUpdateUserTopic};

pub trait UserTopicRepository: Clone + Send + Sync + 'static {
    fn find_user_topics_by_partition_key(
        &self,
        query: &RequestGetTopicsByUsername,
    ) -> impl Future<Output=AppResult<Vec<UserTopic>>> + Send;

    fn update_user_topics(
        &self,
        topic_message: &RequestUpdateUserTopic,
    ) -> impl Future<Output=AppResult<UserTopic>> + Send;
}
