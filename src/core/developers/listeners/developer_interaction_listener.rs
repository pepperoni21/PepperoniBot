use serenity::model::prelude::{interaction::{Interaction, message_component::MessageComponentInteraction, modal::ModalSubmitInteraction, InteractionResponseType}, UserId, component::{ActionRowComponent, InputTextStyle}};
use wither::{bson::{doc, to_bson}, Model};

use crate::{bot::Bot, ContextHTTP, utils::{mention_utils::mention_user, interaction_utils}, core::developers::{developer_manager, models::developer::Developer}};

pub async fn on_interaction(bot: &Bot, context_http: &ContextHTTP, interaction: Interaction) {
    match interaction {
        Interaction::ModalSubmit(interaction) => {
            let component_id = interaction.data.custom_id.clone();

            if !component_id.starts_with("developer:") {
                return;
            }

            let component_id = component_id.strip_prefix("developer:").unwrap();

            if component_id.starts_with("edit:submit=") {
                let user_id = UserId(component_id.strip_prefix("edit:submit=").unwrap().parse::<u64>().unwrap());
                on_edit_developer_submit(bot, context_http, interaction, user_id).await;
            }
        },
        Interaction::MessageComponent(interaction) => {
            let component_id = interaction.data.custom_id.clone();

            if !component_id.starts_with("developer:") {
                return;
            }

            let component_id = component_id.strip_prefix("developer:").unwrap();

            if component_id.starts_with("remove=") {
                let user_id = UserId(component_id.strip_prefix("remove=").unwrap().parse::<u64>().unwrap());
                on_remove_developer(bot, context_http, interaction, user_id).await;
            } else if component_id.starts_with("edit=") {
                let user_id = UserId(component_id.strip_prefix("edit=").unwrap().parse::<u64>().unwrap());
                on_edit_developer(bot, context_http, interaction, user_id).await;
            }else if component_id.starts_with("add=") {
                let user_id = UserId(component_id.strip_prefix("add=").unwrap().parse::<u64>().unwrap());
                on_create_developer(bot, context_http, interaction, user_id).await;
            }
        },
        _ => {}
    };
}

async fn on_create_developer(bot: &Bot, context_http: &ContextHTTP, interaction: MessageComponentInteraction, user_id: UserId) {
    interaction.create_interaction_response(context_http, |response|
        response
        .interaction_response_data(|data| data.ephemeral(true))
        .kind(serenity::model::prelude::interaction::InteractionResponseType::DeferredUpdateMessage)
    ).await.expect("Failed to send interaction response");

    if developer_manager::is_developer(bot, user_id).await {
        interaction.edit_original_interaction_response(context_http, |response|
            response
            .content(format!("{} is already a developer!", mention_user(&user_id)))
        ).await.expect("Failed to send interaction response");
    } else {
        developer_manager::create_developer(bot, context_http, user_id.0).await;
        interaction.edit_original_interaction_response(context_http, |response|
            response
            .content(format!("{} is now a developer!", mention_user(&user_id)))
            .components(|components| components)
        ).await.expect("Failed to send interaction response");
    }
}

async fn on_edit_developer(bot: &Bot, context_http: &ContextHTTP, interaction: MessageComponentInteraction, user_id: UserId) {
    if !developer_manager::is_developer(bot, user_id).await {
        interaction_utils::reply_message_component(context_http, &interaction, format!("{} is not a developer!", mention_user(&user_id))).await;
    } else {
        interaction.create_interaction_response(context_http, |response|
            response
            .kind(InteractionResponseType::Modal)
            .interaction_response_data(|data|
                data
                .custom_id(format!("developer:edit:submit={}", user_id.0))
                .title("Add developer")
                .components(|components|
                    components.create_action_row(|action_row|
                        action_row.create_input_text(|input_text|
                            input_text
                            .custom_id("developer:edit:introduction")
                            .placeholder("Write the introduction here")
                            .label("Introduction")
                            .style(InputTextStyle::Paragraph)
                        )
                    )
                )
            )
        ).await.expect("Failed to send interaction response");
    }
}

async fn on_edit_developer_submit(bot: &Bot, context_http: &ContextHTTP, interaction: ModalSubmitInteraction, user_id: UserId) {
    interaction.create_interaction_response(context_http, |response|
        response
        .interaction_response_data(|data| data.ephemeral(true))
        .kind(serenity::model::prelude::interaction::InteractionResponseType::DeferredChannelMessageWithSource)
    ).await.expect("Failed to send interaction response");

    let member = interaction.member.as_ref().unwrap();

    if member.user.id != user_id && !member.permissions.unwrap().administrator() {
        interaction.edit_original_interaction_response(context_http, |response|
            response
            .content("You are not allowed to edit this developer!")
        ).await.expect("Failed to send interaction response");
        return;
    }

    let developer = developer_manager::fetch_developer(bot, user_id).await;

    if developer.is_none() {
        interaction.edit_original_interaction_response(context_http, |response|
            response
            .content(format!("{} is not a developer!", mention_user(&user_id)))
        ).await.expect("Failed to send interaction response");
    } else {
        let introduction = match interaction.data.components.get(0).unwrap().components.get(0).unwrap() {
            ActionRowComponent::InputText(select_menu) => select_menu.value.clone(),
            _ => panic!("Invalid introduction select menu")
        };

        let mut developer = developer.unwrap();
        developer.introduction = Some(introduction);

        let response_content = if member.user.id == user_id {
            format!("Your developer information has been updated!")
        } else {
            format!("{}'s developer information has been updated!", mention_user(&user_id))
        };

        developer_manager::edit_developer(bot, context_http, &mut developer).await;
        interaction.edit_original_interaction_response(context_http, |response|
            response.content(response_content)
        ).await.expect("Failed to send interaction response");
    }
}

async fn on_remove_developer(bot: &Bot, context_http: &ContextHTTP, interaction: MessageComponentInteraction, user_id: UserId) {
    interaction.create_interaction_response(context_http, |response|
        response
        .interaction_response_data(|data| data.ephemeral(true))
        .kind(InteractionResponseType::DeferredChannelMessageWithSource)
    ).await.expect("Failed to send interaction response");

    if !developer_manager::is_developer(bot, user_id).await {
        interaction.edit_original_interaction_response(context_http, |response|
            response
            .content(format!("{} is not a developer!", mention_user(&user_id)))
        ).await.expect("Failed to send interaction response");
    } else {
        let mut developer = Developer::find_one(&bot.db_info.db, doc! {
            "user_id": to_bson(&user_id.0).unwrap()
        }, None).await.expect("Failed to remove developer").expect("Developer not found");
        developer_manager::remove_developer(bot, context_http, &mut developer).await;
        interaction.edit_original_interaction_response(context_http, |response|
            response
            .content(format!("{} is no longer a developer!", mention_user(&user_id)))
        ).await.expect("Failed to send interaction response");
    }
}