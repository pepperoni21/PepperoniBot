use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct OrderAssets {
    pub order_list_message_id: Option<u64>,
    pub order_channel_id: Option<u64>,
    pub order_channel_message_id: Option<u64>,
}