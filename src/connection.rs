use std::{
    io::{self, Cursor, Read, Write},
    net::TcpStream,
};

use log::{debug, info};
use rand::random;

use crate::{
    error::Error,
    protocol::{
        packets::serverbound::{
            handshake::{Handshake, NextState},
            login::{encryption_response::EncryptionResponse, login_start::LoginStart},
            status::{ping_request::PingRequest, status_request::StatusRequest},
        },
        varint::VarInt,
        Packet, Processable, ReadExt, WriteExt, Writeable,
    },
    socket::Socket,
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
#[derive(Debug)]
pub struct Connection {
    state: State,
    pub verify_token: [u8; 4],
    pub socket: Socket<TcpStream>,
    pub user: Option<String>,
}
impl Connection {
    pub fn new(stream: TcpStream) -> Connection {
        Self {
            socket: Socket::new(stream),
            state: Default::default(),
            user: Default::default(),
            verify_token: random(),
        }
    }
    pub fn change_state(&mut self, state: State) {
        debug!("Changing connection state to {:?}", state);
        self.state = state;
    }
    pub fn enable_encryption(&mut self, key: [u8; 16]) {
        info!("Encryption is enabled for {}", self.user.as_ref().unwrap());
        self.socket.enable_encryption(key);
    }
    pub fn write_packet<P: Writeable + Packet>(&mut self, packet: P) -> Result<(), Error> {
        debug!("Writing packet with identifier 0x{:x}", P::identifier());
        let mut unprefixed = vec![];
        unprefixed.writeable::<VarInt>(VarInt(P::identifier()))?;
        unprefixed.writeable::<P>(packet)?;
        let mut prefixed = vec![];
        prefixed.writeable::<VarInt>(VarInt(unprefixed.len() as i32))?;
        prefixed.extend(unprefixed);
        self.write_all(&prefixed)?;
        Ok(())
    }
    pub fn start_receiving(mut self) -> Result<(), Error> {
        loop {
            let length = i32::from(self.readable::<VarInt>()?);
            let mut buffer = vec![0; length as usize];
            self.read_exact(&mut buffer)?;
            let mut cursor = Cursor::new(buffer);
            let identifier = i32::from(cursor.readable::<VarInt>()?);
            debug!("Received packet with identifier {:#x}", identifier);
            match self.state {
                State::Handshaking if identifier == 0x0 => {
                    cursor.readable::<Handshake>()?.process(&mut self)?
                }
                State::Status => match identifier {
                    0x0 => cursor.readable::<StatusRequest>()?.process(&mut self)?,
                    0x1 => cursor.readable::<PingRequest>()?.process(&mut self)?,
                    _ => return Err(Error::UnsupportedPacket(identifier, self.state)),
                },
                State::Login => match identifier {
                    0x0 => cursor.readable::<LoginStart>()?.process(&mut self)?,
                    0x1 => cursor
                        .readable::<EncryptionResponse>()?
                        .process(&mut self)?,
                    _ => return Err(Error::UnsupportedPacket(identifier, self.state)),
                },
                _ => return Err(Error::UnsupportedPacket(identifier, self.state)),
            }
        }
    }
}
impl Write for Connection {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        self.socket.write(buf)
    }

    fn flush(&mut self) -> io::Result<()> {
        self.socket.flush()
    }
}
impl Read for Connection {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        self.socket.read(buf)
    }
}
