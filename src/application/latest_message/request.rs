use anyhow::bail;
use charybdis::types::{Timeuuid};
use serde::{Deserialize, Serialize};
use thiserror::Error;
use uptop_core::common::result::{AppError, AppResult};
use validator::Validate;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Validate)]
pub struct RequestUpdateLatestMessage {
    pub latest_message_id: Timeuuid,
    #[validate(length(min = 3))]
    pub latest_message_content: String,
    pub topic_id: Timeuuid,
    pub user_id: Timeuuid,
}

impl RequestUpdateLatestMessage {
    pub fn try_into_domain(self) -> AppResult<Self> {
        match self.validate() {
            Ok(_) => (),
            Err(err) => bail!(AppError::BadRequest {
                msg: err.to_string()
            }),
        };

        Ok(Self {
            latest_message_id: self.latest_message_id,
            latest_message_content: self.latest_message_content,
            topic_id: self.topic_id,
            user_id: self.user_id,
        })
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Validate)]
pub struct RequestGetLatestMessagesByUserId {
    pub user_id: Timeuuid,
}

impl RequestGetLatestMessagesByUserId {
    pub fn try_into_domain(self) -> AppResult<Self> {
        match self.validate() {
            Ok(_) => (),
            Err(err) => bail!(AppError::BadRequest {
                msg: err.to_string()
            }),
        };

        Ok(Self {
            user_id: self.user_id,
        })
    }
}

#[derive(Debug, Error)]
pub enum RequestFindLatestMessageError {
    #[error("LatestMessage not found")]
    LatestMessageNotFound,
}
