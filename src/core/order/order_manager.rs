use serenity::model::prelude::GuildId;

use crate::ContextHTTP;

use super::{command::order_command, review::review_manager::ReviewManager};

pub struct OrderManager {
    pub review_manager: ReviewManager,
}

impl OrderManager {
    pub async fn new() -> Self {
        let review_manager = ReviewManager::new();
        let order_manager = Self {
            review_manager,
        };
        order_manager
    }

    pub async fn load(&self, context_http: &ContextHTTP, guild_id: GuildId) {
        order_command::load_command(context_http, &guild_id).await;
        self.review_manager.load(&context_http).await;
    }
}