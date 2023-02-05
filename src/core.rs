use std::env;

use serenity::model::prelude::GuildId;

use crate::{core::{order::order_manager::OrderManager, review::review_manager::ReviewManager}, ContextHTTP};

pub mod db;
pub mod order;
pub mod review;

pub async fn load(context_http: ContextHTTP) {
    println!("Connected to Discord!");

    let db_info = db::DBInfo::new().await;

    let guild_id = GuildId(env::var("GUILD_ID")
        .expect("Expected a GUILD_ID in the environment")
        .parse()
        .expect("GUILD_ID is not a valid ID"));

    OrderManager::new(context_http.clone(), &db_info, guild_id.clone()).await;
    ReviewManager::new();
}
