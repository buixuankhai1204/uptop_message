use crate::application::latest_message::request::{RequestGetLatestMessagesByUserId, RequestUpdateLatestMessage};
use crate::{
    application::latest_message::request::{RequestFindLatestMessageError},
    domain::latest_message::{entity::LatestMessage, repository::LatestMessageRepository},
};
use anyhow::anyhow;
use charybdis::batch::ModelBatch;
use charybdis::operations::{Find, Insert, Update};
use charybdis::types::Text;
use scylla::batch::Batch;
use std::rc::Rc;
use charybdis::errors::CharybdisError;
use scylla::QueryResult;
use uptop_core::common::{
    db_types::CassandraCacheSession,
    result::{AppError, AppResult},
};

#[derive(Clone, Debug)]
pub struct LatestMessageRepo {
    db: CassandraCacheSession,
}

impl LatestMessageRepo {
    pub fn new(db: CassandraCacheSession) -> Self {
        Self { db }
    }

    pub async fn migrate_latest_message_table(&self) -> AppResult<()> {
        let session = self.db.lock().await;
        session
            .execute_unpaged(CREATE_TOPIC_TABLE_QUERY, ())
            .await?;
        session.execute_unpaged(CREATE_USER_ID_INDEX, ()).await?;
        session.execute_unpaged(CREATE_USER_EMAIL_INDEX, ()).await?;
        session.execute_unpaged(CREATE_USER_NAME_INDEX, ()).await?;
        Ok(())
    }
}

impl LatestMessageRepository for LatestMessageRepo {
    async fn find_latest_message_by_partition_key(
        &self,
        query: &RequestGetLatestMessagesByUserId,
    ) -> AppResult<Vec<LatestMessage>> {
        let session = self.db.lock().await;
        let result = LatestMessage {
            user_id: query.user_id.to_owned(),
            ..Default::default()
        }
            .find_by_partition_key()
            .execute(&session)
            .await;

        match result {
            Ok(latest_message) => Ok(latest_message.try_collect().await?),
            Err(err) => {
                tracing::error!("{err:?}");
                Err(anyhow!(RequestFindLatestMessageError::LatestMessageNotFound))
            }
        }
    }


    async fn update_latest_message(&self, latest_message: &RequestUpdateLatestMessage) -> AppResult<LatestMessage> {
        let session = self.db.lock().await;
        let result = LatestMessage {
            user_id: latest_message.user_id,
            latest_message_id: latest_message.latest_message_id,
            latest_message_content: (*latest_message.latest_message_content).parse()?,
            topic_id: latest_message.topic_id,
            ..Default::default()
        };

        match result.update().execute(&session).await {
            Ok(_V) => {
                Ok(result)
            }
            Err(err) => {
                tracing::error!("{err:?}");
                Err(anyhow!(RequestFindLatestMessageError::LatestMessageNotFound))
            }
        }
    }
}

static CREATE_TOPIC_TABLE_QUERY: &str = r#"
    CREATE TABLE IF NOT EXISTS uptop.latest_messages (
        latest_message_id timeuuid,
        latest_message_name text,
        display_name text,
        email text,
        password text,
        status list<text>,
        role text,
        phone_number text,
        language text,
        address text,
        country text,
        region text,
        city text,
        post_code text,
        owners list<timeuuid>,
        admins list<timeuuid>,
        organizations list<timeuuid>,
        active_organization timeuuid,
        other_emails list<text>,
        email_verify_code text,
        email_verified_at timestamp,
        password_recovery_code text,
        password_recovered_at timestamp,
        created_at timestamp,
        updated_at timestamp,
        PRIMARY KEY ((country, region, city), latest_message_id)
    ) WITH CLUSTERING ORDER BY (latest_message_id DESC);
"#;

static CREATE_USER_ID_INDEX: &str = r#"
    CREATE INDEX IF NOT EXISTS uptop_latest_message_id_index ON uptop.latest_messages (latest_message_id);
"#;

static CREATE_USER_EMAIL_INDEX: &str = r#"
    CREATE INDEX IF NOT EXISTS uptop_email_index ON uptop.latest_messages (email);
"#;

static CREATE_USER_NAME_INDEX: &str = r#"
    CREATE INDEX IF NOT EXISTS uptop_latest_message_name_index ON uptop.latest_messages (latest_message_name);
"#;
