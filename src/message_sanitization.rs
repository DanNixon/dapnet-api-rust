use derive_builder::Builder;

/// Options that control exactly how messages are sanitized.
#[derive(Debug, PartialEq, Eq, Builder)]
pub struct MessageSanitizationOptions {
    /// Maximum message character length.
    ///
    /// DAPNET enforces an upper limit of 80, however shorter limits can be set if desired.
    max_length: usize,

    /// The string to include at the end of a truncated message.
    ellipses: String,
}

impl Default for MessageSanitizationOptions {
    fn default() -> Self {
        Self {
            max_length: 80,
            ellipses: "...".to_string(),
        }
    }
}

/// Sanitize a message to ensure it is suitable for both POCSAG and DAPNET.
pub fn sanitize_message(msg: String, options: &MessageSanitizationOptions) -> String {
    if msg.len() > options.max_length {
        let trim_point = options.max_length.saturating_sub(options.ellipses.len());
        format!("{}{}", &msg[0..trim_point], options.ellipses)
    } else {
        msg
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn build_options() {
        let options = MessageSanitizationOptionsBuilder::default()
            .ellipses("~~~".to_string())
            .max_length(50)
            .build()
            .unwrap();

        assert_eq!(
            options,
            MessageSanitizationOptions {
                max_length: 50,
                ellipses: "~~~".to_string()
            }
        );
    }

    #[test]
    fn perfectly_valid_message_with_default_settings() {
        let msg = "This message is less than 80 chars, woo.".to_string();
        let sanitized = sanitize_message(msg.clone(), &MessageSanitizationOptions::default());
        assert_eq!(sanitized, msg);
    }

    #[test]
    fn message_that_is_too_long() {
        let msg = "This message is too long, oh dear.".to_string();
        let sanitized = sanitize_message(
            msg,
            &MessageSanitizationOptions {
                max_length: 20,
                ellipses: "...".to_string(),
            },
        );
        assert_eq!(sanitized, "This message is t...");
        assert_eq!(sanitized.len(), 20);
    }
}
