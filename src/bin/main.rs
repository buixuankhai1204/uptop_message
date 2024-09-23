use message::application::topic::app::{TopicApp};
use message::infrastructure::persistence::IDRepositories;
use message::interfaces::actions::MessageModuleServices;
use message::interfaces::topic_handler::{on_create_new_topic, on_find_topic, TopicHandler};
use scylla::CachingSession;
use std::sync::Arc;
use tokio::sync::Mutex;
use tonic::{transport::Server, Request, Response, Status};
use uptop_core::common::result::AppResult;
use uptop_core::common::trace::tracing_init;
use uptop_core::infrastructure::cassandra::{create_db_session, create_keyspace};

mod message_proto {
    tonic::include_proto!("message");
}

use message_proto::message_server::{Message, MessageServer};
use message_proto::{MessageRequest, MessageResponse};

struct MessageService {
    repositories: Arc<IDRepositories>,
}

impl MessageService {
    fn new(repos: IDRepositories) -> Self {
        Self {
            repositories: Arc::new(repos),
        }
    }
}

#[tonic::async_trait]
impl Message for MessageService {
    async fn send_message(
        &self,
        request: Request<MessageRequest>,
    ) -> Result<Response<MessageResponse>, Status> {
        // Extract the inner message from the request
        let payload = request.into_inner();
        let command = payload.id;
        let message = payload.message;

        let mut status_code = "OK".to_string();
        let mut response: MessageResponse = MessageResponse {
            id: "Internal Server Error".to_owned(),
            message: "Please try again!".to_owned(),
        };

        let topic_app = TopicApp::new(Arc::new(self.repositories.topic.clone()));
        let user_handler = TopicHandler {
            topic_app: Arc::new(topic_app),
        };

        match MessageModuleServices::action(&command) {
            Some(MessageModuleServices::CreateTopic) => {
                let message = match on_create_new_topic(user_handler, message).await {
                    Ok(res) => res,
                    Err(err) => {
                        status_code = "ERROR".to_string();
                        err.to_string()
                    }
                };
                response = MessageResponse {
                    id: status_code,
                    message,
                };
            }
            Some(MessageModuleServices::GetTopic) => {
                let message = match on_find_topic(user_handler, message).await {
                    Ok(res) => res,
                    Err(err) => {
                        status_code = "ERROR".to_string();
                        err.to_string()
                    }
                };
                response = MessageResponse {
                    id: status_code,
                    message,
                };
            }
            Some(MessageModuleServices::GetTopics) => {
                todo!()
            }
            Some(MessageModuleServices::UpdateTopic) => {
                todo!()
            }
            _none => (),
        }

        Ok(Response::new(response))
    }
}

#[tokio::main]
async fn main() -> AppResult<()> {
    dotenv::dotenv().ok();
    let _gaurd = tracing_init();

    let cassandra = create_db_session().await;
    create_keyspace(&cassandra).await?;
    let cache_session = CachingSession::from(cassandra, 1);
    let repos = IDRepositories::new(Arc::new(Mutex::new(cache_session)));
    repos.auto_mod_identification_migrate().await?;

    pub(crate) const FILE_MESSAGE_DESCRIPTOR_SET: &[u8] =
        tonic::include_file_descriptor_set!("message_descriptor");
    let reflect_sv = tonic_reflection::server::Builder::configure()
        .register_encoded_file_descriptor_set(FILE_MESSAGE_DESCRIPTOR_SET)
        .build_v1()
        .unwrap();

    let server_addr = "0.0.0.0:3000".parse().unwrap();
    tracing::info!(message = "Starting server on", %server_addr);
    let msg_service = MessageService::new(repos);

    Server::builder()
        .add_service(reflect_sv)
        .add_service(MessageServer::new(msg_service))
        .serve(server_addr)
        .await
        .unwrap();

    Ok(())
}
