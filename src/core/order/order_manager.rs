use std::env;

use serenity::model::{prelude::{GuildId, PermissionOverwrite, PermissionOverwriteType}, user::User, Permissions};
use wither::Model;

use crate::{ContextHTTP, bot::Bot};

use super::{command::order_command, review::review_manager::ReviewManager, models::{order::Order, order_type::OrderType}};

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

    pub async fn create_order(&self, bot: &Bot, context_http: &ContextHTTP, user: &User, order_type: OrderType, price: i32, description: String){
        let order = Order::new(order_type, price, user.id.0, description);
        let orders_category_id: u64 = env::var("ORDERS_CATEGORY_ID").expect("ORDERS_CATEGORY_ID must be set").parse().expect("ORDERS_CATEGORY_ID must be a number");

        let guild_id: u64 = env::var("GUILD_ID").expect("GUILD_ID must be set").parse().expect("GUILD_ID must be a number");
        let guild_id = GuildId(guild_id);
        let guild = guild_id.to_partial_guild(&context_http).await.expect("Failed to get guild");

        let channel = guild.create_channel(context_http, |channel| {
            channel.name(format!("order-{}", order.order_id));
            channel.category(orders_category_id);
            
            let permissions = vec![PermissionOverwrite {
                allow: Permissions::SEND_MESSAGES,
                deny: Permissions::empty(),
                kind: PermissionOverwriteType::Member(user.id),
            }];

            channel.permissions(permissions);

            channel
        }).await.expect("Failed to create channel");

        todo!("Send messages after filling the message manager")
    }
}