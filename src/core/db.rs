use wither::mongodb::{Client, Database};

#[derive(Clone)]
pub struct DBInfo {
    pub db: Database
}

impl DBInfo {
    pub async fn new() -> Self {
        let mongo_uri = std::env::var("MONGODB_URI").expect("Expected a MONGODB_URI in the environment");
        
        let db = Client::with_uri_str(&mongo_uri).await.expect("Failed to connect to MongoDB").database("db");

        Self {
            db
        }
    }
}
