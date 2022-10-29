use std::io;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Input/Output error\n{0}")]
    Io(#[from] io::Error),
}
