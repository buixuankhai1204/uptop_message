use super::{
    response::PublicTopicMessage,
};
use crate::application::topic_message::request::{RequestGetMessagesByTopicId, RequestUpdateTopicMessage};
use crate::domain::topic_message::{repository::TopicMessageRepository};
use std::{future::Future, sync::Arc};
use uptop_core::common::result::AppResult;
use crate::domain::topic_message::entity::TopicMessage;

pub trait TopicMessageAppInterface: Clone + Send + Sync + 'static {
    fn find_list_messages_by_topic_id(
        &self,
        query: &RequestGetMessagesByTopicId,
    ) -> impl Future<Output=AppResult<Vec<PublicTopicMessage>>> + Send;

    fn update_topic_message(
        &self,
        topic_message: &RequestUpdateTopicMessage,
    ) -> impl Future<Output=AppResult<PublicTopicMessage>> + Send;
}

#[derive(Clone, Debug)]
pub struct TopicMessageApp<TP>
where
    TP: TopicMessageRepository,
{
    topic_message_repo: Arc<TP>,
}

impl<TP> TopicMessageApp<TP>
where
    TP: TopicMessageRepository,
{
    pub fn new(topic_message_repo: Arc<TP>) -> Self {
        Self { topic_message_repo }
    }
}

impl<TP> TopicMessageAppInterface for TopicMessageApp<TP>
where
    TP: TopicMessageRepository,
{
    async fn find_list_messages_by_topic_id(
        &self,
        query: &RequestGetMessagesByTopicId,
    ) -> AppResult<Vec<PublicTopicMessage>> {
        let mut result: Vec<PublicTopicMessage> = vec![];
        self.topic_message_repo
            .find_topic_message_by_partition_key(query)
            .await
            .map(|ref topic_message: Vec<TopicMessage>| {
                for item in topic_message.iter() {
                    result.push(item.try_into().expect("Can not parse data!"));
                }
            })
            .expect("TODO: panic message");
        Ok(result)
    }

    async fn update_topic_message(&self, topic_message: &RequestUpdateTopicMessage) -> AppResult<PublicTopicMessage> {
        self.topic_message_repo
            .update_topic_message(topic_message)
            .await
            .map(|topic_message| PublicTopicMessage::try_from(&topic_message).unwrap())
    }

    // async fn get_full_field_topic_message(&self, query: &RequestGetTopicMessageByTopicMessageName) -> AppResult<TopicMessage> {
    //     self.latest_message_repo.find_latest_message(query).await
    // }
}
