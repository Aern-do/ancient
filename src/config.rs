use std::net::Ipv4Addr;

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Config {
    pub address: Ipv4Addr,
    pub port: u16,
}
impl Default for Config {
    fn default() -> Self {
        Self {
            address: Ipv4Addr::new(127, 0, 0, 1),
            port: 25565,
        }
    }
}
