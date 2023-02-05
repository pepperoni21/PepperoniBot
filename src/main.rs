mod connector;
mod event_handler;
mod core;

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();

    println!("Connecting to Discord!");
    connector::connect().await;
}
