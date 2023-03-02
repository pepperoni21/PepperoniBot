use std::sync::Arc;

use serenity::http::Http;

mod connector;
mod bot;
mod core;
mod utils;

pub type ContextHTTP = Arc<Http>;

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();

    println!("Connecting to Discord!");
    connector::connect().await;
}
