use enum_iterator::all;
use serenity::{model::prelude::{interaction::{Interaction, InteractionResponseType, message_component::MessageComponentInteraction, InteractionType, modal::ModalSubmitInteraction}, component::{InputTextStyle, ActionRowComponent}}, builder::CreateSelectMenu};

use crate::{ContextHTTP, bot::Bot};

use super::{models::review_rating::ReviewRating, review_manager};

pub async fn on_interaction(bot: &Bot, context_http: &ContextHTTP, interaction: Interaction) {
    if interaction.kind() == InteractionType::MessageComponent {
        let interaction = interaction.message_component().unwrap();

        if interaction.guild_id.is_none() || interaction.guild_id.unwrap() != bot.guild_id {
            return;
        }

        let component_id = interaction.data.custom_id.as_str();

        match component_id {
            "review" => on_review_interaction(bot, &context_http, &interaction).await,
            "review-select-order" => on_review_select_order(bot, &context_http, &interaction).await,
            _ => {}
        }

        if component_id.starts_with("review-select-rating:") {
            on_review_select_rating(&context_http, &interaction).await;
        }
    } else if interaction.kind() == InteractionType::ModalSubmit {
        let interaction = interaction.modal_submit().unwrap();

        if interaction.guild_id.is_none() || interaction.guild_id.unwrap() != bot.guild_id {
            return;
        }

        let component_id = interaction.data.custom_id.as_str();

        if component_id.starts_with("review-text-input:"){
            on_review_submit(bot, &context_http, &interaction).await;
        }
    }
}

async fn on_review_interaction(bot: &Bot, context_http: &ContextHTTP, interaction: &MessageComponentInteraction){
    let user = &interaction.user;
    let orders = review_manager::can_review(bot, user.id).await;

    if orders.is_none() {
        interaction.create_interaction_response(context_http, |response| {
            response.kind(InteractionResponseType::ChannelMessageWithSource).interaction_response_data(|data| {
                data.content("You don't have any orders to review").ephemeral(true)
            })
        }).await.expect("Failed to send interaction response");
        return;
    }

    let orders = orders.unwrap();

    interaction.create_interaction_response(context_http, |response|
        response.kind(InteractionResponseType::ChannelMessageWithSource).interaction_response_data(|data|
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

async fn on_review_select_order(_bot: &Bot, context_http: &ContextHTTP, interaction: &MessageComponentInteraction){
    let order_id: i32 = interaction.data.values.get(0).unwrap().parse().unwrap();
    interaction.create_interaction_response(context_http, |response|
        response.kind(InteractionResponseType::ChannelMessageWithSource).interaction_response_data(|data|
            data.ephemeral(true).content("Select a rating").components(|components|
                components.create_action_row(|action_row|
                    action_row.create_select_menu(|select_menu| {
                        select_menu
                        .custom_id(format!("review-select-rating:{}", order_id))
                        .placeholder("Select a rating");

                        select_menu.options(|options| {
                            all::<ReviewRating>().into_iter().for_each(|rating| {
                                options.create_option(|option|
                                    option
                                    .value(rating.get_name())
                                    .label(rating.get_emoji())
                                );
                            });
                            options
                        });

                        select_menu
                    })
                )
            )
        )
    ).await.expect("Failed to send interaction response");
}

async fn on_review_select_rating(context_http: &ContextHTTP, interaction: &MessageComponentInteraction) {
    let custom_id = interaction.data.custom_id.as_str();
    let order_id: i32 = custom_id.split(":").collect::<Vec<&str>>()[1].parse().unwrap();
    let review_rating = ReviewRating::from_name(interaction.data.values.get(0).unwrap().as_str()).unwrap();

    interaction.create_interaction_response(context_http, |response|
        response.kind(InteractionResponseType::Modal).interaction_response_data(|data|
            data
            .custom_id(format!("review-text-input:{}:{}", order_id, review_rating.get_name()))
            .title(format!("{} #{}", review_rating.get_emoji(), order_id))
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

async fn on_review_submit(bot: &Bot, context_http: &ContextHTTP, interaction: &ModalSubmitInteraction) {
    let user = &interaction.user;
    let split = interaction.data.custom_id.as_str().split(":").collect::<Vec<&str>>();
    let order_id: i32 = split[1].parse().unwrap();
    let review_rating = ReviewRating::from_name(split[2]).unwrap();
    let comment = interaction.data.components.get(0).unwrap().components.get(0).unwrap();
    let comment = match comment {
        ActionRowComponent::InputText(input) => input.value.clone(),
        _ => return,
    };

    let result = review_manager::add_review(bot, context_http, user, order_id, review_rating, comment.to_string()).await;

    let response_content = match result {
        Ok(_) => "Your review has been added!",
        Err(_) => "An error occurred while adding your review."
    };
    interaction.create_interaction_response(context_http, |response|
        response.kind(InteractionResponseType::ChannelMessageWithSource).interaction_response_data(|data|
            data.ephemeral(true).content(response_content)
        )
    ).await.expect("Failed to send interaction response");
}

/* TO ADD WHEN THE SELECT MENU IS SUPPORTED BY DISCORD

    let user = &interaction.user;

    let orders = bot.order_manager.review_manager.can_review(bot, user.id).await;

    if orders.is_none() {
        interaction_utils::reply_message_component(context_http, &interaction, "You don't have any order to review").await;
        return;
    }

    let orders = orders.unwrap();

    interaction.create_interaction_response(context_http, |response|
        response
        .kind(InteractionResponseType::Modal)
        .interaction_response_data(|data|
            data
            .custom_id(format!("review:submit"))
            .title("Review")
            .components(|components|
                components.create_action_row(|action_row|
                    action_row
                    .create_select_menu(|select_menu|
                        select_menu
                        .custom_id("review:select-order")
                        .placeholder("Select an order")
                        .options(|options| {

                            for order in orders {
                                options.create_option(|option|
                                    option
                                    .label(format!("Order #{}", order.order_id))
                                    .value(format!("{}", order.order_id))
                                    .description(format!("{}", order.description))
                                );
                            }

                            options
                        })
                    )
                    .create_select_menu(|select_menu| {
                        select_menu
                        .custom_id("review:select-rating")
                        .placeholder("Select a rating")
                        .options(|options| {

                            for review_rating in all::<ReviewRating>() {
                                options.create_option(|option|
                                    option
                                    .label(review_rating.get_emoji())
                                    .value(review_rating.get_name())
                                );
                            }

                            options
                        })
                    })
                    .create_input_text(|input_text|
                        input_text
                        .custom_id("review:comment")
                        .label("Comment")
                        .placeholder("Write your comment here...")
                        .style(InputTextStyle::Paragraph)
                    )
                )
            )
        )
    ).await.expect("Failed to create interaction response");
 */