use charybdis::types::{Text, Timestamp};
use serde::{Deserialize, Serialize};
use uptop_core::common::result::AppResult;
use crate::domain::notification::entity::Notification;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PublicNotification {
    pub username: Text,
    pub from_user: Text,
    pub message: Text,
    pub created_at: Timestamp,
}

impl TryFrom<&Notification> for PublicNotification {
    type Error = anyhow::Error;

    fn try_from(notification: &Notification) -> AppResult<Self> {
        Ok(Self {
            username: (*notification.username).parse()?,
            from_user: (*notification.from_user).parse()?,
            message: (*notification.message).parse()?,
            created_at: notification.created_at,
        })
    }
}
