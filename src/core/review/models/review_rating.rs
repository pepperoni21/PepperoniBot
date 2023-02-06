use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum ReviewRating {
    OneStar,
    TwoStars,
    ThreeStars,
    FourStars,
    FiveStars
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
}