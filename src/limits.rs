// SPDX-FileCopyrightText: 2026 numlike contributors
//
// SPDX-License-Identifier: MIT OR Apache-2.0

//! Finite and extended numeric limits.

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

/// Bundle of limits for a numeric type.
pub trait Limits:
    MinFinite + MaxFinite + MinExtended + MaxExtended + MinExactInteger + MaxExactInteger
{
}
impl<T: MinFinite + MaxFinite + MinExtended + MaxExtended + MinExactInteger + MaxExactInteger>
    Limits for T
{
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

        impl MaxExactInteger for $ty {
            const MAX_EXACT_INTEGER: Self =
                ((1i128 << (<$ty>::MANTISSA_DIGITS as u32)) - 1) as Self;
        }

        impl MinExactInteger for $ty {
            const MIN_EXACT_INTEGER: Self = -<$ty as MaxExactInteger>::MAX_EXACT_INTEGER;
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
