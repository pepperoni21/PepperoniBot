use serenity::model::prelude::GuildId;

use crate::ContextHTTP;

pub async fn load_command(_context_http: &ContextHTTP, _guild_id: &GuildId){
    println!("Loading commands...");
}