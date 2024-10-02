use charybdis::types::{Text, Timeuuid};
use serde::{Deserialize, Serialize};
use uptop_core::common::result::AppResult;
use crate::domain::topic_user::entity::TopicUser;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PublicTopicUser {
    pub topic_id: Timeuuid,
    pub user_id: Timeuuid,
    pub username: Text,
}

impl TryFrom<&TopicUser> for PublicTopicUser {
    type Error = anyhow::Error;

    fn try_from(topic_user: &TopicUser) -> AppResult<Self> {
        Ok(Self {
            topic_id: topic_user.topic_id,
            username: (*topic_user.username).parse()?,
            user_id: topic_user.user_id,
        })
    }
}
