use std::io::{self};

use crate::error::Error;

use super::{ReadExt, Readable, WriteExt, Writeable};

macro_rules! implement {
    (u8) => {
        implement!(@byte u8);
    };
    (i8) => {
        implement!(@byte i8);
    };
    ($target: ident) => {
        impl crate::protocol::Readable for $target {
            fn read<R: std::io::Read>(reader: &mut R) -> Result<Self, crate::error::Error> {
                use byteorder::ReadBytesExt;
                paste::paste!(Ok(reader.[<read_$target>]::<byteorder::BigEndian>()?))
            }
        }
        impl crate::protocol::Writeable for $target {
            fn write<W: std::io::Write>(self, writer: &mut W) -> Result<(), crate::error::Error> {
                use byteorder::WriteBytesExt;
                paste::paste!(writer.[<write_$target>]::<byteorder::BigEndian>(self)?);
                Ok(())
            }
        }
    };
    (@byte $target: ident) => {
        impl crate::protocol::Readable for $target {
            fn read<R: std::io::Read>(reader: &mut R) -> Result<Self, crate::error::Error> {
                use byteorder::ReadBytesExt;
                paste::paste!(Ok(reader.[<read_$target>]()?))
            }
        }
        impl crate::protocol::Writeable for $target {
            fn write<W: std::io::Write>(self, writer: &mut W) -> Result<(), crate::error::Error> {
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
impl Readable for bool {
    fn read<R: io::Read>(reader: &mut R) -> Result<Self, Error> {
        Ok(reader.readable::<u8>()? != 0)
    }
}
impl Writeable for bool {
    fn write<W: io::Write>(self, writer: &mut W) -> Result<(), Error> {
        writer.writeable(self as u8)
    }
}
