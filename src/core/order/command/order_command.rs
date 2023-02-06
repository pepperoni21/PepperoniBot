use serenity::model::prelude::{GuildId};

use crate::ContextHTTP;

pub async fn load_command(context_http: &ContextHTTP, guild_id: &GuildId){
    println!("Loading commands...");
}