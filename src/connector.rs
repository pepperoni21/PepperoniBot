use std::env;

use serenity::{Client, prelude::GatewayIntents, framework::StandardFramework};

use crate::bot;

pub async fn connect() {
    let framework = StandardFramework::new();
    let token = env::var("DISCORD_BOT_TOKEN").expect("Expected a DISCORD_BOT_TOKEN in the environment");
    let intents = GatewayIntents::MESSAGE_CONTENT;

    let mut client = Client::builder(token, intents)
        .framework(framework)
        .event_handler(bot::Bot::new().await)
        .await
        .expect("Error creating client");

    if let Err(why) = client.start().await {
        println!("An error occurred while running the client: {:?}", why);
    }
}
