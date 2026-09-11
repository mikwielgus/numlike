// SPDX-FileCopyrightText: 2026 numlike contributors
//
// SPDX-License-Identifier: MIT OR Apache-2.0

/// Bundle of sign-related functions.
pub trait SignFns: Signum + Abs {}
impl<T: Signum + Abs> SignFns for T {}

/// Returns a number representing sign of `self`.
///
///  - `0` if the number is zero
///  - `1` if the number is positive
///  - `-1` if the number is negative
pub trait Signum {
    /// The resulting type after applying the operation.
    type Output;

    /// Returns a number representing sign of `self`.
    fn signum(self) -> Self::Output;
}

/// Computes the absolute value of `self`.
///
/// # Overflow behavior
///
/// The absolute value of the minimum value of a signed integer type cannot
/// be represented as that same type, and attempting to calculate it will
/// cause an overflow. This means that code in debug mode will trigger a
/// panic on this case and optimized code will return the minimum value
/// without a panic.
pub trait Abs {
    /// The resulting type after applying the operation.
    type Output;

    /// Computes the absolute value of `self`.
    fn abs(self) -> Self::Output;
}

/// Checked absolute value. Computes `self.abs()`, returning `None` if
/// `self` is the minimum value of a signed integer type.
pub trait CheckedAbs {
    /// The resulting type after applying the operation.
    type Output;

    /// Checked absolute value. Computes `self.abs()`, returning `None` if
    /// `self` is the minimum value of a signed integer type.
    fn checked_abs(self) -> Option<Self::Output>;
}

macro_rules! impl_sign_traits_for_unsigned {
    ($ty:ty, $tests_mod:ident) => {
        impl Signum for $ty {
            type Output = $ty;

            #[inline]
            fn signum(self) -> Self::Output {
                (self > 0) as $ty
            }
        }

        impl Abs for $ty {
            type Output = $ty;

            #[inline]
            fn abs(self) -> Self::Output {
                self
            }
        }

        impl CheckedAbs for $ty {
            type Output = $ty;

            #[inline]
            fn checked_abs(self) -> Option<Self::Output> {
                Some(self)
            }
        }

        sign_traits_nonnegative_tests!($ty, $tests_mod);
    };
}

macro_rules! sign_traits_nonnegative_tests {
    ($ty:ty, $tests_mod:ident) => {
        #[cfg(test)]
        mod $tests_mod {
            use crate::elem::*;
            use crate::fns::*;

            #[test]
            fn test_nonnegative_signum() {
                let zero = <$ty as Zero>::ZERO;
                let one = <$ty as One>::ONE;
                let two = one + one;
                let four = two + two;

                assert_eq!(Signum::signum(zero), zero);
                assert_eq!(Signum::signum(one), one);
                assert_eq!(Signum::signum(two), one);
                assert_eq!(Signum::signum(four), one);
            }

            #[test]
            fn test_nonnegative_abs() {
                let zero = <$ty as Zero>::ZERO;
                let one = <$ty as One>::ONE;
                let two = one + one;
                let four = two + two;

                assert_eq!(Abs::abs(zero), zero);
                assert_eq!(Abs::abs(one), one);
                assert_eq!(Abs::abs(two), two);
                assert_eq!(Abs::abs(four), four);
            }

            #[test]
            fn test_nonnegative_checked_abs() {
                let zero = <$ty as Zero>::ZERO;
                let one = <$ty as One>::ONE;
                let two = one + one;
                let four = two + two;

                assert_eq!(CheckedAbs::checked_abs(zero), Some(zero));
                assert_eq!(CheckedAbs::checked_abs(one), Some(one));
                assert_eq!(CheckedAbs::checked_abs(two), Some(two));
                assert_eq!(CheckedAbs::checked_abs(four), Some(four));
            }
        }
    };
}

impl_sign_traits_for_unsigned!(u8, u8_tests);
impl_sign_traits_for_unsigned!(u16, u16_tests);
impl_sign_traits_for_unsigned!(u32, u32_tests);
impl_sign_traits_for_unsigned!(u64, u64_tests);
impl_sign_traits_for_unsigned!(u128, u128_tests);
impl_sign_traits_for_unsigned!(usize, usize_tests);

macro_rules! impl_sign_traits_for_signed {
    ($ty:ty, $nonnegative_tests_mod:ident, $negative_tests_mod:ident) => {
        impl Signum for $ty {
            type Output = $ty;

            #[inline]
            fn signum(self) -> Self::Output {
                <$ty>::signum(self)
            }
        }

        impl Abs for $ty {
            type Output = $ty;

            #[inline]
            fn abs(self) -> Self::Output {
                <$ty>::abs(self)
            }
        }

        impl CheckedAbs for $ty {
            type Output = $ty;

            #[inline]
            fn checked_abs(self) -> Option<Self::Output> {
                <$ty>::checked_abs(self)
            }
        }

        sign_traits_nonnegative_tests!($ty, $nonnegative_tests_mod);
        sign_traits_negative_tests!($ty, $negative_tests_mod);
    };
}

macro_rules! sign_traits_negative_tests {
    ($ty:ty, $tests_mod:ident) => {
        #[cfg(test)]
        mod $tests_mod {
            use crate::elem::*;
            use crate::fns::*;

            #[test]
            fn test_negative_signum() {
                let one = <$ty as One>::ONE;
                let two = one + one;
                let four = two + two;

                assert_eq!(Signum::signum(-one), -one);
                assert_eq!(Signum::signum(-two), -one);
                assert_eq!(Signum::signum(-four), -one);
            }

            #[test]
            fn test_negative_abs() {
                let one = <$ty as One>::ONE;
                let two = one + one;
                let four = two + two;

                assert_eq!(Abs::abs(-one), one);
                assert_eq!(Abs::abs(-two), two);
                assert_eq!(Abs::abs(-four), four);
            }

            #[test]
            fn test_negative_checked_abs() {
                let one = <$ty as One>::ONE;
                let two = one + one;
                let four = two + two;

                assert_eq!(CheckedAbs::checked_abs(-one), Some(one));
                assert_eq!(CheckedAbs::checked_abs(-two), Some(two));
                assert_eq!(CheckedAbs::checked_abs(-four), Some(four));

                // TODO?: make it overflow?
            }
        }
    };
}

impl_sign_traits_for_signed!(i8, i8_nonnegative_tests, i8_negative_tests);
impl_sign_traits_for_signed!(i16, i16_nonnegative_tests, i16_negative_tests);
impl_sign_traits_for_signed!(i32, i32_nonnegative_tests, i32_negative_tests);
impl_sign_traits_for_signed!(i64, i64_nonnegative_tests, i64_negative_tests);
impl_sign_traits_for_signed!(i128, i128_nonnegative_tests, i128_negative_tests);
impl_sign_traits_for_signed!(isize, isize_nonnegative_tests, isize_negative_tests);
