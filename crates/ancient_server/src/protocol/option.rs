use std::io::{Read, Write};

use crate::error::Result;

use super::{Decode, DecodeExt, Encode, EncodeExt};

impl<T: Decode> Decode for Option<T> {
    fn decode<R: Read>(reader: &mut R) -> Result<Self> {
        let has_field = reader.decode()?;
        if has_field {
            Ok(Some(reader.decode()?))
        } else {
            Ok(None)
        }
    }
}
impl<T: Encode> Encode for Option<T> {
    fn encode<W: Write>(self, writer: &mut W) -> Result<()> {
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

#[cfg(test)]
mod tests {
    use crate::{test_decode, test_encode};

    test_decode! {
        decode_option_u8_none<Option<u8>>(vec![0x00]) => None;
        decode_option_u8_some<Option<u8>>(vec![0x01, 0x3d]) => Some(0x3d);
    }

    test_encode! {
        encode_option_u8_none(None as Option<u8>) => vec![0x00];
        encode_option_u8_some(Some(0x3d_u8)) => vec![0x01, 0x3d];

    }
}
