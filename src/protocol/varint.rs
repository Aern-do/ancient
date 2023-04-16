use std::{
    fmt::{self, Display},
    io::{Read, Write},
};

use crate::error::Error;

use super::{Decode, DecodeExt, Encode, EncodeExt};

#[derive(Clone, Copy, Debug)]
pub struct VarInt(pub i32);
impl From<VarInt> for i32 {
    fn from(varint: VarInt) -> Self {
        varint.0
    }
}
impl From<VarInt> for usize {
    fn from(varint: VarInt) -> Self {
        varint.0 as usize
    }
}
impl From<i32> for VarInt {
    fn from(varint: i32) -> Self {
        Self(varint)
    }
}

impl Display for VarInt {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}
impl Decode for VarInt {
    fn decode<R: Read>(reader: &mut R) -> Result<Self, Error> {
        let mut answer = 0;
        for i in 0..4 {
            let byte = reader.decode::<u8>()?;
            answer |= ((byte & 0b0111_1111) as i32) << (7 * i);
            if byte & 0b1000_0000 == 0 {
                break;
            }
        }
        Ok(VarInt(answer))
    }
}
impl Encode for VarInt {
    fn encode<W: Write>(mut self, writer: &mut W) -> Result<(), Error> {
        loop {
            let mut temp = (self.0 & 0b01111111) as u8;
            self.0 >>= 7;
            if self.0 != 0 {
                temp |= 0b10000000;
            }
            writer.encode(temp)?;
            if self.0 == 0 {
                break;
            }
        }
        Ok(())
    }
}
