use charybdis::types::{Text, Timestamp};
use serde::{Deserialize, Serialize};
use uptop_core::common::result::AppResult;
use crate::domain::topic::entity::Topic;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PublicTopic {
    pub topic_name: Text,
    pub topic_description: Option<Text>,
    pub topic_owners: Vec<Text>,
    pub topic_admins: Vec<Text>,
    pub created_at: Timestamp,
}

impl TryFrom<&Topic> for PublicTopic {
    type Error = anyhow::Error;

    fn try_from(topic: &Topic) -> AppResult<Self> {
        Ok(Self {
            topic_name: topic.topic_name.to_owned(),
            topic_description: topic.topic_description.to_owned(),
            topic_owners: topic.topic_owners.to_owned(),
            topic_admins: topic.topic_owners.to_owned(),
            created_at: topic.created_at,
        })
    }
}
