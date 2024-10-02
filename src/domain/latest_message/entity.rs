use charybdis::{
    macros::charybdis_model,
    types::{ Text, Timestamp, Timeuuid},
};
use serde::{Deserialize, Serialize};

#[charybdis_model(
    table_name = uptop.latest_messages,
    partition_keys = [user_id],
    clustering_keys = [],
    global_secondary_indexes = [],
    
)]
#[derive(Default, Debug, Serialize, Deserialize)]
pub struct LatestMessage {
    pub latest_message_id: Timeuuid,
    pub latest_message_content: Text,
    pub topic_id: Timeuuid,
    pub user_id: Timeuuid,
    pub created_at: Timestamp,
    pub updated_at: Timestamp,
}
