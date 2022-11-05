use std::io;
use thiserror::Error;

use crate::connection::State;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Unsupported packet with identifier {0:#x} at {1:?} state")]
    UnsupportedPacket(i32, State),
    #[error("Input/Output error\n{0}")]
    Io(#[from] io::Error),
    #[error("Error when reading config file\n{0}")]
    ReadConfig(#[from] toml::de::Error),
    #[error("Error when writing config\n{0}")]
    WriteConfig(#[from] toml::ser::Error),
}
