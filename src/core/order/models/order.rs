use serde::{Serialize, Deserialize};

use crate::core::review::review::Review;

use super::{order_type::OrderType, order_state::{OrderState}, order_assets::OrderAssets};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Order {
    order_id: i32,
    order_type: OrderType,
    order_state: OrderState,
    price: i32,
    customer_id: f64,
    description: String,
    assets: OrderAssets,
    review: Option<Review>
}

impl Order {
    pub fn new(order_type: OrderType, price: i32, customer_id: f64, description: String) -> Self {
        let order_id = rand::random::<i32>();
        let order_state = OrderState::FirstPayment;
        let assets = OrderAssets;
        let review = None;
        Self {
            order_id,
            order_type,
            order_state,
            price,
            customer_id,
            description,
            assets,
            review
        }
    }
}