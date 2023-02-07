use std::env;

use serenity::{model::{prelude::component::ButtonStyle, user::User}, prelude::Mentionable};
use wither::{Model, bson::doc};

use crate::{ContextHTTP, core::order::models::order::Order, bot::Bot};

use super::{models::{review_rating::ReviewRating, review::Review}, review_listener::ReviewListener};

pub struct ReviewManager {
    pub listener: ReviewListener
}

impl ReviewManager {

    pub fn new() -> Self {
        let review_manager = Self {
            listener: ReviewListener
        };
        review_manager
    }

    pub async fn load(&self, context_http: &ContextHTTP) {
        self.generate_message(context_http).await;
    }

    async fn generate_message(&self, context_http: &ContextHTTP){
        let make_review_channel_id: u64 = env::var("MAKE_REVIEW_CHANNEL_ID")
            .expect("Expected a MAKE_REVIEW_CHANNEL_ID in the environment")
            .parse()
            .expect("MAKE_REVIEW_CHANNEL_ID is not a valid ID");
        let make_review_channel = context_http.get_channel(make_review_channel_id).await.expect("Failed to get make review channel").guild().expect("Failed to get make review channel");
        let history = make_review_channel.messages(context_http, |retriever| retriever.limit(1)).await.expect("Failed to get make review channel messages");
        if history.is_empty() {
            make_review_channel.send_message(context_http, |message| {
                message.embed(|embed| {
                    embed.title("Make a Review");
                    embed.description("**To make a review, you need to have made an order. Then, you can click on the button below to review your order.**\n\nNo review will be deleted, no matter what you write. However, I let myself the right to reply.");
                    embed
                }).components(|components| {
                    components.create_action_row(|action_row|
                        action_row.create_button(|button|
                            button.style(ButtonStyle::Success).custom_id("review").label("Make a Review")))
                })
            }).await.expect("Failed to send message to make review channel");
        }
    }

    async fn _add_review(&self, bot: &Bot, context_http: &ContextHTTP, user: User, order_id: i32, review_rating: ReviewRating, comment: String){
        let db = &bot.db_info.db;

        let mut order: Order = Order::find_one(db, doc! {"order_id": order_id}, None).await.expect("Failed to find order").expect("Order not found");
        
        let review_channel_id: u64 = env::var("REVIEW_CHANNEL_ID")
            .expect("Expected a REVIEW_CHANNEL_ID in the environment")
            .parse()
            .expect("REVIEW_CHANNEL_ID is not a valid ID");
        let review_channel = context_http.get_channel(review_channel_id).await.expect("Failed to get review channel").guild().expect("Failed to get review channel");


        let message = review_channel.send_message(context_http, |message|
            message.embed(|embed| 
                embed.title(format!("Review #{}", order_id))
                    .field("Customer", user.mention(), false)
                    .field("Rating", review_rating.get_emoji(), false)
                    .field("Comment", &comment, true)
                    .author(|author| author.name(&user.name).icon_url(&user.face()))
            )
        ).await.expect("Failed to send message to review channel");
        
        let review = Review {
            rating: review_rating,
            comment,
            message_id: message.id.0,
        };
        order.review = Some(review);
        order.save(db, None).await.expect("Failed to save order");
    }

}