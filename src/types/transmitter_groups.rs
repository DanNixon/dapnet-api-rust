use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct TransmitterGroup {
    pub name: String,
    pub description: String,

    #[serde(rename = "transmitterNames")]
    pub transmitters: Vec<String>,

    #[serde(rename = "ownerNames")]
    pub owners: Vec<String>,
}
