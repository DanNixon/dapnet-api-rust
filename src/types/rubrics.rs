use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Rubric {
    pub name: String,
    pub label: String,
    pub number: i64,

    #[serde(rename = "transmitterGroupNames")]
    pub transmitter_groups: Vec<String>,

    #[serde(rename = "ownerNames")]
    pub owners: Vec<String>,
}
