use super::{
    request::{
        RequestCreateTopic, RequestGetTopicByPartitionKey,
    },
    response::{PublicTopic},
};
use crate::domain::topic::{entity::Topic, repository::TopicRepository};
use std::{future::Future, sync::Arc};
use uptop_core::common::result::AppResult;
use crate::application::topic::request::{RequestGetTopicByIndexKey, RequestGetTopicByPrimaryKey, RequestUpdateTopic};

pub trait TopicAppInterface: Clone + Send + Sync + 'static {
    fn create_topic(
        &self,
        req: RequestCreateTopic,
    ) -> impl Future<Output=AppResult<PublicTopic>> + Send;

    fn find_topic_by_partition_key(
        &self,
        query: &RequestGetTopicByPartitionKey,
    ) -> impl Future<Output=AppResult<Vec<PublicTopic>>> + Send;

    fn find_topic_by_primary_key(
        &self,
        query: &RequestGetTopicByPrimaryKey,
    ) -> impl Future<Output=AppResult<PublicTopic>> + Send;

    fn find_topic_by_index_key(
        &self,
        query: &RequestGetTopicByIndexKey,
    ) -> impl Future<Output=AppResult<Vec<PublicTopic>>> + Send;


    fn update_topic(&self, topic: &RequestUpdateTopic) -> impl Future<Output=AppResult<PublicTopic>> + Send;

    // fn get_full_field_topic(
    //     &self,
    //     query: &RequestGetTopicByTopicName,
    // ) -> impl Future<Output=AppResult<Topic>> + Send;
}

#[derive(Clone, Debug)]
pub struct TopicApp<TP>
where
    TP: TopicRepository,
{
    topic_repo: Arc<TP>,
}

impl<TP> TopicApp<TP>
where
    TP: TopicRepository,
{
    pub fn new(topic_repo: Arc<TP>) -> Self {
        Self { topic_repo }
    }
}

impl<TP> TopicAppInterface for TopicApp<TP>
where
    TP: TopicRepository,
{
    async fn create_topic(&self, req: RequestCreateTopic) -> AppResult<PublicTopic> {
        self.topic_repo
            .create_topic(&Topic::try_from(req)?)
            .await
            .map(|topic| topic.try_into())?
    }

    async fn find_topic_by_partition_key(&self, query: &RequestGetTopicByPartitionKey) -> AppResult<Vec<PublicTopic>> {
        let mut result: Vec<PublicTopic> = vec![];
        self.topic_repo
            .find_topic_by_partition_key(query)
            .await
            .map(|ref topic| {
                for item in topic.iter() {
                    result.push(item.try_into().expect("Can not parse data!"));
                }
            }).expect("TODO: panic message");
        Ok(result)
    }

    async fn find_topic_by_primary_key(&self, query: &RequestGetTopicByPrimaryKey) -> AppResult<PublicTopic> {
        self.topic_repo
            .find_topic_by_primary_key(query)
            .await
            .map(|ref topic| topic.try_into())?
    }

    async fn find_topic_by_index_key(&self, query: &RequestGetTopicByIndexKey) -> AppResult<Vec<PublicTopic>> {
        let mut result: Vec<PublicTopic> = vec![];
        self.topic_repo
            .find_topic_by_index_key(query)
            .await
            .map(|ref topic| {
                for item in topic.iter() {
                    result.push(item.try_into().expect("Can not parse data!"));
                }
            }).expect("TODO: panic message");
        Ok(result)
    }

    async fn update_topic(&self, topic: &RequestUpdateTopic) -> AppResult<PublicTopic> {
        self.topic_repo
            .update_topic(topic)
            .await
            .map(|topic| topic.try_into())?
    }

    // async fn get_full_field_topic(&self, query: &RequestGetTopicByTopicName) -> AppResult<Topic> {
    //     self.topic_repo.find_topic(query).await
    // }
}
