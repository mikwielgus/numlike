// SPDX-FileCopyrightText: 2026 numlike contributors
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use super::round::Floor;

/// Bundle of root-finding functions.
pub trait RootFns: Sqrt + Cbrt {}
impl<T: Sqrt + Cbrt> RootFns for T {}

/// Returns the square root of a number.
///
/// Returns NaN if `self` is a negative number other than `-0.0`.
pub trait Sqrt {
    /// The resulting type after applying the operation.
    type Output;

    /// Returns the square root of a number.
    fn sqrt(self) -> Self::Output;
}

/// Returns the cube root of a number.
pub trait Cbrt {
    /// The resulting type after applying the operation.
    type Output;

    /// Returns the cube root of a number.
    fn cbrt(self) -> Self::Output;
}

/// Returns the square root of a number.
///
/// Returns `None` if the result is not finite (including when `self` is a
/// negative number other than `-0.0`).
pub trait CheckedSqrt {
    /// The resulting type after applying the operation.
    type Output;

    /// Returns the square root of a number.
    fn checked_sqrt(self) -> Option<Self::Output>;
}

/// Returns the integer square root of the number, rounded down.
///
/// This function returns the **principal (non-negative) square root**.
/// For a given number `n`, although both `x` and `-x` satisfy x<sup>2</sup> = n,
/// this function always returns the non-negative value.
///
/// # Panics
///
/// This function will panic if `self` is negative.
pub trait Isqrt {
    /// The resulting type after applying the operation.
    type Output;

    /// Returns the integer square root of the number, rounded down.
    fn isqrt(self) -> Self::Output;
}

/// Returns the integer square root of the number, rounded down.
///
/// This function returns the **principal (non-negative) square root**.
/// For a given number `n`, although both `x` and `-x` satisfy x<sup>2</sup> = n,
/// this function always returns the non-negative value.
///
/// Returns `None` if `self` is negative, or if the result is not finite.
pub trait CheckedIsqrt {
    /// The resulting type after applying the operation.
    type Output;

    /// Returns the integer square root of the number, rounded down.
    fn checked_isqrt(self) -> Option<Self::Output>;
}

macro_rules! impl_root_traits_for_floats {
    ($($ty:ty),*) => {
        $(
            impl Sqrt for $ty {
                type Output = $ty;

                #[inline]
                fn sqrt(self) -> Self::Output {
                    <$ty>::sqrt(self)
                }
            }

            impl Cbrt for $ty {
                type Output = $ty;

                #[inline]
                fn cbrt(self) -> Self::Output {
                    <$ty>::cbrt(self)
                }
            }

            impl Isqrt for $ty {
                type Output = $ty;

                #[inline]
                fn isqrt(self) -> Self::Output {
                    Floor::floor(<$ty>::sqrt(self))
                }
            }

            impl CheckedIsqrt for $ty {
                type Output = $ty;

                #[inline]
                fn checked_isqrt(self) -> Option<Self::Output> {
                    let result = Floor::floor(<$ty>::sqrt(self));

                    result.is_finite().then_some(result)
                }
            }

            impl CheckedSqrt for $ty {
                type Output = $ty;

                #[inline]
                fn checked_sqrt(self) -> Option<Self::Output> {
                    let result = <$ty>::sqrt(self);

                    result.is_finite().then_some(result)
                }
            }
        )*
    };
}

macro_rules! impl_isqrt_trait_for_ints {
    ($($ty:ty),*) => {
        $(
            impl Isqrt for $ty {
                type Output = $ty;

                #[inline]
                fn isqrt(self) -> Self::Output {
                    <$ty>::isqrt(self)
                }
            }
        )*
    };
}

macro_rules! impl_checked_isqrt_trait_for_signed_ints {
    ($($ty:ty),*) => {
        $(
            impl CheckedIsqrt for $ty {
                type Output = $ty;

                #[inline]
                fn checked_isqrt(self) -> Option<Self::Output> {
                    <$ty>::checked_isqrt(self)
                }
            }
        )*
    };
}

macro_rules! impl_checked_isqrt_trait_for_unsigned_ints {
    ($($ty:ty),*) => {
        $(
            impl CheckedIsqrt for $ty {
                type Output = $ty;

                #[inline]
                fn checked_isqrt(self) -> Option<Self::Output> {
                    Some(<$ty>::isqrt(self))
                }
            }
        )*
    };
}

impl_root_traits_for_floats!(f32, f64);
impl_isqrt_trait_for_ints!(i8, i16, i32, i64, i128, isize);
impl_isqrt_trait_for_ints!(u8, u16, u32, u64, u128, usize);
impl_checked_isqrt_trait_for_signed_ints!(i8, i16, i32, i64, i128, isize);
impl_checked_isqrt_trait_for_unsigned_ints!(u8, u16, u32, u64, u128, usize);
