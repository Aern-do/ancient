use std::io::{Read, Write};

use crate::error::Result;

use super::{primitive::Primitive, Decode, DecodeExt, Encode, EncodeExt, Prefix};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PrefixedString<P> {
    prefix: P,
    inner: String,
}

impl<P> PrefixedString<P> {
    pub fn new(prefix: P, inner: String) -> Self {
        Self { prefix, inner }
    }
}

impl<'e, P: Primitive> Encode for &'e PrefixedString<P> {
    fn encode<W: Write>(self, writer: &mut W) -> Result<()> {
        writer.encode(self.prefix)?;

        for char in self.inner.chars() {
            writer.encode(char)?;
        }

        Ok(())
    }
}

impl<P: Primitive + Prefix> Decode for PrefixedString<P> {
    fn decode<R: Read>(reader: &mut R) -> Result<Self> {
        let prefix = reader.decode::<P>()?;

        let mut buffer = Vec::with_capacity(prefix.into());
        for _ in 0..prefix.into() {
            buffer.push(reader.decode::<u8>()?);
        }

        Ok(Self::new(prefix, unsafe { String::from_utf8_unchecked(buffer) }))
    }
}

#[cfg(test)]
mod tests {
    use crate::{protocol::string::PrefixedString, test_decode, test_encode};

    test_decode! {
        decode_empty_string<PrefixedString<u8>>([0x0]) => PrefixedString::new(0, String::new());
        decode_string<PrefixedString<u8>>([0xc, 0x48, 0x65, 0x6c, 0x6c, 0x6f, 0x2c, 0x20, 0x57, 0x6f, 0x72, 0x6c, 0x64]) => PrefixedString::new(12, String::from("Hello, World"));
    }

    test_encode! {
        encode_empty_string(&PrefixedString::new(0_u8, String::new())) => [0x0];
        encode_string(&PrefixedString::new(12_u8, String::from("Hello, World"))) => [0xc, 0x48, 0x65, 0x6c, 0x6c, 0x6f, 0x2c, 0x20, 0x57, 0x6f, 0x72, 0x6c, 0x64];
    }
}
