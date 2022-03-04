use super::Connection;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub enum Status {
    #[serde(rename = "ONLINE")]
    Online,
    #[serde(rename = "SUSPENDED")]
    Suspended,
    #[serde(rename = "ERROR")]
    Error,
}

#[derive(Debug, Deserialize)]
pub struct Node {
    pub name: String,
    pub version: String,

    pub status: Status,

    pub longitude: String,
    pub latitude: String,

    #[serde(rename = "ownerNames")]
    pub owners: Vec<String>,

    #[serde(rename = "address")]
    pub connection: Option<Connection>,
}
