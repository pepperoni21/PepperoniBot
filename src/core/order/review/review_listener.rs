use serenity::model::prelude::interaction::Interaction;

use crate::{ContextHTTP, bot::Bot};

pub struct ReviewListener;

impl ReviewListener {

    pub async fn interaction_create(&self, _bot: &Bot, _context_http: ContextHTTP, interaction: Interaction) {
        let message_component_interaction = interaction.message_component();
        if message_component_interaction.is_none() {
            return;
        }
        let message_component_interaction = message_component_interaction.unwrap();
        let component_id = message_component_interaction.data.custom_id.as_str();
        if component_id != "review" {
            return;
        }

        let _user = message_component_interaction.user;
        todo!("List orders and finish the function")
    }

}