use serenity::model::user::User;

use crate::{bot::Bot, ContextHTTP, core::developers::developer_manager};

pub async fn on_member_leave(bot: &Bot, context_http: &ContextHTTP, user: User) {
    
    let developer = developer_manager::fetch_developer(bot, user.id).await;

    if developer.is_none() {
        return;
    }

    let mut developer = developer.unwrap();

    developer_manager::remove_developer(bot, context_http, &mut developer).await;

}