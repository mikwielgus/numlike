// SPDX-FileCopyrightText: 2026 numlike contributors
//
// SPDX-License-Identifier: MIT OR Apache-2.0

pub trait Zero {
    const ZERO: Self;
}

pub trait One {
    const ONE: Self;
}

macro_rules! impl_elem_traits_for_ints {
    ($($ty:ty),*) => {
        $(
            impl Zero for $ty {
                const ZERO: Self = 0;
            }

            impl One for $ty {
                const ONE: Self = 1;
            }
        )*
    };
}

macro_rules! impl_elem_traits_for_floats {
    ($($ty:ty),*) => {
        $(
            impl Zero for $ty {
                const ZERO: Self = 0.0;
            }

            impl One for $ty {
                const ONE: Self = 1.0;
            }
        )*
    };
}

impl_elem_traits_for_ints!(i8, i16, i32, i64, i128, isize);
impl_elem_traits_for_ints!(u8, u16, u32, u64, u128, usize);
impl_elem_traits_for_floats!(f32, f64);
