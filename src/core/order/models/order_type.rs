use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum OrderType {
    PLUGIN,
    MOD,
    DISCORD,
    OTHER
}

impl OrderType {
    pub fn get_display_name(&self) -> String {
        match *self {
            Self::PLUGIN => "Plugin".to_string(),
            Self::MOD => "Mod".to_string(),
            Self::DISCORD => "Discord".to_string(),
            Self::OTHER => "Other".to_string()
        }
    }
}