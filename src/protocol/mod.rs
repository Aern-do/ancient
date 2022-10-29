pub mod primitive;
pub mod string;
pub mod varint;
pub mod varlong;
pub mod vec;

use std::io::{Read, Write};

use crate::error::Error;

pub trait Readable: Sized {
    fn read<R: Read>(reader: &mut R) -> Result<Self, Error>;
}
pub trait Writeable: Sized {
    fn write<W: Write>(self, writer: &mut W) -> Result<(), Error>;
}
pub trait ReadExt: Read + Sized {
    fn readable<R: Readable>(&mut self) -> Result<R, Error> {
        R::read(self)
    }
}
pub trait WriteExt: Write + Sized {
    fn writeable<W: Writeable>(&mut self, value: W) -> Result<(), Error> {
        value.write(self)
    }
}
impl<T> ReadExt for T where T: Read {}
impl<T> WriteExt for T where T: Write {}
