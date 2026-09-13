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

macro_rules! impl_elem_traits_for_int {
    ($ty:ty) => {
        impl Zero for $ty {
            const ZERO: Self = 0;
        }

        impl One for $ty {
            const ONE: Self = 1;
        }
    };
    ($ty:ty, $nonnegative_tests_mod:ident) => {
        impl_elem_traits_for_int!($ty);

        test_elem_traits_nonnegative!($ty, $nonnegative_tests_mod);
    };
    ($ty:ty, $nonnegative_tests_mod:ident, $negative_tests_mod:ident) => {
        impl_elem_traits_for_int!($ty, $nonnegative_tests_mod);

        test_elem_traits_negative!($ty, $negative_tests_mod);
    };
}

macro_rules! test_elem_traits_nonnegative {
    ($ty:ty, $tests_mod:ident) => {
        #[cfg(test)]
        mod $tests_mod {
            use crate::elem::*;

            #[test]
            fn test_nonnegative_zero_one() {
                let zero = <$ty as Zero>::ZERO;
                let one = <$ty as One>::ONE;

                assert_eq!(one - one, zero);
                assert_eq!(one + zero, one);
                assert_eq!(zero + one, one);
                assert_eq!(zero + zero, zero);
                assert_eq!(one * zero, zero);
                assert_eq!(zero * one, zero);
                assert_eq!(one * one, one);
            }
        }
    };
}

macro_rules! test_elem_traits_negative {
    ($ty:ty, $tests_mod:ident) => {
        #[cfg(test)]
        mod $tests_mod {
            use crate::elem::*;

            #[test]
            fn test_negative_zero_one() {
                let zero = <$ty as Zero>::ZERO;
                let one = <$ty as One>::ONE;

                assert_eq!(-zero, zero);
                assert_eq!(-(-one), one);
                assert_eq!(-one + one, zero);
                assert_eq!(one + (-one), zero);
                assert_eq!(one - (-one), one + one);
            }
        }
    };
}

impl_elem_traits_for_int!(i8, i8_nonnegative_tests, i8_negative_tests);
impl_elem_traits_for_int!(i16, i16_nonnegative_tests, i16_negative_tests);
impl_elem_traits_for_int!(i32, i32_nonnegative_tests, i32_negative_tests);
impl_elem_traits_for_int!(i64, i64_nonnegative_tests, i64_negative_tests);
impl_elem_traits_for_int!(i128, i128_nonnegative_tests, i128_negative_tests);
impl_elem_traits_for_int!(isize, isize_nonnegative_tests, isize_negative_tests);

impl_elem_traits_for_int!(u8, u8_tests);
impl_elem_traits_for_int!(u16, u16_tests);
impl_elem_traits_for_int!(u32, u32_tests);
impl_elem_traits_for_int!(u64, u64_tests);
impl_elem_traits_for_int!(u128, u128_tests);
impl_elem_traits_for_int!(usize, usize_tests);

macro_rules! impl_elem_traits_for_float {
    ($ty:ty, $nonnegative_tests_mod:ident, $negative_tests_mod:ident) => {
        impl Zero for $ty {
            const ZERO: Self = 0.0;
        }

        impl One for $ty {
            const ONE: Self = 1.0;
        }

        test_elem_traits_nonnegative!($ty, $nonnegative_tests_mod);
        test_elem_traits_negative!($ty, $negative_tests_mod);
    };
}

impl_elem_traits_for_float!(f32, f32_nonnegative_tests, f32_negative_tests);
impl_elem_traits_for_float!(f64, f64_nonnegative_tests, f64_negative_tests);
