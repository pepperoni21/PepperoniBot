use serenity::model::{prelude::{GuildId, command::CommandType}, Permissions};

use crate::ContextHTTP;

pub async fn load_command(context_http: &ContextHTTP, guild_id: &GuildId) {
    GuildId::set_application_commands(guild_id, context_http, |commands| {
        commands
        .create_application_command(|command| {
            command
                .name("Add developer")
                .default_member_permissions(Permissions::ADMINISTRATOR)
                .kind(CommandType::User)
        })
        .create_application_command(|command|
            command
                .name("Remove developer")
                .default_member_permissions(Permissions::ADMINISTRATOR)
                .kind(CommandType::User)
        )
    })
    .await
    .expect("Failed to load commands");
}