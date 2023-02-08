use std::env::{self, var};

use serenity::model::{prelude::{GuildId, PermissionOverwrite, PermissionOverwriteType, GuildChannel}, user::User, Permissions};
use wither::{Model, bson::doc};

use crate::{ContextHTTP, bot::Bot};

use super::{command::order_command, review::review_manager::ReviewManager, models::{order::Order, order_type::OrderType, order_state::OrderState}, order_message_manager::OrderMessageManager};

pub struct OrderManager {
    pub review_manager: ReviewManager,
    pub message_manager: OrderMessageManager,
}

impl OrderManager {
    pub async fn new() -> Self {
        let review_manager = ReviewManager::new();
        let order_manager = Self {
            review_manager,
            message_manager: OrderMessageManager,
        };
        order_manager
    }

    pub async fn load(&self, context_http: &ContextHTTP, guild_id: GuildId) {
        order_command::load_command(context_http, &guild_id).await;
        self.review_manager.load(&context_http).await;
    }

    pub async fn create_order(&self, bot: &Bot, context_http: &ContextHTTP, user: &User, order_type: OrderType, price: i32, description: String){
        let mut order = Order::new(order_type, price, user.id.0, description);
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

        let message = channel.send_message(context_http, |message|
            message.set_embed(self.message_manager.order_channel_message(&order))
        ).await.expect("Failed to send message");
        order.assets.order_channel_message_id = Some(*message.id.as_u64());
        message.pin(context_http).await.expect("Failed to pin message");

        let orders_channel = self.get_order_list_channel(context_http).await;
        let order_list_message = self.message_manager.order_list_message(&order, user, &channel).await;
        let components = self.message_manager.generate_action_rows(&order).await;
        let message = orders_channel.send_message(context_http, |message|
            message
            .set_embed(order_list_message)
            .set_components(components)
        ).await.expect("Failed to send message");
        order.assets.order_channel_id = Some(*channel.id.as_u64());
        order.assets.order_list_message_id = Some(*message.id.as_u64());
        order.save(&bot.db_info.db, None);
    }

    pub async fn cancel_order(&self, bot: &Bot, context_http: &ContextHTTP, order: &mut Order) {
        self.end_order(context_http, order).await;
        order.order_state = OrderState::Canceled;
        self.message_manager.add_to_archive(context_http, order).await;
        order.save(&bot.db_info.db, None).await.expect("Failed to save order");
    }

    async fn end_order(&self, context_http: &ContextHTTP, order: &Order) {
        let order_channel_id = order.assets.order_channel_id;
        let order_channel_message_id = order.assets.order_channel_message_id;

        if order_channel_id.is_some() {
            let order_channel = context_http.get_channel(order_channel_id.unwrap()).await.expect("Failed to get order channel").guild().expect("Order channel is not a guild channel");
            order_channel.delete(context_http).await.expect("Failed to delete order channel");
        } if order_channel_message_id.is_some() {
            let orders_list_channel = self.get_order_list_channel(context_http).await;
            let order_list_message = orders_list_channel.message(context_http, order_channel_message_id.unwrap()).await.expect("Failed to get order list message");
            order_list_message.delete(context_http).await.expect("Failed to delete order list message");
        }
    }

    pub async fn validate_first_payment(&self, bot: &Bot, context_http: &ContextHTTP, order: &mut Order) {
        if order.order_state != OrderState::FirstPayment {
            return;
        }
        order.order_state = OrderState::InProgress;
        
        let order_channel_id = order.assets.order_channel_id.unwrap();
        let order_channel = context_http.get_channel(order_channel_id).await.expect("Failed to get order channel").guild().expect("Order channel is not a guild channel");

        let msg_mng = &self.message_manager;
        msg_mng.update_channel_message(context_http, order, &order_channel).await;
        msg_mng.send_first_payment_message(context_http, &order_channel).await;
        msg_mng.update_order_list_message(context_http, &order).await;

        order.save(&bot.db_info.db, None).await;
    }

    pub async fn set_done(&self, bot: &Bot, context_http: &ContextHTTP, order: &mut Order) {
        if order.order_state != OrderState::InProgress {
            return;
        }
        order.order_state = OrderState::SecondPayment;

        let order_channel_id = order.assets.order_channel_id.unwrap();
        let order_channel = context_http.get_channel(order_channel_id).await.expect("Failed to get order channel").guild().expect("Order channel is not a guild channel");

        let msg_mng = &self.message_manager;
        msg_mng.update_channel_message(context_http, order, &order_channel).await;
        msg_mng.send_done_message(context_http, &order_channel).await;
        msg_mng.update_order_list_message(context_http, &order).await;

        order.save(&bot.db_info.db, None).await;
    }

    pub async fn validate_second_payment(&self, bot: &Bot, context_http: &ContextHTTP, order: &mut Order) {
        if order.order_state != OrderState::SecondPayment {
            return;
        }
        order.order_state = OrderState::Delivery;

        let order_channel_id = order.assets.order_channel_id.unwrap();
        let order_channel = context_http.get_channel(order_channel_id).await.expect("Failed to get order channel").guild().expect("Order channel is not a guild channel");

        let msg_mng = &self.message_manager;
        msg_mng.update_channel_message(context_http, order, &order_channel).await;
        msg_mng.send_second_payment_message(context_http, &order_channel).await;
        msg_mng.update_order_list_message(context_http, &order).await;

        order.save(&bot.db_info.db, None).await;
    }

    pub async fn set_delivered(&self, bot: &Bot, context_http: &ContextHTTP, order: &mut Order) {
        if order.order_state != OrderState::Delivery {
            return;
        }
        order.order_state = OrderState::Delivered;

        let order_channel_id = order.assets.order_channel_id.unwrap();
        let order_channel = context_http.get_channel(order_channel_id).await.expect("Failed to get order channel").guild().expect("Order channel is not a guild channel");

        self.message_manager.add_to_archive(context_http, order).await;

        order.save(&bot.db_info.db, None).await;
    }

    pub async fn fetch_order(bot: &Bot, order_id: i32) -> Order {
        Order::find_one(&bot.db_info.db, doc!{
            "order_id": order_id
        }, None).await.expect("Failed to fetch order").expect("Order not found")
    }

    async fn get_order_list_channel(&self, context_http: &ContextHTTP) -> GuildChannel {
        let order_list_channel_id: u64 = env::var("ORDERS_CHANNEL_ID").expect("").parse().expect("Failed to parse order list channel id");
        context_http.get_channel(order_list_channel_id).await.expect("Failed to get order list channel").guild().expect("Order list channel is not a guild channel")
    }
}