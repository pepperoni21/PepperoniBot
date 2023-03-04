use async_recursion::async_recursion;
use serenity::model::prelude::{GuildChannel, component::ButtonStyle};
use wither::Model;

use crate::{ContextHTTP, bot::Bot};

use super::models::developer::Developer;

pub struct DeveloperMessageManager;

impl DeveloperMessageManager {

    #[async_recursion]
    pub async fn send_developers_channel_message(bot: &Bot, context_http: &ContextHTTP, developer: &mut Developer, developers_channel: GuildChannel, initial: bool) {
        let user = context_http.get_user(developer.user_id).await.expect("Failed to get user");
        let message = developers_channel.send_message(context_http, |message|{
            message.embed(|embed|
                embed
                .author(|author| author.name(&user.name).icon_url(&user.face()))
                .title(user.name)
            ).components(|components|
                components.create_action_row(|action_row|
                    action_row.create_button(|button|
                        button
                            .style(ButtonStyle::Primary)
                            .label("Edit")
                            .custom_id(format!("developer:edit={}", developer.user_id))
                    )
                )
            )
        }).await.expect("Failed to send developers channel message");

        developer.assets.developer_list_message_id = Some(message.id.0);

        if !initial {
            developer.save(&bot.db_info.db, None).await.expect("Failed to save developer");
            Self::edit_developers_channel_message(bot, context_http, developer, developers_channel).await;
        }
    }

    pub async fn edit_developers_channel_message(bot: &Bot, context_http: &ContextHTTP, developer: &mut Developer, developers_channel: GuildChannel) {
        if let Some(developers_channel_message_id) = developer.assets.developer_list_message_id {
            if let Err(_) = developers_channel.message(context_http, developers_channel_message_id).await {
                Self::send_introduction_channel_message(bot, context_http, developer, developers_channel, false).await;
                return;
            }
        } else {
            Self::send_introduction_channel_message(bot, context_http, developer, developers_channel, false).await;
            return;
        }

        let user = context_http.get_user(developer.user_id).await.expect("Failed to get user");
        let mut message = developers_channel.message(context_http, developer.assets.developer_list_message_id.unwrap()).await.expect("Failed to get message");
        message.edit(context_http, |message|{
            message.embed(|embed| {
                embed
                .author(|author| author.name(&user.name).icon_url(&user.face()))
                .title(user.name);

                if let Some(introduction) = &developer.introduction {
                    embed.description(introduction);
                }

                embed
            }).components(|components|
                components.create_action_row(|action_row|
                    action_row.create_button(|button|
                        button
                            .style(ButtonStyle::Primary)
                            .label("Edit")
                            .custom_id(format!("developer:edit={}", developer.user_id))
                    )
                )
            )
        }).await.expect("Failed to edit developers channel message");
    }

    #[async_recursion]
    pub async fn send_introduction_channel_message(bot: &Bot, context_http: &ContextHTTP, developer: &mut Developer, introduction_channel: GuildChannel, initial: bool) {
        let user = context_http.get_user(developer.user_id).await.expect("Failed to get user");
        let message = introduction_channel.send_message(context_http, |message|{
            message.embed(|embed|
                embed
                .author(|author| author.name(&user.name).icon_url(&user.face()))
                .title(user.name)
            )
        }).await.expect("Failed to send introduction channel message");

        developer.assets.introduction_message_id = Some(message.id.0);

        if !initial {
            developer.save(&bot.db_info.db, None).await.expect("Failed to save developer");
            Self::edit_introduction_channel_message(bot, context_http, developer, introduction_channel).await;
        }
    }

    pub async fn edit_introduction_channel_message(bot: &Bot, context_http: &ContextHTTP, developer: &mut Developer, introduction_channel: GuildChannel) {
        if let Some(introduction_message_id) = developer.assets.introduction_message_id {
            if let Err(_) = introduction_channel.message(context_http, introduction_message_id).await {
                Self::send_introduction_channel_message(bot, context_http, developer, introduction_channel, false).await;
                return;
            }
        } else {
            Self::send_introduction_channel_message(bot, context_http, developer, introduction_channel, false).await;
            return;
        }

        let user = context_http.get_user(developer.user_id).await.expect("Failed to get user");
        let mut message = introduction_channel.message(context_http, developer.assets.introduction_message_id.unwrap()).await.expect("Failed to get message");
        message.edit(context_http, |message|{
            message.embed(|embed| {
                embed
                .author(|author| author.name(&user.name).icon_url(&user.face()))
                .title(user.name);

                if let Some(introduction) = &developer.introduction {
                    embed.description(introduction);
                }

                embed
            })
        }).await.expect("Failed to edit introduction channel message");
    }

}