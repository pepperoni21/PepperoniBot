use serenity::model::prelude::UserId;

pub fn mention_user(user_id: &UserId) -> String {
    format!("<@{}>", user_id.0)
}

pub fn _mention_role(role_id: u64) -> String {
    format!("<@&{}>", role_id)
}

pub fn _mention_channel(channel_id: u64) -> String {
    format!("<#{}>", channel_id)
}

pub fn _mention_emoji(emoji_id: u64) -> String {
    format!("<:{}>", emoji_id)
}

pub fn _mention_everyone() -> String {
    "@everyone".to_string()
}

pub fn _mention_here() -> String {
    "@here".to_string()
}