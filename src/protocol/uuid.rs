use std::io::{Read, Write};

use uuid::Uuid;

use crate::error::Error;

use super::{Decode, DecodeExt, Encode, EncodeExt};

impl Decode for Uuid {
    fn decode<R: Read>(reader: &mut R) -> Result<Self, Error> {
        Ok(Self::from_u64_pair(reader.decode()?, reader.decode()?))
    }
}
impl Encode for Uuid {
    fn encode<W: Write>(self, writer: &mut W) -> Result<(), Error> {
        let (high_bits, low_bits) = self.as_u64_pair();
        writer.encode(high_bits)?;
        writer.encode(low_bits)?;
        Ok(())
    }
}
