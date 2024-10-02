use super::entity::Topic;
use crate::application::topic::request::{
    RequestGetTopicByIndexKey, RequestGetTopicByPartitionKey, RequestGetTopicByPrimaryKey,
    RequestUpdateTopic,
};
use std::future::Future;
use uptop_core::common::result::AppResult;

pub trait TopicRepository: Clone + Send + Sync + 'static {
    fn create_topic<'c>(
        &self,
        topic: &'c Topic,
    ) -> impl Future<Output = AppResult<&'c Topic>> + Send;

    fn find_topic_by_partition_key(
        &self,
        query: &RequestGetTopicByPartitionKey,
    ) -> impl Future<Output = AppResult<Vec<Topic>>> + Send;

    fn find_topic_by_primary_key(
        &self,
        query: &RequestGetTopicByPrimaryKey,
    ) -> impl Future<Output = AppResult<Topic>> + Send;

    fn find_topic_by_index_key(
        &self,
        query: &RequestGetTopicByIndexKey,
    ) -> impl Future<Output = AppResult<Vec<Topic>>> + Send;

    fn update_topic<'u>(
        &self,
        topic: &'u RequestUpdateTopic,
    ) -> impl Future<Output = AppResult<Topic>> + Send;
}
