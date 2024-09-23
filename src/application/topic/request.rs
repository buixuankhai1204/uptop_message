use std::cell::{Cell, RefCell};
use anyhow::bail;
use charybdis::types::{Timestamp, Timeuuid};
use scylla::{SerializeRow, SerializeValue};
use serde::{Deserialize, Serialize};
use thiserror::Error;
use uptop_core::common::{
    result::{AppError, AppResult},
};
use validator::Validate;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Validate)]
pub struct RequestCreateTopic {
    pub topic_name: String,
    #[validate(length(min = 3))]
    pub topic_description: Option<String>,
    pub topic_owners: Vec<String>,
    pub topic_admins: Vec<String>,
}

impl RequestCreateTopic {
    pub fn try_into_domain(self) -> AppResult<Self> {
        match self.validate() {
            Ok(_) => (),
            Err(err) => bail!(AppError::BadRequest {
                msg: err.to_string()
            }),
        };


        Ok(Self {
            topic_name: self.topic_name,
            topic_description: self.topic_description,
            topic_owners: self.topic_owners,
            topic_admins: self.topic_admins,
        })
    }
}


#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Validate)]
pub struct RequestGetTopicByPartitionKey {
    pub topic_id: Timeuuid,
}

impl RequestGetTopicByPartitionKey {
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

#[derive(Default, Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RequestGetTopicByIndexKey {
    pub topic_name: String,
}

#[derive(Default, Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RequestGetTopicByPrimaryKey {
    pub topic_id: Timeuuid,
    pub created_at: Timestamp,
}

#[derive(Debug, Error)]
pub enum RequestFindTopicError {
    #[error("Topic not found")]
    TopicNotFound,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Validate)]
pub struct RequestUpdateTopic {
    pub topic_id: Timeuuid,
    pub topic_name: Option<String>,
    pub topic_description: Option<String>,
    pub push_to_owners: RefCell<Option<Vec<String>>>,
    pub pop_to_owners: Option<Vec<String>>,
    pub push_to_admins: Option<Vec<String>>,
    pub pop_to_admins: Option<Vec<String>>,
}

impl RequestUpdateTopic {
    pub fn try_into_domain(self) -> AppResult<Self> {
        match self.validate() {
            Ok(_) => (),
            Err(err) => bail!(AppError::BadRequest {
                msg: err.to_string()
            }),
        };

        Ok(Self {
            topic_name: self.topic_name,
            topic_description: self.topic_description,
            push_to_owners: self.push_to_owners,
            pop_to_owners: self.pop_to_owners,
            push_to_admins: self.push_to_admins,
            pop_to_admins: self.pop_to_admins,
        })
    }
}

