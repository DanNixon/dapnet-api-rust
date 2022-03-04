use super::Connection;
use chrono::{DateTime, Utc};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub enum Usage {
    #[serde(rename = "PERSONAL")]
    Personal,
    #[serde(rename = "WIDERANGE")]
    Widerange,
}

#[derive(Debug, Deserialize)]
pub enum AntennaType {
    #[serde(rename = "OMNI")]
    Omnidirectional,
    #[serde(rename = "DIRECTIONAL")]
    Directional,
}

#[derive(Debug, Deserialize)]
pub enum Status {
    #[serde(rename = "OFFLINE")]
    Offline,
    #[serde(rename = "ONLINE")]
    Online,
    #[serde(rename = "ERROR")]
    Error,
}

#[derive(Debug, Deserialize)]
pub struct Transmitter {
    pub name: String,
    pub usage: Usage,

    pub longitude: String,
    pub latitude: String,

    /// Textual representation of the timeslots the transmitter will be active on
    #[serde(rename = "timeSlot")]
    pub timeslots: String,

    #[serde(rename = "ownerNames")]
    pub owners: Vec<String>,

    pub status: Status,

    #[serde(rename = "callCount")]
    pub call_count: u64,

    #[serde(rename = "address")]
    pub connection: Option<Connection>,

    /// Name of the DAPNET node the transmitter connects to
    #[serde(rename = "nodeName")]
    pub node: Option<String>,

    /// Key to be used for authentication by transmitter/modem
    /// Only present when the API user is the owner of the transmitter
    #[serde(rename = "authKey")]
    pub auth_key: Option<String>,

    #[serde(rename = "deviceType")]
    pub device_type: Option<String>,
    #[serde(rename = "deviceVersion")]
    pub device_version: Option<String>,

    /// Transmitter power in watts
    pub power: String,

    /// Antenna elevation above ground in metres
    #[serde(rename = "antennaAboveGroundLevel")]
    pub antenna_height_above_ground: i64,

    #[serde(rename = "antennaType")]
    pub antenna_type: AntennaType,

    /// Antenna direction in degrees (only relevant for directional antennas)
    #[serde(rename = "antennaDirection")]
    pub antenna_direction: f64,

    /// Antenna gain in dBi
    #[serde(rename = "antennaGainDbi")]
    pub antenna_gain: f64,

    #[serde(rename = "identificationAddress")]
    pub identification_address: i64,

    /// Time when the transmitter details were last modified
    #[serde(rename = "lastUpdate")]
    pub last_update: DateTime<Utc>,

    /// Time when the transmitter was last seen online
    #[serde(rename = "lastConnected")]
    pub last_connected: Option<DateTime<Utc>>,

    /// Time at which the transmitter became online
    #[serde(rename = "connectedSince")]
    pub connected_since: Option<DateTime<Utc>>,
}
