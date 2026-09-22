// SPDX-FileCopyrightText: 2026 numlike contributors
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use core::num::{Saturating, Wrapping};

/// Bundle of checked bitwise shift operations.
pub trait CheckedBitOps: CheckedShl + CheckedShr {}
impl<T: CheckedShl + CheckedShr> CheckedBitOps for T {}

/// Checked shift left. Computes `self << rhs`, returning `None` if `rhs` is larger
/// than or equal to the number of bits in `self`.
pub trait CheckedShl {
    /// The resulting type after applying the operation.
    type Output;

    /// Checked shift left. Computes `self << rhs`, returning `None` if `rhs` is larger
    /// than or equal to the number of bits in `self`.
    fn checked_shl(self, rhs: u32) -> Option<Self::Output>;
}

/// Checked shift right. Computes `self >> rhs`, returning `None` if `rhs` is
/// larger than or equal to the number of bits in `self`.
pub trait CheckedShr {
    /// The resulting type after applying the operation.
    type Output;

    /// Checked shift right. Computes `self >> rhs`, returning `None` if `rhs` is
    /// larger than or equal to the number of bits in `self`.
    fn checked_shr(self, rhs: u32) -> Option<Self::Output>;
}

macro_rules! impl_checked_shift_traits_for_ints {
    ($($ty:ty),*) => {
        $(
            impl CheckedShl for $ty {
                type Output = $ty;

                #[inline]
                fn checked_shl(self, rhs: u32) -> Option<Self::Output> {
                    <$ty>::checked_shl(self, rhs)
                }
            }

            impl CheckedShr for $ty {
                type Output = $ty;

                #[inline]
                fn checked_shr(self, rhs: u32) -> Option<Self::Output> {
                    <$ty>::checked_shr(self, rhs)
                }
            }
        )*
    };
}

macro_rules! impl_checked_shift_traits_via_inner {
    ($($ty:ty),*) => {
        $(
            impl CheckedShl for $ty {
                type Output = $ty;

                #[inline]
                fn checked_shl(self, rhs: u32) -> Option<Self::Output> {
                    self.0.checked_shl(rhs).map(Self)
                }
            }

            impl CheckedShr for $ty {
                type Output = $ty;

                #[inline]
                fn checked_shr(self, rhs: u32) -> Option<Self::Output> {
                    self.0.checked_shr(rhs).map(Self)
                }
            }
        )*
    };
}

impl_checked_shift_traits_for_ints!(i8, i16, i32, i64, i128, isize);
impl_checked_shift_traits_for_ints!(u8, u16, u32, u64, u128, usize);
impl_checked_shift_traits_via_inner!(
    Wrapping<i8>,
    Wrapping<i16>,
    Wrapping<i32>,
    Wrapping<i64>,
    Wrapping<i128>,
    Wrapping<isize>,
    Wrapping<u8>,
    Wrapping<u16>,
    Wrapping<u32>,
    Wrapping<u64>,
    Wrapping<u128>,
    Wrapping<usize>
);
impl_checked_shift_traits_via_inner!(
    Saturating<i8>,
    Saturating<i16>,
    Saturating<i32>,
    Saturating<i64>,
    Saturating<i128>,
    Saturating<isize>,
    Saturating<u8>,
    Saturating<u16>,
    Saturating<u32>,
    Saturating<u64>,
    Saturating<u128>,
    Saturating<usize>
);

macro_rules! test_checked_bitshift_traits_int_nonnegative {
    ($ty:ty, $tests_mod:ident) => {
        #[cfg(test)]
        mod $tests_mod {
            use crate::elem::*;
            use crate::ops::*;

            #[test]
            fn test_shl() {
                let zero = <$ty as Zero>::ZERO;
                let one = <$ty as One>::ONE;
                let two = one + one;
                let four = two + two;

                assert_eq!(CheckedShl::checked_shl(zero, 1), Some(zero));
                assert_eq!(CheckedShl::checked_shl(one, 1), Some(two));
                assert_eq!(CheckedShl::checked_shl(two, 1), Some(four));
                assert_eq!(CheckedShl::checked_shl(one, 2), Some(four));
                assert_eq!(CheckedShl::checked_shl(one, <$ty>::BITS), None);
            }

            #[test]
            fn test_shr() {
                let zero = <$ty as Zero>::ZERO;
                let one = <$ty as One>::ONE;
                let two = one + one;
                let four = two + two;

                assert_eq!(CheckedShr::checked_shr(zero, 1), Some(zero));
                assert_eq!(CheckedShr::checked_shr(one, 1), Some(zero));
                assert_eq!(CheckedShr::checked_shr(two, 1), Some(one));
                assert_eq!(CheckedShr::checked_shr(four, 1), Some(two));
                assert_eq!(CheckedShr::checked_shr(four, 2), Some(one));
                assert_eq!(CheckedShr::checked_shr(one, <$ty>::BITS), None);
            }
        }
    };
}

macro_rules! test_checked_bitshift_traits_int_negative {
    ($ty:ty, $tests_mod:ident) => {
        #[cfg(test)]
        mod $tests_mod {
            use crate::elem::*;
            use crate::ops::*;

            #[test]
            fn test_shl() {
                let one = <$ty as One>::ONE;
                let two = one + one;
                let four = two + two;

                assert_eq!(CheckedShl::checked_shl(-one, 1), Some(-two));
                assert_eq!(CheckedShl::checked_shl(-two, 1), Some(-four));
                assert_eq!(CheckedShl::checked_shl(-one, <$ty>::BITS), None);
            }

            #[test]
            fn test_shr() {
                let one = <$ty as One>::ONE;
                let two = one + one;
                let four = two + two;

                assert_eq!(CheckedShr::checked_shr(-two, 1), Some(-one));
                assert_eq!(CheckedShr::checked_shr(-four, 1), Some(-two));
                assert_eq!(CheckedShr::checked_shr(-four, 2), Some(-one));
                assert_eq!(CheckedShr::checked_shr(-one, <$ty>::BITS), None);
            }
        }
    };
}

test_checked_bitshift_traits_int_nonnegative!(i8, i8_nonnegative_tests);
test_checked_bitshift_traits_int_negative!(i8, i8_negative_tests);
test_checked_bitshift_traits_int_nonnegative!(i16, i16_nonnegative_tests);
test_checked_bitshift_traits_int_negative!(i16, i16_negative_tests);
test_checked_bitshift_traits_int_nonnegative!(i32, i32_nonnegative_tests);
test_checked_bitshift_traits_int_negative!(i32, i32_negative_tests);
test_checked_bitshift_traits_int_nonnegative!(i64, i64_nonnegative_tests);
test_checked_bitshift_traits_int_negative!(i64, i64_negative_tests);
test_checked_bitshift_traits_int_nonnegative!(i128, i128_nonnegative_tests);
test_checked_bitshift_traits_int_negative!(i128, i128_negative_tests);
test_checked_bitshift_traits_int_nonnegative!(isize, isize_nonnegative_tests);
test_checked_bitshift_traits_int_negative!(isize, isize_negative_tests);

test_checked_bitshift_traits_int_nonnegative!(u8, u8_tests);
test_checked_bitshift_traits_int_nonnegative!(u16, u16_tests);
test_checked_bitshift_traits_int_nonnegative!(u32, u32_tests);
test_checked_bitshift_traits_int_nonnegative!(u64, u64_tests);
test_checked_bitshift_traits_int_nonnegative!(u128, u128_tests);
test_checked_bitshift_traits_int_nonnegative!(usize, usize_tests);
