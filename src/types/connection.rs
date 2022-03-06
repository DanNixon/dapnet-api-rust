use serde::Deserialize;
use std::{fmt, net::IpAddr};

#[derive(Debug, Deserialize)]
pub struct Connection {
    /// Public IP of the device
    #[serde(rename = "ip_addr")]
    pub ip: IpAddr,

    pub port: u64,
}

impl fmt::Display for Connection {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}:{}", self.ip, self.port)
    }
}
