pub mod db;

pub async fn load() {
    println!("Connected to Discord!");

    let db_info = db::DBInfo::new().await;
}
