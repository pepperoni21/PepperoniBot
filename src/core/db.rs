use mongodb::{Client, options::ClientOptions};

pub struct DBInfo {
    pub mongo_client: Client
}

impl DBInfo {
    pub async fn new() -> Self {
        let mongo_uri = std::env::var("MONGODB_URI").expect("Expected a MONGODB_URI in the environment");
        let client_options = ClientOptions::parse(&mongo_uri).await.expect("Error parsing MongoDB URI");
        let mongo_client = Client::with_options(client_options).expect("Error creating MongoDB client");
        Self {
            mongo_client
        }
    }
}
