// SPDX-FileCopyrightText: 2026 numlike contributors
//
// SPDX-License-Identifier: MIT OR Apache-2.0

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
            fn test_nonnegative_round() {
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
            fn test_nonnegative_trunc() {
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
            fn test_nonnegative_round_ties_even() {
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
            fn test_nonnegative_floor() {
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
            fn test_nonnegative_ceil() {
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
            fn test_negative_round() {
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
            fn test_negative_trunc() {
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
            fn test_negative_round_ties_even() {
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
            fn test_negative_floor() {
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
            fn test_negative_ceil() {
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
