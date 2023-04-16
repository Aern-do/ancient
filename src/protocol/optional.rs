use std::io::{Read, Write};

use crate::error::Error;

use super::{Decode, DecodeExt, Encode, EncodeExt};

impl<T: Decode> Decode for Option<T> {
    fn decode<R: Read>(reader: &mut R) -> Result<Self, Error> {
        let has_field = reader.decode()?;
        if has_field {
            Ok(Some(reader.decode()?))
        } else {
            Ok(None)
        }
    }
}
impl<T: Encode> Encode for Option<T> {
    fn encode<W: Write>(self, writer: &mut W) -> Result<(), Error> {
        match self {
            Some(data) => {
                writer.encode(true)?;
                writer.encode(data)?;
            }
            None => {
                writer.encode(false)?;
            }
        }
        Ok(())
    }
}
