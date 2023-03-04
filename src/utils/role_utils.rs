use serenity::model::prelude::RoleId;

pub fn fetch_guild_role(var: &str) -> RoleId {
    RoleId(std::env::var(var).expect("Failed to fetch guild role").parse().expect("Failed to parse guild role"))
}