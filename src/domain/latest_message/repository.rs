use super::entity::LatestMessage;
use crate::application::latest_message::request::{
    RequestGetLatestMessagesByUserId,
    RequestUpdateLatestMessage,
};
use std::future::Future;
use uptop_core::common::result::AppResult;

pub trait LatestMessageRepository: Clone + Send + Sync + 'static {
    fn find_latest_message_by_partition_key(
        &self,
        query: &RequestGetLatestMessagesByUserId,
    ) -> impl Future<Output=AppResult<Vec<LatestMessage>>> + Send;

    fn update_latest_message(
        &self,
        latest_message: &RequestUpdateLatestMessage,
    ) -> impl Future<Output=AppResult<LatestMessage>> + Send;
}
