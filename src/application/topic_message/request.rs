use anyhow::bail;
use charybdis::types::{Text, Timeuuid};
use serde::{Deserialize, Serialize};
use thiserror::Error;
use uptop_core::common::result::{AppError, AppResult};
use validator::Validate;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Validate)]
pub struct RequestUpdateTopicMessage {
    pub topic_id: Timeuuid,
    pub from_user_id: Timeuuid,
    pub message: Text,
}

impl RequestUpdateTopicMessage {
    pub fn try_into_domain(self) -> AppResult<Self> {
        match self.validate() {
            Ok(_) => (),
            Err(err) => bail!(AppError::BadRequest {
                msg: err.to_string()
            }),
        };

        Ok(Self {
            topic_id: self.topic_id,
            from_user_id: self.from_user_id,
            message: self.message,
        })
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Validate)]
pub struct RequestGetMessagesByTopicId {
    pub topic_id: Timeuuid,
}

impl RequestGetMessagesByTopicId {
    pub fn try_into_domain(self) -> AppResult<Self> {
        match self.validate() {
            Ok(_) => (),
            Err(err) => bail!(AppError::BadRequest {
                msg: err.to_string()
            }),
        };

        Ok(Self {
            topic_id: self.topic_id,
        })
    }
}

#[derive(Debug, Error)]
pub enum RequestFindLatestMessageError {
    #[error("LatestMessage not found")]
    LatestMessageNotFound,
}
