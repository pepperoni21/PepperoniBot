use serenity::{model::{prelude::component::ButtonStyle, user::User}, prelude::Mentionable};
use wither::{Model, bson::doc};

use crate::{ContextHTTP, core::order::models::order::Order, bot::Bot, utils::channel_utils};

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

    async fn generate_message(&self, context_http: &ContextHTTP) {
        let make_review_channel = channel_utils::fetch_guild_channel("MAKE_REVIEW_CHANNEL_ID", context_http).await;

        let history = make_review_channel
            .messages(context_http, |retriever| retriever.limit(1))
            .await
            .expect("Failed to get make review channel messages");

        if history.is_empty() {
            make_review_channel.send_message(context_http, |message| {
                message.embed(|embed|
                    embed
                        .title("Make a Review")
                        .description("**To make a review, you need to have made an order. Then, you can click on the button below to review your order.**\n\nNo review will be deleted, no matter what you write. However, I let myself the right to reply.")
                ).components(|components|
                    components.create_action_row(|action_row|
                        action_row.create_button(|button|
                            button
                                .style(ButtonStyle::Success)
                                .custom_id("review")
                                .label("Make a Review")
                        )
                    )
                )
            }).await.expect("Failed to send message to make review channel");
        }
    }

    pub async fn add_review(&self, bot: &Bot, context_http: &ContextHTTP, user: &User, order_id: i32, review_rating: ReviewRating, comment: String) -> Result<(), String>{
        let db = &bot.db_info.db;

        let mut order: Order = Order::find_one(db, doc! {"order_id": order_id}, None).await.expect("Failed to find order").expect("Order not found");

        let reviews_channel = channel_utils::fetch_guild_channel("REVIEWS_CHANNEL_ID", context_http).await;

        let message = reviews_channel.send_message(context_http, |message|
            message.embed(|embed|
                embed.title(format!("Review #{}", order_id))
                    .field("Customer", user.mention(), false)
                    .field("Rating", review_rating.get_emoji(), false)
                    .field("Comment", &comment, true)
                    .author(|author| author.name(&user.name).icon_url(&user.face()))
            )
        ).await;

        let message = message.expect("Failed to send message to review channel");

        let review = Review {
            rating: review_rating,
            comment,
            message_id: message.id.0,
        };
        
        order.review = Some(review);
        order.save(db, None).await.expect("Failed to save order");

        Ok(())
    }

}
