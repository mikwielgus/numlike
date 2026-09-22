// SPDX-FileCopyrightText: 2026 numlike contributors
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use core::num::{Saturating, Wrapping};

/// Bundle of overflowing bitwise shift operations.
pub trait OverflowingBitOps: OverflowingShl + OverflowingShr {}
impl<T: OverflowingShl + OverflowingShr> OverflowingBitOps for T {}

/// Shifts self left by `rhs` bits.
///
/// Returns a tuple of the shifted version of self along with a boolean indicating whether the shift
/// value was larger than or equal to the number of bits. If the shift value is too large, then value is
/// masked (N-1) where N is the number of bits, and this value is then used to perform the shift.
pub trait OverflowingShl {
    /// The resulting type after applying the operation.
    type Output;

    /// Shifts self left by `rhs` bits.
    fn overflowing_shl(self, rhs: u32) -> (Self::Output, bool);
}

/// Shifts self right by `rhs` bits.
///
/// Returns a tuple of the shifted version of self along with a boolean indicating whether the shift
/// value was larger than or equal to the number of bits. If the shift value is too large, then value is
/// masked (N-1) where N is the number of bits, and this value is then used to perform the shift.
pub trait OverflowingShr {
    /// The resulting type after applying the operation.
    type Output;

    /// Shifts self right by `rhs` bits.
    fn overflowing_shr(self, rhs: u32) -> (Self::Output, bool);
}

macro_rules! impl_overflowing_shift_traits_for_ints {
    ($($ty:ty),*) => {
        $(
            impl OverflowingShl for $ty {
                type Output = $ty;

                #[inline]
                fn overflowing_shl(self, rhs: u32) -> (Self::Output, bool) {
                    <$ty>::overflowing_shl(self, rhs)
                }
            }

            impl OverflowingShr for $ty {
                type Output = $ty;

                #[inline]
                fn overflowing_shr(self, rhs: u32) -> (Self::Output, bool) {
                    <$ty>::overflowing_shr(self, rhs)
                }
            }
        )*
    };
}

macro_rules! impl_overflowing_shift_traits_via_inner {
    ($($ty:ty),*) => {
        $(
            impl OverflowingShl for $ty {
                type Output = $ty;

                #[inline]
                fn overflowing_shl(self, rhs: u32) -> (Self::Output, bool) {
                    let (value, overflowed) = self.0.overflowing_shl(rhs);

                    (Self(value), overflowed)
                }
            }

            impl OverflowingShr for $ty {
                type Output = $ty;

                #[inline]
                fn overflowing_shr(self, rhs: u32) -> (Self::Output, bool) {
                    let (value, overflowed) = self.0.overflowing_shr(rhs);

                    (Self(value), overflowed)
                }
            }
        )*
    };
}

impl_overflowing_shift_traits_for_ints!(i8, i16, i32, i64, i128, isize);
impl_overflowing_shift_traits_for_ints!(u8, u16, u32, u64, u128, usize);
impl_overflowing_shift_traits_via_inner!(
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
impl_overflowing_shift_traits_via_inner!(
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

macro_rules! test_overflowing_bitshift_traits_int_nonnegative {
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

                assert_eq!(OverflowingShl::overflowing_shl(zero, 1), (zero, false));
                assert_eq!(OverflowingShl::overflowing_shl(one, 1), (two, false));
                assert_eq!(OverflowingShl::overflowing_shl(two, 1), (four, false));
                assert_eq!(OverflowingShl::overflowing_shl(one, 2), (four, false));
                assert_eq!(
                    OverflowingShl::overflowing_shl(one, <$ty>::BITS),
                    (one, true)
                );
            }

            #[test]
            fn test_shr() {
                let zero = <$ty as Zero>::ZERO;
                let one = <$ty as One>::ONE;
                let two = one + one;
                let four = two + two;

                assert_eq!(OverflowingShr::overflowing_shr(zero, 1), (zero, false));
                assert_eq!(OverflowingShr::overflowing_shr(one, 1), (zero, false));
                assert_eq!(OverflowingShr::overflowing_shr(two, 1), (one, false));
                assert_eq!(OverflowingShr::overflowing_shr(four, 1), (two, false));
                assert_eq!(OverflowingShr::overflowing_shr(four, 2), (one, false));
                assert_eq!(
                    OverflowingShr::overflowing_shr(one, <$ty>::BITS),
                    (one, true)
                );
            }
        }
    };
}

macro_rules! test_overflowing_bitshift_traits_int_negative {
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

                assert_eq!(OverflowingShl::overflowing_shl(-one, 1), (-two, false));
                assert_eq!(OverflowingShl::overflowing_shl(-two, 1), (-four, false));
                assert_eq!(
                    OverflowingShl::overflowing_shl(-one, <$ty>::BITS),
                    (-one, true)
                );
            }

            #[test]
            fn test_shr() {
                let one = <$ty as One>::ONE;
                let two = one + one;
                let four = two + two;

                assert_eq!(OverflowingShr::overflowing_shr(-two, 1), (-one, false));
                assert_eq!(OverflowingShr::overflowing_shr(-four, 1), (-two, false));
                assert_eq!(OverflowingShr::overflowing_shr(-four, 2), (-one, false));
                assert_eq!(
                    OverflowingShr::overflowing_shr(-one, <$ty>::BITS),
                    (-one, true)
                );
            }
        }
    };
}

test_overflowing_bitshift_traits_int_nonnegative!(i8, i8_nonnegative_tests);
test_overflowing_bitshift_traits_int_negative!(i8, i8_negative_tests);
test_overflowing_bitshift_traits_int_nonnegative!(i16, i16_nonnegative_tests);
test_overflowing_bitshift_traits_int_negative!(i16, i16_negative_tests);
test_overflowing_bitshift_traits_int_nonnegative!(i32, i32_nonnegative_tests);
test_overflowing_bitshift_traits_int_negative!(i32, i32_negative_tests);
test_overflowing_bitshift_traits_int_nonnegative!(i64, i64_nonnegative_tests);
test_overflowing_bitshift_traits_int_negative!(i64, i64_negative_tests);
test_overflowing_bitshift_traits_int_nonnegative!(i128, i128_nonnegative_tests);
test_overflowing_bitshift_traits_int_negative!(i128, i128_negative_tests);
test_overflowing_bitshift_traits_int_nonnegative!(isize, isize_nonnegative_tests);
test_overflowing_bitshift_traits_int_negative!(isize, isize_negative_tests);

test_overflowing_bitshift_traits_int_nonnegative!(u8, u8_tests);
test_overflowing_bitshift_traits_int_nonnegative!(u16, u16_tests);
test_overflowing_bitshift_traits_int_nonnegative!(u32, u32_tests);
test_overflowing_bitshift_traits_int_nonnegative!(u64, u64_tests);
test_overflowing_bitshift_traits_int_nonnegative!(u128, u128_tests);
test_overflowing_bitshift_traits_int_nonnegative!(usize, usize_tests);
