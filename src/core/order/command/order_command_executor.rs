use std::sync::Arc;

use serenity::model::prelude::{interaction::{Interaction, InteractionType, application_command::{ApplicationCommandInteraction, CommandDataOptionValue}, InteractionResponseType}};
use wither::{Model, bson::doc};

use crate::{ContextHTTP, bot::Bot, core::order::models::{order::Order, order_type::OrderType}};

pub async fn on_interaction(bot: &Bot, context_http: &ContextHTTP, interaction: Interaction) {
    if interaction.kind() != InteractionType::ApplicationCommand {
        return;
    }

    let interaction = interaction.application_command().unwrap();

    if interaction.guild_id.is_none() || interaction.guild_id.unwrap() != bot.guild_id {
        return;
    }

    if interaction.data.name != "order" {
        return;
    }

    match interaction.data.options.get(0).unwrap().name.as_str() {
        "create" => on_create_command(bot, context_http, interaction).await,
        "cancel" => on_cancel_command(bot, context_http, interaction).await,
        _ => {}
    }
}


async fn on_create_command(bot: &Bot, context_http: &ContextHTTP, interaction: ApplicationCommandInteraction) {
    let sub_command_option = interaction.data.options.get(0).unwrap();
    let user_option = sub_command_option.options.get(0).expect("Expected user option").resolved.as_ref().expect("Expected user option to be resolved");
    let user = match user_option {
        CommandDataOptionValue::User(u, _member) => u,
        _ => return,
    };

    let order_type_option = sub_command_option.options.get(1).expect("Expected order type option").value.as_ref().expect("Expected order type option to be resolved").as_str().expect("Expected order type option to be a string");
    let order_type = OrderType::from_value(order_type_option);

    let price: i32 = sub_command_option.options.get(2).unwrap().value.as_ref().unwrap().as_i64().unwrap() as i32;

    let description = sub_command_option.options.get(3).unwrap().value.as_ref().unwrap().as_str().unwrap().to_string();

    let order_manager = &bot.order_manager;
    order_manager.create_order(bot, context_http, Arc::new(user.clone()), order_type, price, description).await;

    interaction.create_interaction_response(context_http, |response| {
        response.kind(InteractionResponseType::ChannelMessageWithSource).interaction_response_data(|data| {
            data.content("Order created!").ephemeral(true)
        })
    }).await.expect("Failed to send interaction response");
}

async fn on_cancel_command(bot: &Bot, context_http: &ContextHTTP, interaction: ApplicationCommandInteraction) {
    let order_id: i32 = interaction.data.options.get(0).unwrap().options.get(0).unwrap().value.as_ref().unwrap().as_i64().unwrap() as i32;
    let order = Order::find_one(&bot.db_info.db, doc!{
        "order_id": order_id
    }, None).await.expect("Failed to find order");

    if order.is_none() {
        interaction.create_interaction_response(context_http, |response| {
            response.kind(InteractionResponseType::ChannelMessageWithSource).interaction_response_data(|data| {
                data.content("Order not found").ephemeral(true)
            })
        }).await.expect("Failed to send interaction response");
        return;
    }

    let mut order = order.unwrap();
    bot.order_manager.cancel_order(bot, context_http, &mut order).await;

    interaction.create_interaction_response(context_http, |response| {
        response.kind(InteractionResponseType::ChannelMessageWithSource).interaction_response_data(|data| {
            data.content("Order removed!").ephemeral(true)
        })
    }).await.expect("Failed to send interaction response");
}
