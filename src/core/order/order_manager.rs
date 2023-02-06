use mongodb::Collection;
use serenity::model::prelude::GuildId;

use crate::{core::db::DBInfo, ContextHTTP};

use super::{models::order::Order, command::order_command};

pub struct OrderManager {
    pub orders_collection: Collection<Order>
}

impl OrderManager {
    pub async fn new(db_info: &DBInfo) -> Self {
        let orders_collection = db_info.db.collection("orders");
        let order_manager = Self {
            orders_collection
        };
        order_manager
    }

    pub async fn load(&self, context_http: ContextHTTP, guild_id: GuildId) {
        order_command::load_command(&context_http, &guild_id).await;
    }
}