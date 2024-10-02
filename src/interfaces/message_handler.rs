use crate::application::topic::{
    app::TopicAppInterface,
    request::{RequestCreateTopic},
};
use std::sync::Arc;
use uptop_core::common::result::AppResult;
use crate::application::latest_message::app::LatestMessageAppInterface;
use crate::application::latest_message::request::{RequestGetLatestMessagesByUserId, RequestUpdateLatestMessage};
use crate::application::latest_message::response::PublicLatestMessage;
use crate::application::notification::app::NotificationAppInterface;
use crate::application::notification::request::{RequestGetNotificationByUsername, RequestUpdateNotification};
use crate::application::notification::response::PublicNotification;
use crate::application::topic::request::{RequestGetTopicByPartitionKey, RequestUpdateTopic};
use crate::application::topic::response::PublicTopic;
use crate::application::topic_message::app::TopicMessageAppInterface;
use crate::application::topic_message::request::{RequestGetMessagesByTopicId, RequestUpdateTopicMessage};
use crate::application::topic_message::response::PublicTopicMessage;
use crate::application::topic_user::app::TopicUserAppInterface;
use crate::application::topic_user::request::{RequestGetUsersByTopicId, RequestUpdateTopicUser};
use crate::application::topic_user::response::PublicTopicUser;
use crate::application::user_topic::app::UserTopicAppInterface;
use crate::application::user_topic::request::{RequestGetTopicsByUsername, RequestUpdateUserTopic};
use crate::application::user_topic::response::PublicUserTopic;

#[derive(Clone, Debug)]
pub struct MessageHandler<
    TAI: TopicAppInterface,
    LTI: LatestMessageAppInterface,
    NI: NotificationAppInterface,
    UTI: UserTopicAppInterface,
    TUI: TopicUserAppInterface,
    TMI: TopicMessageAppInterface,
> {
    pub topic_app: Arc<TAI>,
    pub latest_message_app: Arc<LTI>,
    pub notification_app: Arc<NI>,
    pub user_topic_app: Arc<UTI>,
    pub topic_user_app: Arc<TUI>,
    pub topic_message_app: Arc<TMI>,

}
impl<
    TAI: TopicAppInterface,
    LTI: LatestMessageAppInterface,
    NI: NotificationAppInterface,
    UTI: UserTopicAppInterface,
    TUI: TopicUserAppInterface,
    TMI: TopicMessageAppInterface
> MessageHandler<TAI, LTI, NI, UTI, TUI, TMI>
{
    pub async fn on_create_new_topic(&self, payload: String,
    ) -> AppResult<String> {
        let body: RequestCreateTopic = serde_json::from_str(&payload)?;
        let req = body.try_into_domain()?;

        let result = self.topic_app.create_topic(req).await?;
        Ok(serde_json::to_string(&result)?)
    }

    pub async fn on_find_topic<UA: TopicAppInterface>(
        &self,
        payload: String,
    ) -> AppResult<Vec<PublicTopic>> {
        let query: RequestGetTopicByPartitionKey = serde_json::from_str(&payload)?;
        Ok(self.topic_app.find_topic_by_partition_key(&query).await?)
    }

    pub async fn update_topic<TA: TopicAppInterface>(
        &self,
        payload: String,
    ) -> AppResult<PublicTopic> {
        let query: RequestUpdateTopic = serde_json::from_str(&payload)?;
        Ok(self.topic_app.update_topic(&query).await?)
    }

    pub async fn on_find_notification(
        &self,
        payload: String,
    ) -> AppResult<Vec<PublicNotification>> {
        let query: RequestGetNotificationByUsername = serde_json::from_str(&payload)?;
        Ok(self.notification_app.find_list_notification_by_username(&query).await?)
    }

    pub async fn update_notification(
        &self,
        payload: String,
    ) -> AppResult<PublicNotification> {
        let query: RequestUpdateNotification = serde_json::from_str(&payload)?;
        Ok(self.notification_app.update_notification(&query).await?)
    }

    pub async fn on_find_user_topic(
        &self,
        payload: String,
    ) -> AppResult<Vec<PublicUserTopic>> {
        let query: RequestGetTopicsByUsername = serde_json::from_str(&payload)?;
        Ok(self.user_topic_app.find_list_topics_by_username(&query).await?)
    }

    pub async fn update_user_topic(
        &self,
        payload: String,
    ) -> AppResult<PublicUserTopic> {
        let query: RequestUpdateUserTopic = serde_json::from_str(&payload)?;
        Ok(self.user_topic_app.update_user_topic(&query).await?)
    }

    pub async fn on_find_topic_user(
        &self,
        payload: String,
    ) -> AppResult<Vec<PublicTopicUser>> {
        let query: RequestGetUsersByTopicId = serde_json::from_str(&payload)?;
        Ok(self.topic_user_app.find_list_users_by_topic_id(&query).await?)
    }

    pub async fn update_topic_user(
        &self,
        payload: String,
    ) -> AppResult<PublicTopicUser> {
        let query: RequestUpdateTopicUser = serde_json::from_str(&payload)?;
        Ok(self.topic_user_app.update_topic_user(&query).await?)
    }

    pub async fn on_find_topic_message(
        &self,
        payload: String,
    ) -> AppResult<Vec<PublicTopicMessage>> {
        let query: RequestGetMessagesByTopicId = serde_json::from_str(&payload)?;
        Ok(self.topic_message_app.find_list_messages_by_topic_id(&query).await?)
    }

    pub async fn update_topic_message(
        &self,
        payload: String,
    ) -> AppResult<PublicTopicMessage> {
        let query: RequestUpdateTopicMessage = serde_json::from_str(&payload)?;
        Ok(self.topic_message_app.update_topic_message(&query).await?)
    }

    pub async fn on_find_latest_message(
        &self,
        payload: String,
    ) -> AppResult<Vec<PublicLatestMessage>> {
        let query: RequestGetLatestMessagesByUserId = serde_json::from_str(&payload)?;
        Ok(self.latest_message_app.find_list_latest_messages_by_user_id(&query).await?)
    }

    pub async fn update_latest_message(
        &self,
        payload: String,
    ) -> AppResult<PublicLatestMessage> {
        let query: RequestUpdateLatestMessage = serde_json::from_str(&payload)?;
        Ok(self.latest_message_app.update_latest_message(&query).await?)
    }
}

