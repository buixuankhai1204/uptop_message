use charybdis::types::{Text, Timeuuid};
use serde::{Deserialize, Serialize};
use uptop_core::common::result::AppResult;
use crate::domain::topic_message::entity::TopicMessage;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PublicTopicMessage {
    pub topic_id: Timeuuid,
    pub from_user_id: Timeuuid,
    pub message: Text,
}

impl TryFrom<&TopicMessage> for PublicTopicMessage {
    type Error = anyhow::Error;

    fn try_from(topic_message: &TopicMessage) -> AppResult<Self> {
        Ok(Self {
            topic_id: topic_message.topic_id,
            message: (*topic_message.message).parse()?,
            from_user_id: topic_message.from_user_id,
        })
    }
}
