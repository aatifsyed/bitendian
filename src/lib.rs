//! Convenience methods for encoding and decoding numbers in either big-endian
//! or little-endian.
//!
//! Primitive integers implement [`BitEndian`](crate::BitEndian).
//! ```
//! use bitendian::BitEndian;
//!
//! let it: u16 = 256;
//! assert_eq!(BitEndian::to_be_bytes(it), [1, 0]);
//! assert_eq!(BitEndian::to_le_bytes(it), [0, 1]);
//! ```
//!
//! Extension methods provide convenient readers and writers.
//! ```
//! use bitendian::{io::WriteExt as _, tokio::AsyncReadExt as _};
//!
//! # async fn doit() -> std::io::Result<()> {
//! let mut buf = vec![];
//! buf.write_be(1u16)?;
//! let swapped = buf.as_slice().read_le().await?;
//! assert_eq!(256u16, swapped);
//! # Ok(())
//! # }
//! # futures::executor::block_on(doit()).unwrap();
//! ```
//!
//! # Comparison with [`byteorder`].
//! - This crate leverages type inference to avoid [defining dozens of e.g write_uXX methods].
//!   ```
//!   use byteorder::{ReadBytesExt as _, BE, LE};
//!   use bitendian::io::ReadExt as _;
//!   use std::io;
//!
//!   # struct Header {
//!   #     count: u16,
//!   #     offset: i32,
//!   # }
//!   fn read_header(mut r: impl io::Read) -> io::Result<Header> {
//!   # let _: io::Result<_> =
//!       // before...
//!       Ok(Header {
//!           count: r.read_u16::<BE>()?,
//!                      // ^ this can be inferred
//!           offset: r.read_i32::<LE>()?
//!                             // ^ this could be a plain method
//!       })
//!   # ;
//!       // after
//!       Ok(Header {
//!           count: r.read_be()?,
//!           offset: r.read_le()?,
//!       })
//!   }
//!   ```
//! - This crate supports run-time endianness.
//! - This crate supports [`futures::io`] and [`tokio::io`] via the `futures`
//!   and `tokio` features respectively.
//! - This crate only supports rust's built-in types, not, eg. [`u24`].
//! - Both crates support `#![no_std]` by disabling the default `std` feature.
//!
//! [`byteorder`]: https://docs.rs/byteorder/1/byteorder/index.html
//! [defining dozens of e.g write_uXX methods]: https://docs.rs/byteorder/1/byteorder/trait.WriteBytesExt.html#method.write_u8
//! [`u24`]: https://docs.rs/byteorder/1/byteorder/trait.WriteBytesExt.html#method.write_u24
//! [`futures::io`]: https://docs.rs/futures/0.3/futures/io/
//! [`tokio::io`]: https://docs.rs/tokio/1/tokio/io/index.html

#![cfg_attr(do_doc_cfg, feature(doc_cfg))]
#![cfg_attr(not(feature = "std"), no_std)]
#![allow(rustdoc::redundant_explicit_links)] // required for `cargo-rdme`

#[cfg(feature = "futures")]
#[cfg_attr(do_doc_cfg, doc(cfg(feature = "futures")))]
pub mod futures;
#[cfg(feature = "std")]
#[cfg_attr(do_doc_cfg, doc(cfg(feature = "std")))]
pub mod io;
#[cfg(feature = "tokio")]
#[cfg_attr(do_doc_cfg, doc(cfg(feature = "tokio")))]
pub mod tokio;

/// A type that can be infallibly written to or read from an array in an
/// [endian](Endian)-dependent manner.
///
/// This trait does not provide [`to_le`](u32::to_le) etc., since they can be
/// found in [`num::Primint`](https://docs.rs/num/0.4/num/trait.PrimInt.html#tymethod.to_le).
///
/// See the [module documentation](mod@self) for usage examples.
pub trait BitEndian<const N: usize> {
    /// Return the memory representation of this integer as a byte array in
    /// little-endian byte order.
    fn to_le_bytes(self) -> [u8; N];
    /// Return the memory representation of this integer as a byte array in
    /// big-endian (network) byte order.
    fn to_be_bytes(self) -> [u8; N];
    /// Return the memory representation of this integer as a byte array in
    /// native byte order.
    ///
    /// As the target platform's native endianness is used, portable code
    /// should use [`Self::to_be_bytes`] or [`Self::to_le_bytes`], as appropriate,
    /// instead.
    fn to_ne_bytes(self) -> [u8; N];

    /// Delegates to the appropriate method according to a run-time endianness.
    fn to_bytes_endian(self, endian: Endian) -> [u8; N]
    where
        Self: Sized,
    {
        match endian {
            Endian::Little => self.to_le_bytes(),
            Endian::Big | Endian::Network => self.to_be_bytes(),
            Endian::Native => self.to_ne_bytes(),
        }
    }

    /// Create a native endian integer value from its representation
    /// as a byte array in little endian.
    fn from_le_bytes(bytes: [u8; N]) -> Self;
    /// Create a native endian integer value from its representation
    /// as a byte array in big (network) endian.
    fn from_be_bytes(bytes: [u8; N]) -> Self;
    /// Create a native endian integer value from its memory representation
    /// as a byte array in native endianness.
    ///
    /// As the target platform's native endianness is used, portable code
    /// likely wants to use [`Self::from_be_bytes`] or [`Self::from_le_bytes`], as
    /// appropriate instead.
    fn from_ne_bytes(bytes: [u8; N]) -> Self;

    /// Delegates to the appropriate method according to a run-time endianness.
    fn from_bytes_endian(bytes: [u8; N], endian: Endian) -> Self
    where
        Self: Sized,
    {
        match endian {
            Endian::Little => Self::from_le_bytes(bytes),
            Endian::Big | Endian::Network => Self::from_be_bytes(bytes),
            Endian::Native => Self::from_ne_bytes(bytes),
        }
    }
}

macro_rules! bit_endian {
    ($($width:literal { $($ty:ty),* $(,)? }),* $(,)?) => {
        $( // each width
            $( // each type
                impl BitEndian<$width> for $ty {
                    fn to_le_bytes(self) -> [u8; $width] {
                        <$ty>::to_le_bytes(self)
                    }
                    fn to_be_bytes(self) -> [u8; $width] {
                        <$ty>::to_be_bytes(self)
                    }
                    fn to_ne_bytes(self) -> [u8; $width] {
                        <$ty>::to_ne_bytes(self)
                    }

                    fn from_le_bytes(bytes: [u8; $width]) -> Self {
                        <$ty>::from_le_bytes(bytes)
                    }
                    fn from_be_bytes(bytes: [u8; $width]) -> Self {
                        <$ty>::from_be_bytes(bytes)
                    }
                    fn from_ne_bytes(bytes: [u8; $width]) -> Self {
                        <$ty>::from_ne_bytes(bytes)
                    }
                }
            )* // each type
        )* // each width
    };
}
bit_endian!(
    1 { u8, i8 },
    2 { u16, i16 },
    4 { u32, i32, f32 },
    8 { u64, i64, f64 },
    16 { u128, i128 },
);

#[cfg(target_pointer_width = "8")]
bit_endian!(1 { usize, isize });
#[cfg(target_pointer_width = "16")]
bit_endian!(2 { usize, isize });
#[cfg(target_pointer_width = "32")]
bit_endian!(4 { usize, isize });
#[cfg(target_pointer_width = "64")]
bit_endian!(8 { usize, isize });
#[cfg(target_pointer_width = "128")]
bit_endian!(16 { usize, isize });

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord, Default)]
pub enum Endian {
    /// Least Significant Byte first.
    Little,
    /// Most Significant Byte first.
    Big,
    /// Conventially used for exchange over a network.
    /// Same as [`Endian::Big`]
    Network,
    /// The endianness of the current processor.
    #[default]
    Native,
}

impl Endian {
    /// Return an [`Endian::Big`] or [`Endian::Little`] accordingly.
    pub fn canonical(self) -> Self {
        match self {
            Endian::Little => Endian::Little,
            Endian::Big | Endian::Network => Endian::Big,
            #[cfg(target_endian = "big")]
            Endian::Native => Endian::Big,
            #[cfg(target_endian = "little")]
            Endian::Native => Endian::Little,
        }
    }
    /// Returns true if [`Endian::Big`], [`Endian::Network`], or if [`Endian::Native`]
    /// and on a big-endian processor.
    pub fn is_big(&self) -> bool {
        matches!(self.canonical(), Endian::Big)
    }
    /// Returns true if [`Endian::Little`], or if [`Endian::Native`]
    /// and on a little-endian processor.
    pub fn is_little(&self) -> bool {
        matches!(self.canonical(), Endian::Little)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn readme() {
        assert!(
            std::process::Command::new("cargo")
                .args(["rdme", "--check"])
                .output()
                .expect("couldn't run `cargo rdme`")
                .status
                .success(),
            "README.md is out of date - bless the new version by running `cargo rdme`"
        )
    }
}
