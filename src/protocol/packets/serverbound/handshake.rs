use std::io::{Read, Write};

use derive_macro::Readable;

use crate::{
    connection::{Connection, State},
    error::Error,
    protocol::{varint::VarInt, Processable, ReadExt, Readable},
};

#[derive(Clone, Debug)]
pub enum NextState {
    Status,
    Login,
}
impl From<i32> for NextState {
    fn from(next_state: i32) -> Self {
        match next_state {
            1 => Self::Status,
            2 => Self::Login,
            _ => unreachable!(),
        }
    }
}

impl Readable for NextState {
    fn read<R: Read>(reader: &mut R) -> Result<Self, Error> {
        Ok(NextState::from(i32::from(reader.readable::<VarInt>()?)))
    }
}
#[derive(Clone, Debug, Readable)]
pub struct Handshake {
    #[inner(VarInt)]
    pub protocol_version: i32,
    pub server_address: String,
    pub server_port: u16,
    pub next_state: NextState,
}

impl Processable for Handshake {
    fn process<S: Read + Write>(self, _: &mut S, connection: &mut Connection) -> Result<(), Error> {
        connection.change_state(State::from(self.next_state));
        Ok(())
    }
}
