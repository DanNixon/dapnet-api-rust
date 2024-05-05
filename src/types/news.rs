use chrono::{DateTime, Utc};
use derive_builder::Builder;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Builder)]
pub struct OutgoingNews {
    /// Name of the rubric to send to
    #[serde(rename = "rubricName")]
    pub(crate) rubric: String,

    /// Message text of the news item
    pub(crate) text: String,

    /// News position (1-10) for Skyper pagers
    #[builder(default = "1")]
    pub(crate) number: i8,
}

#[derive(Debug, Deserialize)]
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
