use super::{
    response::PublicLatestMessage,
};
use crate::application::latest_message::request::{RequestGetLatestMessagesByUserId, RequestUpdateLatestMessage};
use crate::domain::latest_message::{repository::LatestMessageRepository};
use std::{future::Future, sync::Arc};
use uptop_core::common::result::AppResult;
use crate::domain::latest_message::entity::LatestMessage;

pub trait LatestMessageAppInterface: Clone + Send + Sync + 'static {
    fn find_list_latest_messages_by_user_id(
        &self,
        query: &RequestGetLatestMessagesByUserId,
    ) -> impl Future<Output=AppResult<Vec<PublicLatestMessage>>> + Send;

    fn update_latest_message(
        &self,
        latest_message: &RequestUpdateLatestMessage,
    ) -> impl Future<Output=AppResult<PublicLatestMessage>> + Send;
}

#[derive(Clone, Debug)]
pub struct LatestMessageApp<TP>
where
    TP: LatestMessageRepository,
{
    latest_message_repo: Arc<TP>,
}

impl<TP> LatestMessageApp<TP>
where
    TP: LatestMessageRepository,
{
    pub fn new(latest_message_repo: Arc<TP>) -> Self {
        Self { latest_message_repo }
    }
}

impl<TP> LatestMessageAppInterface for LatestMessageApp<TP>
where
    TP: LatestMessageRepository,
{
    async fn find_list_latest_messages_by_user_id(
        &self,
        query: &RequestGetLatestMessagesByUserId,
    ) -> AppResult<Vec<PublicLatestMessage>> {
        let mut result: Vec<PublicLatestMessage> = vec![];
        self.latest_message_repo
            .find_latest_message_by_partition_key(query)
            .await
            .map(|ref latest_message: Vec<LatestMessage>| {
                for item in latest_message.iter() {
                    result.push(item.try_into().expect("Can not parse data!"));
                }
            })
            .expect("TODO: panic message");
        Ok(result)
    }

    async fn update_latest_message(&self, latest_message: &RequestUpdateLatestMessage) -> AppResult<PublicLatestMessage> {
        self.latest_message_repo
            .update_latest_message(latest_message)
            .await
            .map(|latest_message| PublicLatestMessage::try_from(&latest_message).unwrap())
    }

    // async fn get_full_field_latest_message(&self, query: &RequestGetLatestMessageByLatestMessageName) -> AppResult<LatestMessage> {
    //     self.latest_message_repo.find_latest_message(query).await
    // }
}
