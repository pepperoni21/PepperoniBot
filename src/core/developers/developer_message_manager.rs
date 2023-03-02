use serenity::model::prelude::{GuildChannel, component::ButtonStyle};

use crate::ContextHTTP;

pub struct DeveloperMessageManager;

impl DeveloperMessageManager {

    pub async fn send_developers_channel_message(context_http: &ContextHTTP, user_id: u64, introduction: String, developers_channel: GuildChannel) -> Result<u64, String> {
        let user = context_http.get_user(user_id).await.expect("Failed to get user");
        let message = developers_channel.send_message(context_http, |message|{
            message.embed(|embed|
                embed
                .author(|author| author.name(&user.name).icon_url(&user.face()))
                .description(introduction)
                .title(user.name)
            ).components(|components|
                components.create_action_row(|action_row|
                    action_row.create_button(|button|
                        button
                            .style(ButtonStyle::Primary)
                            .label("Edit")
                            .custom_id(format!("dev-edit-{}", user_id))
                    )
                )
            )
        }).await.expect("Failed to send developers channel message");

        Ok(message.id.0)
    }

    pub async fn edit_developers_channel_message(context_http: &ContextHTTP, message_id: u64, user_id: u64, introduction: String, developers_channel: GuildChannel) -> Result<(), String> {
        let user = context_http.get_user(user_id).await.expect("Failed to get user");
        let mut message = developers_channel.message(context_http, message_id).await.expect("Failed to get message");
        message.edit(context_http, |message|{
            message.embed(|embed|
                embed
                .author(|author| author.name(&user.name).icon_url(&user.face()))
                .description(introduction)
                .title(user.name)
            ).components(|components|
                components.create_action_row(|action_row|
                    action_row.create_button(|button|
                        button
                            .style(ButtonStyle::Primary)
                            .label("Edit")
                            .custom_id(format!("dev-edit-{}", user_id))
                    )
                )
            )
        }).await.expect("Failed to edit developers channel message");

        Ok(())
    }

    pub async fn send_introduction_channel_message(context_http: &ContextHTTP, user_id: u64, introduction: String, introduction_channel: GuildChannel) -> Result<u64, String> {
        let user = context_http.get_user(user_id).await.expect("Failed to get user");
        let message = introduction_channel.send_message(context_http, |message|{
            message.embed(|embed|
                embed
                .author(|author| author.name(&user.name).icon_url(&user.face()))
                .description(introduction)
                .title(user.name)
            )
        }).await.expect("Failed to send introduction channel message");

        Ok(message.id.0)
    }

    pub async fn edit_introduction_channel_message(context_http: &ContextHTTP, message_id: u64, user_id: u64, introduction: String, introduction_channel: GuildChannel) -> Result<(), String> {
        let user = context_http.get_user(user_id).await.expect("Failed to get user");
        let mut message = introduction_channel.message(context_http, message_id).await.expect("Failed to get message");
        message.edit(context_http, |message|{
            message.embed(|embed|
                embed
                .author(|author| author.name(&user.name).icon_url(&user.face()))
                .description(introduction)
                .title(user.name)
            )
        }).await.expect("Failed to edit introduction channel message");

        Ok(())
    }

}