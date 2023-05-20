use std::{sync::Arc, env};

use serenity::http::Http;

mod connector;
mod bot;
mod core;
mod utils;

pub type ContextHTTP = Arc<Http>;

#[tokio::main]
async fn main() {
    let dev: bool = env::var("DEV").unwrap_or("false".to_string()).parse().unwrap();
    if dev {
        dotenv::from_filename("dev.env").ok();
    } else {
        dotenv::dotenv().ok();
    }

    println!("Starting up in {} mode!", if dev { "dev" } else { "prod" });

    println!("Connecting to Discord!");
    connector::connect().await;
}
