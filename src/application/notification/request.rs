use anyhow::bail;
use charybdis::types::{Text, Timeuuid};
use serde::{Deserialize, Serialize};
use thiserror::Error;
use uptop_core::common::result::{AppError, AppResult};
use validator::Validate;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Validate)]
pub struct RequestUpdateNotification {
    pub topic_id: Timeuuid,
    pub username: Text,
    pub from_user: Text,
    pub message: Text,
}

impl RequestUpdateNotification {
    pub fn try_into_domain(self) -> AppResult<Self> {
        match self.validate() {
            Ok(_) => (),
            Err(err) => bail!(AppError::BadRequest {
                msg: err.to_string()
            }),
        };

        Ok(Self {
            topic_id: self.topic_id,
            username: self.username,
            from_user: self.from_user,
            message: self.message,
        })
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Validate)]
pub struct RequestGetNotificationByUsername {
    pub username: Text,
}

impl RequestGetNotificationByUsername {
    pub fn try_into_domain(self) -> AppResult<Self> {
        match self.validate() {
            Ok(_) => (),
            Err(err) => bail!(AppError::BadRequest {
                msg: err.to_string()
            }),
        };

        Ok(Self {
            username: self.username,
        })
    }
}

#[derive(Debug, Error)]
pub enum RequestFindLatestMessageError {
    #[error("LatestMessage not found")]
    LatestMessageNotFound,
}
