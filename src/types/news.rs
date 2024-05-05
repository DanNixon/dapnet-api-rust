use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct OutgoingNews {
    /// Name of the rubric to send to
    #[serde(rename = "rubricName")]
    pub rubric: String,

    /// Message text of the news item
    pub text: String,

    /// News position (1-10) for Skyper pagers
    pub number: Option<i8>,
}

impl OutgoingNews {
    /// Crates a new news item to be sent
    pub fn new(rubric: String, text: String) -> Self {
        Self {
            rubric,
            text,
            number: Some(1),
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct News {
    /// Name of the rubric to send to
    #[serde(rename = "rubricName")]
    pub rubric: String,

    /// Message text of the news item
    pub text: String,

    /// News position (1-10) for Skyper pagers
    pub number: Option<i8>,

    /// Time at which the news was sent
    pub timestamp: DateTime<Utc>,

    /// User who submitted the news
    #[serde(rename = "ownerName")]
    pub sender: String,
}
