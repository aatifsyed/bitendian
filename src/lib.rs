pub mod io;

use core::{
    borrow::{Borrow, BorrowMut},
    hash::Hash,
    ops::{Index, IndexMut},
};

pub trait ByteOrderedInt {
    type ByteArray: ByteArray;

    /// Return the memory representation of this integer as a byte array in
    /// little-endian byte order.
    fn to_le_bytes(self) -> Self::ByteArray;
    /// Return the memory representation of this integer as a byte array in
    /// big-endian (network) byte order.
    fn to_be_bytes(self) -> Self::ByteArray;
    /// Return the memory representation of this integer as a byte array in
    /// native byte order.
    ///
    /// As the target platform's native endianness is used, portable code
    /// should use [`Self::to_be_bytes`] or [`Self::to_le_bytes`], as appropriate,
    /// instead.
    fn to_ne_bytes(self) -> Self::ByteArray;

    /// Delegates to the appropriate method according to a run-time endianness.
    fn to_bytes_endian(self, endian: Endian) -> Self::ByteArray
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
    fn from_le_bytes(bytes: Self::ByteArray) -> Self;
    /// Create a native endian integer value from its representation
    /// as a byte array in big endian.
    fn from_be_bytes(bytes: Self::ByteArray) -> Self;
    /// Create a native endian integer value from its memory representation
    /// as a byte array in native endianness.
    ///
    /// As the target platform's native endianness is used, portable code
    /// likely wants to use [`Self::from_be_bytes`] or [`Self::from_le_bytes`], as
    /// appropriate instead.
    fn from_ne_bytes(bytes: Self::ByteArray) -> Self;

    /// Delegates to the appropriate method according to a run-time endianness.
    fn from_bytes_endian(bytes: Self::ByteArray, endian: Endian) -> Self
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
    ($($ty:ty),* $(,)?) => {
        $(
            impl ByteOrderedInt for $ty {
                type ByteArray = [u8; { core::mem::size_of::<$ty>() }];
                fn to_le_bytes(self) -> Self::ByteArray {
                    <$ty>::to_le_bytes(self)
                }
                fn to_be_bytes(self) -> Self::ByteArray {
                    <$ty>::to_be_bytes(self)
                }
                fn to_ne_bytes(self) -> Self::ByteArray {
                    <$ty>::to_ne_bytes(self)
                }

                fn from_le_bytes(bytes: Self::ByteArray) -> Self {
                    <$ty>::from_le_bytes(bytes)
                }
                fn from_be_bytes(bytes: Self::ByteArray) -> Self {
                    <$ty>::from_be_bytes(bytes)
                }
                fn from_ne_bytes(bytes: Self::ByteArray) -> Self {
                    <$ty>::from_ne_bytes(bytes)
                }
            }
        )*
    };
}

byte_order!(u8, u16, u32, u64, u128, usize);
byte_order!(i8, i16, i32, i64, i128, isize);

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord, Default)]
pub enum Endian {
    Little,
    #[doc(alias = "Network")]
    Big,
    #[default]
    Native,
}

pub trait ByteArray
where
    Self: IntoIterator<Item = u8>,
    // Self::IntoIter: ExactSizeIterator,
    Self: Copy + Hash + Default,
    Self: PartialEq + Eq + PartialEq<[u8]>,
    Self: PartialOrd + Ord,
    Self: AsRef<[u8]> + AsMut<[u8]>,
    Self: Borrow<[u8]> + BorrowMut<[u8]>,
    Self: Index<usize> + IndexMut<usize>,
    Self: for<'a> TryFrom<&'a [u8]>,
    Self: for<'a> TryFrom<&'a mut [u8]>,
{
    const WIDTH: usize;
}

impl<const N: usize> ByteArray for [u8; N]
where
    Self: Default,
{
    const WIDTH: usize = N;
}
