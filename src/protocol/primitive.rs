use std::io::{Read, Write};

use super::{Decode, DecodeExt, Encode, EncodeExt};
use crate::{error::Result, protocol::ProtocolEndian};
use byteorder::{ReadBytesExt, WriteBytesExt};

macro_rules! implement_primitives {
    ($($primitive: ident $(<$endian: ident>)?),* $(,)?) => {
        $(
            impl Decode for $primitive {
                fn decode<R: Read>(reader: &mut R) -> Result<Self> {
                    paste::paste! {
                        Ok(reader.[<read_ $primitive>]$(::<$endian>)?()?)
                    }
                }
            }

            impl Encode for $primitive {
                fn encode<W: Write>(self, writer: &mut W) -> Result<()> {
                    paste::paste! {
                        writer.[<write_ $primitive>]$(::<$endian>)?(self)?;
                        Ok(())
                    }
                }
            }
        )*
    };
}

pub trait Primitive: Clone + Copy + Encode + Decode + Default {}
impl<P: Clone + Copy + Encode + Decode + Default> Primitive for P {}

implement_primitives! {
    i8, i16<ProtocolEndian>, i32<ProtocolEndian>, i64<ProtocolEndian>,
    u8, u16<ProtocolEndian>, u32<ProtocolEndian>, u64<ProtocolEndian>,
    f32<ProtocolEndian>, f64<ProtocolEndian>
}

impl Decode for char {
    fn decode<R: Read>(reader: &mut R) -> Result<Self> {
        Ok(reader.decode::<u8>()? as char)
    }
}

impl Encode for char {
    fn encode<W: Write>(self, writer: &mut W) -> Result<()> {
        writer.encode(self as u8)?;
        Ok(())
    }
}

impl Decode for bool {
    fn decode<R: Read>(reader: &mut R) -> Result<Self> {
        Ok(reader.decode::<u8>()? != 0)
    }
}

impl Encode for bool {
    fn encode<W: Write>(self, writer: &mut W) -> Result<()> {
        writer.encode(self as u8)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::{test_decode, test_encode};

    test_decode! {
        decode_i8<i8>([0xF6]) => -10;
        decode_i16<i16>([0x01, 0x01]) => 257;
        decode_i32<i32>([0x00, 0x01, 0x00, 0x01]) => 65537;
        decode_i64<i64>([0xFF, 0xFF, 0xFF, 0xFE, 0xFF, 0xFF, 0xFF, 0xFF]) => -4294967297;
        decode_u8<u8>([0x0B]) => 11;
        decode_u16<u16>([0x01, 0x02]) => 258;
        decode_u32<u32>([0x00, 0x01, 0x00, 0x02]) => 65538;
        decode_u64<u64>([0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x02]) => 4294967298;
        decode_f32<f32>([0x3F, 0x9D, 0xF3, 0xB6]) => 1.234;
        decode_f64<f64>([0x3F, 0xF3, 0xC0, 0xCA, 0x42, 0x83, 0xDE, 0x1B]) => 1.23456789;
        decode_char<char>([0x41]) => 'A';
        decode_true<bool>([0x1]) => true;
        decode_false<bool>([0x0]) => false;
    }

    test_encode! {
        encode_i8(-10_i8) => [0xF6];
        encode_i16(257_i16) => [0x01, 0x01];
        encode_i32(65537_i32) => [0x00, 0x01, 0x00, 0x01];
        encode_i64(-4294967297_i64) => [0xFF, 0xFF, 0xFF, 0xFE, 0xFF, 0xFF, 0xFF, 0xFF];
        encode_u8(11_u8) => [0x0B];
        encode_u16(258_u16) => [0x01, 0x02];
        encode_u32(65538_u32) => [0x00, 0x01, 0x00, 0x02];
        encode_u64(4294967298_u64) => [0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x02];
        encode_f32(1.234_f32) => [0x3F, 0x9D, 0xF3, 0xB6];
        encode_f64(1.23456789_f64) => [0x3F, 0xF3, 0xC0, 0xCA, 0x42, 0x83, 0xDE, 0x1B];
        encode_char('A') => [0x41];
        encode_true(true) => [0x1];
        encode_false(false) => [0x0];
    }
}
