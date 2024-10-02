use charybdis::types::{Text, Timeuuid};
use serde::{Deserialize, Serialize};
use uptop_core::common::result::AppResult;
use crate::domain::latest_message::entity::LatestMessage;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PublicLatestMessage {
    pub latest_message_id: Timeuuid,
    pub latest_message_content: Text,
    pub topic_id: Timeuuid,
    pub user_id: Timeuuid,
}

impl TryFrom<&LatestMessage> for PublicLatestMessage {
    type Error = anyhow::Error;

    fn try_from(latest_message: &LatestMessage) -> AppResult<Self> {
        Ok(Self {
            latest_message_id: latest_message.latest_message_id,
            topic_id: latest_message.topic_id,
            user_id: latest_message.user_id,
            latest_message_content: (*latest_message.latest_message_content).parse()?,
        })
    }
}
