use async_trait::async_trait;
use serenity::{model::prelude::GuildChannel, utils::Color};
use wither::Model;

use crate::{core::order::{state::order_state::{OrderState, self}, models::order::Order}, bot::Bot, ContextHTTP};

pub struct SecondPaymentState;

impl SecondPaymentState {
    pub const ID: &'static str = "second-payment";
    pub const SHORT_NAME: &'static str = "Waiting second payment";
    pub const INSTRUCTION: &'static str = "Please process the second payment of %price% USD to the following address: pariselias00@gmail.com";
    pub const VALIDATE_ACTION_LABEL: &'static str = "Set second payment paid";
}

#[async_trait]
impl OrderState for SecondPaymentState {

    fn id(&self) -> String {
        Self::ID.to_string()
    }

    fn short_name(&self) -> String {
        Self::SHORT_NAME.to_string()
    }

    fn instruction(&self) -> Option<String> {
        Some(Self::INSTRUCTION.to_string())
    }

    fn validateable(&self) -> bool {
        true
    }

    fn validate_action_label(&self) -> Option<String> {
        Some(Self::VALIDATE_ACTION_LABEL.to_string())
    }

    async fn validate(&self, bot: &Bot, context_http: &ContextHTTP, order: &mut Order) {
        order.set_order_state(&order_state::DELIVERY_STATE);

        let order_channel_id = order.assets.order_channel_id.unwrap();
        let order_channel = context_http.get_channel(order_channel_id).await.expect("Failed to get order channel").guild().expect("Order channel is not a guild channel");

        let msg_mng = &bot.order_manager.message_manager;
        msg_mng.update_channel_message(context_http, order, &order_channel).await;
        msg_mng.update_order_list_message(context_http, &order).await;

        Self::send_second_payment_message(context_http, &order_channel).await;

        order.save(&bot.db_info.db, None).await.expect("Failed to save order");
    }

    fn validate_message(&self) -> Option<String> {
        Some("Second payment validated!".to_string())
    }

}

impl SecondPaymentState {

    pub async fn send_second_payment_message(context_http: &ContextHTTP, channel: &GuildChannel) {
        channel.send_message(context_http, |message|
            message.embed(|embed|
                embed
                .title("Second payment validated!")
                .colour(Color::DARK_GREEN)
            )
        ).await.expect("Failed to send message");
    }
    
}