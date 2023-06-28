use std::{io, result};

use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("io error")]
    Io(#[from] io::Error)
}

pub type Result<T, E = Error> = result::Result<T, E>;