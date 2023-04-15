use std::{
    io::{Read, Write},
    marker::PhantomData,
};

use crate::error::Error;

use super::{DecodeExt, Decode, EncodeExt, Encode};

#[derive(Debug)]
pub struct PrefixedVec<T, P> {
    pub inner: Vec<T>,
    _preifx: PhantomData<P>,
}

impl<T, P> Default for PrefixedVec<T, P> {
    fn default() -> Self {
        Self {
            inner: Default::default(),
            _preifx: Default::default(),
        }
    }
}

impl<T, P> From<PrefixedVec<T, P>> for Vec<T> {
    fn from(prefixed_vec: PrefixedVec<T, P>) -> Self {
        prefixed_vec.inner
    }
}

impl<T, P> From<Vec<T>> for PrefixedVec<T, P> {
    fn from(inner: Vec<T>) -> Self {
        PrefixedVec {
            inner,
            ..Default::default()
        }
    }
}

impl<T: Decode, P: Into<usize> + Decode> Decode for PrefixedVec<T, P> {
    fn decode<R: Read>(reader: &mut R) -> Result<Self, Error> {
        let length: usize = reader.decode::<P>()?.into();
        let mut vec = vec![];
        for _ in 0..length {
            vec.push(reader.decode::<T>()?)
        }
        Ok(Self {
            inner: vec,
            ..Default::default()
        })
    }
}

impl<T: Encode, P: From<i32> + Encode> Encode for PrefixedVec<T, P> {
    fn encode<W: Write>(self, writer: &mut W) -> Result<(), Error> {
        writer.encode(P::from(self.inner.len() as i32))?;
        for element in self.inner {
            writer.encode(element)?;
        }
        Ok(())
    }
}
