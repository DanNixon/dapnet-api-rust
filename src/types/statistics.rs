use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Statistics {
    pub users: i64,

    #[serde(rename = "callSigns")]
    pub callsigns: i64,

    pub calls: i64,
    #[serde(rename = "callsTotal")]
    pub calls_total: i64,

    #[serde(rename = "nodesOnline")]
    pub nodes_online: i64,
    #[serde(rename = "nodesTotal")]
    pub nodes_total: i64,

    #[serde(rename = "transmittersOnline")]
    pub transmitters_online: i64,
    #[serde(rename = "transmittersTotal")]
    pub transmitters_total: i64,

    pub rubrics: i64,

    pub news: i64,
    #[serde(rename = "newsTotal")]
    pub news_total: i64,
}
