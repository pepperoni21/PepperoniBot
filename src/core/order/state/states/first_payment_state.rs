use async_trait::async_trait;
use serenity::{utils::Color, model::prelude::GuildChannel};
use wither::Model;

use crate::{core::order::{state::order_state::{OrderState, self}, models::order::Order, order_message_manager}, bot::Bot, ContextHTTP};

pub struct FirstPaymentState;

impl FirstPaymentState {
    pub const ID: &'static str = "first-payment";
    pub const SHORT_NAME: &'static str = "Waiting first payment";
    pub const INSTRUCTION: &'static str = "Please process the first payment of %price% USD to the following address: pariselias00@gmail.com";
    pub const VALIDATE_ACTION_LABEL: &'static str = "Set first payment paid";
}

#[async_trait]
impl OrderState for FirstPaymentState {

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
        order.set_order_state(&order_state::IN_PROGRESS_STATE);

        let order_channel_id = order.assets.order_channel_id.unwrap();
        let order_channel = context_http.get_channel(order_channel_id).await.expect("Failed to get order channel").guild().expect("Order channel is not a guild channel");

        order_message_manager::update_channel_message(context_http, order, &order_channel).await;
        Self::send_first_payment_message(context_http, &order_channel).await;
        order_message_manager::update_order_list_message(context_http, &order).await;

        order.save(&bot.db_info.db, None).await.expect("Failed to save order");
    }

    fn validate_message(&self) -> Option<String> {
        Some("First payment validated!".to_string())
    }

}

impl FirstPaymentState {

    pub async fn send_first_payment_message(context_http: &ContextHTTP, channel: &GuildChannel) {
        channel.send_message(context_http, |message|
            message.embed(|embed|
                embed
                .title("First payment validated!")
                .colour(Color::DARK_GREEN)
            )
        ).await.expect("Failed to send message");
    }
    
}