use serenity::model::prelude::UserId;
use wither::{Model, bson::{doc, to_bson}};

use crate::{bot::Bot, ContextHTTP, utils::channel_utils};

use super::{models::{developer::Developer, developer_assets::DeveloperAssets}, developer_message_manager::DeveloperMessageManager, command::developer_command};

pub struct DeveloperManager;

impl DeveloperManager {

    pub async fn load(&self, bot: &Bot, context_http: &ContextHTTP) {
        developer_command::load_command(context_http, &bot.guild_id).await;
    }

    pub async fn create_developer(&self, bot: &Bot, context_http: &ContextHTTP, user_id: u64, introduction: String) {
        let developers_channel = channel_utils::fetch_guild_channel("DEVELOPERS_CHANNEL_ID", &context_http).await;

        let developer_list_message_id = DeveloperMessageManager::send_developers_channel_message(context_http, user_id, introduction.clone(), developers_channel)
            .await
            .expect("Failed to send developers channel message");

        let introduction_channel = channel_utils::fetch_guild_channel("DEVELOPERS_INTRODUCTION_CHANNEL_ID", &context_http).await;

        let introduction_message_id = DeveloperMessageManager::send_introduction_channel_message(context_http, user_id, introduction.clone(), introduction_channel)
            .await
            .expect("Failed to send developers introduction channel message");

        let assets: DeveloperAssets = DeveloperAssets { developer_list_message_id, introduction_message_id };
        let mut developer = Developer::new(user_id, introduction, assets);
        developer.save(&bot.db_info.db, None).await.expect("Failed to create developer");
    }

    pub async fn is_developer(&self, bot: &Bot, user_id: UserId) -> bool {
        Developer::find_one(&bot.db_info.db, doc! { "user_id": to_bson(&user_id.0).unwrap() }, None).await.expect("Failed to find developer").is_some()
    }

}