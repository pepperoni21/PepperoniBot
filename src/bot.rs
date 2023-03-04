use std::env;

use serenity::{async_trait, prelude::{EventHandler, Context}, model::prelude::{Ready, GuildId, interaction::Interaction, User, Member}};

use crate::{core::{order::{order_manager, command::order_command_executor, review::review_listener, order_listener}, db, developers::{developer_manager, command::developer_command_executor, listeners::{developer_interaction_listener, developer_leave_listener}}}, ContextHTTP, utils::interaction_utils};

#[derive(Clone)]
pub struct Bot {
    pub db_info: db::DBInfo,
    pub guild_id: GuildId
}

impl Bot {

    pub async fn new() -> Self {
        let db_info = db::DBInfo::new().await;
        let guild_id = GuildId(env::var("GUILD_ID")
            .expect("Expected a GUILD_ID in the environment")
            .parse()
            .expect("GUILD_ID is not a valid ID"));

        let bot = Self {
            db_info,
            guild_id
        };

        bot
    }

    async fn load(&self, context_http: ContextHTTP){
        println!("Connected to Discord!");

        tokio::spawn({
            let bot = self.clone();
            let context_http = context_http.clone();
            async move {
                order_manager::load(&bot, &context_http).await;
            }
        });

        tokio::spawn({
            let bot = self.clone();
            let context_http = context_http.clone();
            async move {
                developer_manager::load(&bot, &context_http).await;
            }
        });
    }

}

#[async_trait]
impl EventHandler for Bot {
    async fn ready(&self, ctx: Context, _ready: Ready) {
        self.load(ctx.http).await;
    }

    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        let guild_id = interaction_utils::get_interaction_guild(&interaction);
        if guild_id.is_none() || guild_id.unwrap() != self.guild_id {
            return;
        }
        
        let context_http: ContextHTTP = ctx.http;

        review_listener::on_interaction(self, &context_http, interaction.clone()).await;
        
        order_listener::on_interaction(self, &context_http, interaction.clone()).await;
        order_command_executor::on_interaction(&self, &context_http, interaction.clone()).await;

        developer_command_executor::on_interaction(&self, &context_http, interaction.clone()).await;
        developer_interaction_listener::on_interaction(&self, &context_http, interaction.clone()).await;
    }

    async fn guild_member_removal(&self, ctx: Context, guild_id: GuildId, user: User, _member_data: Option<Member>) {
        if guild_id != self.guild_id {
            return;
        }

        let context_http: ContextHTTP = ctx.http;

        developer_leave_listener::on_member_leave(self, &context_http, user).await;
    }
}

#[async_trait]
trait InteractionListener : Send + Sync {
    async fn on_interaction(&self, bot: &Bot, context_http: &ContextHTTP, interaction: Interaction);
}