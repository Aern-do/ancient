use std::io::{Read, Write};

use crate::error::Error;

use super::{varint::VarInt, Decode, DecodeExt, Encode, EncodeExt};

impl Decode for String {
    fn decode<R: Read>(reader: &mut R) -> Result<Self, Error> {
        let length = i32::from(reader.decode::<VarInt>()?);
        let mut buffer = vec![0; length as usize];
        reader.read_exact(&mut buffer)?;
        unsafe { Ok(String::from_utf8_unchecked(buffer)) }
    }
}
impl Encode for String {
    fn encode<W: Write>(self, writer: &mut W) -> Result<(), Error> {
        writer.encode(VarInt(self.len() as i32))?;
        for byte in self.bytes() {
            writer.encode(byte)?;
        }
        Ok(())
    }
}
