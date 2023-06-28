use std::{
    io::{Read, Write},
    marker::PhantomData,
    mem::MaybeUninit,
};

use crate::error::Result;

use super::{primitive::Primitive, Decode, DecodeExt, Encode, EncodeExt, Prefix};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PrefixedArray<P, T, const SIZE: usize> {
    _prefix: PhantomData<P>,
    inner: [T; SIZE],
}

impl<P, T, const SIZE: usize> PrefixedArray<P, T, SIZE> {
    pub fn new(inner: [T; SIZE]) -> Self {
        Self { _prefix: Default::default(), inner }
    }
}

impl<P: Primitive + Prefix + TryFrom<usize>, T: Encode, const SIZE: usize> Encode
    for PrefixedArray<P, T, SIZE>
{
    fn encode<W: Write>(self, writer: &mut W) -> Result<()> {
        writer.encode(P::try_from(SIZE).unwrap_or_default())?;

        for element in self.inner {
            writer.encode(element)?;
        }

        Ok(())
    }
}

impl<P: Decode, T: Decode, const SIZE: usize> Decode for PrefixedArray<P, T, SIZE> {
    fn decode<R: Read>(reader: &mut R) -> Result<Self> {
        reader.decode::<P>()?;

        let mut uninit: [MaybeUninit<T>; SIZE] = unsafe { MaybeUninit::uninit().assume_init() };
        for element in uninit.iter_mut().take(SIZE) {
            *element = MaybeUninit::new(reader.decode::<T>()?);
        }

        Ok(Self::new(uninit.map(|uninit| unsafe { uninit.assume_init() })))
    }
}

#[cfg(test)]
mod tests {
    use crate::{protocol::array::PrefixedArray, test_decode, test_encode};

    test_decode! {
        decode_empty_array<PrefixedArray<u8, u8, 0>>([0x0]) => PrefixedArray::new([]);
        decode_array<PrefixedArray<u8, u8, 4>>([0x4, 0x0, 0x0, 0x0, 0x0]) => PrefixedArray::new([0x0, 0x0, 0x0, 0x0]);
    }

    test_encode! {
        encode_empty_array(PrefixedArray::<u8, u8, 0>::new([])) => vec![0x0];
        encode_array(PrefixedArray::<u8, u8, 4>::new([0x0, 0x0, 0x0, 0x0])) => vec![0x4, 0x0, 0x0, 0x0, 0x0];
    }
}
