use serde::Deserialize;
use std::net::IpAddr;

#[derive(Debug, Deserialize)]
pub struct Connection {
    /// Public IP of the device
    #[serde(rename = "ip_addr")]
    pub ip: IpAddr,

    pub port: u64,
}
