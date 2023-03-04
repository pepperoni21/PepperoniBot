use std::env;

use serenity::model::prelude::GuildChannel;

use crate::ContextHTTP;

pub async fn fetch_guild_channel(var: &str, context_http: &ContextHTTP) -> GuildChannel {
    let channel_id: u64 = env::var(var)
        .expect(format!("{} is not set", var).as_str())
        .parse()
        .expect(format!("{} is not a valid u64", var).as_str());
    context_http.get_channel(channel_id).await
        .expect(format!("Failed to get {} channel", var).as_str())
        .guild()
        .expect(format!("{} channel is not a guild channel", var).as_str())
}