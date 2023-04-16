use std::io::Read;

use derive_macro::Decode;

use crate::{
    connection::{Connection, State},
    error::Error,
    protocol::{varint::VarInt, Decode, DecodeExt, Processable},
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

impl Decode for NextState {
    fn decode<R: Read>(reader: &mut R) -> Result<Self, Error> {
        Ok(NextState::from(i32::from(reader.decode::<VarInt>()?)))
    }
}
#[derive(Clone, Debug, Decode)]
pub struct Handshake {
    #[inner(VarInt)]
    pub protocol_version: i32,
    pub server_address: String,
    pub server_port: u16,
    pub next_state: NextState,
}

impl Processable for Handshake {
    fn process(self, connection: &mut Connection) -> Result<(), Error> {
        connection.change_state(State::from(self.next_state));
        Ok(())
    }
}
