use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum OrderState {
    FirstPayment,
    InProgress,
    SecondPayment,
    Delivery,
    Delivered,
    Canceled
}

impl OrderState {
    pub fn get_name(&self) -> String {
        match *self {
            Self::FirstPayment => "Waiting first payment".to_string(),
            Self::InProgress => "In progress".to_string(),
            Self::SecondPayment => "Waiting second payment".to_string(),
            Self::Delivery => "Waiting delivery".to_string(),
            Self::Delivered => "Delivered".to_string(),
            Self::Canceled => "Canceled".to_string()
        }
    }

    pub fn get_message(&self) -> Option<String> {
        match *self {
            Self::FirstPayment => Some("Please process the first payment of %price% USD to the following address: pariselias00@gmail.com".to_string()),
            Self::InProgress => Some("Your order is in progress...".to_string()),
            Self::SecondPayment => Some("Please process the second payment of %price% USD to the following address: pariselias00@gmail.com".to_string()),
            Self::Delivery => Some("Your delivery is coming...".to_string()),
            Self::Delivered => None,
            Self::Canceled => None
        }
    }

    pub fn get_action(&self) -> Option<String> {
        match *self {
            Self::FirstPayment => Some("order:first-payment".to_string()),
            Self::InProgress => Some("order:done".to_string()),
            Self::SecondPayment => Some("order:second-payment".to_string()),
            Self::Delivery => Some("order:delivery".to_string()),
            Self::Delivered => None,
            Self::Canceled => None
        }
    }

    pub fn get_action_row_label(&self) -> Option<String> {
        match *self {
            Self::FirstPayment => Some("Set first payment paid".to_string()),
            Self::InProgress => Some("Set as done".to_string()),
            Self::SecondPayment => Some("Set second payment paid".to_string()),
            Self::Delivery => Some("Set as delivered".to_string()),
            Self::Delivered => None,
            Self::Canceled => None
        }
    }
}