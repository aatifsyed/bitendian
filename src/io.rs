use crate::{ByteOrder, Endian};
use std::io;

pub trait ReadExt<const N: usize>: io::Read {
    fn read_endian<T: ByteOrder<N>>(&mut self, endian: Endian) -> io::Result<T> {
        let mut bytes = [0u8; N];
        self.read_exact(bytes.as_mut())?;
        Ok(T::from_bytes_endian(bytes, endian))
    }
    fn read_be<T: ByteOrder<N>>(&mut self) -> io::Result<T> {
        self.read_endian(Endian::Big)
    }
    fn read_le<T: ByteOrder<N>>(&mut self) -> io::Result<T> {
        self.read_endian(Endian::Little)
    }
    fn read_ne<T: ByteOrder<N>>(&mut self) -> io::Result<T> {
        self.read_endian(Endian::Native)
    }
}
impl<const N: usize, R> ReadExt<N> for R where R: io::Read {}

pub trait WriteExt<const N: usize>: io::Write {
    fn write_endian<T: ByteOrder<N>>(&mut self, it: T, endian: Endian) -> io::Result<()> {
        self.write_all(it.to_bytes_endian(endian).as_ref())
    }
    fn write_be<T: ByteOrder<N>>(&mut self, it: T) -> io::Result<()> {
        self.write_endian(it, Endian::Big)
    }
    fn write_le<T: ByteOrder<N>>(&mut self, it: T) -> io::Result<()> {
        self.write_endian(it, Endian::Little)
    }
    fn write_ne<T: ByteOrder<N>>(&mut self, it: T) -> io::Result<()> {
        self.write_endian(it, Endian::Native)
    }
}
impl<const N: usize, W> WriteExt<N> for W where W: io::Write {}
