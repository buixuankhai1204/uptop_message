use crate::application::topic::{
    app::TopicAppInterface,
    request::{RequestCreateTopic},
};
use std::sync::Arc;
use uptop_core::common::result::AppResult;
use crate::application::topic::request::{RequestGetTopicByPartitionKey, RequestUpdateTopic};

#[derive(Clone, Debug)]
pub struct TopicHandler<UA: TopicAppInterface> {
    pub topic_app: Arc<UA>,
}

pub async fn on_create_new_topic<UA: TopicAppInterface>(
    handler: TopicHandler<UA>,
    payload: String,
) -> AppResult<String> {
    let body: RequestCreateTopic = serde_json::from_str(&payload)?;
    let req = body.try_into_domain()?;

    let result = handler.topic_app.create_topic(req).await?;
    Ok(serde_json::to_string(&result)?)
}

pub async fn on_find_topic<UA: TopicAppInterface>(
    handler: TopicHandler<UA>,
    payload: String,
) -> AppResult<String> {
    let query: RequestGetTopicByPartitionKey = serde_json::from_str(&payload)?;
    let result = handler.topic_app.find_topic_by_partition_key(&query).await?;
    Ok(serde_json::to_string(&result)?)
}

pub async fn update_topic<TA: TopicAppInterface>(handler: TopicHandler<TA>,
                                                 payload: String, ) -> AppResult<String> {
    let query: RequestUpdateTopic = serde_json::from_str(&payload)?;
    let result = handler.topic_app.update_topic(&query).await?;
    
    Ok(serde_json::to_string(&result)?)
}
