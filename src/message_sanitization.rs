use derive_builder::Builder;

/// Options that control exactly how messages are sanitized.
#[derive(Debug, PartialEq, Eq, Builder)]
pub struct MessageSanitizationOptions {
    /// Maximum message character length.
    ///
    /// DAPNET enforces an upper limit of 80, however shorter limits can be set if desired.
    #[builder(default = "80")]
    max_length: usize,

    /// The string to include at the end of a truncated message.
    #[builder(default = "\"...\".to_string()")]
    ellipses: String,

    /// What to do with non-ASCII characters in the message.
    #[builder(default = "MessageSanitizationNonAsciiPolicy::ReplaceWith('?')")]
    non_ascii_policy: MessageSanitizationNonAsciiPolicy,
}

impl Default for MessageSanitizationOptions {
    fn default() -> Self {
        Self {
            max_length: 80,
            ellipses: "...".to_string(),
            non_ascii_policy: MessageSanitizationNonAsciiPolicy::ReplaceWith('?'),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MessageSanitizationNonAsciiPolicy {
    /// Don't attempt to change any non-ASCII characters and let DAPNET handle them.
    DoNothing,

    /// Just remove all non-ASCII characters.
    Remove,

    /// Replace each non-ASCII character with a specified char.
    ReplaceWith(char),
}

/// Sanitize a message to ensure it is suitable for both POCSAG and DAPNET.
pub fn sanitize_message(msg: String, options: &MessageSanitizationOptions) -> String {
    // Remove non-ASCII characters
    let msg = match &options.non_ascii_policy {
        MessageSanitizationNonAsciiPolicy::DoNothing => msg,
        MessageSanitizationNonAsciiPolicy::Remove => msg.chars().filter(|c| c.is_ascii()).collect(),
        MessageSanitizationNonAsciiPolicy::ReplaceWith(replacement) => msg
            .chars()
            .map(|c| if c.is_ascii() { c } else { *replacement })
            .collect(),
    };

    // Enforce maximum length
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
    fn options_builder_default_matches_options_default() {
        let a = MessageSanitizationOptionsBuilder::default()
            .build()
            .unwrap();
        let b = MessageSanitizationOptions::default();
        assert_eq!(a, b);
    }

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
                ellipses: "~~~".to_string(),
                non_ascii_policy: MessageSanitizationNonAsciiPolicy::ReplaceWith('?'),
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
                non_ascii_policy: MessageSanitizationNonAsciiPolicy::DoNothing,
            },
        );
        assert_eq!(sanitized, "This message is t...");
        assert_eq!(sanitized.len(), 20);
    }

    #[test]
    fn message_with_non_ascii_do_nothing() {
        let msg = "→ This message has non-ascii chars ❤, oh dear.".to_string();
        let sanitized = sanitize_message(
            msg.clone(),
            &MessageSanitizationOptions {
                max_length: 80,
                ellipses: "...".to_string(),
                non_ascii_policy: MessageSanitizationNonAsciiPolicy::DoNothing,
            },
        );
        assert_eq!(sanitized, msg);
    }

    #[test]
    fn message_with_non_ascii_remove() {
        let msg = "→ This message has non-ascii chars ❤, oh dear.".to_string();
        let sanitized = sanitize_message(
            msg,
            &MessageSanitizationOptions {
                max_length: 80,
                ellipses: "...".to_string(),
                non_ascii_policy: MessageSanitizationNonAsciiPolicy::Remove,
            },
        );
        assert_eq!(sanitized, " This message has non-ascii chars , oh dear.");
    }

    #[test]
    fn message_with_non_ascii_replace_with() {
        let msg = "→ This message has non-ascii chars ❤, oh dear.".to_string();
        let sanitized = sanitize_message(
            msg,
            &MessageSanitizationOptions {
                max_length: 80,
                ellipses: "...".to_string(),
                non_ascii_policy: MessageSanitizationNonAsciiPolicy::ReplaceWith('_'),
            },
        );
        assert_eq!(sanitized, "_ This message has non-ascii chars _, oh dear.");
    }

    #[test]
    fn non_ascii_removal_and_message_truncation_interoperate_correctly() {
        let msg = "❤❤❤❤❤123456789".to_string();
        let sanitized = sanitize_message(
            msg,
            &MessageSanitizationOptions {
                max_length: 8,
                ellipses: "...".to_string(),
                non_ascii_policy: MessageSanitizationNonAsciiPolicy::Remove,
            },
        );
        assert_eq!(sanitized, "12345...");
    }
}
