use super::entity::TopicMessage;
use crate::application::topic_message::request::{RequestGetMessagesByTopicId, RequestUpdateTopicMessage};
use std::future::Future;
use uptop_core::common::result::AppResult;

pub trait TopicMessageRepository: Clone + Send + Sync + 'static {
    fn find_topic_message_by_partition_key(
        &self,
        query: &RequestGetMessagesByTopicId,
    ) -> impl Future<Output=AppResult<Vec<TopicMessage>>> + Send;

    fn update_topic_message(
        &self,
        topic_message: &RequestUpdateTopicMessage,
    ) -> impl Future<Output=AppResult<TopicMessage>> + Send;
}
