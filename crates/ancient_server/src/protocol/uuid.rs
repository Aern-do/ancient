use std::io::{Read, Write};

use uuid::Uuid;

use crate::error::Result;

use super::{Decode, DecodeExt, Encode, EncodeExt};

impl Decode for Uuid {
    fn decode<R: Read>(reader: &mut R) -> Result<Self> {
        Ok(Self::from_u64_pair(reader.decode()?, reader.decode()?))
    }
}

impl Encode for Uuid {
    fn encode<W: Write>(self, writer: &mut W) -> Result<()> {
        let (high_bits, low_bits) = self.as_u64_pair();
        writer.encode(high_bits)?;
        writer.encode(low_bits)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use uuid::Uuid;

    use crate::{test_decode, test_encode};

    test_decode! {
        decode_uuid<Uuid>(vec![0x3d, 0x0d, 0x3a, 0x5d, 0x4c, 0x96, 0x4a, 0xe0, 0xb8, 0x03, 0x91, 0x3e, 0x48, 0x2c, 0x0e, 0x2a]) => Uuid::parse_str("3d0d3a5d-4c96-4ae0-b803-913e482c0e2a").unwrap();
    }

    test_encode! {
        encode_uuid(Uuid::parse_str("3d0d3a5d-4c96-4ae0-b803-913e482c0e2a").unwrap()) => vec![0x3d, 0x0d, 0x3a, 0x5d, 0x4c, 0x96, 0x4a, 0xe0, 0xb8, 0x03, 0x91, 0x3e, 0x48, 0x2c, 0x0e, 0x2a];
    }
}
