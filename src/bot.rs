use std::env;

use serenity::{async_trait, prelude::{EventHandler, Context}, model::prelude::{Ready, GuildId}};

use crate::{core::{db::{self, DBInfo}, order::order_manager::OrderManager, review::review_manager::ReviewManager}, ContextHTTP};

pub struct Bot {
    db_info: DBInfo,
    order_manager: OrderManager,
    review_manager: ReviewManager,
}

impl Bot {

    pub async fn new() -> Self {
        let db_info = db::DBInfo::new().await;
        let bot = Self {
            db_info: db_info.clone(),
            order_manager: OrderManager::new(&db_info.clone()).await,
            review_manager: ReviewManager::new(),
        };

        bot
    }

    async fn load(&self, context_http: ContextHTTP){
        println!("Connected to Discord!");

        let guild_id = GuildId(env::var("GUILD_ID")
            .expect("Expected a GUILD_ID in the environment")
            .parse()
            .expect("GUILD_ID is not a valid ID"));

        self.order_manager.load(&context_http, guild_id).await;
        self.review_manager.load(&context_http).await;
    }

}

#[async_trait]
impl EventHandler for Bot {
    async fn ready(&self, ctx: Context, _ready: Ready) {
        self.load(ctx.http).await;
    }
}