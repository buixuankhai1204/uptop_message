use super::entity::TopicUser;
use std::future::Future;
use uptop_core::common::result::AppResult;
use crate::application::topic_user::request::{RequestGetUsersByTopicId, RequestUpdateTopicUser};

pub trait TopicUserRepository: Clone + Send + Sync + 'static {
    fn find_topic_users_by_partition_key(
        &self,
        query: &RequestGetUsersByTopicId,
    ) -> impl Future<Output=AppResult<Vec<TopicUser>>> + Send;

    fn update_topic_users(
        &self,
        topic_message: &RequestUpdateTopicUser,
    ) -> impl Future<Output=AppResult<TopicUser>> + Send;
}
