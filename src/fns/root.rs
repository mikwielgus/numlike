// SPDX-FileCopyrightText: 2026 numlike contributors
//
// SPDX-License-Identifier: MIT OR Apache-2.0

#[cfg(any(feature = "std", feature = "libm"))]
use crate::fns::round::Floor;

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
/// This trait's function returns the **principal (non-negative) square root**.
/// For a given number `n`, although both `x` and `-x` satisfy x<sup>2</sup> =
/// n, this function always returns the non-negative value.
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
/// This trait's function returns the **principal (non-negative) square root**.
/// For a given number `n`, although both `x` and `-x` satisfy x<sup>2</sup> =
/// n, this function always returns the non-negative value.
///
/// Returns `None` if `self` is negative, or if the result is not finite.
pub trait CheckedIsqrt {
    /// The resulting type after applying the operation.
    type Output;

    /// Returns the integer square root of the number, rounded down.
    fn checked_isqrt(self) -> Option<Self::Output>;
}

#[cfg(any(feature = "std", feature = "libm"))]
macro_rules! impl_root_traits_for_float {
    ($ty:ty, $sqrt:path, $cbrt:path) => {
        impl Sqrt for $ty {
            type Output = $ty;

            #[inline]
            fn sqrt(self) -> Self::Output {
                $sqrt(self)
            }
        }

        impl Cbrt for $ty {
            type Output = $ty;

            #[inline]
            fn cbrt(self) -> Self::Output {
                $cbrt(self)
            }
        }

        impl Isqrt for $ty {
            type Output = $ty;

            #[inline]
            fn isqrt(self) -> Self::Output {
                Floor::floor($sqrt(self))
            }
        }

        impl CheckedIsqrt for $ty {
            type Output = $ty;

            #[inline]
            fn checked_isqrt(self) -> Option<Self::Output> {
                let result = Floor::floor($sqrt(self));

                result.is_finite().then_some(result)
            }
        }

        impl CheckedSqrt for $ty {
            type Output = $ty;

            #[inline]
            fn checked_sqrt(self) -> Option<Self::Output> {
                let result = $sqrt(self);

                result.is_finite().then_some(result)
            }
        }
    };
    ($ty:ty, $sqrt:path, $cbrt:path, $nonnegative_tests_mod:ident, $negative_tests_mod:ident) => {
        impl_root_traits_for_float!($ty, $sqrt, $cbrt);

        test_root_traits_float_nonnegative!($ty, $nonnegative_tests_mod);
        test_root_traits_float_negative!($ty, $negative_tests_mod);
    };
}

#[cfg(any(feature = "std", feature = "libm"))]
macro_rules! test_root_traits_float_nonnegative {
    ($ty:ty, $tests_mod:ident) => {
        #[cfg(test)]
        mod $tests_mod {
            use crate::elem::*;
            use crate::fns::*;

            #[test]
            fn test_sqrt() {
                let zero = <$ty as Zero>::ZERO;
                let one = <$ty as One>::ONE;
                let two = one + one;
                let four = two + two;

                assert_eq!(Sqrt::sqrt(zero), zero);
                assert_eq!(Sqrt::sqrt(one), one);
                assert_eq!(Sqrt::sqrt(four), two);
            }

            #[test]
            fn test_cbrt() {
                let zero = <$ty as Zero>::ZERO;
                let one = <$ty as One>::ONE;
                let two = one + one;
                let four = two + two;
                let eight = four + four;

                assert_eq!(Cbrt::cbrt(zero), zero);
                assert_eq!(Cbrt::cbrt(one), one);
                assert_eq!(Cbrt::cbrt(eight), two);
            }

            #[test]
            fn test_isqrt() {
                let zero = <$ty as Zero>::ZERO;
                let one = <$ty as One>::ONE;
                let two = one + one;
                let four = two + two;

                assert_eq!(Isqrt::isqrt(zero), zero);
                assert_eq!(Isqrt::isqrt(one), one);
                assert_eq!(Isqrt::isqrt(two), one);
                assert_eq!(Isqrt::isqrt(four), two);
            }

            #[test]
            fn test_checked_sqrt() {
                let zero = <$ty as Zero>::ZERO;
                let one = <$ty as One>::ONE;
                let two = one + one;
                let four = two + two;

                assert_eq!(CheckedSqrt::checked_sqrt(zero), Some(zero));
                assert_eq!(CheckedSqrt::checked_sqrt(one), Some(one));
                assert_eq!(CheckedSqrt::checked_sqrt(four), Some(two));
            }

            #[test]
            fn test_checked_isqrt() {
                let zero = <$ty as Zero>::ZERO;
                let one = <$ty as One>::ONE;
                let two = one + one;
                let four = two + two;

                assert_eq!(CheckedIsqrt::checked_isqrt(zero), Some(zero));
                assert_eq!(CheckedIsqrt::checked_isqrt(one), Some(one));
                assert_eq!(CheckedIsqrt::checked_isqrt(two), Some(one));
                assert_eq!(CheckedIsqrt::checked_isqrt(four), Some(two));
            }
        }
    };
}

#[cfg(any(feature = "std", feature = "libm"))]
macro_rules! test_root_traits_float_negative {
    ($ty:ty, $tests_mod:ident) => {
        #[cfg(test)]
        mod $tests_mod {
            use crate::elem::*;
            use crate::fns::*;

            #[test]
            fn test_cbrt() {
                let one = <$ty as One>::ONE;
                let two = one + one;
                let four = two + two;
                let eight = four + four;

                assert_eq!(Cbrt::cbrt(-one), -one);
                assert_eq!(Cbrt::cbrt(-eight), -two);
            }

            #[test]
            fn test_checked_sqrt() {
                let one = <$ty as One>::ONE;

                assert_eq!(CheckedSqrt::checked_sqrt(-one), None);
            }

            #[test]
            fn test_checked_isqrt() {
                let one = <$ty as One>::ONE;

                assert_eq!(CheckedIsqrt::checked_isqrt(-one), None);
            }
        }
    };
}

#[cfg(feature = "std")]
impl_root_traits_for_float!(
    f32,
    f32::sqrt,
    f32::cbrt,
    f32_nonnegative_tests,
    f32_negative_tests
);
#[cfg(feature = "std")]
impl_root_traits_for_float!(
    f64,
    f64::sqrt,
    f64::cbrt,
    f64_nonnegative_tests,
    f64_negative_tests
);
#[cfg(all(not(feature = "std"), feature = "libm"))]
impl_root_traits_for_float!(
    f32,
    libm::sqrtf,
    libm::cbrtf,
    f32_nonnegative_tests,
    f32_negative_tests
);
#[cfg(all(not(feature = "std"), feature = "libm"))]
impl_root_traits_for_float!(
    f64,
    libm::sqrt,
    libm::cbrt,
    f64_nonnegative_tests,
    f64_negative_tests
);

macro_rules! impl_isqrt_trait_for_int {
    ($ty:ty) => {
        impl Isqrt for $ty {
            type Output = $ty;

            #[inline]
            fn isqrt(self) -> Self::Output {
                <$ty>::isqrt(self)
            }
        }
    };
}

macro_rules! impl_checked_isqrt_trait_for_signed_int {
    ($ty:ty) => {
        impl CheckedIsqrt for $ty {
            type Output = $ty;

            #[inline]
            fn checked_isqrt(self) -> Option<Self::Output> {
                <$ty>::checked_isqrt(self)
            }
        }
    };
}

macro_rules! impl_checked_isqrt_trait_for_unsigned_int {
    ($ty:ty) => {
        impl CheckedIsqrt for $ty {
            type Output = $ty;

            #[inline]
            fn checked_isqrt(self) -> Option<Self::Output> {
                Some(<$ty>::isqrt(self))
            }
        }
    };
}

macro_rules! impl_isqrt_traits_for_signed_int {
    ($ty:ty, $nonnegative_tests_mod:ident, $negative_tests_mod:ident) => {
        impl_isqrt_trait_for_int!($ty);
        impl_checked_isqrt_trait_for_signed_int!($ty);

        test_isqrt_traits_nonnegative!($ty, $nonnegative_tests_mod);
        test_isqrt_traits_negative!($ty, $negative_tests_mod);
    };
}

macro_rules! impl_isqrt_traits_for_unsigned_int {
    ($ty:ty, $tests_mod:ident) => {
        impl_isqrt_trait_for_int!($ty);
        impl_checked_isqrt_trait_for_unsigned_int!($ty);

        test_isqrt_traits_nonnegative!($ty, $tests_mod);
    };
}

macro_rules! test_isqrt_traits_nonnegative {
    ($ty:ty, $tests_mod:ident) => {
        #[cfg(test)]
        mod $tests_mod {
            use crate::elem::*;
            use crate::fns::*;

            #[test]
            fn test_isqrt() {
                let zero = <$ty as Zero>::ZERO;
                let one = <$ty as One>::ONE;
                let two = one + one;
                let three = two + one;
                let four = two + two;
                let eight = four + four;
                let nine = eight + one;

                assert_eq!(Isqrt::isqrt(zero), zero);
                assert_eq!(Isqrt::isqrt(one), one);
                assert_eq!(Isqrt::isqrt(two), one);
                assert_eq!(Isqrt::isqrt(three), one);
                assert_eq!(Isqrt::isqrt(four), two);
                assert_eq!(Isqrt::isqrt(eight), two);
                assert_eq!(Isqrt::isqrt(nine), two + one);
            }

            #[test]
            fn test_checked_isqrt() {
                let zero = <$ty as Zero>::ZERO;
                let one = <$ty as One>::ONE;
                let two = one + one;
                let four = two + two;
                let nine = four + four + one;

                assert_eq!(CheckedIsqrt::checked_isqrt(zero), Some(zero));
                assert_eq!(CheckedIsqrt::checked_isqrt(one), Some(one));
                assert_eq!(CheckedIsqrt::checked_isqrt(two), Some(one));
                assert_eq!(CheckedIsqrt::checked_isqrt(four), Some(two));
                assert_eq!(CheckedIsqrt::checked_isqrt(nine), Some(two + one));
            }
        }
    };
}

macro_rules! test_isqrt_traits_negative {
    ($ty:ty, $tests_mod:ident) => {
        #[cfg(test)]
        mod $tests_mod {
            use crate::elem::*;
            use crate::fns::*;

            #[test]
            fn test_checked_isqrt() {
                let one = <$ty as One>::ONE;

                assert_eq!(CheckedIsqrt::checked_isqrt(-one), None);
            }
        }
    };
}

impl_isqrt_traits_for_signed_int!(i8, i8_nonnegative_tests, i8_negative_tests);
impl_isqrt_traits_for_signed_int!(i16, i16_nonnegative_tests, i16_negative_tests);
impl_isqrt_traits_for_signed_int!(i32, i32_nonnegative_tests, i32_negative_tests);
impl_isqrt_traits_for_signed_int!(i64, i64_nonnegative_tests, i64_negative_tests);
impl_isqrt_traits_for_signed_int!(i128, i128_nonnegative_tests, i128_negative_tests);
impl_isqrt_traits_for_signed_int!(isize, isize_nonnegative_tests, isize_negative_tests);

impl_isqrt_traits_for_unsigned_int!(u8, u8_tests);
impl_isqrt_traits_for_unsigned_int!(u16, u16_tests);
impl_isqrt_traits_for_unsigned_int!(u32, u32_tests);
impl_isqrt_traits_for_unsigned_int!(u64, u64_tests);
impl_isqrt_traits_for_unsigned_int!(u128, u128_tests);
impl_isqrt_traits_for_unsigned_int!(usize, usize_tests);
