use serenity::{model::prelude::UserId, futures::StreamExt};
use wither::{Model, bson::{doc, to_bson}};

use crate::{bot::Bot, ContextHTTP, utils::{channel_utils, role_utils}, core::order::order_manager};

use super::{models::{developer::Developer, developer_assets::DeveloperAssets}, developer_message_manager::DeveloperMessageManager, command::developer_command};

pub async fn load(bot: &Bot, context_http: &ContextHTTP) {
    developer_command::load_command(context_http).await;

    let developers = Developer::find(&bot.db_info.db, None, None)
        .await
        .expect("Failed to fetch developers")
        .map(|developer| developer.expect("Failed to fetch developer"))
        .collect::<Vec<Developer>>().await;

    for developer in developers {
        load_developer(bot, context_http, developer).await;
    }
}

async fn load_developer(bot: &Bot, context_http: &ContextHTTP, mut developer: Developer) {
    let user = context_http.get_user(developer.user_id).await.expect("Failed to fetch user");
    let member = bot.guild_id.member(context_http, user.id).await;
    if member.is_err() {
        remove_developer(bot, context_http, &mut developer).await;
        return;
    }

    let developers_channel = channel_utils::fetch_guild_channel("DEVELOPERS_CHANNEL_ID", &context_http).await;
    let developer_list_message_id = developer.assets.developer_list_message_id;
    let developers_channel_message = match developer_list_message_id {
        None => None,
        Some(message_id) => {
            match developers_channel.message(context_http, message_id).await {
                Ok(message) => Some(message),
                Err(_) => None,
            }
        }
    };
    if developers_channel_message.is_none() {
        DeveloperMessageManager::send_developers_channel_message(bot, context_http, &mut developer, developers_channel, false).await;
    }

    let introduction_channel = channel_utils::fetch_guild_channel("DEVELOPERS_INTRODUCTION_CHANNEL_ID", &context_http).await;
    let introduction_message_id = developer.assets.introduction_message_id;
    let introduction_channel_message = match introduction_message_id {
        None => None,
        Some(message_id) => {
            match introduction_channel.message(context_http, message_id).await {
                Ok(message) => Some(message),
                Err(_) => None,
            }
        }
    };
    if introduction_channel_message.is_none() {
        DeveloperMessageManager::send_introduction_channel_message(bot, context_http, &mut developer, introduction_channel, false).await;
    }
}

pub async fn create_developer(bot: &Bot, context_http: &ContextHTTP, user_id: u64) {
    let developers_channel = channel_utils::fetch_guild_channel("DEVELOPERS_CHANNEL_ID", &context_http).await;

    let introduction_channel = channel_utils::fetch_guild_channel("DEVELOPERS_INTRODUCTION_CHANNEL_ID", &context_http).await;

    let assets: DeveloperAssets = DeveloperAssets::new();
    let mut developer = Developer::new(user_id, assets);

    DeveloperMessageManager::send_developers_channel_message(bot, context_http, &mut developer, developers_channel, true).await;
    DeveloperMessageManager::send_introduction_channel_message(bot, context_http, &mut developer, introduction_channel, true).await;

    developer.save(&bot.db_info.db, None).await.expect("Failed to create developer");

    let mut member = bot.guild_id.member(context_http, UserId(user_id)).await.expect("Failed to fetch member");
    member.add_role(context_http, role_utils::fetch_guild_role("DEVELOPER_ROLE_ID")).await.expect("Failed to add developer role");
}

pub async fn edit_developer(bot: &Bot, context_http: &ContextHTTP, developer: &mut Developer) {
    let developers_channel = channel_utils::fetch_guild_channel("DEVELOPERS_CHANNEL_ID", &context_http).await;
    DeveloperMessageManager::edit_developers_channel_message(bot, context_http, developer, developers_channel).await;

    let introduction_channel = channel_utils::fetch_guild_channel("DEVELOPERS_INTRODUCTION_CHANNEL_ID", &context_http).await;
    DeveloperMessageManager::edit_introduction_channel_message(bot, context_http, developer, introduction_channel).await;

    developer.save(&bot.db_info.db, None).await.expect("Failed save changes to developer");
}

pub async fn remove_developer(bot: &Bot, context_http: &ContextHTTP, developer: &mut Developer) {
    developer.delete(&bot.db_info.db).await.expect("Failed to delete developer");

    tokio::spawn({
        let developer_assets = developer.assets.clone();
        let context_http = context_http.clone();
        async move {
            developer_assets.delete_assets(&context_http).await;
        }
    });

    let mut member = bot.guild_id.member(context_http, developer.user_id).await.expect("Failed to fetch member");
    
    if let Err(_) = member.remove_role(context_http, role_utils::fetch_guild_role("DEVELOPER_ROLE_ID")).await {
        println!("Failed to remove developer role from {}", member.user.tag());
    }

    let orders = order_manager::fetch_current_orders_by_developer(bot, developer.user_id).await;
    for mut order in orders {
        order_manager::cancel_order(bot, context_http, &mut order).await;
    }
}

pub async fn fetch_developer(bot: &Bot, user_id: UserId) -> Option<Developer> {
    Developer::find_one(&bot.db_info.db, doc! {
        "user_id": to_bson(&user_id.0).unwrap()
    }, None).await.expect("Failed to find developer")
}

pub async fn is_developer(bot: &Bot, user_id: UserId) -> bool {
    Developer::find_one(&bot.db_info.db, doc! {
        "user_id": to_bson(&user_id.0).unwrap()
    }, None).await.expect("Failed to find developer").is_some()
}