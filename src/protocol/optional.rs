use std::io::{Read, Write};

use crate::error::Error;

use super::{ReadExt, Readable, WriteExt, Writeable};

impl<T: Readable> Readable for Option<T> {
    fn read<R: Read>(reader: &mut R) -> Result<Self, Error> {
        let has_field = reader.readable()?;
        if has_field {
            Ok(Some(reader.readable()?))
        } else {
            Ok(None)
        }
    }
}
impl<T: Writeable> Writeable for Option<T> {
    fn write<W: Write>(self, writer: &mut W) -> Result<(), Error> {
        match self {
            Some(data) => {
                writer.writeable(true)?;
                writer.writeable(data)?;
            }
            None => {
                writer.writeable(false)?;
            }
        }
        Ok(())
    }
}
