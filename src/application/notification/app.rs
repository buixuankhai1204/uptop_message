use super::{
    response::PublicNotification,
};
use crate::application::notification::request::{RequestGetNotificationByUsername, RequestUpdateNotification};
use crate::domain::notification::{repository::NotificationRepository};
use std::{future::Future, sync::Arc};
use uptop_core::common::result::AppResult;
use crate::domain::notification::entity::Notification;

pub trait NotificationAppInterface: Clone + Send + Sync + 'static {
    fn find_list_notification_by_username(
        &self,
        query: &RequestGetNotificationByUsername,
    ) -> impl Future<Output=AppResult<Vec<PublicNotification>>> + Send;

    fn update_notification(
        &self,
        notification: &RequestUpdateNotification,
    ) -> impl Future<Output=AppResult<PublicNotification>> + Send;
}

#[derive(Clone, Debug)]
pub struct NotificationApp<TP>
where
    TP: NotificationRepository,
{
    notification_repo: Arc<TP>,
}

impl<TP> NotificationApp<TP>
where
    TP: NotificationRepository,
{
    pub fn new(notification_repo: Arc<TP>) -> Self {
        Self { notification_repo }
    }
}

impl<TP> NotificationAppInterface for NotificationApp<TP>
where
    TP: NotificationRepository,
{
    async fn find_list_notification_by_username(
        &self,
        query: &RequestGetNotificationByUsername,
    ) -> AppResult<Vec<PublicNotification>> {
        let mut result: Vec<PublicNotification> = vec![];
        self.notification_repo
            .find_notifications_by_partition_key(query)
            .await
            .map(|ref notification: Vec<Notification>| {
                for item in notification.iter() {
                    result.push(item.try_into().expect("Can not parse data!"));
                }
            })
            .expect("TODO: panic message");
        Ok(result)
    }

    async fn update_notification(&self, notification: &RequestUpdateNotification) -> AppResult<PublicNotification> {
        self.notification_repo
            .update_notifications(notification)
            .await
            .map(|notification| PublicNotification::try_from(&notification).unwrap())
    }

    // async fn get_full_field_notification(&self, query: &RequestGetNotificationByNotificationName) -> AppResult<Notification> {
    //     self.latest_message_repo.find_latest_message(query).await
    // }
}
