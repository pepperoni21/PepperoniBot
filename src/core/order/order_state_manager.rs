use wither::Model;

use crate::{bot::Bot, ContextHTTP};

use super::models::{order_state::OrderState, order::Order};

pub struct OrderStateManager;

impl OrderStateManager {
    pub async fn validate_first_payment(&self, bot: &Bot, context_http: &ContextHTTP, order: &mut Order) {
        if order.order_state != OrderState::FirstPayment {
            return;
        }
        order.order_state = OrderState::InProgress;

        let order_channel_id = order.assets.order_channel_id.unwrap();
        let order_channel = context_http.get_channel(order_channel_id).await.expect("Failed to get order channel").guild().expect("Order channel is not a guild channel");

        let msg_mng = &bot.order_manager.message_manager;
        msg_mng.update_channel_message(context_http, order, &order_channel).await;
        msg_mng.send_first_payment_message(context_http, &order_channel).await;
        msg_mng.update_order_list_message(context_http, &order).await;

        order.save(&bot.db_info.db, None).await.expect("Failed to save order");
    }

    pub async fn set_done(&self, bot: &Bot, context_http: &ContextHTTP, order: &mut Order) {
        if order.order_state != OrderState::InProgress {
            return;
        }
        order.order_state = OrderState::SecondPayment;

        let order_channel_id = order.assets.order_channel_id.unwrap();
        let order_channel = context_http.get_channel(order_channel_id).await.expect("Failed to get order channel").guild().expect("Order channel is not a guild channel");

        let msg_mng = &bot.order_manager.message_manager;
        msg_mng.update_channel_message(context_http, order, &order_channel).await;
        msg_mng.send_done_message(context_http, &order_channel).await;
        msg_mng.update_order_list_message(context_http, &order).await;

        order.save(&bot.db_info.db, None).await.expect("Failed to save order");
    }

    pub async fn validate_second_payment(&self, bot: &Bot, context_http: &ContextHTTP, order: &mut Order) {
        if order.order_state != OrderState::SecondPayment {
            return;
        }
        order.order_state = OrderState::Delivery;

        let order_channel_id = order.assets.order_channel_id.unwrap();
        let order_channel = context_http.get_channel(order_channel_id).await.expect("Failed to get order channel").guild().expect("Order channel is not a guild channel");

        let msg_mng = &bot.order_manager.message_manager;
        msg_mng.update_channel_message(context_http, order, &order_channel).await;
        msg_mng.send_second_payment_message(context_http, &order_channel).await;
        msg_mng.update_order_list_message(context_http, &order).await;

        order.save(&bot.db_info.db, None).await.expect("Failed to save order");
    }

    pub async fn set_delivered(&self, bot: &Bot, context_http: &ContextHTTP, order: &mut Order) {
        if order.order_state != OrderState::Delivery {
            return;
        }
        
        let order_manager = &bot.order_manager;

        order_manager.end_order(context_http, order).await;
        order.order_state = OrderState::Delivered;
        let _ = &order_manager.message_manager.add_to_archive(context_http, order).await;
        order.save(&bot.db_info.db, None).await.expect("Failed to save order");
    }
}