use super::{
    response::PublicUserTopic,
};
use crate::application::user_topic::request::{RequestGetTopicsByUsername, RequestUpdateUserTopic};
use crate::domain::user_topic::{repository::UserTopicRepository};
use std::{future::Future, sync::Arc};
use uptop_core::common::result::AppResult;
use crate::domain::user_topic::entity::UserTopic;

pub trait UserTopicAppInterface: Clone + Send + Sync + 'static {
    fn find_list_topics_by_username(
        &self,
        query: &RequestGetTopicsByUsername,
    ) -> impl Future<Output=AppResult<Vec<PublicUserTopic>>> + Send;

    fn update_user_topic(
        &self,
        user_topic: &RequestUpdateUserTopic,
    ) -> impl Future<Output=AppResult<PublicUserTopic>> + Send;
}

#[derive(Clone, Debug)]
pub struct UserTopicApp<TP>
where
    TP: UserTopicRepository,
{
    user_topic_repo: Arc<TP>,
}

impl<TP> UserTopicApp<TP>
where
    TP: UserTopicRepository,
{
    pub fn new(user_topic_repo: Arc<TP>) -> Self {
        Self { user_topic_repo }
    }
}

impl<TP> UserTopicAppInterface for UserTopicApp<TP>
where
    TP: UserTopicRepository,
{
    async fn find_list_topics_by_username(
        &self,
        query: &RequestGetTopicsByUsername,
    ) -> AppResult<Vec<PublicUserTopic>> {
        let mut result: Vec<PublicUserTopic> = vec![];
        self.user_topic_repo
            .find_user_topics_by_partition_key(query)
            .await
            .map(|ref user_topic: Vec<UserTopic>| {
                for item in user_topic.iter() {
                    result.push(item.try_into().expect("Can not parse data!"));
                }
            })
            .expect("TODO: panic message");
        Ok(result)
    }

    async fn update_user_topic(&self, user_topic: &RequestUpdateUserTopic) -> AppResult<PublicUserTopic> {
        self.user_topic_repo
            .update_user_topics(user_topic)
            .await
            .map(|user_topic| PublicUserTopic::try_from(&user_topic).unwrap())
    }

    // async fn get_full_field_user_topic(&self, query: &RequestGetUserTopicByUserTopicName) -> AppResult<UserTopic> {
    //     self.latest_message_repo.find_latest_message(query).await
    // }
}
