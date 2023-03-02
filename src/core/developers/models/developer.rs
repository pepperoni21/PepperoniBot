use serde::{Serialize, Deserialize};
use wither::{Model, bson::{oid::ObjectId, doc}};

use super::developer_assets::DeveloperAssets;

#[derive(Serialize, Deserialize, Debug, Model, Clone)]
#[model(collection_name="developers")]
pub struct Developer {
    #[serde(rename="_id", skip_serializing_if="Option::is_none")]
    pub id: Option<ObjectId>,

    pub user_id: u64,
    pub introduction: String,

    pub assets: DeveloperAssets
}

impl Developer {
    pub fn new(user_id: u64, introduction: String, assets: DeveloperAssets) -> Self {
        Self {
            id: None,
            user_id,
            introduction,
            assets
        }
    }
}