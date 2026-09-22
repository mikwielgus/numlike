// SPDX-FileCopyrightText: 2026 numlike contributors
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use core::num::{Saturating, Wrapping};

/// Bundle of rounding functions.
pub trait RoundFns: Round + Trunc + RoundTiesEven + Floor + Ceil {}
impl<T: Round + Trunc + RoundTiesEven + Floor + Ceil> RoundFns for T {}

/// Returns the nearest integer to `self`. If a value is half-way between two
/// integers, round away from `0.0`.
///
/// This function always returns the precise result.
pub trait Round {
    /// The resulting type after applying the operation.
    type Output;

    /// Returns the nearest integer to `self`. If a value is half-way between two
    /// integers, round away from `0.0`.
    fn round(self) -> Self::Output;
}

/// Returns the integer part of `self`.
/// This means that non-integer numbers are always truncated towards zero.
///
/// This function always returns the precise result.
pub trait Trunc {
    /// The resulting type after applying the operation.
    type Output;

    /// Returns the integer part of `self`.
    /// This means that non-integer numbers are always truncated towards zero.
    fn trunc(self) -> Self::Output;
}

/// Returns the nearest integer to a number. Rounds half-way cases to the number
/// with an even least significant digit.
///
/// This function always returns the precise result.
pub trait RoundTiesEven {
    /// The resulting type after applying the operation.
    type Output;

    /// Returns the nearest integer to a number. Rounds half-way cases to the number
    /// with an even least significant digit.
    fn round_ties_even(self) -> Self::Output;
}

/// Returns the largest integer that is less than or equal to `self`.
///
/// This function always returns the precise result.
pub trait Floor {
    /// The resulting type after applying the operation.
    type Output;

    /// Returns the largest integer that is less than or equal to `self`.
    fn floor(self) -> Self::Output;
}

/// Returns the smallest integer that is greater than or equal to `self`.
///
/// This function always returns the precise result.
pub trait Ceil {
    /// The resulting type after applying the operation.
    type Output;

    /// Returns the smallest integer that is greater than or equal to `self`.
    fn ceil(self) -> Self::Output;
}

macro_rules! impl_round_traits_for_int {
    (
        $ty:ty
    ) => {
        impl Round for $ty {
            type Output = $ty;

            #[inline]
            fn round(self) -> Self::Output {
                self
            }
        }

        impl Trunc for $ty {
            type Output = $ty;

            #[inline]
            fn trunc(self) -> Self::Output {
                self
            }
        }

        impl RoundTiesEven for $ty {
            type Output = $ty;

            #[inline]
            fn round_ties_even(self) -> Self::Output {
                self
            }
        }

        impl Floor for $ty {
            type Output = $ty;

            #[inline]
            fn floor(self) -> Self::Output {
                self
            }
        }

        impl Ceil for $ty {
            type Output = $ty;

            #[inline]
            fn ceil(self) -> Self::Output {
                self
            }
        }
    };
    ($ty:ty, $nonnegative_tests_mod:ident) => {
        impl_round_traits_for_int!($ty);

        // TODO: Let's implement tests too.
        // We can't reuse existing tests because these contain fractional numbers.
        //test_round_traits_nonnegative!($ty, $nonnegative_tests_mod);
    };
    ($ty:ty, $nonnegative_tests_mod:ident, $negative_tests_mod:ident) => {
        impl_round_traits_for_int!($ty, $nonnegative_tests_mod);

        // TODO: Let's implement tests too.
        // We can't reuse existing tests because these contain fractional numbers.
        //test_round_traits_negative!($ty, $negative_tests_mod);
    };
}

#[cfg(any(feature = "std", feature = "libm"))]
macro_rules! impl_round_traits_for_float {
    (
        $ty:ty,
        $round:path,
        $trunc:path,
        $round_ties_even:path,
        $floor:path,
        $ceil:path,
        $nonnegative_tests_mod:ident,
        $negative_tests_mod:ident
    ) => {
        impl Round for $ty {
            type Output = $ty;

            #[inline]
            fn round(self) -> Self::Output {
                $round(self)
            }
        }

        impl Trunc for $ty {
            type Output = $ty;

            #[inline]
            fn trunc(self) -> Self::Output {
                $trunc(self)
            }
        }

        impl RoundTiesEven for $ty {
            type Output = $ty;

            #[inline]
            fn round_ties_even(self) -> Self::Output {
                $round_ties_even(self)
            }
        }

        impl Floor for $ty {
            type Output = $ty;

            #[inline]
            fn floor(self) -> Self::Output {
                $floor(self)
            }
        }

        impl Ceil for $ty {
            type Output = $ty;

            #[inline]
            fn ceil(self) -> Self::Output {
                $ceil(self)
            }
        }

        test_round_traits_nonnegative!($ty, $nonnegative_tests_mod);
        test_round_traits_negative!($ty, $negative_tests_mod);
    };
}

#[cfg(any(feature = "std", feature = "libm"))]
macro_rules! test_round_traits_nonnegative {
    ($ty:ty, $tests_mod:ident) => {
        #[cfg(test)]
        mod $tests_mod {
            use crate::elem::*;
            use crate::fns::*;

            #[test]
            fn test_round() {
                let zero = <$ty as Zero>::ZERO;
                let one = <$ty as One>::ONE;
                let two = one + one;
                let four = two + two;
                let half = one / two;
                let one_and_half = one + half;
                let two_and_half = two + half;

                assert_eq!(Round::round(zero), zero);
                assert_eq!(Round::round(one), one);
                assert_eq!(Round::round(two), two);
                assert_eq!(Round::round(four), four);
                assert_eq!(Round::round(half), one);
                assert_eq!(Round::round(one_and_half), two);
                assert_eq!(Round::round(two_and_half), two + one);
            }

            #[test]
            fn test_trunc() {
                let zero = <$ty as Zero>::ZERO;
                let one = <$ty as One>::ONE;
                let two = one + one;
                let four = two + two;
                let half = one / two;
                let one_and_half = one + half;
                let two_and_half = two + half;

                assert_eq!(Trunc::trunc(zero), zero);
                assert_eq!(Trunc::trunc(one), one);
                assert_eq!(Trunc::trunc(two), two);
                assert_eq!(Trunc::trunc(four), four);
                assert_eq!(Trunc::trunc(half), zero);
                assert_eq!(Trunc::trunc(one_and_half), one);
                assert_eq!(Trunc::trunc(two_and_half), two);
            }

            #[test]
            fn test_round_ties_even() {
                let zero = <$ty as Zero>::ZERO;
                let one = <$ty as One>::ONE;
                let two = one + one;
                let four = two + two;
                let half = one / two;
                let one_and_half = one + half;
                let two_and_half = two + half;

                assert_eq!(RoundTiesEven::round_ties_even(zero), zero);
                assert_eq!(RoundTiesEven::round_ties_even(one), one);
                assert_eq!(RoundTiesEven::round_ties_even(two), two);
                assert_eq!(RoundTiesEven::round_ties_even(four), four);
                assert_eq!(RoundTiesEven::round_ties_even(half), zero);
                assert_eq!(RoundTiesEven::round_ties_even(one_and_half), two);
                assert_eq!(RoundTiesEven::round_ties_even(two_and_half), two);
            }

            #[test]
            fn test_floor() {
                let zero = <$ty as Zero>::ZERO;
                let one = <$ty as One>::ONE;
                let two = one + one;
                let four = two + two;
                let half = one / two;
                let one_and_half = one + half;
                let two_and_half = two + half;

                assert_eq!(Floor::floor(zero), zero);
                assert_eq!(Floor::floor(one), one);
                assert_eq!(Floor::floor(two), two);
                assert_eq!(Floor::floor(four), four);
                assert_eq!(Floor::floor(half), zero);
                assert_eq!(Floor::floor(one_and_half), one);
                assert_eq!(Floor::floor(two_and_half), two);
            }

            #[test]
            fn test_ceil() {
                let zero = <$ty as Zero>::ZERO;
                let one = <$ty as One>::ONE;
                let two = one + one;
                let four = two + two;
                let half = one / two;
                let one_and_half = one + half;
                let two_and_half = two + half;

                assert_eq!(Ceil::ceil(zero), zero);
                assert_eq!(Ceil::ceil(one), one);
                assert_eq!(Ceil::ceil(two), two);
                assert_eq!(Ceil::ceil(four), four);
                assert_eq!(Ceil::ceil(half), one);
                assert_eq!(Ceil::ceil(one_and_half), two);
                assert_eq!(Ceil::ceil(two_and_half), two + one);
            }
        }
    };
}

#[cfg(any(feature = "std", feature = "libm"))]
macro_rules! test_round_traits_negative {
    ($ty:ty, $tests_mod:ident) => {
        #[cfg(test)]
        mod $tests_mod {
            use crate::elem::*;
            use crate::fns::*;

            #[test]
            fn test_round() {
                let one = <$ty as One>::ONE;
                let two = one + one;
                let half = one / two;
                let one_and_half = one + half;
                let two_and_half = two + half;

                assert_eq!(Round::round(-one), -one);
                assert_eq!(Round::round(-two), -two);
                assert_eq!(Round::round(-half), -one);
                assert_eq!(Round::round(-one_and_half), -two);
                assert_eq!(Round::round(-two_and_half), -(two + one));
            }

            #[test]
            fn test_trunc() {
                let one = <$ty as One>::ONE;
                let two = one + one;
                let half = one / two;
                let one_and_half = one + half;
                let two_and_half = two + half;

                assert_eq!(Trunc::trunc(-one), -one);
                assert_eq!(Trunc::trunc(-two), -two);
                assert_eq!(Trunc::trunc(-half), <$ty as Zero>::ZERO);
                assert_eq!(Trunc::trunc(-one_and_half), -one);
                assert_eq!(Trunc::trunc(-two_and_half), -two);
            }

            #[test]
            fn test_round_ties_even() {
                let one = <$ty as One>::ONE;
                let two = one + one;
                let half = one / two;
                let one_and_half = one + half;
                let two_and_half = two + half;

                assert_eq!(RoundTiesEven::round_ties_even(-one), -one);
                assert_eq!(RoundTiesEven::round_ties_even(-two), -two);
                assert_eq!(RoundTiesEven::round_ties_even(-half), <$ty as Zero>::ZERO);
                assert_eq!(RoundTiesEven::round_ties_even(-one_and_half), -two);
                assert_eq!(RoundTiesEven::round_ties_even(-two_and_half), -two);
            }

            #[test]
            fn test_floor() {
                let one = <$ty as One>::ONE;
                let two = one + one;
                let half = one / two;
                let one_and_half = one + half;
                let two_and_half = two + half;

                assert_eq!(Floor::floor(-one), -one);
                assert_eq!(Floor::floor(-two), -two);
                assert_eq!(Floor::floor(-half), -one);
                assert_eq!(Floor::floor(-one_and_half), -two);
                assert_eq!(Floor::floor(-two_and_half), -(two + one));
            }

            #[test]
            fn test_ceil() {
                let one = <$ty as One>::ONE;
                let two = one + one;
                let half = one / two;
                let one_and_half = one + half;
                let two_and_half = two + half;

                assert_eq!(Ceil::ceil(-one), -one);
                assert_eq!(Ceil::ceil(-two), -two);
                assert_eq!(Ceil::ceil(-half), <$ty as Zero>::ZERO);
                assert_eq!(Ceil::ceil(-one_and_half), -one);
                assert_eq!(Ceil::ceil(-two_and_half), -two);
            }
        }
    };
}

impl_round_traits_for_int!(i8, i8_nonnegative_tests, i8_negative_tests);
impl_round_traits_for_int!(i16, i16_nonnegative_tests, i16_negative_tests);
impl_round_traits_for_int!(i32, i32_nonnegative_tests, i32_negative_tests);
impl_round_traits_for_int!(i64, i64_nonnegative_tests, i64_negative_tests);
impl_round_traits_for_int!(i128, i128_nonnegative_tests, i128_negative_tests);
impl_round_traits_for_int!(isize, isize_nonnegative_tests, isize_negative_tests);

impl_round_traits_for_int!(u8, u8_tests);
impl_round_traits_for_int!(u16, u16_tests);
impl_round_traits_for_int!(u32, u32_tests);
impl_round_traits_for_int!(u64, u64_tests);
impl_round_traits_for_int!(u128, u128_tests);
impl_round_traits_for_int!(usize, usize_tests);

impl_round_traits_for_int!(Wrapping<i8>, wrapping_i8_tests);
impl_round_traits_for_int!(Wrapping<i16>, wrapping_i16_tests);
impl_round_traits_for_int!(Wrapping<i32>, wrapping_i32_tests);
impl_round_traits_for_int!(Wrapping<i64>, wrapping_i64_tests);
impl_round_traits_for_int!(Wrapping<i128>, wrapping_i128_tests);
impl_round_traits_for_int!(Wrapping<isize>, wrapping_isize_tests);

impl_round_traits_for_int!(Wrapping<u8>, wrapping_u8_tests);
impl_round_traits_for_int!(Wrapping<u16>, wrapping_u16_tests);
impl_round_traits_for_int!(Wrapping<u32>, wrapping_u32_tests);
impl_round_traits_for_int!(Wrapping<u64>, wrapping_u64_tests);
impl_round_traits_for_int!(Wrapping<u128>, wrapping_u128_tests);
impl_round_traits_for_int!(Wrapping<usize>, wrapping_usize_tests);

impl_round_traits_for_int!(Saturating<i8>, saturating_i8_tests);
impl_round_traits_for_int!(Saturating<i16>, saturating_i16_tests);
impl_round_traits_for_int!(Saturating<i32>, saturating_i32_tests);
impl_round_traits_for_int!(Saturating<i64>, saturating_i64_tests);
impl_round_traits_for_int!(Saturating<i128>, saturating_i128_tests);
impl_round_traits_for_int!(Saturating<isize>, saturating_isize_tests);

impl_round_traits_for_int!(Saturating<u8>, saturating_u8_tests);
impl_round_traits_for_int!(Saturating<u16>, saturating_u16_tests);
impl_round_traits_for_int!(Saturating<u32>, saturating_u32_tests);
impl_round_traits_for_int!(Saturating<u64>, saturating_u64_tests);
impl_round_traits_for_int!(Saturating<u128>, saturating_u128_tests);
impl_round_traits_for_int!(Saturating<usize>, saturating_usize_tests);

#[cfg(feature = "std")]
impl_round_traits_for_float!(
    f32,
    f32::round,
    f32::trunc,
    f32::round_ties_even,
    f32::floor,
    f32::ceil,
    f32_nonnegative_tests,
    f32_negative_tests
);
#[cfg(feature = "std")]
impl_round_traits_for_float!(
    f64,
    f64::round,
    f64::trunc,
    f64::round_ties_even,
    f64::floor,
    f64::ceil,
    f64_nonnegative_tests,
    f64_negative_tests
);
#[cfg(all(not(feature = "std"), feature = "libm"))]
impl_round_traits_for_float!(
    f32,
    libm::roundf,
    libm::truncf,
    libm::roundevenf,
    libm::floorf,
    libm::ceilf,
    f32_nonnegative_tests,
    f32_negative_tests
);
#[cfg(all(not(feature = "std"), feature = "libm"))]
impl_round_traits_for_float!(
    f64,
    libm::round,
    libm::trunc,
    libm::roundeven,
    libm::floor,
    libm::ceil,
    f64_nonnegative_tests,
    f64_negative_tests
);
