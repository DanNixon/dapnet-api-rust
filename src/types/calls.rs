use chrono::{DateTime, Utc};
use derive_builder::Builder;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Builder)]
#[builder(build_fn(validate = "Self::validate"))]
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

impl OutgoingCallBuilder {
    fn validate(&self) -> Result<(), String> {
        match &self.text {
            Some(text) => {
                if text.len() > 80 {
                    Err("Text must be 80 characters or less".to_string())
                } else {
                    Ok(())
                }
            }
            None => Err("Text must be set".to_string()),
        }
    }
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

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn build_80_char() {
        let text =
            "01234567890123456789012345678901234567890123456789012345678901234567890123456789";
        assert_eq!(text.len(), 80);

        let call = OutgoingCallBuilder::default()
            .recipients(vec!["m0nxn".to_string()])
            .transmitter_groups(vec!["all".to_string()])
            .text(text.to_string())
            .build()
            .unwrap();

        assert_eq!(call.text, text);
    }

    #[test]
    #[should_panic]
    fn build_81_char() {
        let text =
            "012345678901234567890123456789012345678901234567890123456789012345678901234567890";
        assert_eq!(text.len(), 81);

        let _ = OutgoingCallBuilder::default()
            .recipients(vec!["m0nxn".to_string()])
            .transmitter_groups(vec!["all".to_string()])
            .text(text.to_string())
            .build()
            .unwrap();
    }

    #[test]
    #[should_panic]
    fn build_no_text() {
        let _ = OutgoingCallBuilder::default()
            .recipients(vec!["m0nxn".to_string()])
            .transmitter_groups(vec!["all".to_string()])
            .build()
            .unwrap();
    }
}
