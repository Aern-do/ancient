use std::{
    fmt::Debug,
    io::{self, Read, Write},
};

use aes::{
    cipher::{AsyncStreamCipher, KeyIvInit},
    Aes128,
};
use cfb8::{Decryptor, Encryptor};

type Cfb8Decryptor = Decryptor<Aes128>;
type Cfb8Encryptor = Encryptor<Aes128>;
pub struct Socket<T> {
    inner: T,
    key: Option<[u8; 16]>,
}

impl<T> Socket<T> {
    pub fn new(inner: T) -> Self {
        Self { inner, key: None }
    }
    pub fn enable_encryption(&mut self, key: [u8; 16]) {
        self.key = Some(key);
    }
}

impl<T> Debug for Socket<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("EncryptedStream").finish()
    }
}
impl<T: Read> Read for Socket<T> {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        match self.key {
            Some(ref key) => {
                let decryptor = Cfb8Decryptor::new_from_slices(key, key).unwrap();
                let readed = self.inner.read(buf)?;
                let length = buf.len();
                decryptor.decrypt(&mut buf[(length - readed)..]);
                Ok(length)
            }
            None => self.inner.read(buf),
        }
    }
}
impl<T: Write> Write for Socket<T> {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        match self.key {
            Some(ref key) => {
                let encryptor = Cfb8Encryptor::new_from_slices(key, key).unwrap();
                let mut buf = Vec::from(buf);
                encryptor.encrypt(&mut buf);
                self.inner.write(&buf)
            }
            None => self.inner.write(buf),
        }
    }
    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}
