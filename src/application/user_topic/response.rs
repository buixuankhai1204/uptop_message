use charybdis::types::{Text, Timestamp, Timeuuid};
use serde::{Deserialize, Serialize};
use uptop_core::common::result::AppResult;
use crate::domain::user_topic::entity::UserTopic;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PublicUserTopic {
    pub topic_id: Timeuuid,
    pub username: Text,
    pub created_at: Timestamp,
}

impl TryFrom<&UserTopic> for PublicUserTopic {
    type Error = anyhow::Error;

    fn try_from(user_topic: &UserTopic) -> AppResult<Self> {
        Ok(Self {
            topic_id: user_topic.topic_id,
            username: (*user_topic.username).parse()?,
            created_at: user_topic.created_at,
        })
    }
}
