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
pub trait Zero: AssocConstZero + ZeroFns {}
impl<T: AssocConstZero + ZeroFns> Zero for T {}

/// Defines a distinguished `0` value as an associated constant.
pub trait AssocConstZero {
    /// The type of the distinguished `0` value.
    type Value;

    /// A distinguished `0` value.
    const ZERO: Self::Value;
}

impl<B: AssocConstZero<Value = B>> AssocConstZero for Object<B> {
    type Value = Self;

    const ZERO: Self::Value = Object(B::ZERO);
}

impl<B: AssocConstZero<Value = B>> Object<B> {
    /// A distinguished `0` value.
    pub const ZERO: Self = Object(B::ZERO);
}

/// Static functions defining a distinguished `0` value.
pub trait ZeroFns: FnZero + IsZero {}
impl<T: FnZero + IsZero> ZeroFns for T {}

/// Returns a distinguished `0` value.
pub trait FnZero {
    /// Returns a distinguished `0` value.
    fn zero() -> Self;
}

impl<B: FnZero> FnZero for Object<B> {
    fn zero() -> Self {
        Object(B::zero())
    }
}

impl<B: FnZero> Object<B> {
    /// Returns a distinguished `0` value.
    pub fn zero() -> Self {
        Object(B::zero())
    }
}

/// Returns `true` if `self` is zero and `false` otherwise.
pub trait IsZero {
    /// Returns `true` if `self` is zero and `false` otherwise.
    fn is_zero(self) -> bool;
}

impl<B: IsZero> IsZero for Object<B> {
    fn is_zero(self) -> bool {
        <B as IsZero>::is_zero(self.0)
    }
}

impl<B: IsZero> Object<B> {
    /// Returns `true` if `self` is zero and `false` otherwise.
    pub fn is_zero(self) -> bool {
        <B as IsZero>::is_zero(self.0)
    }
}

/// Defines a distinguished `1` value.
///
/// This is usually the multiplicative identity, but it does not have to be
/// so. It may also merely be the generating element (generator) of addition,
/// or an element that maps to the classical real one, as it sometimes is in
/// descriptions of tropical semirings.
pub trait One: AssocConstOne + OneFns {}
impl<T: AssocConstOne + OneFns> One for T {}

/// Defines a distinguished `1` value as an associated constant.
pub trait AssocConstOne {
    /// The type of the distinguished `1` value.
    type Value;

    /// A distinguished `1` value.
    const ONE: Self::Value;
}

impl<B: AssocConstOne<Value = B>> AssocConstOne for Object<B> {
    type Value = Self;

    const ONE: Self::Value = Object(B::ONE);
}

impl<B: AssocConstOne<Value = B>> Object<B> {
    /// A distinguished `1` value.
    pub const ONE: Self = Object(B::ONE);
}

/// Static functions defining a distinguished `1` value.
pub trait OneFns: FnOne + IsOne {}
impl<T: FnOne + IsOne> OneFns for T {}

/// Returns a distinguished `1` value.
pub trait FnOne {
    /// Returns a distinguished `1` value.
    fn one() -> Self;
}

impl<B: FnOne> FnOne for Object<B> {
    fn one() -> Self {
        Object(B::one())
    }
}

impl<B: FnOne> Object<B> {
    /// Returns a distinguished `1` value.
    pub fn one() -> Self {
        Object(B::one())
    }
}

/// Returns `true` if `self` is one and `false` otherwise.
pub trait IsOne {
    /// Returns `true` if `self` is one and `false` otherwise.
    fn is_one(self) -> bool;
}

impl<B: IsOne> IsOne for Object<B> {
    fn is_one(self) -> bool {
        <B as IsOne>::is_one(self.0)
    }
}

impl<B: IsOne> Object<B> {
    /// Returns `true` if `self` is one and `false` otherwise.
    pub fn is_one(self) -> bool {
        <B as IsOne>::is_one(self.0)
    }
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

/// Not a Number (NaN) value for a floating-point type as an associated
/// constant.
pub trait AssocConstNan {
    /// Not a Number (NaN) value for a floating-point type as an associated
    /// constant.
    const NAN: Self;
}

/// Not a Number (NaN) value for a floating-point type.
pub trait FnNan {
    /// Returns a Not a Number (NaN) value for a floating-point type.
    fn nan() -> Self;
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
        impl AssocConstZero for $ty {
            type Value = Self;

            const ZERO: Self::Value = $zero;
        }

        impl FnZero for $ty {
            #[inline]
            fn zero() -> Self {
                $zero
            }
        }

        impl AssocConstOne for $ty {
            type Value = Self;

            const ONE: Self::Value = $one;
        }

        impl FnOne for $ty {
            #[inline]
            fn one() -> Self {
                $one
            }
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

                assert_eq!(<$ty as FnZero>::zero(), zero);
                assert_eq!(<$ty as FnOne>::one(), one);

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
        impl FnZero for $ty {
            #[inline]
            fn zero() -> Self {
                0.0
            }
        }

        impl AssocConstZero for $ty {
            type Value = Self;

            const ZERO: Self::Value = 0.0;
        }

        impl FnOne for $ty {
            #[inline]
            fn one() -> Self {
                1.0
            }
        }

        impl AssocConstOne for $ty {
            type Value = Self;

            const ONE: Self::Value = 1.0;
        }

        impl MinusInfinity for $ty {
            const MINUS_INFINITY: Self = <$ty>::NEG_INFINITY;
        }

        impl PlusInfinity for $ty {
            const PLUS_INFINITY: Self = <$ty>::INFINITY;
        }

        impl FnNan for $ty {
            #[inline]
            fn nan() -> Self {
                <$ty>::NAN
            }
        }

        impl AssocConstNan for $ty {
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
