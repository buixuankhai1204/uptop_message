use charybdis::{
    macros::charybdis_model,
    types::{Text, Timestamp, Timeuuid},
};
use serde::{Deserialize, Serialize};

#[charybdis_model(
    table_name = uptop.user_topic,
    partition_keys = [topic_id],
    clustering_keys = [created_at],
    global_secondary_indexes = [],

)]
#[derive(Default, Debug, Serialize, Deserialize)]
pub struct UserTopic {
    pub topic_id: Timeuuid,
    pub username: Text,
    pub created_at: Timestamp,
}