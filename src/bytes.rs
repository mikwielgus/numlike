// SPDX-FileCopyrightText: 2026 numlike contributors
//
// SPDX-License-Identifier: MIT OR Apache-2.0

//! Byte array conversions for numeric values.

use core::mem::size_of;

/// Returns the memory representation of this value as a byte array in
/// big-endian (network) byte order.
pub trait ToBeBytes {
    /// The resulting type after applying the operation.
    type Output;

    /// Returns the memory representation of this number as a byte array in
    /// big-endian (network) byte order.
    fn to_be_bytes(self) -> Self::Output;
}

/// Returns the memory representation of this value as a byte array in
/// little-endian byte order.
pub trait ToLeBytes {
    /// The resulting type after applying the operation.
    type Output;

    /// Returns the memory representation of this number as a byte array in
    /// little-endian byte order.
    fn to_le_bytes(self) -> Self::Output;
}

/// Returns the memory representation of this value as a byte array in native
/// byte order.
///
/// As the target platform’s native endianness is used, portable code should use
/// [`to_be_bytes`] or [`to_le_bytes`], as appropriate, instead.
pub trait ToNeBytes {
    /// The resulting type after applying the operation.
    type Output;

    /// Returns the memory representation of this number as a byte array in
    /// native byte order.
    fn to_ne_bytes(self) -> Self::Output;
}

/// Creates a value from its representation as a byte array in big-endian byte
/// order.
pub trait FromBeBytes {
    /// The resulting type after applying the operation.
    type Output;

    /// Creates a value from its representation as a byte array in big-endian
    /// byte order.
    fn from_be_bytes(bytes: Self::Output) -> Self;
}

/// Creates a value from its representation as a byte array in little-endian
/// byte order.
pub trait FromLeBytes {
    /// The resulting type after applying the operation.
    type Output;

    /// Creates a value from its representation as a byte array in little-endian
    /// byte order.
    fn from_le_bytes(bytes: Self::Output) -> Self;
}

/// Creates a value from its representation as a byte array in native byte
/// order.
///
/// As the target platform’s native endianness is used, portable code should use
/// [`from_be_bytes`] or [`from_le_bytes`], as appropriate, instead.
pub trait FromNeBytes {
    /// The resulting type after applying the operation.
    type Output;

    /// Creates a value from its representation as a byte array in native byte
    /// order.
    ///
    /// As the target platform’s native endianness is used, portable code should
    /// use [`from_be_bytes`] or [`from_le_bytes`], as appropriate, instead.
    fn from_ne_bytes(bytes: Self::Output) -> Self;
}

macro_rules! impl_bytes_traits {
    ($ty:ty) => {
        impl ToBeBytes for $ty {
            type Output = [u8; size_of::<$ty>()];

            #[inline]
            fn to_be_bytes(self) -> Self::Output {
                <$ty>::to_be_bytes(self)
            }
        }

        impl ToLeBytes for $ty {
            type Output = [u8; size_of::<$ty>()];

            #[inline]
            fn to_le_bytes(self) -> Self::Output {
                <$ty>::to_le_bytes(self)
            }
        }

        impl ToNeBytes for $ty {
            type Output = [u8; size_of::<$ty>()];

            #[inline]
            fn to_ne_bytes(self) -> Self::Output {
                <$ty>::to_ne_bytes(self)
            }
        }

        impl FromBeBytes for $ty {
            type Output = [u8; size_of::<$ty>()];

            #[inline]
            fn from_be_bytes(bytes: Self::Output) -> Self {
                <$ty>::from_be_bytes(bytes)
            }
        }

        impl FromLeBytes for $ty {
            type Output = [u8; size_of::<$ty>()];

            #[inline]
            fn from_le_bytes(bytes: Self::Output) -> Self {
                <$ty>::from_le_bytes(bytes)
            }
        }

        impl FromNeBytes for $ty {
            type Output = [u8; size_of::<$ty>()];

            #[inline]
            fn from_ne_bytes(bytes: Self::Output) -> Self {
                <$ty>::from_ne_bytes(bytes)
            }
        }
    };
    ($ty:ty, $nonnegative_tests_mod:ident) => {
        impl_bytes_traits!($ty);

        test_bytes_traits_nonnegative!($ty, $nonnegative_tests_mod);
    };
    ($ty:ty, $nonnegative_tests_mod:ident, $negative_tests_mod:ident) => {
        impl_bytes_traits!($ty, $nonnegative_tests_mod);

        test_bytes_traits_negative!($ty, $negative_tests_mod);
    };
}

macro_rules! test_bytes_traits_nonnegative {
    ($ty:ty, $tests_mod:ident) => {
        #[cfg(test)]
        mod $tests_mod {
            use crate::bytes::*;
            use crate::elem::*;

            #[test]
            fn test_roundtrip() {
                let zero = <$ty as Zero>::ZERO;
                let one = <$ty as One>::ONE;
                let two = one + one;
                let four = two + two;

                assert_eq!(
                    <$ty as FromBeBytes>::from_be_bytes(<$ty as ToBeBytes>::to_be_bytes(zero)),
                    zero
                );
                assert_eq!(
                    <$ty as FromBeBytes>::from_be_bytes(<$ty as ToBeBytes>::to_be_bytes(one)),
                    one
                );
                assert_eq!(
                    <$ty as FromBeBytes>::from_be_bytes(<$ty as ToBeBytes>::to_be_bytes(two)),
                    two
                );
                assert_eq!(
                    <$ty as FromBeBytes>::from_be_bytes(<$ty as ToBeBytes>::to_be_bytes(four)),
                    four
                );

                assert_eq!(
                    <$ty as FromLeBytes>::from_le_bytes(<$ty as ToLeBytes>::to_le_bytes(zero)),
                    zero
                );
                assert_eq!(
                    <$ty as FromLeBytes>::from_le_bytes(<$ty as ToLeBytes>::to_le_bytes(one)),
                    one
                );
                assert_eq!(
                    <$ty as FromLeBytes>::from_le_bytes(<$ty as ToLeBytes>::to_le_bytes(two)),
                    two
                );
                assert_eq!(
                    <$ty as FromLeBytes>::from_le_bytes(<$ty as ToLeBytes>::to_le_bytes(four)),
                    four
                );

                assert_eq!(
                    <$ty as FromNeBytes>::from_ne_bytes(<$ty as ToNeBytes>::to_ne_bytes(zero)),
                    zero
                );
                assert_eq!(
                    <$ty as FromNeBytes>::from_ne_bytes(<$ty as ToNeBytes>::to_ne_bytes(one)),
                    one
                );
                assert_eq!(
                    <$ty as FromNeBytes>::from_ne_bytes(<$ty as ToNeBytes>::to_ne_bytes(two)),
                    two
                );
                assert_eq!(
                    <$ty as FromNeBytes>::from_ne_bytes(<$ty as ToNeBytes>::to_ne_bytes(four)),
                    four
                );
            }

            #[test]
            fn test_be_to_le() {
                let one = <$ty as One>::ONE;
                let two = one + one;
                let four = two + two;

                let mut be_one = <$ty as ToBeBytes>::to_be_bytes(one);
                let mut be_two = <$ty as ToBeBytes>::to_be_bytes(two);
                let mut be_four = <$ty as ToBeBytes>::to_be_bytes(four);

                be_one.reverse();
                be_two.reverse();
                be_four.reverse();

                assert_eq!(be_one, <$ty as ToLeBytes>::to_le_bytes(one));
                assert_eq!(be_two, <$ty as ToLeBytes>::to_le_bytes(two));
                assert_eq!(be_four, <$ty as ToLeBytes>::to_le_bytes(four));
            }
        }
    };
}

macro_rules! test_bytes_traits_negative {
    ($ty:ty, $tests_mod:ident) => {
        #[cfg(test)]
        mod $tests_mod {
            use crate::bytes::*;
            use crate::elem::*;

            #[test]
            fn test_roundtrip() {
                let one = <$ty as One>::ONE;
                let two = one + one;
                let four = two + two;

                assert_eq!(
                    <$ty as FromBeBytes>::from_be_bytes(<$ty as ToBeBytes>::to_be_bytes(-one)),
                    -one
                );
                assert_eq!(
                    <$ty as FromBeBytes>::from_be_bytes(<$ty as ToBeBytes>::to_be_bytes(-two)),
                    -two
                );
                assert_eq!(
                    <$ty as FromBeBytes>::from_be_bytes(<$ty as ToBeBytes>::to_be_bytes(-four)),
                    -four
                );

                assert_eq!(
                    <$ty as FromLeBytes>::from_le_bytes(<$ty as ToLeBytes>::to_le_bytes(-one)),
                    -one
                );
                assert_eq!(
                    <$ty as FromLeBytes>::from_le_bytes(<$ty as ToLeBytes>::to_le_bytes(-two)),
                    -two
                );
                assert_eq!(
                    <$ty as FromLeBytes>::from_le_bytes(<$ty as ToLeBytes>::to_le_bytes(-four)),
                    -four
                );

                assert_eq!(
                    <$ty as FromNeBytes>::from_ne_bytes(<$ty as ToNeBytes>::to_ne_bytes(-one)),
                    -one
                );
                assert_eq!(
                    <$ty as FromNeBytes>::from_ne_bytes(<$ty as ToNeBytes>::to_ne_bytes(-two)),
                    -two
                );
                assert_eq!(
                    <$ty as FromNeBytes>::from_ne_bytes(<$ty as ToNeBytes>::to_ne_bytes(-four)),
                    -four
                );
            }

            #[test]
            fn test_be_to_le() {
                let one = <$ty as One>::ONE;
                let two = one + one;
                let four = two + two;

                let mut be_one = <$ty as ToBeBytes>::to_be_bytes(-one);
                let mut be_two = <$ty as ToBeBytes>::to_be_bytes(-two);
                let mut be_four = <$ty as ToBeBytes>::to_be_bytes(-four);

                be_one.reverse();
                be_two.reverse();
                be_four.reverse();

                assert_eq!(be_one, <$ty as ToLeBytes>::to_le_bytes(-one));
                assert_eq!(be_two, <$ty as ToLeBytes>::to_le_bytes(-two));
                assert_eq!(be_four, <$ty as ToLeBytes>::to_le_bytes(-four));
            }
        }
    };
}

impl_bytes_traits!(i8, i8_nonnegative_tests, i8_negative_tests);
impl_bytes_traits!(i16, i16_nonnegative_tests, i16_negative_tests);
impl_bytes_traits!(i32, i32_nonnegative_tests, i32_negative_tests);
impl_bytes_traits!(i64, i64_nonnegative_tests, i64_negative_tests);
impl_bytes_traits!(i128, i128_nonnegative_tests, i128_negative_tests);
impl_bytes_traits!(isize, isize_nonnegative_tests, isize_negative_tests);

impl_bytes_traits!(u8, u8_tests);
impl_bytes_traits!(u16, u16_tests);
impl_bytes_traits!(u32, u32_tests);
impl_bytes_traits!(u64, u64_tests);
impl_bytes_traits!(u128, u128_tests);
impl_bytes_traits!(usize, usize_tests);

impl_bytes_traits!(f32, f32_nonnegative_tests, f32_negative_tests);
impl_bytes_traits!(f64, f64_nonnegative_tests, f64_negative_tests);
