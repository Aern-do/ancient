use std::io;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Unsupported packet {0:#x}")]
    UnsupportedPacket(i32),
    #[error("Input/Output error\n{0}")]
    Io(#[from] io::Error),
}
