// SPDX-FileCopyrightText: 2026 numlike contributors
//
// SPDX-License-Identifier: MIT OR Apache-2.0

//! Finite and extended numeric limits.

/// Bundle of limits for a floating-point numeric type.
pub trait FloatLimits: Limits + Digits + MantissaDigits + Epsilon {}
impl<T: Limits + Digits + MantissaDigits + Epsilon> FloatLimits for T {}

/// Approximate number of significant digits in base 10 of a floating-point type.
///
/// This is the maximum `x` such that any decimal number with `x` significant
/// digits can be converted to `f32` and back without loss.
///
/// Equal to `floor(log10(2^(MANTISSA_DIGITS − 1)))`.
///
/// This trait is only available for floating-point types. It would make no
/// sense for integer types, since their accuracy is the same for any number
/// of digits.
pub trait Digits {
    /// Approximate number of significant digits in base 10 of a floating-point type.
    const DIGITS: u32;
}

/// Number of significant digits in base 2 of a floating-point type.
///
/// Note that the size of the mantissa in the bitwise representation is one
/// smaller than this since the leading 1 is not stored explicitly.
///
/// This trait is only available for floating-point types. It would make no
/// sense for integer types, since their accuracy is the same for any number
/// of digits.
pub trait MantissaDigits {
    /// Number of significant digits in base 2 of a floating-point type.
    const MANTISSA_DIGITS: u32;
}

/// [Machine epsilon](https://en.wikipedia.org/wiki/Machine_epsilon) value for
/// this floating-point type.
///
/// This is the difference between 1.0 and the next larger representable number.
///
/// This trait is only available for floating-point types. It would make no
/// sense for integer types, since their accuracy is the same for any number
/// of digits.
pub trait Epsilon {
    /// [Machine epsilon](https://en.wikipedia.org/wiki/Machine_epsilon) value
    /// for this floating-point type.
    const EPSILON: Self;
}

/// Bundle of limits for a numeric type.
pub trait Limits:
    MinFinite + MaxFinite + MinExtended + MaxExtended + MinExactInteger + MaxExactInteger + Bits
{
}
impl<
    T: MinFinite + MaxFinite + MinExtended + MaxExtended + MinExactInteger + MaxExactInteger + Bits,
> Limits for T
{
}

/// Smallest finite value.
pub trait MinFinite {
    /// Smallest finite value.
    const MIN_FINITE: Self;
}

/// Largest finite value.
pub trait MaxFinite {
    /// Largest finite value.
    const MAX_FINITE: Self;
}

/// Negative infinity if present in the type, otherwise smallest finite value.
pub trait MinExtended {
    /// Negative infinity if present in the type, otherwise smallest finite value.
    const MIN_EXTENDED: Self;
}

/// Positive infinity if present in the type, otherwise greatest finite value.
pub trait MaxExtended {
    /// Positive infinity if present in the type, otherwise greatest finite value.
    const MAX_EXTENDED: Self;
}

/// Minimum integer that can be represented exactly in this type, with no other
/// integer converting to the same value.
///
/// For an integer `x` which satisfies
/// `MIN_EXACT_INTEGER <= x <= MAX_EXACT_INTEGER`, there is a one-to-one mapping
/// between that integer and a value of this type. `MAX_EXACT_INTEGER + 1` also
/// converts losslessly, but `MAX_EXACT_INTEGER + 2` converts to the same value
/// as `MAX_EXACT_INTEGER + 1`, so there is not a one-to-one mapping.
///
/// For integer types this is the same as the type's minimum. For floating
/// point types this is equal to `1 - 2^MANTISSA_DIGITS` (that is,
/// `-MAX_EXACT_INTEGER`).
pub trait MinExactInteger {
    /// Minimum integer that can be represented exactly in this type, with no
    /// other integer converting to the same value.
    const MIN_EXACT_INTEGER: Self;
}

/// Maximum integer that can be represented exactly in this type, with no other
/// integer converting to the same value.
///
/// For an integer `x` which satisfies
/// `MIN_EXACT_INTEGER <= x <= MAX_EXACT_INTEGER`, there is a one-to-one mapping
/// between that integer and a value of this type. `MAX_EXACT_INTEGER + 1` also
/// converts losslessly, but `MAX_EXACT_INTEGER + 2` converts to the same value
/// as `MAX_EXACT_INTEGER + 1`, so there is not a one-to-one mapping.
///
/// For integer types this is the same as the type's maximum. For floating point
/// types this is equal to `2^MANTISSA_DIGITS - 1`.
pub trait MaxExactInteger {
    /// Maximum integer that can be represented exactly in this type, with no
    /// other integer converting to the same value.
    const MAX_EXACT_INTEGER: Self;
}

/// The size of this type in bits.
pub trait Bits {
    /// The size of this type in bits.
    const BITS: u32;
}

macro_rules! impl_limits_traits_for_int {
    ($ty:ty) => {
        impl MinFinite for $ty {
            const MIN_FINITE: Self = <$ty>::MIN;
        }

        impl MaxFinite for $ty {
            const MAX_FINITE: Self = <$ty>::MAX;
        }

        impl MinExtended for $ty {
            const MIN_EXTENDED: Self = <$ty>::MIN;
        }

        impl MaxExtended for $ty {
            const MAX_EXTENDED: Self = <$ty>::MAX;
        }

        impl MinExactInteger for $ty {
            const MIN_EXACT_INTEGER: Self = <$ty>::MIN;
        }

        impl MaxExactInteger for $ty {
            const MAX_EXACT_INTEGER: Self = <$ty>::MAX;
        }

        impl Bits for $ty {
            const BITS: u32 = <$ty>::BITS;
        }

        // No `DIGITS` for ints.
        /*impl Digits for $ty {
            const DIGITS: u32 = <$ty>::DIGITS;
        }*/

        // No `MANTISSA_DIGITS` for ints.
        /*impl Digits for $ty {
            const MANTISSA_DIGITS: u32 = <$ty>::MANTISSA_DIGITS;
        }*/
    };
    ($ty:ty, $nonnegative_tests_mod:ident) => {
        impl_limits_traits_for_int!($ty);

        test_limits_traits_int_nonnegative!($ty, $nonnegative_tests_mod);
    };
    ($ty:ty, $nonnegative_tests_mod:ident, $negative_tests_mod:ident) => {
        impl_limits_traits_for_int!($ty, $nonnegative_tests_mod);

        test_limits_traits_int_negative!($ty, $negative_tests_mod);
    };
}

macro_rules! test_limits_traits_int_nonnegative {
    ($ty:ty, $tests_mod:ident) => {
        #[cfg(test)]
        mod $tests_mod {
            use crate::limits::*;

            #[test]
            fn test_limits() {
                assert_eq!(<$ty as MinFinite>::MIN_FINITE, <$ty>::MIN);
                assert_eq!(<$ty as MinExtended>::MIN_EXTENDED, <$ty>::MIN);
                assert_eq!(<$ty as MinExactInteger>::MIN_EXACT_INTEGER, <$ty>::MIN);

                assert_eq!(<$ty as MaxFinite>::MAX_FINITE, <$ty>::MAX);
                assert_eq!(<$ty as MaxExtended>::MAX_EXTENDED, <$ty>::MAX);
                assert_eq!(<$ty as MaxExactInteger>::MAX_EXACT_INTEGER, <$ty>::MAX);

                assert_eq!(
                    <$ty as MaxFinite>::MAX_FINITE,
                    <$ty as MaxExtended>::MAX_EXTENDED
                );
                assert_eq!(
                    <$ty as MaxFinite>::MAX_FINITE,
                    <$ty as MaxExactInteger>::MAX_EXACT_INTEGER
                );
            }
        }
    };
}

macro_rules! test_limits_traits_int_negative {
    ($ty:ty, $tests_mod:ident) => {
        #[cfg(test)]
        mod $tests_mod {
            use crate::limits::*;

            #[test]
            fn test_limits() {
                assert_eq!(<$ty as MinFinite>::MIN_FINITE, <$ty>::MIN);
                assert_eq!(<$ty as MinExtended>::MIN_EXTENDED, <$ty>::MIN);
                assert_eq!(<$ty as MinExactInteger>::MIN_EXACT_INTEGER, <$ty>::MIN);

                assert_eq!(
                    <$ty as MinFinite>::MIN_FINITE,
                    <$ty as MinExtended>::MIN_EXTENDED
                );
                assert_eq!(
                    <$ty as MinFinite>::MIN_FINITE,
                    <$ty as MinExactInteger>::MIN_EXACT_INTEGER
                );

                assert!(<$ty as MinFinite>::MIN_FINITE < <$ty as MaxFinite>::MAX_FINITE);
            }
        }
    };
}

impl_limits_traits_for_int!(i8, i8_nonnegative_tests, i8_negative_tests);
impl_limits_traits_for_int!(i16, i16_nonnegative_tests, i16_negative_tests);
impl_limits_traits_for_int!(i32, i32_nonnegative_tests, i32_negative_tests);
impl_limits_traits_for_int!(i64, i64_nonnegative_tests, i64_negative_tests);
impl_limits_traits_for_int!(i128, i128_nonnegative_tests, i128_negative_tests);
impl_limits_traits_for_int!(isize, isize_nonnegative_tests, isize_negative_tests);

impl_limits_traits_for_int!(u8, u8_tests);
impl_limits_traits_for_int!(u16, u16_tests);
impl_limits_traits_for_int!(u32, u32_tests);
impl_limits_traits_for_int!(u64, u64_tests);
impl_limits_traits_for_int!(u128, u128_tests);
impl_limits_traits_for_int!(usize, usize_tests);

macro_rules! impl_limits_traits_for_float {
    ($ty:ty, $nonnegative_tests_mod:ident, $negative_tests_mod:ident) => {
        impl Bits for $ty {
            // `BITS` for floats hasn't been stabilized yet, so we calculate it
            // from `size_of`.
            const BITS: u32 = core::mem::size_of::<$ty>() as u32 * 8;
        }

        impl Digits for $ty {
            const DIGITS: u32 = <$ty>::DIGITS;
        }

        impl MantissaDigits for $ty {
            const MANTISSA_DIGITS: u32 = <$ty>::MANTISSA_DIGITS;
        }

        impl Epsilon for $ty {
            const EPSILON: Self = <$ty>::EPSILON;
        }

        impl MinFinite for $ty {
            const MIN_FINITE: Self = <$ty>::MIN;
        }

        impl MaxFinite for $ty {
            const MAX_FINITE: Self = <$ty>::MAX;
        }

        impl MinExtended for $ty {
            const MIN_EXTENDED: Self = <$ty>::NEG_INFINITY;
        }

        impl MaxExtended for $ty {
            const MAX_EXTENDED: Self = <$ty>::INFINITY;
        }

        impl MinExactInteger for $ty {
            const MIN_EXACT_INTEGER: Self = -<$ty as MaxExactInteger>::MAX_EXACT_INTEGER;
        }

        impl MaxExactInteger for $ty {
            const MAX_EXACT_INTEGER: Self =
                ((1i128 << (<$ty>::MANTISSA_DIGITS as u32)) - 1) as Self;
        }

        test_limits_traits_float_nonnegative!($ty, $nonnegative_tests_mod);
        test_limits_traits_float_negative!($ty, $negative_tests_mod);
    };
}

macro_rules! test_limits_traits_float_nonnegative {
    ($ty:ty, $tests_mod:ident) => {
        #[cfg(test)]
        mod $tests_mod {
            use crate::limits::*;

            #[test]
            fn test_limits() {
                // These constants are always non-negative, so they are only
                // tested in nonnegative tests, not in negative tests.
                assert_eq!(<$ty as Bits>::BITS, core::mem::size_of::<$ty>() as u32 * 8);
                assert_eq!(<$ty as Digits>::DIGITS, <$ty>::DIGITS);
                assert_eq!(
                    <$ty as MantissaDigits>::MANTISSA_DIGITS,
                    <$ty>::MANTISSA_DIGITS
                );
                assert_eq!(<$ty as Epsilon>::EPSILON, <$ty>::EPSILON);

                assert_eq!(<$ty as MaxFinite>::MAX_FINITE, <$ty>::MAX);
                assert_eq!(<$ty as MaxExtended>::MAX_EXTENDED, <$ty>::INFINITY);
                assert_eq!(
                    <$ty as MaxExactInteger>::MAX_EXACT_INTEGER,
                    ((1i128 << (<$ty>::MANTISSA_DIGITS as u32)) - 1) as $ty
                );

                assert!(<$ty as MaxFinite>::MAX_FINITE.is_finite());
                assert!(<$ty as MaxExtended>::MAX_EXTENDED.is_infinite());
                assert!(<$ty as MaxExactInteger>::MAX_EXACT_INTEGER.is_finite());

                assert!(<$ty as MaxExtended>::MAX_EXTENDED > <$ty as MaxFinite>::MAX_FINITE);
                assert!(
                    <$ty as MaxExactInteger>::MAX_EXACT_INTEGER < <$ty as MaxFinite>::MAX_FINITE
                );
            }

            #[test]
            fn test_exact_integer() {
                let max = <$ty as MaxExactInteger>::MAX_EXACT_INTEGER;
                let max_int = max as i128;

                // Let's check the identities described in `f32`/`f64`'s
                // `MAX_EXACT_INTEGER` doc comment.

                assert_eq!(max_int, max_int as $ty as i128);
                assert_eq!(max_int + 1, (max_int + 1) as $ty as i128);
                assert_ne!(max_int + 2, (max_int + 2) as $ty as i128);

                assert_eq!((max_int + 1) as $ty, (max_int + 2) as $ty);
            }
        }
    };
}

macro_rules! test_limits_traits_float_negative {
    ($ty:ty, $tests_mod:ident) => {
        #[cfg(test)]
        mod $tests_mod {
            use crate::limits::*;

            #[test]
            fn test_limits() {
                assert_eq!(<$ty as MinFinite>::MIN_FINITE, <$ty>::MIN);
                assert_eq!(<$ty as MinExtended>::MIN_EXTENDED, <$ty>::NEG_INFINITY);
                assert_eq!(
                    <$ty as MinExactInteger>::MIN_EXACT_INTEGER,
                    -<$ty as MaxExactInteger>::MAX_EXACT_INTEGER
                );

                assert!(<$ty as MinFinite>::MIN_FINITE.is_finite());
                assert!(<$ty as MinExtended>::MIN_EXTENDED.is_infinite());
                assert!(<$ty as MinExactInteger>::MIN_EXACT_INTEGER.is_finite());

                assert!(<$ty as MinExtended>::MIN_EXTENDED < <$ty as MinFinite>::MIN_FINITE);
                assert!(
                    <$ty as MinExactInteger>::MIN_EXACT_INTEGER > <$ty as MinFinite>::MIN_FINITE
                );
            }

            #[test]
            fn test_exact_integer() {
                let min = <$ty as MinExactInteger>::MIN_EXACT_INTEGER;
                let min_int = min as i128;

                // Let's check the identities described in `f32`/`f64`'s
                // `MIN_EXACT_INTEGER` doc comment.

                assert_eq!(min_int, min_int as $ty as i128);
                assert_eq!(min_int - 1, (min_int - 1) as $ty as i128);
                assert_ne!(min_int - 2, (min_int - 2) as $ty as i128);

                assert_eq!((min_int - 1) as $ty, (min_int - 2) as $ty);
            }
        }
    };
}

impl_limits_traits_for_float!(f32, f32_nonnegative_tests, f32_negative_tests);
impl_limits_traits_for_float!(f64, f64_nonnegative_tests, f64_negative_tests);
