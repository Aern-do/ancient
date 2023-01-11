use std::{
    io::{Read, Write},
    marker::PhantomData,
    mem::MaybeUninit,
};

use crate::error::Error;

use super::{ReadExt, Readable, WriteExt, Writeable};

#[derive(Debug, Clone)]
pub struct PrefixedArray<P, T, const S: usize> {
    inner: [T; S],
    __prefix: PhantomData<P>,
}

impl<P, T, const S: usize> PrefixedArray<P, T, S> {
    pub fn new(inner: [T; S]) -> Self {
        Self {
            inner,
            __prefix: Default::default(),
        }
    }
}

impl<P, T, const S: usize> From<[T; S]> for PrefixedArray<P, T, S> {
    fn from(inner: [T; S]) -> Self {
        Self::new(inner)
    }
}
impl<P, T, const S: usize> From<PrefixedArray<P, T, S>> for [T; S] {
    fn from(prefixed_array: PrefixedArray<P, T, S>) -> Self {
        prefixed_array.inner
    }
}
impl<P: From<i32> + Writeable, T: Writeable, const S: usize> Writeable for PrefixedArray<P, T, S> {
    fn write<W: Write>(self, writer: &mut W) -> Result<(), Error> {
        writer.writeable(P::from(S as i32))?;
        for element in self.inner {
            writer.writeable(element)?;
        }
        Ok(())
    }
}

impl<P: Readable, T: Readable, const S: usize> Readable for PrefixedArray<P, T, S> {
    fn read<R: Read>(reader: &mut R) -> Result<Self, Error> {
        reader.readable::<P>()?;
        let mut uninit: [MaybeUninit<T>; S] = unsafe { MaybeUninit::uninit().assume_init() };
        for element in uninit.iter_mut().take(S) {
            *element = MaybeUninit::new(reader.readable::<T>()?);
        }
        Ok(Self {
            inner: uninit.map(|uninit| unsafe { uninit.assume_init() }),
            __prefix: Default::default(),
        })
    }
}
