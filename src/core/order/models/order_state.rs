use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum OrderState {
    FirstPayment,
    InProgress,
    SecondPayment,
    Delivery,
    Delivered,
    Canceled
}

impl OrderState {
    pub fn _get_name(&self) -> &str {
        match *self {
            Self::FirstPayment => "Waiting first payment",
            Self::InProgress => "In progress",
            Self::SecondPayment => "Waiting second payment",
            Self::Delivery => "Waiting delivery",
            Self::Delivered => "Delivered",
            Self::Canceled => "Canceled"
        }
    }

    pub fn _get_message(&self) -> Option<&str> {
        match *self {
            Self::FirstPayment => Some("Please process the first payment of %price% USD to the following address: https://paypal.me/MaxiGiantFR"),
            Self::InProgress => Some("Your order is in progress..."),
            Self::SecondPayment => Some("Please process the second payment of %price% USD to the following address: https://paypal.me/MaxiGiantFR"),
            Self::Delivery => Some("Your delivery is coming..."),
            Self::Delivered => None,
            Self::Canceled => None
        }
    }

    pub fn _get_action(&self) -> Option<&str> {
        match *self {
            Self::FirstPayment => Some("order:first-payment"),
            Self::InProgress => Some("order:done"),
            Self::SecondPayment => Some("order:second-payment"),
            Self::Delivery => Some("order:delivery"),
            Self::Delivered => None,
            Self::Canceled => None
        }
    }

    pub fn _get_action_row_label(&self) -> Option<&str> {
        match *self {
            Self::FirstPayment => Some("Set first payment paid"),
            Self::InProgress => Some("Set as done"),
            Self::SecondPayment => Some("Set second payment paid"),
            Self::Delivery => Some("Set as delivered"),
            Self::Delivered => None,
            Self::Canceled => None
        }
    }
}