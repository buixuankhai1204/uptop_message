use charybdis::{
    macros::charybdis_model,
    types::{Text, Timestamp, Timeuuid},
};
use serde::{Deserialize, Serialize};

#[charybdis_model(
    table_name = uptop.topic_messages,
    partition_keys = [topic_id],
    clustering_keys = [created_at],
    global_secondary_indexes = [],

)]
#[derive(Default, Debug, Serialize, Deserialize)]
pub struct TopicMessage {
    pub topic_id: Timeuuid,
    pub from_user_id: Timeuuid,
    pub message: Text,
    pub created_at: Timestamp,
}
