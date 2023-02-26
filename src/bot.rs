use std::{env, sync::Arc};

use serenity::{async_trait, prelude::{EventHandler, Context}, model::prelude::{Ready, GuildId, interaction::Interaction}};

use crate::{core::{order::{order_manager::OrderManager, command::order_command_executor}, db}, ContextHTTP};

pub struct Bot {
    pub db_info: db::DBInfo,
    pub order_manager: Arc<OrderManager>,
}

impl Bot {

    pub async fn new() -> Self {
        let db_info = db::DBInfo::new().await;
        let bot = Self {
            db_info,
            order_manager: Arc::new(OrderManager::new().await),
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
    }

}

#[async_trait]
impl EventHandler for Bot {
    async fn ready(&self, ctx: Context, _ready: Ready) {
        self.load(ctx.http).await;
    }

    async fn interaction_create(&self, ctx: Context, interaction: Interaction){
        let context_http: ContextHTTP = ctx.http;

        self.order_manager.review_manager.listener.interaction_create(self, &context_http, interaction.clone()).await;
        self.order_manager.listener.on_interaction(self, &context_http, interaction.clone()).await;
        order_command_executor::on_interaction(&self, &context_http, interaction.clone()).await;
    }
}
