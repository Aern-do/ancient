use std::io::{Read, Write};

use serde::{de::DeserializeOwned, Serialize};

use crate::error::Error;

use super::{varint::VarInt, ReadExt, Readable, WriteExt, Writeable};

#[derive(Clone, Debug)]
pub struct Json<T>(T);
impl<T: DeserializeOwned> Readable for Json<T> {
    fn read<R: Read>(reader: &mut R) -> Result<Self, Error> {
        let length = i32::from(reader.readable::<VarInt>()?);
        let mut buffer = vec![0; length as usize];
        reader.read_exact(&mut buffer)?;
        Ok(Self(serde_json::from_slice(&buffer)?))
    }
}
impl<T: Serialize> Writeable for Json<T> {
    fn write<W: Write>(self, writer: &mut W) -> Result<(), Error> {
        writer.writeable(serde_json::to_string(&self.0)?)
    }
}
