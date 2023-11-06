use crate::ByteOrder;

use super::Endian;
use std::io;

pub trait ReadExt: io::Read {
    fn read_endian<const N: usize, T: ByteOrder<N>>(&mut self, endian: Endian) -> io::Result<T> {
        let mut bytes = [0u8; N];
        self.read_exact(bytes.as_mut())?;
        Ok(T::from_bytes_endian(bytes, endian))
    }
    fn read_be<const N: usize, T: ByteOrder<N>>(&mut self) -> io::Result<T> {
        self.read_endian(Endian::Big)
    }
    fn read_le<const N: usize, T: ByteOrder<N>>(&mut self) -> io::Result<T> {
        self.read_endian(Endian::Little)
    }
    fn read_ne<const N: usize, T: ByteOrder<N>>(&mut self) -> io::Result<T> {
        self.read_endian(Endian::Native)
    }
}
impl<R> ReadExt for R where R: io::Read {}

pub trait WriteExt: io::Write {
    fn write_endian<const N: usize, T: ByteOrder<N>>(
        &mut self,
        it: T,
        endian: Endian,
    ) -> io::Result<()> {
        self.write_all(it.to_bytes_endian(endian).as_ref())
    }
    fn write_be<const N: usize, T: ByteOrder<N>>(&mut self, it: T) -> io::Result<()> {
        self.write_endian(it, Endian::Big)
    }
    fn write_le<const N: usize, T: ByteOrder<N>>(&mut self, it: T) -> io::Result<()> {
        self.write_endian(it, Endian::Little)
    }
    fn write_ne<const N: usize, T: ByteOrder<N>>(&mut self, it: T) -> io::Result<()> {
        self.write_endian(it, Endian::Native)
    }
}
impl<W> WriteExt for W where W: io::Write {}
