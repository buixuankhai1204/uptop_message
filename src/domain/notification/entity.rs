use charybdis::{
    macros::charybdis_model,
    types::{Text, Timestamp, Timeuuid},
};
use serde::{Deserialize, Serialize};

#[charybdis_model(
    table_name = uptop.notification,
    partition_keys = [username],
    clustering_keys = [created_at],
    global_secondary_indexes = [],

)]
#[derive(Default, Debug, Serialize, Deserialize)]
pub struct Notification {
    pub topic_id: Timeuuid,
    pub username: Text,
    pub from_user: Text,
    pub message: Text,
    pub created_at: Timestamp,
}