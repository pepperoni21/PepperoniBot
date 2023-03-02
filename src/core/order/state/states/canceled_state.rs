use async_trait::async_trait;

use crate::{core::order::{state::order_state::OrderState, models::order::Order}, bot::Bot, ContextHTTP};

pub struct CanceledState;

impl CanceledState {
    pub const ID: &'static str = "canceled";
    pub const SHORT_NAME: &'static str = "Canceled";
}

#[async_trait]
impl OrderState for CanceledState {

    fn id(&self) -> String {
        Self::ID.to_string()
    }

    fn short_name(&self) -> String {
        Self::SHORT_NAME.to_string()
    }

    fn instruction(&self) -> Option<String> {
        None
    }

    fn validateable(&self) -> bool {
        false
    }

    fn validate_action_label(&self) -> Option<String> {
        None
    }

    async fn validate(&self, _bot: &Bot, _context_http: &ContextHTTP, _order: &mut Order) {}

    fn validate_message(&self) -> Option<String> {
        Some("Order canceled!".to_string())
    }

}