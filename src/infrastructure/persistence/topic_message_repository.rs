use crate::application::topic_message::request::{RequestGetMessagesByTopicId, RequestUpdateTopicMessage};
use crate::{
    application::topic_message::request::{RequestFindLatestMessageError},
    domain::topic_message::{entity::TopicMessage, repository::TopicMessageRepository},
};
use anyhow::anyhow;
use charybdis::operations::{Find, Update};
use uptop_core::common::{
    db_types::CassandraCacheSession,
    result::{AppError, AppResult},
};

#[derive(Clone, Debug)]
pub struct TopicMessageRepo {
    db: CassandraCacheSession,
}

impl TopicMessageRepo {
    pub fn new(db: CassandraCacheSession) -> Self {
        Self { db }
    }

    pub async fn migrate_topic_message_table(&self) -> AppResult<()> {
        let session = self.db.lock().await;
        session.execute_unpaged(CREATE_TOPIC_TABLE_QUERY, ()).await?;
        session.execute_unpaged(CREATE_USER_ID_INDEX, ()).await?;
        session.execute_unpaged(CREATE_USER_EMAIL_INDEX, ()).await?;
        session.execute_unpaged(CREATE_USER_NAME_INDEX, ()).await?;
        Ok(())
    }
}

impl TopicMessageRepository for TopicMessageRepo {
    async fn find_topic_message_by_partition_key(
        &self,
        query: &RequestGetMessagesByTopicId,
    ) -> AppResult<Vec<TopicMessage>> {
        let session = self.db.lock().await;
        let result = TopicMessage {
            topic_id: query.topic_id.to_owned(),
            ..Default::default()
        }
            .find_by_partition_key()
            .execute(&session)
            .await;

        match result {
            Ok(topic_message) => Ok(topic_message.try_collect().await?),
            Err(err) => {
                tracing::error!("{err:?}");
                Err(anyhow!(RequestFindLatestMessageError::LatestMessageNotFound))
            }
        }
    }


    async fn update_topic_message(&self, topic_message: &RequestUpdateTopicMessage) -> AppResult<TopicMessage> {
        let session = self.db.lock().await;
        let result = TopicMessage {
            from_user_id: topic_message.from_user_id,
            topic_id: topic_message.topic_id,
            message: (*topic_message.message).parse()?,
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
    CREATE TABLE IF NOT EXISTS uptop.topic_messages (
        topic_message_id timeuuid,
        topic_message_name text,
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
        PRIMARY KEY ((country, region, city), topic_message_id)
    ) WITH CLUSTERING ORDER BY (topic_message_id DESC);
"#;

static CREATE_USER_ID_INDEX: &str = r#"
    CREATE INDEX IF NOT EXISTS uptop_topic_message_id_index ON uptop.topic_messages (topic_message_id);
"#;

static CREATE_USER_EMAIL_INDEX: &str = r#"
    CREATE INDEX IF NOT EXISTS uptop_email_index ON uptop.topic_messages (email);
"#;

static CREATE_USER_NAME_INDEX: &str = r#"
    CREATE INDEX IF NOT EXISTS uptop_topic_message_name_index ON uptop.topic_messages (topic_message_name);
"#;
