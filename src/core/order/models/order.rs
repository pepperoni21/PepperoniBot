use serde::{Serialize, Deserialize};
use wither::{bson::{oid::ObjectId, doc}, Model};

use crate::core::order::{review::models::review::Review, state::order_state::{self, OrderState}};

use super::{order_type::OrderType, order_assets::OrderAssets};

#[derive(Serialize, Deserialize, Debug, Model, Clone)]
#[model(index(keys=r#"doc!{"order_id": 1}"#, options=r#"doc!{"unique": true}"#), collection_name="orders")]
pub struct Order {
    #[serde(rename="_id", skip_serializing_if="Option::is_none")]
    pub id: Option<ObjectId>,

    pub order_id: i32,
    pub order_type: OrderType,
    pub order_state_id: String,
    pub price: i32,
    pub developer_id: u64,
    pub customer_id: u64,
    pub description: String,
    pub assets: OrderAssets,
    pub review: Option<Review>
}

impl Order {
    pub fn new(order_type: OrderType, price: i32, developer_id: u64, customer_id: u64, description: String) -> Self {
        let order_id = rand::random::<i32>().abs();
        
        let order_state_id = order_state::initial_state().id();

        let assets = OrderAssets::new();
        let review = None;
        Self {
            id: None,
            order_id,
            order_type,
            order_state_id,
            price,
            developer_id,
            customer_id,
            description,
            assets,
            review
        }
    }

    pub fn get_order_state(&self) -> Option<&dyn OrderState> {
        order_state::get_state_by_id(&self.order_state_id)
    }

    pub fn set_order_state(&mut self, order_state: &dyn OrderState) {
        self.order_state_id = order_state.id();
    }
}
