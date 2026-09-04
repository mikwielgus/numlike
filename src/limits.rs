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

/// Bundle of limits for a numeric type.
pub trait Limits: MinFinite + MaxFinite + MinExtended + MaxExtended {}
impl<T: MinFinite + MaxFinite + MinExtended + MaxExtended> Limits for T {}

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
        )*
    };
}

impl_limits_traits_for_ints!(i8, i16, i32, i64, i128, isize);
impl_limits_traits_for_ints!(u8, u16, u32, u64, u128, usize);
impl_limits_traits_for_floats!(f32, f64);
