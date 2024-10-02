use crate::application::user_topic::request::{RequestGetTopicsByUsername, RequestFindLatestMessageError, RequestUpdateUserTopic};
use crate::{
    domain::user_topic::{entity::UserTopic, repository::UserTopicRepository},
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
pub struct UserTopicRepo {
    db: CassandraCacheSession,
}

impl UserTopicRepo {
    pub fn new(db: CassandraCacheSession) -> Self {
        Self { db }
    }

    pub async fn migrate_user_topic_table(&self) -> AppResult<()> {
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

impl UserTopicRepository for UserTopicRepo {
    async fn find_user_topics_by_partition_key(
        &self,
        query: &RequestGetTopicsByUsername,
    ) -> AppResult<Vec<UserTopic>> {
        let session = self.db.lock().await;
        let result = UserTopic {
            username: (*query.username).parse()?,
            ..Default::default()
        }
            .find_by_partition_key()
            .execute(&session)
            .await;

        match result {
            Ok(user_topic) => Ok(user_topic.try_collect().await?),
            Err(err) => {
                tracing::error!("{err:?}");
                Err(anyhow!(RequestFindLatestMessageError::LatestMessageNotFound))
            }
        }
    }


    async fn update_user_topics(&self, user_topic: &RequestUpdateUserTopic) -> AppResult<UserTopic> {
        let session = self.db.lock().await;
        let result = UserTopic {
            topic_id: user_topic.topic_id,
            username: (*user_topic.username).parse()?,
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
    CREATE TABLE IF NOT EXISTS uptop.user_topics (
        user_topic_id timeuuid,
        user_topic_name text,
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
        PRIMARY KEY ((country, region, city), user_topic_id)
    ) WITH CLUSTERING ORDER BY (user_topic_id DESC);
"#;

static CREATE_USER_ID_INDEX: &str = r#"
    CREATE INDEX IF NOT EXISTS uptop_user_topic_id_index ON uptop.user_topics (user_topic_id);
"#;

static CREATE_USER_EMAIL_INDEX: &str = r#"
    CREATE INDEX IF NOT EXISTS uptop_email_index ON uptop.user_topics (email);
"#;

static CREATE_USER_NAME_INDEX: &str = r#"
    CREATE INDEX IF NOT EXISTS uptop_user_topic_name_index ON uptop.user_topics (user_topic_name);
"#;
