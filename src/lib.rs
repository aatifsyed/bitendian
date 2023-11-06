pub mod io;

use core::{
    borrow::{Borrow, BorrowMut},
    hash::Hash,
    ops::{Index, IndexMut},
};

pub trait ByteOrderedInt<const N: usize> {
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

macro_rules! byte_ordered_int {
    ($($width:literal { $($ty:ty),* $(,)? }),* $(,)?) => {
        $( // each width
            $( // each type
                impl ByteOrderedInt<$width> for $ty {
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
byte_ordered_int!(1{u8, i8});

// byte_ordered_int!(u8, u16, u32, u64, u128, usize);
// byte_ordered_int!(i8, i16, i32, i64, i128, isize);

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord, Default)]
pub enum Endian {
    Little,
    #[doc(alias = "Network")]
    Big,
    #[default]
    Native,
}
