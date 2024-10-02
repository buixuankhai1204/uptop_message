use super::{
    response::PublicTopicUser,
};
use crate::application::topic_user::request::{RequestGetUsersByTopicId, RequestUpdateTopicUser};
use crate::domain::topic_user::{repository::TopicUserRepository};
use std::{future::Future, sync::Arc};
use uptop_core::common::result::AppResult;
use crate::domain::topic_user::entity::TopicUser;

pub trait TopicUserAppInterface: Clone + Send + Sync + 'static {
    fn find_list_users_by_topic_id(
        &self,
        query: &RequestGetUsersByTopicId,
    ) -> impl Future<Output=AppResult<Vec<PublicTopicUser>>> + Send;

    fn update_topic_user(
        &self,
        topic_user: &RequestUpdateTopicUser,
    ) -> impl Future<Output=AppResult<PublicTopicUser>> + Send;
}

#[derive(Clone, Debug)]
pub struct TopicUserApp<TP>
where
    TP: TopicUserRepository,
{
    topic_user_repo: Arc<TP>,
}

impl<TP> TopicUserApp<TP>
where
    TP: TopicUserRepository,
{
    pub fn new(topic_user_repo: Arc<TP>) -> Self {
        Self { topic_user_repo }
    }
}

impl<TP> TopicUserAppInterface for TopicUserApp<TP>
where
    TP: TopicUserRepository,
{
    async fn find_list_users_by_topic_id(
        &self,
        query: &RequestGetUsersByTopicId,
    ) -> AppResult<Vec<PublicTopicUser>> {
        let mut result: Vec<PublicTopicUser> = vec![];
        self.topic_user_repo
            .find_topic_users_by_partition_key(query)
            .await
            .map(|ref topic_user: Vec<TopicUser>| {
                for item in topic_user.iter() {
                    result.push(item.try_into().expect("Can not parse data!"));
                }
            })
            .expect("TODO: panic message");
        Ok(result)
    }

    async fn update_topic_user(&self, topic_user: &RequestUpdateTopicUser) -> AppResult<PublicTopicUser> {
        self.topic_user_repo
            .update_topic_users(topic_user)
            .await
            .map(|topic_user| PublicTopicUser::try_from(&topic_user).unwrap())
    }

    // async fn get_full_field_topic_user(&self, query: &RequestGetTopicUserByTopicUserName) -> AppResult<TopicUser> {
    //     self.latest_message_repo.find_latest_message(query).await
    // }
}
