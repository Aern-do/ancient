use std::io::{Read, Write};

use crate::error::Result;

use super::{primitive::Primitive, Decode, DecodeExt, Encode, EncodeExt, IntoInner, Prefix};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PrefixedVec<P, T> {
    inner: Vec<T>,
    prefix: P,
}

impl<P, T> IntoInner for PrefixedVec<P, T> {
    type Inner = Vec<T>;

    fn into_inner(self) -> Self::Inner {
        self.inner
    }
}

impl<P, T> PrefixedVec<P, T> {
    pub fn new(inner: Vec<T>, prefix: P) -> Self {
        Self { inner, prefix }
    }
}

impl<P: Primitive + Prefix, T: Decode> Decode for PrefixedVec<P, T> {
    fn decode<R: Read>(reader: &mut R) -> Result<Self> {
        let prefix = reader.decode::<P>()?;
        let mut inner = Vec::with_capacity(prefix.into());

        for _ in 0..prefix.into() {
            inner.push(reader.decode::<T>()?);
        }

        Ok(PrefixedVec::new(inner, prefix))
    }
}

impl<P: Encode, T: Encode> Encode for PrefixedVec<P, T> {
    fn encode<W: Write>(self, writer: &mut W) -> Result<()> {
        writer.encode(self.prefix)?;

        for element in self.inner {
            writer.encode(element)?;
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::{protocol::vec::PrefixedVec, test_decode, test_encode};

    test_decode! {
        decode_empty_prefixed_vec<PrefixedVec<u8, u8>>(vec![0x0]) => PrefixedVec::new(vec![], 0x0);
        decode_prefixed_vec_four_elements<PrefixedVec<u8, u8>>(vec![0x4, 0x0, 0x0, 0x0, 0x0]) => PrefixedVec::new(vec![0x0, 0x0, 0x0, 0x0], 0x4);
    }

    test_encode! {
        encode_empty_prefixed_vec(PrefixedVec::<u8, u8>::new(vec![], 0x0)) => vec![0x0];
        encode_prefixed_vec_four_elements(PrefixedVec::<u8, u8>::new(vec![0x0, 0x0, 0x0, 0x0], 0x4)) => vec![0x4, 0x0, 0x0, 0x0, 0x0];
    }
}
