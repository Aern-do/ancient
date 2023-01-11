use std::io::{Read, Write};

use uuid::Uuid;

use crate::error::Error;

use super::{ReadExt, Readable, WriteExt, Writeable};

impl Readable for Uuid {
    fn read<R: Read>(reader: &mut R) -> Result<Self, Error> {
        Ok(Self::from_u64_pair(reader.readable()?, reader.readable()?))
    }
}
impl Writeable for Uuid {
    fn write<W: Write>(self, writer: &mut W) -> Result<(), Error> {
        let (high_bits, low_bits) = self.as_u64_pair();
        writer.writeable(high_bits)?;
        writer.writeable(low_bits)?;
        Ok(())
    }
}
