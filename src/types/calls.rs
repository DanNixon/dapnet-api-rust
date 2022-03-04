use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Call {
    /// Message text of the call
    pub text: String,

    /// Time at which the call was sent
    pub timestamp: Option<DateTime<Utc>>,

    /// User who submitted the call
    #[serde(rename = "ownerName")]
    pub sender: Option<String>,

    /// Call signs of this calls recipients
    #[serde(rename = "callSignNames")]
    pub recipients: Vec<String>,

    /// Names of transmitter groups used to transmit this call
    #[serde(rename = "transmitterGroupNames")]
    pub transmitter_groups: Vec<String>,

    /// Flag indicating if this call was sent with high priority
    pub emergency: bool,
}

impl Call {
    pub fn new(text: String, recipients: Vec<String>, transmitter_groups: Vec<String>) -> Self {
        Self {
            text,
            timestamp: None,
            sender: None,
            recipients,
            transmitter_groups,
            emergency: false,
        }
    }
}
