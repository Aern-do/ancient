use std::{
    io::{Cursor, Read, Write},
    net::TcpStream,
};

use log::debug;

use crate::{
    error::Error,
    protocol::{
        packets::serverbound::{
            handshake::{Handshake, NextState},
            status::{ping_request::PingRequest, status_request::StatusRequest},
        },
        varint::VarInt,
        Packet, Processable, ReadExt, WriteExt, Writeable,
    },
};
#[derive(Clone, Debug)]
pub enum State {
    Handshaking,
    Status,
    Login,
    Play,
}
impl Default for State {
    fn default() -> Self {
        Self::Handshaking
    }
}
impl From<NextState> for State {
    fn from(next_state: NextState) -> Self {
        match next_state {
            NextState::Status => Self::Status,
            NextState::Login => Self::Login,
        }
    }
}
#[derive(Clone, Debug, Default)]
pub struct Connection {
    state: State,
}
impl Connection {
    pub fn new() -> Connection {
        Default::default()
    }
    pub fn change_state(&mut self, state: State) {
        debug!("Changing connection state to {:?}", state);
        self.state = state;
    }
    pub fn write_packet<P: Writeable + Packet, S: Write + Read>(
        &mut self,
        stream: &mut S,
        packet: P,
    ) -> Result<(), Error> {
        let mut unprefixed = vec![];
        unprefixed.writeable::<VarInt>(VarInt(P::identifier()))?;
        unprefixed.writeable::<P>(packet)?;
        let mut prefixed = vec![];
        prefixed.writeable::<VarInt>(VarInt(unprefixed.len() as i32))?;
        prefixed.extend(unprefixed);
        stream.write_all(&prefixed)?;
        Ok(())
    }
    pub fn start_receiving(mut self, mut stream: TcpStream) -> Result<(), Error> {
        loop {
            let length = i32::from(stream.readable::<VarInt>()?);
            let mut buffer = vec![0; length as usize];
            stream.read_exact(&mut buffer)?;
            let mut cursor = Cursor::new(buffer);
            let identifier = i32::from(cursor.readable::<VarInt>()?);
            debug!("Recivied packet with identifier {:#x}", identifier);
            match self.state {
                State::Handshaking if identifier == 0x0 => cursor
                    .readable::<Handshake>()?
                    .process(&mut stream, &mut self)?,
                State::Status => match identifier {
                    0x0 => cursor
                        .readable::<StatusRequest>()?
                        .process(&mut stream, &mut self)?,
                    0x1 => cursor
                        .readable::<PingRequest>()?
                        .process(&mut stream, &mut self)?,
                    _ => return Err(Error::UnsupportedPacket(identifier)),
                },
                _ => return Err(Error::UnsupportedPacket(identifier)),
            }
        }
    }
}
