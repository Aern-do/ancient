use std::io::{Read, Write};

use serde::{de::DeserializeOwned, Serialize};

use crate::error::Error;

use super::{varint::VarInt, DecodeExt, Decode, EncodeExt, Encode};

#[derive(Clone, Debug)]
pub struct Json<T>(T);
impl<T> Json<T> {
    pub fn into_inner(self) -> T {
        self.0
    }
}
impl<T: DeserializeOwned> Decode for Json<T> {
    fn decode<R: Read>(reader: &mut R) -> Result<Self, Error> {
        let length = i32::from(reader.decode::<VarInt>()?);
        let mut buffer = vec![0; length as usize];
        reader.read_exact(&mut buffer)?;
        Ok(Self(serde_json::from_slice(&buffer)?))
    }
}
impl<T: Serialize> Encode for Json<T> {
    fn encode<W: Write>(self, writer: &mut W) -> Result<(), Error> {
        writer.encode(serde_json::to_string(&self.0)?)
    }
}
