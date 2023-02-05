use serenity::{async_trait, prelude::{EventHandler, Context}, model::prelude::Ready};

use crate::core;

pub struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, _ctx: Context, _ready: Ready) {
        core::load().await;
    }
}
