// SPDX-FileCopyrightText: 2026 numlike contributors
//
// SPDX-License-Identifier: MIT OR Apache-2.0

//! Distinguished elements such as zero, one, NaN, infinities.

use core::num::{Saturating, Wrapping};

use crate::types::Object;

/// Defines a distinguished `0` value.
///
/// This is usually the additive identity, but it does not have to be so. It
/// may also merely be the absorbing element (absorber) of multiplication, as
/// it commonly is in definitions of absorption magmas and absorption monoids,
/// or an element that maps to the classical real zero, as it sometimes is in
/// descriptions of tropical semirings.
pub trait Zero {
    /// Returns a distinguished `0` value.
    fn zero() -> Self;
}

/// Defines a distinguished `0` value like [`Zero`] does, but as an associated
/// constant.
pub trait AssocConstZero {
    /// A distinguished `0` value.
    const ZERO: Self;
}

impl<B: AssocConstZero> AssocConstZero for Object<B> {
    const ZERO: Self = Object(B::ZERO);
}

impl<B: AssocConstZero> Object<B> {
    /// A distinguished `0` value.
    pub const ZERO: Self = Object(B::ZERO);
}

/// Defines a distinguished `1` value.
///
/// This is usually the multiplicative identity, but it does not have to be
/// so. It may also merely be the generating element (generator) of addition,
/// or an element that maps to the classical real one, as it sometimes is in
/// descriptions of tropical semirings.
pub trait One {
    /// Returns a distinguished `1` value.
    fn one() -> Self;
}

/// Defines a distinguished `1` value like [`One`] does, but as an associated
/// constant.
pub trait AssocConstOne {
    /// A distinguished `1` value.
    const ONE: Self;
}

impl<B: AssocConstOne> AssocConstOne for Object<B> {
    const ONE: Self = Object(B::ONE);
}

impl<B: AssocConstOne> Object<B> {
    /// A distinguished `1` value.
    pub const ONE: Self = Object(B::ONE);
}

/// Negative infinity (-∞).
///
/// This is an infinity going in the negative direction. This trait is not for
/// a directionless infinity, such as the one present in Riemann sphere and in
/// projectively extended reals.
pub trait MinusInfinity {
    /// Negative infinity (-∞).
    const MINUS_INFINITY: Self;
}

/// Positive infinity (+∞).
///
/// This is an infinity going in the positive direction. This trait is not
/// for a directionless infinity, such as the one present in Riemann sphere and in
/// projectively extended reals.
pub trait PlusInfinity {
    /// Positive infinity (+∞).
    const PLUS_INFINITY: Self;
}

/// Not a Number (NaN) value for a floating-point type.
pub trait Nan {
    /// Returns a Not a Number (NaN) value for a floating-point type.
    fn nan() -> Self;
}

/// Not a Number (NaN) value for a floating-point type as an associated
/// constant.
pub trait ConstNan {
    /// Not a Number (NaN) value for a floating-point type as an associated
    /// constant.
    const NAN: Self;
}

/// Returns `true` if `self` is NaN and `false` otherwise.
///
/// Types that don't have NaN values can implement this trait, always just
/// returning false.
pub trait IsNan {
    /// Returns `true` if `self` is NaN and `false` otherwise.
    fn is_nan(self) -> bool;
}

macro_rules! impl_elem_traits_for_int {
    ($ty:ty) => {
        impl_elem_traits_for_int!($ty, 0, 1);
    };
    ($ty:ty, $nonnegative_tests_mod:ident) => {
        impl_elem_traits_for_int!($ty);

        test_elem_traits_nonnegative!($ty, $nonnegative_tests_mod);
    };
    ($ty:ty, $nonnegative_tests_mod:ident, $negative_tests_mod:ident) => {
        impl_elem_traits_for_int!($ty, $nonnegative_tests_mod);

        test_elem_traits_negative!($ty, $negative_tests_mod);
    };
    ($ty:ty, $zero:expr, $one:expr) => {
        impl Zero for $ty {
            #[inline]
            fn zero() -> Self {
                $zero
            }
        }

        impl AssocConstZero for $ty {
            const ZERO: Self = $zero;
        }

        impl One for $ty {
            #[inline]
            fn one() -> Self {
                $one
            }
        }

        impl AssocConstOne for $ty {
            const ONE: Self = $one;
        }

        // Ints can implement `IsNan`, they just always return false.
        impl IsNan for $ty {
            #[inline]
            fn is_nan(self) -> bool {
                false
            }
        }

        // No implementations of NaN, +infinity, -infinity, obviously, since
        // these values don't exist for ints.
    };
    ($ty:ty, $zero:expr, $one:expr, $nonnegative_tests_mod:ident) => {
        impl_elem_traits_for_int!($ty, $zero, $one);

        test_elem_traits_nonnegative!($ty, $nonnegative_tests_mod);
    };
    ($ty:ty, $zero:expr, $one:expr, $nonnegative_tests_mod:ident, $negative_tests_mod:ident) => {
        impl_elem_traits_for_int!($ty, $zero, $one, $nonnegative_tests_mod);

        test_elem_traits_negative!($ty, $negative_tests_mod);
    };
}

macro_rules! test_elem_traits_nonnegative {
    ($ty:ty, $tests_mod:ident) => {
        #[cfg(test)]
        mod $tests_mod {
            use crate::elem::*;

            // TODO: Perhaps test `NAN`, `PLUS_INFINITY`, `MINUS_INFINITY`
            // somehow.

            #[test]
            fn test_zero_one() {
                let zero = <$ty as AssocConstZero>::ZERO;
                let one = <$ty as AssocConstOne>::ONE;

                assert_eq!(<$ty as Zero>::zero(), zero);
                assert_eq!(<$ty as One>::one(), one);

                assert_eq!(one - one, zero);
                assert_eq!(one + zero, one);
                assert_eq!(zero + one, one);
                assert_eq!(zero + zero, zero);
                assert_eq!(one * zero, zero);
                assert_eq!(zero * one, zero);
                assert_eq!(one * one, one);
            }

            #[test]
            fn test_is_nan() {
                let zero = <$ty as AssocConstZero>::ZERO;
                let one = <$ty as AssocConstOne>::ONE;

                assert!(!IsNan::is_nan(zero));
                assert!(!IsNan::is_nan(one));
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
            fn test_zero_one() {
                let zero = <$ty as AssocConstZero>::ZERO;
                let one = <$ty as AssocConstOne>::ONE;

                assert_eq!(-zero, zero);
                assert_eq!(-(-one), one);
                assert_eq!(-one + one, zero);
                assert_eq!(one + (-one), zero);
                assert_eq!(one - (-one), one + one);
            }
        }
    };
}

for_signed_ints!(impl_elem_traits_for_int);

for_unsigned_ints!(impl_elem_traits_for_int);

impl_elem_traits_for_int!(
    Wrapping<i8>,
    Wrapping(0),
    Wrapping(1),
    wrapping_i8_nonnegative_tests,
    wrapping_i8_negative_tests
);
impl_elem_traits_for_int!(
    Wrapping<i16>,
    Wrapping(0),
    Wrapping(1),
    wrapping_i16_nonnegative_tests,
    wrapping_i16_negative_tests
);
impl_elem_traits_for_int!(
    Wrapping<i32>,
    Wrapping(0),
    Wrapping(1),
    wrapping_i32_nonnegative_tests,
    wrapping_i32_negative_tests
);
impl_elem_traits_for_int!(
    Wrapping<i64>,
    Wrapping(0),
    Wrapping(1),
    wrapping_i64_nonnegative_tests,
    wrapping_i64_negative_tests
);
impl_elem_traits_for_int!(
    Wrapping<i128>,
    Wrapping(0),
    Wrapping(1),
    wrapping_i128_nonnegative_tests,
    wrapping_i128_negative_tests
);
impl_elem_traits_for_int!(
    Wrapping<isize>,
    Wrapping(0),
    Wrapping(1),
    wrapping_isize_nonnegative_tests,
    wrapping_isize_negative_tests
);

impl_elem_traits_for_int!(Wrapping<u8>, Wrapping(0), Wrapping(1), wrapping_u8_tests);
impl_elem_traits_for_int!(Wrapping<u16>, Wrapping(0), Wrapping(1), wrapping_u16_tests);
impl_elem_traits_for_int!(Wrapping<u32>, Wrapping(0), Wrapping(1), wrapping_u32_tests);
impl_elem_traits_for_int!(Wrapping<u64>, Wrapping(0), Wrapping(1), wrapping_u64_tests);
impl_elem_traits_for_int!(
    Wrapping<u128>,
    Wrapping(0),
    Wrapping(1),
    wrapping_u128_tests
);
impl_elem_traits_for_int!(
    Wrapping<usize>,
    Wrapping(0),
    Wrapping(1),
    wrapping_usize_tests
);

impl_elem_traits_for_int!(
    Saturating<i8>,
    Saturating(0),
    Saturating(1),
    saturating_i8_nonnegative_tests,
    saturating_i8_negative_tests
);
impl_elem_traits_for_int!(
    Saturating<i16>,
    Saturating(0),
    Saturating(1),
    saturating_i16_nonnegative_tests,
    saturating_i16_negative_tests
);
impl_elem_traits_for_int!(
    Saturating<i32>,
    Saturating(0),
    Saturating(1),
    saturating_i32_nonnegative_tests,
    saturating_i32_negative_tests
);
impl_elem_traits_for_int!(
    Saturating<i64>,
    Saturating(0),
    Saturating(1),
    saturating_i64_nonnegative_tests,
    saturating_i64_negative_tests
);
impl_elem_traits_for_int!(
    Saturating<i128>,
    Saturating(0),
    Saturating(1),
    saturating_i128_nonnegative_tests,
    saturating_i128_negative_tests
);
impl_elem_traits_for_int!(
    Saturating<isize>,
    Saturating(0),
    Saturating(1),
    saturating_isize_nonnegative_tests,
    saturating_isize_negative_tests
);

impl_elem_traits_for_int!(
    Saturating<u8>,
    Saturating(0),
    Saturating(1),
    saturating_u8_tests
);
impl_elem_traits_for_int!(
    Saturating<u16>,
    Saturating(0),
    Saturating(1),
    saturating_u16_tests
);
impl_elem_traits_for_int!(
    Saturating<u32>,
    Saturating(0),
    Saturating(1),
    saturating_u32_tests
);
impl_elem_traits_for_int!(
    Saturating<u64>,
    Saturating(0),
    Saturating(1),
    saturating_u64_tests
);
impl_elem_traits_for_int!(
    Saturating<u128>,
    Saturating(0),
    Saturating(1),
    saturating_u128_tests
);
impl_elem_traits_for_int!(
    Saturating<usize>,
    Saturating(0),
    Saturating(1),
    saturating_usize_tests
);

macro_rules! impl_elem_traits_for_float {
    ($ty:ty, $nonnegative_tests_mod:ident, $negative_tests_mod:ident) => {
        impl Zero for $ty {
            #[inline]
            fn zero() -> Self {
                0.0
            }
        }

        impl AssocConstZero for $ty {
            const ZERO: Self = 0.0;
        }

        impl One for $ty {
            #[inline]
            fn one() -> Self {
                1.0
            }
        }

        impl AssocConstOne for $ty {
            const ONE: Self = 1.0;
        }

        impl MinusInfinity for $ty {
            const MINUS_INFINITY: Self = <$ty>::NEG_INFINITY;
        }

        impl PlusInfinity for $ty {
            const PLUS_INFINITY: Self = <$ty>::INFINITY;
        }

        impl Nan for $ty {
            #[inline]
            fn nan() -> Self {
                <$ty>::NAN
            }
        }

        impl ConstNan for $ty {
            const NAN: Self = <$ty>::NAN;
        }

        impl IsNan for $ty {
            #[inline]
            fn is_nan(self) -> bool {
                <$ty>::is_nan(self)
            }
        }

        test_elem_traits_nonnegative!($ty, $nonnegative_tests_mod);
        test_elem_traits_negative!($ty, $negative_tests_mod);
    };
}

impl_elem_traits_for_float!(f32, f32_nonnegative_tests, f32_negative_tests);
impl_elem_traits_for_float!(f64, f64_nonnegative_tests, f64_negative_tests);
