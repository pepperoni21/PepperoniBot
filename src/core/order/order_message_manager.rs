use std::env;

use serenity::{builder::{CreateEmbed, CreateComponents}, model::{prelude::{GuildChannel, component::ButtonStyle}, user::User}, prelude::Mentionable};

use crate::ContextHTTP;

use super::models::order::Order;

pub struct OrderMessageManager;

impl OrderMessageManager {

    pub fn order_channel_message(&self, order: &Order) -> CreateEmbed {
        let mut create_embed = CreateEmbed::default();
        create_embed.title(format!("Order #{}", order.order_id));
        create_embed.fields(vec![
            ("Type", order.order_type.get_display_name(), true),
            ("Price", format!("{} USD", order.price.to_string()), true),
        ]);

        let order_state = order.get_order_state().unwrap();

        create_embed.description(order_state.instruction().unwrap().replace("%price%", &(order.price / 2).to_string()));
        create_embed
    }

    pub async fn update_channel_message(&self, context_http: &ContextHTTP, order: &Order, channel: &GuildChannel) {
        let order_channel_message_id = order.assets.order_channel_message_id.unwrap();
        let mut message = channel.message(context_http, order_channel_message_id).await.expect("Failed to get message");
        let create_embed = self.order_channel_message(order);
        message.edit(context_http, |message| {
            message.embed(|embed| {
                embed.0 = create_embed.0;
                embed
            });
            message
        }).await.expect("Failed to edit message");
    }

    pub async fn update_order_list_message(&self, context_http: &ContextHTTP, order: &Order) {
        let orders_channel_id: u64 = env::var("ORDERS_CHANNEL_ID").expect("ORDERS_CHANNEL_ID must be set").parse().expect("ORDERS_CHANNEL_ID must be a number");
        let orders_channel = context_http.get_channel(orders_channel_id).await.expect("Failed to get channel").guild().unwrap();
        let order_list_message_id = order.assets.order_list_message_id.unwrap();
        let mut message = orders_channel.message(context_http, order_list_message_id).await.expect("Failed to get message");
        let user = context_http.get_user(order.customer_id).await.expect("Failed to get user");

        let create_embed = self.order_list_message(order, &user, &orders_channel).await;
        let components = self.generate_action_rows(order).await;

        message.edit(context_http, |message| {
            message.embed(|embed| {
                embed.0 = create_embed.0;
                embed
            }).set_components(components)
        }).await.expect("Failed to edit message");
    }

    pub async fn order_list_message(&self, order: &Order, user: &User, channel: &GuildChannel) -> CreateEmbed {
        let mut create_embed = CreateEmbed::default();

        let order_state = order.get_order_state().unwrap();

        create_embed
        .title(format!("Order #{}", order.order_id))
        .fields(vec![
            ("Customer", user.tag(), true),
            ("Type", order.order_type.get_display_name(), false),
            ("Price", format!("{}$", order.price.to_string()), false),
            ("Channel", channel.mention().to_string(), false),
            ("State", order_state.short_name(), false),
        ]);

        create_embed
    }

    pub async fn generate_action_rows(&self, order: &Order) -> CreateComponents {
        let order_state = order.get_order_state().unwrap();
        let mut components = CreateComponents::default();

        components.create_action_row(|row|
            row.create_button(|button|
                button
                .custom_id(format!("validate={}", order.order_id))
                .label(order_state.validate_action_label().unwrap())
                .style(ButtonStyle::Primary)
            ).create_button(|button|
                button
                .custom_id(format!("order:cancel={}", order.order_id))
                .label("Cancel")
                .style(ButtonStyle::Danger)
            )
        );

        components
    }

    pub async fn add_to_archive(&self, context_http: &ContextHTTP, order: &Order) {
        let orders_archive_channel_id: u64 = env::var("ORDERS_ARCHIVE_CHANNEL_ID").expect("ORDERS_ARCHIVE_CHANNEL_ID must be set").parse().expect("ORDERS_ARCHIVE_CHANNEL_ID must be a number");
        let orders_archive_channel = context_http.get_channel(orders_archive_channel_id).await.expect("Failed to get channel").guild().unwrap();

        let user = context_http.get_user(order.customer_id).await.expect("Failed to get user");

        let order_state = order.get_order_state().unwrap();

        orders_archive_channel.send_message(context_http, |message|
            message.embed(|embed|
                embed
                .title(format!("Order #{}", order.order_id))
                .fields(vec![
                    ("Customer", user.tag(), true),
                    ("Type", order.order_type.get_display_name(), false),
                    ("Price", format!("{}$", order.price.to_string()), false),
                    ("State", order_state.short_name(), false),
                ])
            )
        ).await.expect("Failed to send message");
    }

}
