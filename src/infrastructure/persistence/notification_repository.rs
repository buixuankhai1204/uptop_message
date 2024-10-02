use crate::application::notification::request::{RequestUpdateNotification, RequestFindLatestMessageError, RequestGetNotificationByUsername};
use crate::{
    domain::notification::{entity::Notification, repository::NotificationRepository},
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
pub struct NotificationRepo {
    db: CassandraCacheSession,
}

impl NotificationRepo {
    pub fn new(db: CassandraCacheSession) -> Self {
        Self { db }
    }

    pub async fn migrate_notification_table(&self) -> AppResult<()> {
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

impl NotificationRepository for NotificationRepo {
    async fn find_notifications_by_partition_key(
        &self,
        query: &RequestGetNotificationByUsername,
    ) -> AppResult<Vec<Notification>> {
        let session = self.db.lock().await;
        let result = Notification {
            username: (*query.username).parse()?,
            ..Default::default()
        }
            .find_by_partition_key()
            .execute(&session)
            .await;

        match result {
            Ok(notification) => Ok(notification.try_collect().await?),
            Err(err) => {
                tracing::error!("{err:?}");
                Err(anyhow!(RequestFindLatestMessageError::LatestMessageNotFound))
            }
        }
    }

    async fn update_notifications(&self, notification: &RequestUpdateNotification) -> AppResult<Notification> {
        let session = self.db.lock().await;
        let result = Notification {
            topic_id: notification.topic_id,
            username: (*notification.username).parse()?,
            from_user: (*notification.from_user).parse()?,
            message: (*notification.message).parse()?,
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
    CREATE TABLE IF NOT EXISTS uptop.notifications (
        notification_id timeuuid,
        notification_name text,
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
        PRIMARY KEY ((country, region, city), notification_id)
    ) WITH CLUSTERING ORDER BY (notification_id DESC);
"#;

static CREATE_USER_ID_INDEX: &str = r#"
    CREATE INDEX IF NOT EXISTS uptop_notification_id_index ON uptop.notifications (notification_id);
"#;

static CREATE_USER_EMAIL_INDEX: &str = r#"
    CREATE INDEX IF NOT EXISTS uptop_email_index ON uptop.notifications (email);
"#;

static CREATE_USER_NAME_INDEX: &str = r#"
    CREATE INDEX IF NOT EXISTS uptop_notification_name_index ON uptop.notifications (notification_name);
"#;
