use mongodb::Collection;
use serenity::model::prelude::GuildId;

use crate::{core::db::DBInfo, ContextHTTP};

use super::{models::order::Order, command::order_command};

pub struct OrderManager {
    pub context_http: ContextHTTP,
    pub orders_collection: Collection<Order>,
    pub guild_id: GuildId
}

impl OrderManager {
    pub async fn new(context_http: ContextHTTP, db_info: &DBInfo, guild_id: GuildId) -> Self {
        let orders_collection = db_info.db.collection("orders");
        let order_manager = Self {
            context_http,
            orders_collection,
            guild_id
        };
        order_manager.load().await;
        order_manager
    }

    pub async fn load(&self) {
        order_command::load_command(&self.context_http, &self.guild_id).await;
    }
}