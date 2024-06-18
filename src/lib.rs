//! This library provides access to the DAPNET v1 API.
//!
//! Details of the API are available [here](https://github.com/DecentralizedAmateurPagingNetwork/Core/wiki/Beschreibung%20der%20REST%20API)
//! and [here](https://hampager.de/dokuwiki/doku.php?id=dapnetapisendcall).
//!
//! Currently the library focuses on reading data from the API.
//! The only non-idempotent operation it supports is sending a new call/page/message and new rubric
//! news item.

mod client;
mod error;
mod message_sanitization;
mod types;

pub use crate::{
    client::Client,
    error::{Error, Result},
    message_sanitization::{
        sanitize_message, MessageSanitizationOptions, MessageSanitizationOptionsBuilder,
        MessageSanitizationOptionsBuilderError,
    },
    types::{
        Call, Callsign, Connection, News, Node, OutgoingCall, OutgoingCallBuilder,
        OutgoingCallBuilderError, OutgoingNews, OutgoingNewsBuilder, OutgoingNewsBuilderError,
        Rubric, Statistics, Transmitter, TransmitterGroup,
    },
};
