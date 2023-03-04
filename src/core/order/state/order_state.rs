use serenity::async_trait;

use crate::{core::order::models::order::Order, bot::Bot, ContextHTTP};

use super::states::{first_payment_state::FirstPaymentState, in_progress_state::InProgressState, second_payment_state::SecondPaymentState, delivery_state::DeliveryState, delivered_state::DeliveredState, canceled_state::CanceledState};

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

pub const FIRST_PAYMENT_STATE: FirstPaymentState = FirstPaymentState;
pub const IN_PROGRESS_STATE: InProgressState = InProgressState;
pub const SECOND_PAYMENT_STATE: SecondPaymentState = SecondPaymentState;
pub const DELIVERY_STATE: DeliveryState = DeliveryState;
pub const DELIVERED_STATE: DeliveredState = DeliveredState;
pub const CANCELED_STATE: CanceledState = CanceledState;

pub fn initial_state() -> &'static dyn OrderState {
    &FIRST_PAYMENT_STATE
}

pub fn get_state_by_id(id: &str) -> Option<&'static dyn OrderState> {
    match id {
        FirstPaymentState::ID => Some(&FIRST_PAYMENT_STATE),
        InProgressState::ID => Some(&IN_PROGRESS_STATE),
        SecondPaymentState::ID => Some(&SECOND_PAYMENT_STATE),
        DeliveryState::ID => Some(&DELIVERY_STATE),
        DeliveredState::ID => Some(&DELIVERED_STATE),
        CanceledState::ID => Some(&CANCELED_STATE),
        _ => None
    }
}