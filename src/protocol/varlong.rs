use std::{
    fmt::{self, Display},
    io::{Read, Write},
};

use crate::error::Error;

use super::{DecodeExt, Decode, EncodeExt, Encode};

#[derive(Clone, Copy, Debug)]
pub struct VarLong(i64);
impl From<VarLong> for i64 {
    fn from(varlong: VarLong) -> Self {
        varlong.0
    }
}
impl From<i64> for VarLong {
    fn from(varlong: i64) -> Self {
        Self(varlong)
    }
}
impl Display for VarLong {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}
impl Decode for VarLong {
    fn decode<R: Read>(reader: &mut R) -> Result<Self, Error> {
        let mut answer = 0;
        for i in 0..8 {
            let byte = reader.decode::<u8>()?;
            answer |= ((byte & 0b0111_1111) as i64) << (7 * i);
            if byte & 0b1000_0000 == 0 {
                break;
            }
        }
        Ok(VarLong(answer))
    }
}
impl Encode for VarLong {
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
