use std::{
    io::{Cursor, Read},
    net::TcpStream,
};

use log::debug;

use crate::{
    error::Error,
    protocol::{varint::VarInt, ReadExt},
};
#[derive(Clone, Debug, Default)]
pub struct Connection {}
impl Connection {
    pub fn new() -> Connection {
        Default::default()
    }
    pub fn start_receiving(self, mut stream: TcpStream) -> Result<(), Error> {
        loop {
            let length = i32::from(stream.readable::<VarInt>()?);
            let mut buffer = vec![0; length as usize];
            stream.read_exact(&mut buffer)?;
            let mut cursor = Cursor::new(buffer);
            let identifier = i32::from(cursor.readable::<VarInt>()?);
            debug!("Recivied packet with identifier {:#x}", identifier);
        }
    }
}
