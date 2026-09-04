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
    ($ty:ty, $round:path, $trunc:path, $round_ties_even:path, $floor:path, $ceil:path) => {
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
    };
}

#[cfg(feature = "std")]
impl_round_traits_for_float!(
    f32,
    f32::round,
    f32::trunc,
    f32::round_ties_even,
    f32::floor,
    f32::ceil
);
#[cfg(feature = "std")]
impl_round_traits_for_float!(
    f64,
    f64::round,
    f64::trunc,
    f64::round_ties_even,
    f64::floor,
    f64::ceil
);
#[cfg(all(not(feature = "std"), feature = "libm"))]
impl_round_traits_for_float!(
    f32,
    libm::roundf,
    libm::truncf,
    libm::roundevenf,
    libm::floorf,
    libm::ceilf
);
#[cfg(all(not(feature = "std"), feature = "libm"))]
impl_round_traits_for_float!(
    f64,
    libm::round,
    libm::trunc,
    libm::roundeven,
    libm::floor,
    libm::ceil
);
