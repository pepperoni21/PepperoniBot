use async_trait::async_trait;
use wither::Model;

use crate::{core::order::{state::order_state::{OrderState, self}, models::order::Order}, bot::Bot, ContextHTTP};

pub struct DeliveryState;

impl DeliveryState {
    pub const ID: &'static str = "delivery";
    pub const SHORT_NAME: &'static str = "Waiting delivery";
    pub const INSTRUCTION: &'static str = "Your delivery is coming...";
    pub const VALIDATE_ACTION_LABEL: &'static str = "Set delivered";
}

#[async_trait]
impl OrderState for DeliveryState {

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
        let order_manager = &bot.order_manager;

        order_manager.end_order(context_http, order).await;
        order.set_order_state(&order_state::DELIVERED_STATE);
        let _ = &order_manager.message_manager.add_to_archive(context_http, order).await;
        order.save(&bot.db_info.db, None).await.expect("Failed to save order");
    }

    fn validate_message(&self) -> Option<String> {
        Some("Order set as delivered!".to_string())
    }

}