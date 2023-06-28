pub mod primitive;
pub mod string;

use std::io::{Read, Write};

use byteorder::BigEndian;

use crate::error::Result;

#[macro_export]
macro_rules! test_decode {
    ($($name: ident<$target: ty>($input: expr) => $expected: expr);* $(;)?) => {
        use $crate::protocol::DecodeExt;
        $(
            #[test]
            fn $name() {
                let mut reader = std::io::Cursor::new($input);
                assert_eq!(reader.decode::<$target>().unwrap(), $expected)
            }
        )*
    };
}

#[macro_export]
macro_rules! test_encode {
    ($($name: ident<$target: ty>($input: expr) => $expected: expr);* $(;)?) => {
        use $crate::protocol::EncodeExt;
        $(
            #[test]
            fn $name() {
                let mut writer = Vec::new();
                writer.encode($input).unwrap();
                assert_eq!(writer, $expected)
            }
        )*
    };
}

pub type ProtocolEndian = BigEndian;

pub trait Prefix: Into<usize> {}
impl<L: Into<usize>> Prefix for L {}

pub trait Decode: Sized {
    fn decode<R: Read>(reader: &mut R) -> Result<Self>;
}

pub trait Encode {
    fn encode<W: Write>(self, writer: &mut W) -> Result<()>;
}

impl<'e, E: Encode + Copy> Encode for &'e E {
    fn encode<W: Write>(self, writer: &mut W) -> Result<()> {
        E::encode(*self, writer)
    }
}

pub trait DecodeExt: Read {
    fn decode<D: Decode>(&mut self) -> Result<D>
    where
        Self: Sized,
    {
        D::decode(self)
    }
}

pub trait EncodeExt: Write {
    fn encode<E: Encode>(&mut self, value: E) -> Result<()>
    where
        Self: Sized,
    {
        E::encode(value, self)
    }
}

impl<R: Read> DecodeExt for R {}
impl<W: Write> EncodeExt for W {}
