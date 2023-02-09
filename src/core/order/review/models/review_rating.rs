use enum_iterator::Sequence;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Sequence)]
pub enum ReviewRating {
    FiveStars,
    FourStars,
    ThreeStars,
    TwoStars,
    OneStar,
}

impl ReviewRating {
    pub fn get_name(&self) -> &str {
        match *self {
            Self::OneStar => "1",
            Self::TwoStars => "2",
            Self::ThreeStars => "3",
            Self::FourStars => "4",
            Self::FiveStars => "5"
        }
    }

    pub fn get_emoji(&self) -> &str {
        match *self {
            Self::OneStar => "⭐",
            Self::TwoStars => "⭐⭐",
            Self::ThreeStars => "⭐⭐⭐",
            Self::FourStars => "⭐⭐⭐⭐",
            Self::FiveStars => "⭐⭐⭐⭐⭐"
        }
    }

    pub fn from_name(id: &str) -> Option<Self> {
        match id {
            "1" => Some(Self::OneStar),
            "2" => Some(Self::TwoStars),
            "3" => Some(Self::ThreeStars),
            "4" => Some(Self::FourStars),
            "5" => Some(Self::FiveStars),
            _ => None
        }
    }
}
