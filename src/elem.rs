// SPDX-FileCopyrightText: 2026 numlike contributors
//
// SPDX-License-Identifier: MIT OR Apache-2.0

//! Distinguished elements such as zero and one.

/// Defines a distinguished `0` value.
///
/// This is usually the additive identity, but it does not have to be so. It
/// may also merely be the absorbing element (absorber) of multiplication, as
/// it commonly is in definitions of absorption magmas and absorption monoids,
/// or an element that maps to the classical real zero, as it sometimes is in
/// descriptions of tropical semirings.
pub trait Zero {
    /// A distinguished `0` value.
    const ZERO: Self;
}

/// Defines a distinguished `1` value.
///
/// This is usually the multiplicative identity, but it does not have to be
/// so. It may also merely be the generating element (generator) of addition,
/// or an element that maps to the classical real one, as it sometimes is in
/// descriptions of tropical semirings.
pub trait One {
    /// A distinguished `0` value.
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
