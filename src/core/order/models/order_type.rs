use enum_iterator::Sequence;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Sequence)]
pub enum OrderType {
    PLUGIN,
    MOD,
    DISCORD,
    OTHER
}

impl OrderType {

    pub fn get_value(&self) -> String {
        match *self {
            Self::PLUGIN => "plugin".to_string(),
            Self::MOD => "mod".to_string(),
            Self::DISCORD => "discord".to_string(),
            Self::OTHER => "other".to_string()
        }
    }

    pub fn get_display_name(&self) -> String {
        match *self {
            Self::PLUGIN => "Plugin".to_string(),
            Self::MOD => "Mod".to_string(),
            Self::DISCORD => "Discord".to_string(),
            Self::OTHER => "Other".to_string()
        }
    }
}
