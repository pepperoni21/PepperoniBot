use serenity::async_trait;

use crate::{core::order::models::order::Order, bot::Bot, ContextHTTP};

use super::states::{in_progress_state::InProgressState, payment_state::PaymentState, delivery_state::DeliveryState, delivered_state::DeliveredState, canceled_state::CanceledState};

#[async_trait]
pub trait OrderState : Send + Sync {
    fn id(&self) -> String;
    
    fn short_name(&self) -> String;

    fn instruction(&self) -> Option<String> {
        None
    }

    fn validateable(&self) -> bool;

    fn validate_action_label(&self) -> Option<String> {
        None
    }

    async fn validate(&self, bot: &Bot, context_http: &ContextHTTP, order: &mut Order);

    fn validate_message(&self) -> Option<String> {
        None
    }
}

pub const IN_PROGRESS_STATE: InProgressState = InProgressState;
pub const PAYMENT_STATE: PaymentState = PaymentState;
pub const DELIVERY_STATE: DeliveryState = DeliveryState;
pub const DELIVERED_STATE: DeliveredState = DeliveredState;
pub const CANCELED_STATE: CanceledState = CanceledState;

pub fn initial_state() -> &'static dyn OrderState {
    &IN_PROGRESS_STATE
}

pub fn get_state_by_id(id: &str) -> Option<&'static dyn OrderState> {
    match id {
        InProgressState::ID => Some(&IN_PROGRESS_STATE),
        PaymentState::ID => Some(&PAYMENT_STATE),
        DeliveryState::ID => Some(&DELIVERY_STATE),
        DeliveredState::ID => Some(&DELIVERED_STATE),
        CanceledState::ID => Some(&CANCELED_STATE),
        _ => None
    }
}