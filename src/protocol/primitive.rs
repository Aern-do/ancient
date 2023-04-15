use std::io::{self};

use crate::error::Error;

use super::{DecodeExt, Decode, EncodeExt, Encode};

macro_rules! implement {
    (u8) => {
        implement!(@byte u8);
    };
    (i8) => {
        implement!(@byte i8);
    };
    ($target: ident) => {
        impl crate::protocol::Decode for $target {
            fn decode<R: std::io::Read>(reader: &mut R) -> Result<Self, crate::error::Error> {
                use byteorder::ReadBytesExt;
                paste::paste!(Ok(reader.[<read_$target>]::<byteorder::BigEndian>()?))
            }
        }
        impl crate::protocol::Encode for $target {
            fn encode<W: std::io::Write>(self, writer: &mut W) -> Result<(), crate::error::Error> {
                use byteorder::WriteBytesExt;
                paste::paste!(writer.[<write_$target>]::<byteorder::BigEndian>(self)?);
                Ok(())
            }
        }
    };
    (@byte $target: ident) => {
        impl crate::protocol::Decode for $target {
            fn decode<R: std::io::Read>(reader: &mut R) -> Result<Self, crate::error::Error> {
                use byteorder::ReadBytesExt;
                paste::paste!(Ok(reader.[<read_$target>]()?))
            }
        }
        impl crate::protocol::Encode for $target {
            fn encode<W: std::io::Write>(self, writer: &mut W) -> Result<(), crate::error::Error> {
                use byteorder::WriteBytesExt;
                paste::paste!(writer.[<write_$target>](self)?);
                Ok(())
            }
        }
    };
    ($($target: ident),+ $(,)?) => {
        $(implement!($target);)+
    };
}

implement!(u8, i8, u16, i16, u32, i32, u64, i64);
impl Decode for bool {
    fn decode<R: io::Read>(reader: &mut R) -> Result<Self, Error> {
        Ok(reader.decode::<u8>()? != 0)
    }
}
impl Encode for bool {
    fn encode<W: io::Write>(self, writer: &mut W) -> Result<(), Error> {
        writer.encode(self as u8)
    }
}
