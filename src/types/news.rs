use chrono::{DateTime, Utc};
use derive_builder::Builder;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Builder)]
#[builder(build_fn(validate = "Self::validate"))]
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

impl OutgoingNewsBuilder {
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

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn build_80_char() {
        let text =
            "01234567890123456789012345678901234567890123456789012345678901234567890123456789";
        assert_eq!(text.len(), 80);

        let news = OutgoingNewsBuilder::default()
            .rubric("test".to_string())
            .text(text.to_string())
            .build()
            .unwrap();

        assert_eq!(news.text, text);
    }

    #[test]
    #[should_panic]
    fn build_81_char() {
        let text =
            "012345678901234567890123456789012345678901234567890123456789012345678901234567890";
        assert_eq!(text.len(), 81);

        let _ = OutgoingNewsBuilder::default()
            .rubric("test".to_string())
            .text(text.to_string())
            .build()
            .unwrap();
    }

    #[test]
    #[should_panic]
    fn build_no_text() {
        let _ = OutgoingNewsBuilder::default()
            .rubric("test".to_string())
            .build()
            .unwrap();
    }
}
