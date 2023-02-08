use enum_iterator::all;
use serenity::{model::{prelude::{GuildId, command::CommandOptionType}, Permissions}, builder::CreateApplicationCommandOption};

use crate::{ContextHTTP, core::order::models::order_type::OrderType};

pub async fn load_command(context_http: &ContextHTTP, guild_id: &GuildId){
    GuildId::set_application_commands(guild_id, context_http, |commands|{
        commands.create_application_command(|command| {
            command
            .name("order")
            .description("Manager orders")
            .default_member_permissions(Permissions::MODERATE_MEMBERS)
            .create_option(|option| {
                fill_create_command(option);
                option
            })
            .create_option(|option| {
                fill_cancel_command(option);
                option
            })
        })
    }).await.expect("Failed to load commands");
}


fn fill_create_command(option: &mut CreateApplicationCommandOption){
    option
                .name("create")
                .description("Create an order")
                .kind(CommandOptionType::SubCommand)
                .required(true)
                .create_sub_option(|user_option|
                    user_option
                    .name("user")
                    .description("User who ordered")
                    .kind(CommandOptionType::User)
                    .required(true)
                    .create_sub_option(|type_option| {
                        type_option
                        .name("type")
                        .description("Type of order")
                        .kind(CommandOptionType::String)
                        .required(true);

                        all::<OrderType>().into_iter().for_each(|order_type| {
                            type_option.add_string_choice(order_type.get_value(), order_type.get_display_name());
                        });

                        type_option.create_sub_option(|price_option|
                            price_option
                            .name("price")
                            .description("Price of order")
                            .kind(CommandOptionType::Integer)
                            .required(true)
                            .create_sub_option(|description_option|
                                description_option
                                .name("description")
                                .description("Description of order")
                                .kind(CommandOptionType::String)
                                .required(true)
                            )
                        )
                    })
                );
}

fn fill_cancel_command(option: &mut CreateApplicationCommandOption){
    option
    .name("cancel")
    .description("Cancel an order")
    .create_sub_option(|id_option|
        id_option
        .name("id")
        .description("Id of order")
        .kind(CommandOptionType::Integer)
        .required(true)
    );
}
