use serenity::model::prelude::interaction::{Interaction, InteractionType, InteractionResponseType, message_component::MessageComponentInteraction};

use crate::{bot::Bot, ContextHTTP};

use super::state::order_state::{self, OrderState};

pub struct OrderListener;

impl OrderListener {
    pub async fn on_interaction(&self, bot: &Bot, context_http: &ContextHTTP, interaction: Interaction) {
        if interaction.kind() != InteractionType::MessageComponent {
            return;
        }
        let interaction = interaction.message_component().unwrap();
        let component_id = interaction.data.custom_id.as_str();

        if !component_id.starts_with("order:") {
            return;
        }

        let split = component_id.split(":").collect::<Vec<&str>>().get(1).unwrap().split("=").collect::<Vec<&str>>();
        let action = *split.get(0).unwrap();
        let order_id = (*split).get(1).unwrap().parse::<i32>().unwrap();
        let mut order = bot.order_manager.fetch_order(bot, order_id).await;

        let order_state = order.get_order_state().unwrap();


        match action {
            "validate" => {
                order_state.validate(bot, context_http, &mut order.clone()).await;
                self.reply(context_http, &interaction, order_state.validate_message().unwrap()).await;
            },
            "cancel" => {
                bot.order_manager.cancel_order(bot, context_http, &mut order).await;
                self.reply( context_http, &interaction, order_state::CANCELED_STATE.validate_message().unwrap()).await;
            },
            _ => {}
        }
    }

    async fn reply(&self, context_http: &ContextHTTP, interaction: &MessageComponentInteraction, content: String) {
        interaction
            .create_interaction_response(context_http, |response| {
                response
                    .kind(InteractionResponseType::ChannelMessageWithSource)
                    .interaction_response_data(|message| message.content(content).ephemeral(true))
            }).await.expect("Failed to send interaction response");
    }
}
