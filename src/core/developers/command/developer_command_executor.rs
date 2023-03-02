use serenity::model::prelude::{interaction::{InteractionType, Interaction, application_command::ApplicationCommandInteraction}, component::ButtonStyle};

use crate::{bot::Bot, ContextHTTP, utils::{mention_utils, interaction_utils}, core::developers::developer_manager::DeveloperManager};

pub async fn on_interaction(bot: &Bot, context_http: &ContextHTTP, interaction: Interaction) {
    if interaction.kind() != InteractionType::ApplicationCommand {
        return;
    }

    let interaction = interaction.application_command().unwrap();

    if interaction.guild_id.is_none() || interaction.guild_id.unwrap() != bot.guild_id {
        return;
    }

    let command_name = interaction.data.name.as_str();

    match command_name {
        "Add developer" => on_add_developer(&bot, context_http, interaction).await,
        "Remove developer" => on_remove_developer(&bot.developer_manager, context_http, interaction).await,
        _ => {}
    }
}

async fn on_add_developer(bot: &Bot, context_http: &ContextHTTP, interaction: ApplicationCommandInteraction) {
    let user = interaction.data.resolved.users.values().next().unwrap();
    if bot.developer_manager.is_developer(bot, user.id).await {
        interaction_utils::reply_application_command(context_http, &interaction, format!("{} is already a developer!", mention_utils::mention_user(&user.id))).await;
    } else {
        interaction.create_interaction_response(context_http, |response|
            response.interaction_response_data(|data|
                data
                .ephemeral(true)
                .content(format!("Are you sure you want to add {} as a developer?", mention_utils::mention_user(&user.id)))
                .components(|components|
                    components.create_action_row(|action_row|
                        action_row.create_button(|button|
                            button
                            .style(ButtonStyle::Primary)
                            .custom_id(format!("add_developer={}", user.id.0))
                            .label("Yes")
                        )
                    )
                )
            )
        ).await.expect("Failed to send interaction response");
    }
}

async fn on_remove_developer(developer_manager: &DeveloperManager, context_http: &ContextHTTP, interaction: ApplicationCommandInteraction) {

}