use anyhow::bail;
use charybdis::types::{Text, Timeuuid};
use serde::{Deserialize, Serialize};
use thiserror::Error;
use uptop_core::common::result::{AppError, AppResult};
use validator::Validate;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Validate)]
pub struct RequestUpdateTopicUser {
    pub topic_id: Timeuuid,
    pub username: Text,
    pub user_id: Timeuuid,
}

impl RequestUpdateTopicUser {
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
            user_id: self.user_id,
        })
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Validate)]
pub struct RequestGetUsersByTopicId {
    pub topic_id: Timeuuid,
}

impl RequestGetUsersByTopicId {
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
