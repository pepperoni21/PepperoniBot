use mongodb::{Client, options::ClientOptions, Database};

#[derive(Clone)]
pub struct DBInfo {
    pub mongo_client: Client,
    pub db: Database
}

impl DBInfo {
    pub async fn new() -> Self {
        let mongo_uri = std::env::var("MONGODB_URI").expect("Expected a MONGODB_URI in the environment");
        let client_options = ClientOptions::parse(&mongo_uri).await.expect("Error parsing MongoDB URI");
        let mongo_client = Client::with_options(client_options).expect("Error creating MongoDB client");
        let db = mongo_client.database("db");
        Self {
            mongo_client,
            db
        }
    }
}
