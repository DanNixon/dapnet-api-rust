use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Callsign {
    pub name: String,
    pub description: String,

    pub numeric: bool,

    #[serde(rename = "ownerNames")]
    pub owners: Vec<String>,
}
