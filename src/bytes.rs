// SPDX-FileCopyrightText: 2026 numlike contributors
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use core::mem::size_of;

/// Returns the memory representation of this value as a byte array in
/// big-endian (network) byte order.
pub trait ToBeBytes {
    type Bytes;

    /// Returns the memory representation of this number as a byte array in
    /// big-endian (network) byte order.
    fn to_be_bytes(self) -> Self::Bytes;
}

/// Returns the memory representation of this value as a byte array in
/// little-endian byte order.
pub trait ToLeBytes {
    type Bytes;

    /// Returns the memory representation of this number as a byte array in
    /// little-endian byte order.
    fn to_le_bytes(self) -> Self::Bytes;
}

/// Returns the memory representation of this value as a byte array in native
/// byte order.
///
/// As the target platform’s native endianness is used, portable code should use
/// [`to_be_bytes`] or [`to_le_bytes`], as appropriate, instead.
pub trait ToNeBytes {
    type Bytes;

    /// Returns the memory representation of this number as a byte array in
    /// native byte order.
    fn to_ne_bytes(self) -> Self::Bytes;
}

/// Creates a value from its representation as a byte array in big-endian byte
/// order.
pub trait FromBeBytes {
    type Bytes;

    /// Creates a value from its representation as a byte array in big-endian
    /// byte order.
    fn from_be_bytes(bytes: Self::Bytes) -> Self;
}

/// Creates a value from its representation as a byte array in little-endian
/// byte order.
pub trait FromLeBytes {
    type Bytes;

    /// Creates a value from its representation as a byte array in little-endian
    /// byte order.
    fn from_le_bytes(bytes: Self::Bytes) -> Self;
}

/// Creates a value from its representation as a byte array in native byte
/// order.
///
/// As the target platform’s native endianness is used, portable code should use
/// [`from_be_bytes`] or [`from_le_bytes`], as appropriate, instead.
pub trait FromNeBytes {
    type Bytes;

    /// Creates a value from its representation as a byte array in native byte
    /// order.
    ///
    /// As the target platform’s native endianness is used, portable code should
    /// use [`from_be_bytes`] or [`from_le_bytes`], as appropriate, instead.
    fn from_ne_bytes(bytes: Self::Bytes) -> Self;
}

macro_rules! impl_bytes_traits {
    ($($ty:ty),*) => {
        $(
            impl ToBeBytes for $ty {
                type Bytes = [u8; size_of::<$ty>()];

                #[inline]
                fn to_be_bytes(self) -> Self::Bytes {
                    <$ty>::to_be_bytes(self)
                }
            }

            impl ToLeBytes for $ty {
                type Bytes = [u8; size_of::<$ty>()];

                #[inline]
                fn to_le_bytes(self) -> Self::Bytes {
                    <$ty>::to_le_bytes(self)
                }
            }

            impl ToNeBytes for $ty {
                type Bytes = [u8; size_of::<$ty>()];

                #[inline]
                fn to_ne_bytes(self) -> Self::Bytes {
                    <$ty>::to_ne_bytes(self)
                }
            }

            impl FromBeBytes for $ty {
                type Bytes = [u8; size_of::<$ty>()];

                #[inline]
                fn from_be_bytes(bytes: Self::Bytes) -> Self {
                    <$ty>::from_be_bytes(bytes)
                }
            }

            impl FromLeBytes for $ty {
                type Bytes = [u8; size_of::<$ty>()];

                #[inline]
                fn from_le_bytes(bytes: Self::Bytes) -> Self {
                    <$ty>::from_le_bytes(bytes)
                }
            }

            impl FromNeBytes for $ty {
                type Bytes = [u8; size_of::<$ty>()];

                #[inline]
                fn from_ne_bytes(bytes: Self::Bytes) -> Self {
                    <$ty>::from_ne_bytes(bytes)
                }
            }
        )*
    };
}

impl_bytes_traits!(i8, i16, i32, i64, i128, isize);
impl_bytes_traits!(u8, u16, u32, u64, u128, usize);
impl_bytes_traits!(f32, f64);
