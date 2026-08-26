// SPDX-FileCopyrightText: 2026 numlike contributors
//
// SPDX-License-Identifier: MIT OR Apache-2.0

pub trait MinFinite {
    const MIN_FINITE: Self;
}

pub trait MaxFinite {
    const MAX_FINITE: Self;
}

pub trait MinExtended {
    const MIN_EXTENDED: Self;
}

pub trait MaxExtended {
    const MAX_EXTENDED: Self;
}

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
