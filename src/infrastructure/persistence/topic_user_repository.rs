use crate::application::topic_user::request::{RequestGetUsersByTopicId, RequestFindLatestMessageError, RequestUpdateTopicUser};
use crate::{
    domain::topic_user::{entity::TopicUser, repository::TopicUserRepository},
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
pub struct TopicUserRepo {
    db: CassandraCacheSession,
}

impl TopicUserRepo {
    pub fn new(db: CassandraCacheSession) -> Self {
        Self { db }
    }

    pub async fn migrate_topic_user_table(&self) -> AppResult<()> {
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

impl TopicUserRepository for TopicUserRepo {
    async fn find_topic_users_by_partition_key(
        &self,
        query: &RequestGetUsersByTopicId,
    ) -> AppResult<Vec<TopicUser>> {
        let session = self.db.lock().await;
        let result = TopicUser {
            topic_id: query.topic_id,
            ..Default::default()
        }
            .find_by_partition_key()
            .execute(&session)
            .await;

        match result {
            Ok(topic_user) => Ok(topic_user.try_collect().await?),
            Err(err) => {
                tracing::error!("{err:?}");
                Err(anyhow!(RequestFindLatestMessageError::LatestMessageNotFound))
            }
        }
    }


    async fn update_topic_users(&self, topic_user: &RequestUpdateTopicUser) -> AppResult<TopicUser> {
        let session = self.db.lock().await;
        let result = TopicUser {
            user_id: topic_user.user_id,
            topic_id: topic_user.topic_id,
            username: (*topic_user.username).parse()?,
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
    CREATE TABLE IF NOT EXISTS uptop.topic_users (
        topic_user_id timeuuid,
        topic_user_name text,
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
        PRIMARY KEY ((country, region, city), topic_user_id)
    ) WITH CLUSTERING ORDER BY (topic_user_id DESC);
"#;

static CREATE_USER_ID_INDEX: &str = r#"
    CREATE INDEX IF NOT EXISTS uptop_topic_user_id_index ON uptop.topic_users (topic_user_id);
"#;

static CREATE_USER_EMAIL_INDEX: &str = r#"
    CREATE INDEX IF NOT EXISTS uptop_email_index ON uptop.topic_users (email);
"#;

static CREATE_USER_NAME_INDEX: &str = r#"
    CREATE INDEX IF NOT EXISTS uptop_topic_user_name_index ON uptop.topic_users (topic_user_name);
"#;
