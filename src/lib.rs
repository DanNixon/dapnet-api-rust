//! This library provides access to the DAPNET v1 API.
//!
//! Details of the API are available [here](https://github.com/DecentralizedAmateurPagingNetwork/Core/wiki/Beschreibung%20der%20REST%20API)
//! and [here](https://hampager.de/dokuwiki/doku.php?id=dapnetapisendcall).
//!
//! Currently the library focuses on reading data from the API.
//! The only non-idempotent operation it supports is sending a new call/page/message.
//!
//! ## Example
//! ```no_run
//! use dapnet_api::{Client, OutgoingCall};
//!
//! #[tokio::main]
//! async fn main() {
//!     let client = Client::new("m0nxn", "my_super_secret_password");
//!
//!     client
//!         .new_call(&OutgoingCall::new(
//!             "M0NXN: this is a test".to_string(),
//!             vec!["m0nxn".to_string()],
//!             vec!["uk-all".to_string()],
//!         ))
//!         .await
//!         .unwrap();
//!
//!     let calls = client.get_calls_by("m0nxn").await.unwrap();
//!     println!("calls: {:?}", calls);
//! }
//! ```

mod client;
mod error;
mod types;

pub use crate::{
    client::Client,
    error::{Error, Result},
    types::{
        Call, Callsign, Connection, News, Node, OutgoingCall, OutgoingNews, Rubric, Statistics,
        Transmitter, TransmitterGroup,
    },
};
