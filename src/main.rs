use std::sync::Arc;

use serenity::http::Http;

mod connector;
mod event_handler;
mod core;

pub type ContextHTTP = Arc<Http>;

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();

    println!("Connecting to Discord!");
    connector::connect().await;
}
