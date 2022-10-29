use std::{
    io::{Read, Write},
    marker::PhantomData,
};

use crate::error::Error;

use super::{ReadExt, Readable, WriteExt, Writeable};

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

impl<T: Readable, P: Into<usize> + Readable> Readable for PrefixedVec<T, P> {
    fn read<R: Read>(reader: &mut R) -> Result<Self, Error> {
        let length: usize = reader.readable::<P>()?.into();
        let mut vec = vec![];
        for _ in 0..length {
            vec.push(reader.readable::<T>()?)
        }
        Ok(Self {
            inner: vec,
            ..Default::default()
        })
    }
}

impl<T: Writeable, P: From<usize> + Writeable> Writeable for PrefixedVec<T, P> {
    fn write<W: Write>(self, writer: &mut W) -> Result<(), Error> {
        writer.writeable(P::from(self.inner.len()))?;
        for element in self.inner {
            writer.writeable(element)?;
        }
        Ok(())
    }
}
