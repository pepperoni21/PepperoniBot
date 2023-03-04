use serenity::model::{prelude::command::{CommandType, Command}, Permissions};

use crate::ContextHTTP;

pub async fn load_command(context_http: &ContextHTTP) {
    Command::create_global_application_command(context_http, |command| {
        command
                .name("Add developer")
                .default_member_permissions(Permissions::ADMINISTRATOR)
                .kind(CommandType::User)
    })
    .await
    .expect("Failed to load commands");

    Command::create_global_application_command(context_http, |command| {
        command
                .name("Remove developer")
                .default_member_permissions(Permissions::ADMINISTRATOR)
                .kind(CommandType::User)
    })
    .await
    .expect("Failed to load commands");
}