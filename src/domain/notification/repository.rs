use super::entity::Notification;
use std::future::Future;
use uptop_core::common::result::AppResult;
use crate::application::notification::request::{RequestGetNotificationByUsername, RequestUpdateNotification};

pub trait NotificationRepository: Clone + Send + Sync + 'static {
    fn find_notifications_by_partition_key(
        &self,
        query: &RequestGetNotificationByUsername,
    ) -> impl Future<Output=AppResult<Vec<Notification>>> + Send;

    fn update_notifications(
        &self,
        topic_message: &RequestUpdateNotification,
    ) -> impl Future<Output=AppResult<Notification>> + Send;
}
