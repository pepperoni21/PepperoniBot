use enum_iterator::all;
use serenity::{model::prelude::{interaction::{Interaction, InteractionResponseType, message_component::MessageComponentInteraction}, component::InputTextStyle}, futures::TryStreamExt, builder::CreateSelectMenu};
use wither::{Model, bson::{doc, to_bson}};

use crate::{ContextHTTP, bot::Bot, core::order::models::{order::Order, order_state::OrderState}};

use super::models::{review::Review, review_rating::ReviewRating};

pub struct ReviewListener;

impl ReviewListener {

    pub async fn interaction_create(&self, bot: &Bot, context_http: ContextHTTP, interaction: Interaction) {
        println!("Interaction: {:?}", interaction.kind());
        let message_component_interaction = interaction.message_component();
        if message_component_interaction.is_none() {
            return;
        }
        let message_component_interaction = message_component_interaction.unwrap();
        let component_id = message_component_interaction.data.custom_id.as_str();

        match component_id {
            "review" => self.on_review_interaction(bot, &context_http, &message_component_interaction).await,
            "review-select-order" => self.on_review_select_order(bot, &context_http, &message_component_interaction).await,
            _ => {}
        }

        if component_id.starts_with("review-select-rating:") {
            self.on_review_select_rating(bot, &context_http, &message_component_interaction).await;
        } else if component_id.starts_with("review-text-input:"){
            self.on_review_submit(bot, &context_http, &message_component_interaction).await;
        }
    }

    async fn on_review_interaction(&self, bot: &Bot, context_http: &ContextHTTP, interaction: &MessageComponentInteraction){
        let user = &interaction.user;
        
        let orders_cursor = Order::find(&bot.db_info.db, doc! {
            "customer_id": user.id.to_string(),
            "state": to_bson(&OrderState::Delivered).unwrap(),
            "review": to_bson(&None::<Review>).unwrap()
        }, None).await.expect("Failed to find orders");
        let orders: Vec<Order> = orders_cursor.try_collect().await.expect("Failed to collect orders");

        if orders.is_empty() {
            interaction.create_interaction_response(context_http, |response| {
                response.kind(InteractionResponseType::Pong).interaction_response_data(|data| {
                    data.content("You don't have any orders to review")
                })
            }).await.expect("Failed to send interaction response");
            return;
        }

        interaction.create_interaction_response(context_http, |response|
            response.kind(InteractionResponseType::Pong).interaction_response_data(|data|
                data.ephemeral(true).content("Select an order to review")
                    .components(|components| {
                        components.create_action_row(|action_row| {
                            action_row.create_select_menu(|select_menu: &mut CreateSelectMenu| {
                                select_menu
                                .custom_id("review-select-order")
                                .placeholder("Select an order to review");

                                for order in orders {
                                    select_menu.options(|options|
                                        options.create_option(|option|
                                            option
                                            .value(order.order_id.to_string())
                                            .label(format!("#{}", order.order_id))
                                            .description(order.description)
                                        )
                                    );
                                }
                                select_menu
                            })
                        })
                    })
            )
        ).await.expect("Failed to send interaction response");
    }

    async fn on_review_select_order(&self, _bot: &Bot, context_http: &ContextHTTP, interaction: &MessageComponentInteraction){
        let order_id: i32 = interaction.data.values.get(0).unwrap().parse().unwrap();
        interaction.create_interaction_response(context_http, |response|
            response.kind(InteractionResponseType::Pong).interaction_response_data(|data|
                data.ephemeral(true).content("Select a rating").components(|components|
                    components.create_action_row(|action_row|
                        action_row.create_select_menu(|select_menu| {
                            select_menu
                            .custom_id(format!("review-select-rating:{}", order_id))
                            .placeholder("Select a rating");

                            all::<ReviewRating>().into_iter().for_each(|rating| {
                                select_menu.options(|options|
                                    options.create_option(|option|
                                        option
                                        .value(rating.get_name())
                                        .label(rating.get_emoji())
                                    )
                                );
                            });

                            select_menu
                        })
                    )
                )
            )
        ).await.expect("Failed to send interaction response");
    }

    async fn on_review_select_rating(&self, _bot: &Bot, context_http: &ContextHTTP, interaction: &MessageComponentInteraction) {
        let custom_id = interaction.data.custom_id.as_str();
        let order_id: i32 = custom_id.split(":").collect::<Vec<&str>>()[1].parse().unwrap();
        let review_rating = ReviewRating::from_name(interaction.data.values.get(0).unwrap().as_str()).unwrap();

        interaction.create_interaction_response(context_http, |response|
            response.kind(InteractionResponseType::Modal).interaction_response_data(|data|
                data
                .custom_id(format!("review-text-input:{}:{}", order_id, review_rating.get_name()))
                .content(format!("{} #{}", review_rating.get_emoji(), order_id))
                .components(|components|
                    components.create_action_row(|action_row|
                        action_row.create_input_text(|input_text|
                            input_text
                            .custom_id("review-comment")
                            .placeholder("Write your comment here")
                            .label("Write your comment here")
                            .style(InputTextStyle::Paragraph)
                        )
                    )
                )
            )
        ).await.expect("Failed to send interaction response");
    }

    async fn on_review_submit(&self, bot: &Bot, context_http: &ContextHTTP, interaction: &MessageComponentInteraction) {
        let user = &interaction.user;
        let split = interaction.data.custom_id.as_str().split(":").collect::<Vec<&str>>();
        let order_id: i32 = split[1].parse().unwrap();
        let review_rating = ReviewRating::from_name(split[2]).unwrap();
        let comment = interaction.data.values.get(0).unwrap().as_str();

        let result = bot.order_manager.review_manager
        .add_review(bot, context_http, user, order_id, review_rating, comment.to_string()).await;

        let response_content = match result {
            Ok(_) => "Your review has been added!",
            Err(_) => "An error occurred while adding your review."
        };
        interaction.create_interaction_response(context_http, |response|
            response.kind(InteractionResponseType::Pong).interaction_response_data(|data|
                data.ephemeral(true).content(response_content)
            )
        ).await.expect("Failed to send interaction response");
    }

}