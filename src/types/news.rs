use chrono::{DateTime, Utc};
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct News {
    /// Name of the rubric to send to
    #[serde(rename = "rubricName")]
    pub rubric: String,

    /// Message text of the news item
    pub text: String,

    /// News position (1-10) for Skyper pagers
    pub number: Option<i8>,

    /// Time at which the news was sent
    pub timestamp: Option<DateTime<Utc>>,

    /// User who submitted the news
    #[serde(rename = "ownerName")]
    pub sender: Option<String>,
}

impl News {
    /// Crates a new news item to be sent
    pub fn new(rubric: String, text: String) -> News {
        News {
            rubric,
            text,
            number: Some(1),
            timestamp: None,
            sender: None,
        }
    }
}
