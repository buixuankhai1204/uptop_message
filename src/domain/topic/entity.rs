use crate::application::topic::request::{RequestCreateTopic, RequestUpdateTopic};
use charybdis::{
    macros::charybdis_model,
    types::{List, Text, Timestamp, Timeuuid},
};
use chrono::Utc;
use serde::{Deserialize, Serialize};
use uptop_core::common::{result::AppResult, utils::now_timeuuid};

#[charybdis_model(
    table_name = uptop.topics,
    partition_keys = [topic_id],
    clustering_keys = [created_at],
    global_secondary_indexes = [topic_name],
    table_options = r#"
        CLUSTERING ORDER BY (created_adt DESC);
    "#
)]
#[derive(Default, Debug, Serialize, Deserialize)]
pub struct Topic {
    pub topic_id: Timeuuid,
    pub topic_name: Text,
    pub topic_description: Option<Text>,
    pub topic_owners: List<Text>,
    pub topic_admins: List<Text>,
    pub created_at: Timestamp,
    pub updated_at: Timestamp,
}

impl TryFrom<RequestCreateTopic> for Topic {
    type Error = anyhow::Error;

    fn try_from(value: RequestCreateTopic) -> AppResult<Self> {
        let mut topic = Topic {
            topic_id: now_timeuuid(),
            created_at: Utc::now(),
            updated_at: Utc::now(),
            ..Default::default()
        };
        topic.topic_name = value.topic_name;
        topic.topic_owners = value.topic_owners;
        topic.topic_admins = value.topic_admins;
        Ok(topic)
    }
}

impl TryFrom<RequestUpdateTopic> for Topic {
    type Error = anyhow::Error;

    fn try_from(_value: RequestUpdateTopic) -> AppResult<Self> {
        todo!()
    }
}
