//! Extension methods for standard library IO.
//!
//! ```
//! use bitendian::io::{ReadExt as _, WriteExt as _};
//!
//! # fn doit() -> std::io::Result<()> {
//! let mut buf = vec![];
//! buf.write_be(1u16)?;
//! let swapped = buf.as_slice().read_le()?;
//! assert_eq!(256u16, swapped);
//! # Ok(())
//! # }
//! # doit().unwrap()
//! ```

use crate::{BitEndian, Endian};
use std::io;

/// Extends [`std::io::Read`] with methods for reading in an endian-dependant way.
///
/// See [module docs](mod@self) for usage examples.
pub trait ReadExt<const N: usize>: io::Read {
    /// Read according to a run-time endianness.
    fn read_endian<T: BitEndian<N>>(&mut self, endian: Endian) -> io::Result<T> {
        let mut bytes = [0u8; N];
        self.read_exact(bytes.as_mut())?;
        Ok(T::from_bytes_endian(bytes, endian))
    }
    /// Read with [`Endian::Big`].
    fn read_be<T: BitEndian<N>>(&mut self) -> io::Result<T> {
        self.read_endian(Endian::Big)
    }
    /// Read with [`Endian::Little`].
    fn read_le<T: BitEndian<N>>(&mut self) -> io::Result<T> {
        self.read_endian(Endian::Little)
    }
    /// Read with [`Endian::Native`].
    fn read_ne<T: BitEndian<N>>(&mut self) -> io::Result<T> {
        self.read_endian(Endian::Native)
    }
}
impl<const N: usize, R> ReadExt<N> for R where R: io::Read {}

/// Extends [`std::io::Write`] with methods for writing in an endian-dependent way.
///
/// See [module docs](mod@self) for usage examples.
pub trait WriteExt<const N: usize>: io::Write {
    /// Write according to a run-time endianness.
    fn write_endian<T: BitEndian<N>>(&mut self, it: T, endian: Endian) -> io::Result<()> {
        self.write_all(it.to_bytes_endian(endian).as_ref())
    }
    /// Write with [`Endian::Big`].
    fn write_be<T: BitEndian<N>>(&mut self, it: T) -> io::Result<()> {
        self.write_endian(it, Endian::Big)
    }
    /// Write with [`Endian::Little`].
    fn write_le<T: BitEndian<N>>(&mut self, it: T) -> io::Result<()> {
        self.write_endian(it, Endian::Little)
    }
    /// Write with [`Endian::Native`].
    fn write_ne<T: BitEndian<N>>(&mut self, it: T) -> io::Result<()> {
        self.write_endian(it, Endian::Native)
    }
}
impl<const N: usize, W> WriteExt<N> for W where W: io::Write {}
