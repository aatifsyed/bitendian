use crate::ByteOrderedInt;

use super::Endian;
use std::io;

pub trait ReadExt: io::Read {
    fn read_endian<T: ByteOrderedInt>(&mut self, endian: Endian) -> io::Result<T> {
        let mut bytes = T::ByteArray::default();
        self.read_exact(bytes.as_mut())?;
        Ok(T::from_bytes_endian(bytes, endian))
    }
    fn read_be<T: ByteOrderedInt>(&mut self) -> io::Result<T> {
        self.read_endian(Endian::Big)
    }
    fn read_le<T: ByteOrderedInt>(&mut self) -> io::Result<T> {
        self.read_endian(Endian::Little)
    }
    fn read_ne<T: ByteOrderedInt>(&mut self) -> io::Result<T> {
        self.read_endian(Endian::Native)
    }
}
impl<R> ReadExt for R where R: io::Read {}

pub trait WriteExt: io::Write {
    fn write_endian<T: ByteOrderedInt>(&mut self, it: T, endian: Endian) -> io::Result<()> {
        self.write_all(it.to_bytes_endian(endian).as_ref())
    }
    fn write_be<T: ByteOrderedInt>(&mut self, it: T) -> io::Result<()> {
        self.write_endian(it, Endian::Big)
    }
    fn write_le<T: ByteOrderedInt>(&mut self, it: T) -> io::Result<()> {
        self.write_endian(it, Endian::Little)
    }
    fn write_ne<T: ByteOrderedInt>(&mut self, it: T) -> io::Result<()> {
        self.write_endian(it, Endian::Native)
    }
}
impl<W> WriteExt for W where W: io::Write {}
