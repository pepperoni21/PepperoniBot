use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeveloperAssets {
    pub developer_list_message_id: u64,
}