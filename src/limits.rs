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
/// For integer types this is the same as the type's minimum. For floating point
/// types this is equal to `1 - 2^MANTISSA_DIGITS` (that is, `-MAX_EXACT_INTEGER`).
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

macro_rules! impl_limits_traits_for_ints {
    ($($ty:ty),*) => {
        $(
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
        )*
    };
}

macro_rules! impl_limits_traits_for_floats {
    ($($ty:ty),*) => {
        $(
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
                const MIN_EXACT_INTEGER: Self =
                    -<$ty as MaxExactInteger>::MAX_EXACT_INTEGER;
            }
        )*
    };
}

impl_limits_traits_for_ints!(i8, i16, i32, i64, i128, isize);
impl_limits_traits_for_ints!(u8, u16, u32, u64, u128, usize);
impl_limits_traits_for_floats!(f32, f64);
