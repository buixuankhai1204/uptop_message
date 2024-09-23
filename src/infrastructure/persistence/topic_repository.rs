use std::rc::Rc;
use crate::{
    application::topic::request::{
        RequestFindTopicError, RequestGetTopicByPartitionKey,
    },
    domain::topic::{entity::Topic, repository::TopicRepository},
};
use anyhow::anyhow;
use charybdis::{
    operations::{Find, Insert, Update},
};
use charybdis::batch::ModelBatch;
use charybdis::types::Text;
use scylla::batch::Batch;
use uptop_core::common::{
    db_types::CassandraCacheSession,
    result::{AppError, AppResult},
};
use crate::application::topic::request::{RequestGetTopicByIndexKey, RequestGetTopicByPrimaryKey, RequestUpdateTopic};

#[derive(Clone, Debug)]
pub struct TopicRepo {
    db: CassandraCacheSession,
}

impl TopicRepo {
    pub fn new(db: CassandraCacheSession) -> Self {
        Self { db }
    }

    pub async fn migrate_topic_table(&self) -> AppResult<()> {
        let session = self.db.lock().await;
        session.execute_unpaged(CREATE_TOPIC_TABLE_QUERY, ()).await?;
        session.execute_unpaged(CREATE_USER_ID_INDEX, ()).await?;
        session.execute_unpaged(CREATE_USER_EMAIL_INDEX, ()).await?;
        session.execute_unpaged(CREATE_USER_NAME_INDEX, ()).await?;
        Ok(())
    }
}

impl TopicRepository for TopicRepo {
    async fn create_topic<'c>(&self, topic: &'c Topic) -> AppResult<&'c Topic> {
        let session = self.db.lock().await;
        match topic.insert().execute(&session).await {
            Ok(_) => Ok(topic),
            Err(err) => {
                tracing::error!("{err:?}");
                Err(anyhow!(AppError::InternalServerError))
            }
        }
    }

    async fn find_topic_by_partition_key(&self, query: &RequestGetTopicByPartitionKey) -> AppResult<Vec<Topic>> {
        let session = self.db.lock().await;
        let result = Topic {
            topic_id: query.topic_id.to_owned(),
            ..Default::default()
        }
            .find_by_partition_key()
            .execute(&session)
            .await;

        match result {
            Ok(topic) => Ok(topic.try_collect().await?),
            Err(err) => {
                tracing::error!("{err:?}");
                Err(anyhow!(RequestFindTopicError::TopicNotFound))
            }
        }
    }

    async fn find_topic_by_primary_key(
        &self,
        request_topic_by_primary_key: &RequestGetTopicByPrimaryKey,
    ) -> AppResult<Topic> {
        let session = self.db.lock().await;
        let topic = Topic {
            topic_id: request_topic_by_primary_key.topic_id.to_owned(),
            created_at: request_topic_by_primary_key.created_at,
            ..Default::default()
        }.find_by_primary_key().execute(&session).await;

        match topic {
            Ok(topic) => {
                Ok(topic)
            }
            Err(err) => {
                tracing::error!("{err:?}");
                Err(anyhow!(AppError::InternalServerError))
            }
        }
    }

    async fn find_topic_by_index_key(&self, query: &RequestGetTopicByIndexKey) -> AppResult<Vec<Topic>> {
        let session = self.db.lock().await;
        let results = Topic::find_by_topic_name(Text::from(query.topic_name.clone())).execute(&session).await;

        match results {
            Ok(value) => match value.try_collect().await {
                Ok(val) => Ok(val),
                Err(err) => {
                    tracing::error!("{err:?}");
                    Err(anyhow!(AppError::InternalServerError))
                }
            },
            Err(err) => {
                tracing::error!("{err:?}");
                Err(anyhow!(AppError::InternalServerError))
            }
        }
    }


    async fn update_topic<'u>(&self, topic: &'u RequestUpdateTopic) -> AppResult<&'u Topic> {
        let session = self.db.lock().await;
        let mut batch = Topic::batch();
        if topic.push_to_admins.is_some() {
            batch.append_statement(Topic::PUSH_TOPIC_ADMINS_QUERY, Topic { topic_owners: topic.clone().push_to_admins.unwrap(), ..Default::default() });
        }

        if topic.pop_to_admins.is_some() {
            batch.append_statement(Topic::PULL_TOPIC_ADMINS_QUERY, Topic { topic_owners: topic.clone().pop_to_admins.unwrap(), ..Default::default() });
        }

        match batch.execute(&session).await {
            Ok(_) => Ok(topic),
            Err(err) => {
                tracing::error!("{err:?}");
                Err(anyhow!(AppError::InternalServerError))
            }
        }
    }
}

static CREATE_TOPIC_TABLE_QUERY: &str = r#"
    CREATE TABLE IF NOT EXISTS uptop.topics (
        topic_id timeuuid,
        topic_name text,
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
        PRIMARY KEY ((country, region, city), topic_id)
    ) WITH CLUSTERING ORDER BY (topic_id DESC);
"#;

static CREATE_USER_ID_INDEX: &str = r#"
    CREATE INDEX IF NOT EXISTS uptop_topic_id_index ON uptop.topics (topic_id);
"#;

static CREATE_USER_EMAIL_INDEX: &str = r#"
    CREATE INDEX IF NOT EXISTS uptop_email_index ON uptop.topics (email);
"#;

static CREATE_USER_NAME_INDEX: &str = r#"
    CREATE INDEX IF NOT EXISTS uptop_topic_name_index ON uptop.topics (topic_name);
"#;
