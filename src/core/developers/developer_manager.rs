use wither::Model;

use crate::{bot::Bot, ContextHTTP};

use super::models::{developer::Developer, developer_assets::DeveloperAssets};

pub struct DeveloperManager;

impl DeveloperManager {

    pub async fn load(&self, bot: &Bot, context_http: &ContextHTTP) {
        
    }

    pub async fn create_developer(&self, bot: &Bot, user_id: u64, introduction: String) {
        let assets: DeveloperAssets = todo!();
        let mut developer = Developer::new(user_id, introduction, assets);
        developer.save(&bot.db_info.db, None).await.expect("Failed to create developer");
    }

}