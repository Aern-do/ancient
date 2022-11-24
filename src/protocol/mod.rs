pub mod chat;
pub mod packets;
pub mod primitive;
pub mod string;
pub mod varint;
pub mod varlong;
pub mod vec;

use std::io::{Read, Write};

use serde::Serialize;

use crate::{connection::Connection, error::Error};

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Status {
    pub version: Version,
    pub players: Players,
    pub description: Description,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub favicon: Option<String>,
    pub previews_chat: bool,
    pub enforces_secure_chat: bool,
}
#[derive(Debug, Clone, Serialize)]
pub struct Version {
    pub name: String,
    pub protocol: usize,
}
#[derive(Debug, Clone, Serialize)]
pub struct Players {
    pub max: usize,
    pub online: usize,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub sample: Vec<Sample>,
}
#[derive(Debug, Clone, Serialize)]
pub struct Sample {
    pub name: String,
    pub id: String,
}
#[derive(Debug, Clone, Serialize)]
pub struct Description {
    pub text: String,
}
pub trait Packet {
    fn identifier() -> i32;
}
#[macro_export]
macro_rules! packet {
    ($identifier: literal -> $target: ident) => {
        impl $crate::protocol::Packet for $target {
            fn identifier() -> i32 {
                $identifier
            }
        }
    };
}
pub trait Processable: Sized {
    fn process<S: Read + Write>(
        self,
        stream: &mut S,
        connection: &mut Connection,
    ) -> Result<(), Error>;
}
pub trait Readable: Sized {
    fn read<R: Read>(reader: &mut R) -> Result<Self, Error>;
}
pub trait Writeable: Sized {
    fn write<W: Write>(self, writer: &mut W) -> Result<(), Error>;
}
pub trait ReadExt: Read + Sized {
    fn readable<R: Readable>(&mut self) -> Result<R, Error> {
        R::read(self)
    }
}
pub trait WriteExt: Write + Sized {
    fn writeable<W: Writeable>(&mut self, value: W) -> Result<(), Error> {
        value.write(self)
    }
}
impl<T> ReadExt for T where T: Read {}
impl<T> WriteExt for T where T: Write {}
