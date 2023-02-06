use serde::{Serialize, Deserialize};

use super::review_rating::ReviewRating;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Review {
    rating: ReviewRating,
    comment: String,
    message_id: u64
}