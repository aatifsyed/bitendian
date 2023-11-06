//! Convenience methods for encoding and decoding numbers in either big-endian
//! or little-endian order.
//!
//! # Comparison with [`byteorder`](::byteorder).
//! - This crate leverages type inference to [avoid defining dozens of write_uXX methods](::byteorder::WriteBytesExt).
//! - This crate provides run-time endianness.
//! - This crate only supports rust's built-in types, not, eg. [`u24`](::byteorder::WriteBytesExt::write_u24).

pub mod io;

/// A type that can be infallibly written to or read from an array in an
/// [endian](Endian)-dependent manner.
///
/// See the [module documentation](mod@self) for usage examples.
pub trait ByteOrder<const N: usize> {
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
            Endian::Big => self.to_be_bytes(),
            Endian::Native => self.to_ne_bytes(),
        }
    }

    /// Create a native endian integer value from its representation
    /// as a byte array in little endian.
    fn from_le_bytes(bytes: [u8; N]) -> Self;
    /// Create a native endian integer value from its representation
    /// as a byte array in big endian.
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
            Endian::Big => Self::from_be_bytes(bytes),
            Endian::Native => Self::from_ne_bytes(bytes),
        }
    }
}

macro_rules! byte_order {
    ($($width:literal { $($ty:ty),* $(,)? }),* $(,)?) => {
        $( // each width
            $( // each type
                impl ByteOrder<$width> for $ty {
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
byte_order!(
    1 { u8, i8 },
    2 { u16, i16 },
    4 { u32, i32, f32 },
    8 { u64, i64, f64 },
    16 { u128, i128 },
);

#[cfg(target_pointer_width = "8")]
byte_order!(1 { usize, isize });
#[cfg(target_pointer_width = "16")]
byte_order!(2 { usize, isize });
#[cfg(target_pointer_width = "32")]
byte_order!(4 { usize, isize });
#[cfg(target_pointer_width = "64")]
byte_order!(8 { usize, isize });
#[cfg(target_pointer_width = "128")]
byte_order!(16 { usize, isize });

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord, Default)]
pub enum Endian {
    Little,
    #[doc(alias = "Network")]
    Big,
    #[default]
    Native,
}
