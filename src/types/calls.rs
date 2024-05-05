use chrono::{DateTime, Utc};
use derive_builder::Builder;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Builder)]
pub struct OutgoingCall {
    /// Message text of the call
    pub(crate) text: String,

    /// Call signs of this calls recipients
    #[serde(rename = "callSignNames")]
    pub(crate) recipients: Vec<String>,

    /// Names of transmitter groups used to transmit this call
    #[serde(rename = "transmitterGroupNames")]
    pub(crate) transmitter_groups: Vec<String>,

    /// Flag indicating if this call was sent with high priority
    #[builder(default = "false")]
    pub(crate) emergency: bool,
}

#[derive(Debug, Deserialize)]
pub struct Call {
    /// Message text of the call
    pub text: String,

    /// Time at which the call was sent
    pub timestamp: DateTime<Utc>,

    /// User who submitted the call
    #[serde(rename = "ownerName")]
    pub sender: String,

    /// Call signs of this calls recipients
    #[serde(rename = "callSignNames")]
    pub recipients: Vec<String>,

    /// Names of transmitter groups used to transmit this call
    #[serde(rename = "transmitterGroupNames")]
    pub transmitter_groups: Vec<String>,

    /// Flag indicating if this call was sent with high priority
    pub emergency: bool,
}
