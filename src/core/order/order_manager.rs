use std::{env, sync::Arc};

use serenity::{model::{prelude::{GuildId, PermissionOverwrite, PermissionOverwriteType, GuildChannel, RoleId}, user::User, Permissions}, futures::StreamExt};
use wither::{Model, bson::{doc, to_bson}};

use crate::{ContextHTTP, bot::Bot};

use super::{command::order_command, models::{order::Order, order_type::OrderType}, state::order_state::{self, OrderState}, order_message_manager, review::review_manager};

pub async fn load(bot: &Bot, context_http: &ContextHTTP) {
    order_command::load_command(context_http, &bot.guild_id).await;
    review_manager::load(&context_http).await;
}

pub async fn create_order(bot: &Bot, context_http: &ContextHTTP, developer: User, customer: Arc<User>, order_type: OrderType, price: i32, description: String){
    let mut order = Order::new(order_type, price, developer.id.0, customer.id.0, description);
    let orders_category_id: u64 = env::var("ORDERS_CATEGORY_ID").expect("ORDERS_CATEGORY_ID must be set").parse().expect("ORDERS_CATEGORY_ID must be a number");

    let guild_id: u64 = env::var("GUILD_ID").expect("GUILD_ID must be set").parse().expect("GUILD_ID must be a number");
    let guild_id = GuildId(guild_id);
    let guild = guild_id.to_partial_guild(&context_http).await.expect("Failed to get guild");

    tokio::spawn({
        let context_http = context_http.clone();
        let user = customer.clone();
        async move {
            add_customer_role(&context_http, &user).await;
        }
    });
    
    let channel = guild.create_channel(context_http, |channel| {
        channel.name(format!("order-{}", order.order_id));
        channel.category(orders_category_id);
    let permissions = vec![
        PermissionOverwrite {
            allow: Permissions::empty(),
            deny: Permissions::VIEW_CHANNEL,
            kind: PermissionOverwriteType::Role(RoleId(1047410767902814238)),
        },
        PermissionOverwrite {
            allow: Permissions::VIEW_CHANNEL,
            deny: Permissions::empty(),
            kind: PermissionOverwriteType::Member(customer.id),
        },
        PermissionOverwrite {
            allow: Permissions::SEND_MESSAGES,
            deny: Permissions::empty(),
            kind: PermissionOverwriteType::Member(customer.id),
        },
        PermissionOverwrite {
            allow: Permissions::VIEW_CHANNEL,
            deny: Permissions::empty(),
            kind: PermissionOverwriteType::Member(developer.id),
        },
        PermissionOverwrite {
            allow: Permissions::SEND_MESSAGES,
            deny: Permissions::empty(),
            kind: PermissionOverwriteType::Member(developer.id),
        }];

        channel.permissions(permissions);

        channel
    }).await.expect("Failed to create channel");

    let message = channel.send_message(&context_http, |message|
        message.set_embed(order_message_manager::order_channel_message(&order))
    ).await.expect("Failed to send message");
    order.assets.order_channel_message_id = Some(*message.id.as_u64());
    message.pin(context_http).await.expect("Failed to pin message");

    let orders_channel = get_order_list_channel(&context_http).await;
    let order_list_message = order_message_manager::order_list_message(&order, &developer, &customer, &channel).await;
    let components = order_message_manager::generate_action_rows(&order).await;
    let message = orders_channel.send_message(context_http, |message|
        message
        .set_embed(order_list_message)
        .set_components(components)
    ).await.expect("Failed to send message");
    order.assets.order_channel_id = Some(*channel.id.as_u64());
    order.assets.order_list_message_id = Some(*message.id.as_u64());
    
    order.save(&bot.db_info.db, None).await.expect("Failed to save order");
}

async fn add_customer_role(context_http: &ContextHTTP, user: &User) {
    let customer_role_id: u64 = env::var("CUSTOMER_ROLE_ID").expect("CUSTOMER_ROLE_ID must be set").parse().expect("CUSTOMER_ROLE_ID must be a number");
    let guild_id: u64 = env::var("GUILD_ID").expect("GUILD_ID must be set").parse().expect("GUILD_ID must be a number");
    let guild_id = GuildId(guild_id);
    let guild = guild_id.to_partial_guild(&context_http).await.expect("Failed to get guild");
    let mut member = guild.member(context_http, user.id).await.expect("Failed to get member");
    
    if let Err(_) = member.add_role(context_http, RoleId(customer_role_id)).await {
        println!("Failed to add role");
    }
}

pub async fn cancel_order(bot: &Bot, context_http: &ContextHTTP, order: &mut Order) {
    order.order_state_id = order_state::CANCELED_STATE.id();
    
    tokio::spawn({
        let context_http = context_http.clone();
        let order = order.clone();
        async move {
            end_order(&context_http, &order).await;
        }
    });
    
    tokio::spawn({
        let context_http = context_http.clone();
        let order = order.clone();
        async move {
            order_message_manager::add_to_archive(&context_http, &order).await;
        }
    });

    order.save(&bot.db_info.db, None).await.expect("Failed to save order");
}

pub async fn end_order(context_http: &ContextHTTP, order: &Order) {
    let order_channel_id = order.assets.order_channel_id;
    let order_list_message_id = order.assets.order_list_message_id;

    if order_channel_id.is_some() {
        let order_channel = context_http.get_channel(order_channel_id.unwrap()).await.expect("Failed to get order channel").guild().expect("Order channel is not a guild channel");
        order_channel.delete(context_http).await.expect("Failed to delete order channel");
    } if order_list_message_id.is_some() {
        let orders_list_channel = get_order_list_channel(context_http).await;
        let order_list_message = orders_list_channel.message(context_http, order_list_message_id.unwrap()).await.expect("Failed to get order list message");
        order_list_message.delete(context_http).await.expect("Failed to delete order list message");
    }
}

pub async fn fetch_order(bot: &Bot, order_id: i32) -> Order {
    Order::find_one(&bot.db_info.db, doc!{
        "order_id": order_id
    }, None).await.expect("Failed to fetch order").expect("Order not found")
}

pub async fn fetch_orders_by_developer(bot: &Bot, developer_id: u64) -> Vec<Order> {
    Order::find(&bot.db_info.db, doc!{
        "developer_id": to_bson(&developer_id).unwrap()
    }, None)
    .await
    .expect("Failed to fetch orders")
    .map(|order| order.expect("Failed to fetch order"))
    .collect::<Vec<Order>>().await
}

async fn get_order_list_channel(context_http: &ContextHTTP) -> GuildChannel {
    let order_list_channel_id: u64 = env::var("ORDERS_CHANNEL_ID").expect("").parse().expect("Failed to parse order list channel id");
    context_http.get_channel(order_list_channel_id).await.expect("Failed to get order list channel").guild().expect("Order list channel is not a guild channel")
}