use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum OrderType {
    PLUGIN,
    MOD,
    DISCORD,
    OTHER
}

impl OrderType {
    pub fn get_display_name(&self) -> &str {
        match *self {
            Self::PLUGIN => "Plugin",
            Self::MOD => "Mod",
            Self::DISCORD => "Discord",
            Self::OTHER => "Other"
        }
    }
}