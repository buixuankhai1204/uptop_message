pub enum MessageModuleServices {
    CreateTopic,
    GetTopic,
    GetTopics,
    UpdateTopic,
}

impl MessageModuleServices {
    pub fn action(input: &str) -> Option<MessageModuleServices> {
        match input {
            "CREATE_USER" => Some(MessageModuleServices::CreateTopic),
            "GET_USER" => Some(MessageModuleServices::GetTopic),
            "GET_USERS" => Some(MessageModuleServices::GetTopics),
            "UPDATE_USER" => Some(MessageModuleServices::UpdateTopic),
            _ => None,
        }
    }
}
