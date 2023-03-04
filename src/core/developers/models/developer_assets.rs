use serde::{Serialize, Deserialize};

use crate::{ContextHTTP, utils::channel_utils};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeveloperAssets {
    pub developer_list_message_id: Option<u64>,
    pub introduction_message_id: Option<u64>
}

impl DeveloperAssets {

    pub fn new() -> Self {
        Self {
            developer_list_message_id: None,
            introduction_message_id: None
        }
    }

    pub async fn delete_assets(&self, context_http: &ContextHTTP) {
        if let Some(developer_list_message_id) = self.developer_list_message_id {
            let developer_list_channel = channel_utils::fetch_guild_channel("DEVELOPERS_CHANNEL_ID", context_http).await;
            let developer_list_message = developer_list_channel.message(context_http, developer_list_message_id).await.expect("Failed to fetch developer list message");
            developer_list_message.delete(context_http).await.expect("Failed to delete developer list message");
        }

        if let Some(introduction_message_id) = self.introduction_message_id {
            let introduction_channel = channel_utils::fetch_guild_channel("DEVELOPERS_INTRODUCTION_CHANNEL_ID", context_http).await;
            let introduction_message = introduction_channel.message(context_http, introduction_message_id).await.expect("Failed to fetch introduction message");
            introduction_message.delete(context_http).await.expect("Failed to delete introduction message");
        }
    }
    
}