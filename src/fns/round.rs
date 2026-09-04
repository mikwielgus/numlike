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

macro_rules! impl_round_traits_for_floats {
    ($($ty:ty),*) => {
        $(
            impl Round for $ty {
                type Output = $ty;

                #[inline]
                fn round(self) -> Self::Output {
                    <$ty>::round(self)
                }
            }

            impl Trunc for $ty {
                type Output = $ty;

                #[inline]
                fn trunc(self) -> Self::Output {
                    <$ty>::trunc(self)
                }
            }

            impl RoundTiesEven for $ty {
                type Output = $ty;

                #[inline]
                fn round_ties_even(self) -> Self::Output {
                    <$ty>::round_ties_even(self)
                }
            }

            impl Floor for $ty {
                type Output = $ty;

                #[inline]
                fn floor(self) -> Self::Output {
                    <$ty>::floor(self)
                }
            }

            impl Ceil for $ty {
                type Output = $ty;

                #[inline]
                fn ceil(self) -> Self::Output {
                    <$ty>::ceil(self)
                }
            }
        )*
    };
}

impl_round_traits_for_floats!(f32, f64);
