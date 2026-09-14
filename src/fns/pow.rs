// SPDX-FileCopyrightText: 2026 numlike contributors
//
// SPDX-License-Identifier: MIT OR Apache-2.0

/// Raises `self` to the power of `rhs`.
///
/// For integers, this uses exponentiation by squaring.
///
/// For floats, this may use an integer or floating-point power depending on
/// the exponent type. Using an integer power is generally faster than using
/// a floating-point power. It might have a different sequence of rounding
/// operations than a floating-point power, so the results are not
/// guaranteed to agree.
///
/// Note that the floating-point power is special in that it can return
/// non-NaN results for NaN inputs. For example, `f32::NAN.pow(0.0)` returns
/// `1.0`. However, if an input is a *signaling* NaN, then the result is
/// non-deterministically either a NaN or the result that the corresponding
/// quiet NaN would produce.
pub trait Pow<Rhs = Self> {
    /// The resulting type after applying the operation.
    type Output;

    /// Raises `self` to the power of `rhs`.
    fn pow(self, rhs: Rhs) -> Self::Output;
}

/// Checked exponentiation. Computes `self.pow(rhs)`, returning `None` if
/// overflow occurred or the result is not finite.
pub trait CheckedPow<Rhs = Self> {
    /// The resulting type after applying the operation.
    type Output;

    /// Checked exponentiation. Computes `self.pow(rhs)`, returning `None` if
    /// overflow occurred or the result is not finite.
    fn checked_pow(self, rhs: Rhs) -> Option<Self::Output>;
}

macro_rules! impl_pow_for_int {
    ($t:ty, $rhs:ty) => {
        impl Pow<$rhs> for $t {
            type Output = $t;

            #[inline]
            fn pow(self, rhs: $rhs) -> $t {
                //<$t>::pow(
                //self,
                //rhs.try_into()
                //.expect("exponent out of range for integer `pow`"),
                //)
                <$t>::pow(self, u32::from(rhs))
            }
        }

        impl CheckedPow<$rhs> for $t {
            type Output = $t;

            #[inline]
            fn checked_pow(self, rhs: $rhs) -> Option<$t> {
                //rhs.try_into()
                //.ok()
                //.and_then(|exp| <$t>::checked_pow(self, exp))
                <$t>::checked_pow(self, u32::from(rhs))
            }
        }
    };
}

macro_rules! impl_pow_for_int_for_all_rhs {
    ($t:ty) => {
        impl_pow_for_int!($t, u8);
        impl_pow_for_int!($t, u16);
        impl_pow_for_int!($t, u32);
        //impl_pow_for_int!($t, u64);
        //impl_pow_for_int!($t, u128);
        //impl_pow_for_int!($t, usize);
        //impl_pow_for_int!($t, i8);
        //impl_pow_for_int!($t, i16);
        //impl_pow_for_int!($t, i32);
        //impl_pow_for_int!($t, i64);
        //impl_pow_for_int!($t, i128);
        //impl_pow_for_int!($t, isize);
    };
    ($t:ty, $nonnegative_tests_mod:ident) => {
        impl_pow_for_int_for_all_rhs!($t);

        test_pow_traits_int_nonnegative!($t, $nonnegative_tests_mod);
    };
    ($t:ty, $nonnegative_tests_mod:ident, $negative_tests_mod:ident) => {
        impl_pow_for_int_for_all_rhs!($t, $nonnegative_tests_mod);

        test_pow_traits_int_negative!($t, $negative_tests_mod);
    };
}

macro_rules! test_pow_traits_int_nonnegative {
    ($ty:ty, $tests_mod:ident) => {
        #[cfg(test)]
        mod $tests_mod {
            use crate::elem::*;
            use crate::fns::*;

            #[test]
            fn test_nonnegative_pow() {
                let zero = <$ty as Zero>::ZERO;
                let one = <$ty as One>::ONE;
                let two = one + one;
                let four = two + two;
                let eight = four + four;

                assert_eq!(Pow::pow(zero, 0u8), one);
                assert_eq!(Pow::pow(zero, 1u8), zero);
                assert_eq!(Pow::pow(one, 0u8), one);
                assert_eq!(Pow::pow(one, 5u8), one);
                assert_eq!(Pow::pow(two, 0u8), one);
                assert_eq!(Pow::pow(two, 1u8), two);
                assert_eq!(Pow::pow(two, 2u8), four);
                assert_eq!(Pow::pow(two, 3u8), eight);

                assert_eq!(Pow::pow(two, 2u16), four);
                assert_eq!(Pow::pow(two, 2u32), four);
            }

            #[test]
            fn test_nonnegative_checked_pow() {
                let zero = <$ty as Zero>::ZERO;
                let one = <$ty as One>::ONE;
                let two = one + one;
                let four = two + two;
                let eight = four + four;

                assert_eq!(CheckedPow::checked_pow(zero, 0u8), Some(one));
                assert_eq!(CheckedPow::checked_pow(zero, 1u8), Some(zero));
                assert_eq!(CheckedPow::checked_pow(one, 5u8), Some(one));
                assert_eq!(CheckedPow::checked_pow(two, 2u8), Some(four));
                assert_eq!(CheckedPow::checked_pow(two, 3u8), Some(eight));

                assert_eq!(CheckedPow::checked_pow(two, 2u16), Some(four));
                assert_eq!(CheckedPow::checked_pow(two, 2u32), Some(four));

                assert_eq!(CheckedPow::checked_pow(two, 128u8), None);
            }
        }
    };
}

macro_rules! test_pow_traits_int_negative {
    ($ty:ty, $tests_mod:ident) => {
        #[cfg(test)]
        mod $tests_mod {
            use crate::elem::*;
            use crate::fns::*;

            #[test]
            fn test_negative_pow() {
                let one = <$ty as One>::ONE;
                let two = one + one;
                let four = two + two;
                let eight = four + four;

                assert_eq!(Pow::pow(-one, 0u8), one);
                assert_eq!(Pow::pow(-one, 1u8), -one);
                assert_eq!(Pow::pow(-one, 2u8), one);
                assert_eq!(Pow::pow(-two, 2u8), four);
                assert_eq!(Pow::pow(-two, 3u8), -eight);

                assert_eq!(Pow::pow(-two, 2u16), four);
                assert_eq!(Pow::pow(-two, 3u32), -eight);
            }

            #[test]
            fn test_negative_checked_pow() {
                let one = <$ty as One>::ONE;
                let two = one + one;
                let four = two + two;
                let eight = four + four;

                assert_eq!(CheckedPow::checked_pow(-one, 0u8), Some(one));
                assert_eq!(CheckedPow::checked_pow(-one, 1u8), Some(-one));
                assert_eq!(CheckedPow::checked_pow(-two, 2u8), Some(four));
                assert_eq!(CheckedPow::checked_pow(-two, 3u8), Some(-eight));

                assert_eq!(CheckedPow::checked_pow(-two, 128u8), None);
            }
        }
    };
}

impl_pow_for_int_for_all_rhs!(u8, u8_tests);
impl_pow_for_int_for_all_rhs!(u16, u16_tests);
impl_pow_for_int_for_all_rhs!(u32, u32_tests);
impl_pow_for_int_for_all_rhs!(u64, u64_tests);
impl_pow_for_int_for_all_rhs!(u128, u128_tests);
impl_pow_for_int_for_all_rhs!(usize, usize_tests);

impl_pow_for_int_for_all_rhs!(i8, i8_nonnegative_tests, i8_negative_tests);
impl_pow_for_int_for_all_rhs!(i16, i16_nonnegative_tests, i16_negative_tests);
impl_pow_for_int_for_all_rhs!(i32, i32_nonnegative_tests, i32_negative_tests);
impl_pow_for_int_for_all_rhs!(i64, i64_nonnegative_tests, i64_negative_tests);
impl_pow_for_int_for_all_rhs!(i128, i128_nonnegative_tests, i128_negative_tests);
impl_pow_for_int_for_all_rhs!(isize, isize_nonnegative_tests, isize_negative_tests);

#[cfg(any(feature = "std", feature = "libm"))]
macro_rules! impl_pow_for_float {
    ($t:ty, $rhs:ty, $desired_rhs:ty, $method:path) => {
        impl Pow<$rhs> for $t {
            type Output = $t;

            #[inline]
            fn pow(self, rhs: $rhs) -> $t {
                //<$t>::powi(
                //self,
                //rhs.try_into().expect("exponent out of range for `powi`"),
                //)
                $method(self, rhs as $desired_rhs)
            }
        }

        impl CheckedPow<$rhs> for $t {
            type Output = $t;

            #[inline]
            fn checked_pow(self, rhs: $rhs) -> Option<$t> {
                //let result = rhs.try_into().ok().map(|exp| <$t>::powi(self, exp))?;
                let result = $method(self, rhs as $desired_rhs);

                result.is_finite().then_some(result)
            }
        }
    };
}

#[cfg(any(feature = "std", feature = "libm"))]
macro_rules! impl_pow_for_float_for_all_rhs {
    ($t:ty, $int_pow:path, $int_rhs:ty, $float_pow:path) => {
        impl_pow_for_float!($t, i8, $int_rhs, $int_pow);
        impl_pow_for_float!($t, u8, $int_rhs, $int_pow);
        impl_pow_for_float!($t, i16, $int_rhs, $int_pow);
        impl_pow_for_float!($t, u16, $int_rhs, $int_pow);
        impl_pow_for_float!($t, i32, $int_rhs, $int_pow);
        impl_pow_for_float!($t, u32, $t, $float_pow);
        //impl_pow_for_float!($t, u32);
        //impl_pow_for_float!($t, i64);
        //impl_pow_for_float!($t, u64);
        //impl_pow_for_float!($t, i128);
        //impl_pow_for_float!($t, u128);
        //impl_pow_for_float!($t, isize);
        //impl_pow_for_float!($t, usize);
        impl_pow_for_float!($t, $t, $t, $float_pow);
    };
    (
        $t:ty,
        $int_pow:path,
        $int_rhs:ty,
        $float_pow:path,
        $nonnegative_tests_mod:ident,
        $negative_tests_mod:ident
    ) => {
        impl_pow_for_float_for_all_rhs!($t, $int_pow, $int_rhs, $float_pow);

        test_pow_traits_float_nonnegative!($t, $nonnegative_tests_mod);
        test_pow_traits_float_negative!($t, $negative_tests_mod);
    };
}

#[cfg(any(feature = "std", feature = "libm"))]
macro_rules! test_pow_traits_float_nonnegative {
    ($ty:ty, $tests_mod:ident) => {
        #[cfg(test)]
        mod $tests_mod {
            use crate::elem::*;
            use crate::fns::*;
            use crate::limits::*;

            #[test]
            fn test_nonnegative_pow() {
                let zero = <$ty as Zero>::ZERO;
                let one = <$ty as One>::ONE;
                let two = one + one;
                let four = two + two;
                let eight = four + four;

                assert_eq!(Pow::pow(zero, 0), one);
                assert_eq!(Pow::pow(zero, 1), zero);
                assert_eq!(Pow::pow(one, 5), one);
                assert_eq!(Pow::pow(two, 0), one);
                assert_eq!(Pow::pow(two, 1), two);
                assert_eq!(Pow::pow(two, 2), four);
                assert_eq!(Pow::pow(two, 3), eight);

                assert_eq!(Pow::pow(two, 2u8), four);
                assert_eq!(Pow::pow(two, 2u16), four);
                assert_eq!(Pow::pow(two, 2u32), four);
                assert_eq!(Pow::pow(two, two), four);
            }

            #[test]
            fn test_nonnegative_checked_pow() {
                let zero = <$ty as Zero>::ZERO;
                let one = <$ty as One>::ONE;
                let two = one + one;
                let four = two + two;
                let eight = four + four;

                assert_eq!(CheckedPow::checked_pow(zero, 0), Some(one));
                assert_eq!(CheckedPow::checked_pow(two, 2), Some(four));
                assert_eq!(CheckedPow::checked_pow(two, 3u8), Some(eight));
                assert_eq!(CheckedPow::checked_pow(two, two), Some(four));

                assert_eq!(
                    CheckedPow::checked_pow(two, <$ty as MaxFinite>::MAX_FINITE),
                    None
                );
            }
        }
    };
}

#[cfg(any(feature = "std", feature = "libm"))]
macro_rules! test_pow_traits_float_negative {
    ($ty:ty, $tests_mod:ident) => {
        #[cfg(test)]
        mod $tests_mod {
            use crate::elem::*;
            use crate::fns::*;

            #[test]
            fn test_negative_pow() {
                let one = <$ty as One>::ONE;
                let two = one + one;
                let four = two + two;
                let eight = four + four;
                let half = one / two;
                let quarter = one / four;

                assert_eq!(Pow::pow(-one, 0), one);
                assert_eq!(Pow::pow(-one, 1), -one);
                assert_eq!(Pow::pow(-one, 2), one);
                assert_eq!(Pow::pow(-two, 2), four);
                assert_eq!(Pow::pow(-two, 3), -eight);

                assert_eq!(Pow::pow(two, -1), half);
                assert_eq!(Pow::pow(two, -2), quarter);
                assert_eq!(Pow::pow(two, -one), half);
                assert_eq!(Pow::pow(four, -one), quarter);
            }

            #[test]
            fn test_negative_checked_pow() {
                let one = <$ty as One>::ONE;
                let two = one + one;
                let four = two + two;
                let eight = four + four;
                let half = one / two;
                let quarter = one / four;

                assert_eq!(CheckedPow::checked_pow(-two, 2), Some(four));
                assert_eq!(CheckedPow::checked_pow(-two, 3), Some(-eight));
                assert_eq!(CheckedPow::checked_pow(two, -1), Some(half));
                assert_eq!(CheckedPow::checked_pow(two, -2), Some(quarter));
                assert_eq!(CheckedPow::checked_pow(two, -one), Some(half));
            }
        }
    };
}

#[cfg(feature = "std")]
impl_pow_for_float_for_all_rhs!(
    f32,
    f32::powi,
    i32,
    f32::powf,
    f32_nonnegative_tests,
    f32_negative_tests
);
#[cfg(feature = "std")]
impl_pow_for_float_for_all_rhs!(
    f64,
    f64::powi,
    i32,
    f64::powf,
    f64_nonnegative_tests,
    f64_negative_tests
);
#[cfg(all(not(feature = "std"), feature = "libm"))]
impl_pow_for_float_for_all_rhs!(
    f32,
    libm::powf,
    f32,
    libm::powf,
    f32_nonnegative_tests,
    f32_negative_tests
);
#[cfg(all(not(feature = "std"), feature = "libm"))]
impl_pow_for_float_for_all_rhs!(
    f64,
    libm::pow,
    f64,
    libm::pow,
    f64_nonnegative_tests,
    f64_negative_tests
);
