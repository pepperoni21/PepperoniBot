use serde::{Serialize, Deserialize};

use super::review_rating::ReviewRating;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Review {
    pub rating: ReviewRating,
    pub comment: String,
    pub message_id: u64
}