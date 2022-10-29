use std::io::{Read, Write};

use crate::error::Error;

use super::{varint::VarInt, ReadExt, Readable, WriteExt, Writeable};

impl Readable for String {
    fn read<R: Read>(reader: &mut R) -> Result<Self, Error> {
        let length = i32::from(reader.readable::<VarInt>()?);
        let mut buffer = vec![0; length as usize];
        reader.read_exact(&mut buffer)?;
        unsafe { Ok(String::from_utf8_unchecked(buffer)) }
    }
}
impl Writeable for String {
    fn write<W: Write>(self, writer: &mut W) -> Result<(), Error> {
        writer.writeable(VarInt(self.len() as i32))?;
        for byte in self.bytes() {
            writer.writeable(byte)?;
        }
        Ok(())
    }
}
