// SPDX-FileCopyrightText: 2026 numlike contributors
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use crate::types::Object;

/// Bundle of overflowing arithmetic and fused arithmetic operations.
pub trait FullOverflowingArithOps<Rhs = Self>:
    OverflowingArithOps<Rhs> + OverflowingFusedArithOps<Rhs>
{
}

/// Bundle of overflowing fused arithmetic operations.
pub trait OverflowingFusedArithOps<Rhs = Self>: OverflowingMulAdd<Rhs, Rhs> {}
impl<Rhs, T: OverflowingMulAdd<Rhs, Rhs>> OverflowingFusedArithOps<Rhs> for T {}

/// Overflowing fused multiply-add. Computes `(self * a) + b`.
///
/// Returns a tuple of the result along with a boolean indicating whether an
/// arithmetic overflow would occur. If an overflow would have occurred then
/// the wrapped value is returned.
pub trait OverflowingMulAdd<A = Self, B = Self> {
    /// The resulting type after applying the operation.
    type Output;

    /// Overflowing fused multiply-add. Computes `(self * a) + b`.
    fn overflowing_mul_add(self, a: A, b: B) -> (Self::Output, bool);
}

impl<B: OverflowingMulAdd<Output = B>> OverflowingMulAdd for Object<B> {
    type Output = Self;

    fn overflowing_mul_add(self, a: Self, b: Self) -> (Self::Output, bool) {
        let (value, overflow) = <B as OverflowingMulAdd>::overflowing_mul_add(self.0, a.0, b.0);

        (Object(value), overflow)
    }
}

impl<B: OverflowingMulAdd<Output = B>> Object<B> {
    /// Overflowing fused multiply-add. Computes `(self * a) + b`.
    pub fn overflowing_mul_add(self, a: Self, b: Self) -> (Self, bool) {
        let (value, overflow) = <B as OverflowingMulAdd>::overflowing_mul_add(self.0, a.0, b.0);

        (Object(value), overflow)
    }
}

/// Bundle of overflowing arithmetic operations.
pub trait OverflowingArithOps<Rhs = Self>:
    OverflowingEuclidOps<Rhs>
    + OverflowingFieldOps<Rhs>
    + OverflowingRem<Rhs, Output = Self>
    + OverflowingDivRem<Rhs, Output = Self>
{
}
impl<
    Rhs,
    T: OverflowingEuclidOps<Rhs>
        + OverflowingFieldOps<Rhs>
        + OverflowingRem<Rhs, Output = Self>
        + OverflowingDivRem<Rhs, Output = Self>,
> OverflowingArithOps<Rhs> for T
{
}

/// Calculates the remainder when `self` is divided by `other`.
///
/// Returns a tuple of the remainder after dividing along with a boolean indicating whether an
/// arithmetic overflow would occur. If an overflow would occur then zero is returned.
///
/// # Panics
///
/// This function will panic if `other` is zero.
pub trait OverflowingRem<Rhs = Self> {
    /// The resulting type after applying the operation.
    type Output;

    /// Calculates the remainder when `self` is divided by `other`.
    fn overflowing_rem(self, other: Rhs) -> (Self::Output, bool);
}

impl<B: OverflowingRem<Output = B>> OverflowingRem for Object<B> {
    type Output = Self;

    fn overflowing_rem(self, other: Self) -> (Self::Output, bool) {
        let (value, overflow) = <B as OverflowingRem>::overflowing_rem(self.0, other.0);

        (Object(value), overflow)
    }
}

impl<B: OverflowingRem<Output = B>> Object<B> {
    /// Calculates the remainder when `self` is divided by `other`.
    pub fn overflowing_rem(self, other: Self) -> (Self, bool) {
        let (value, overflow) = <B as OverflowingRem>::overflowing_rem(self.0, other.0);

        (Object(value), overflow)
    }
}

/// Calculates the quotient and remainder when `self` is divided by `other`.
///
/// Returns a tuple of `(quotient, remainder)` along with a boolean indicating whether an
/// arithmetic overflow would occur.
///
/// # Panics
///
/// This function will panic if `other` is zero.
pub trait OverflowingDivRem<Rhs> {
    /// The resulting type after applying the operation.
    type Output;

    /// Calculates the quotient and remainder when `self` is divided by `other`.
    fn overflowing_div_rem(self, other: Rhs) -> ((Self::Output, Self::Output), bool);
}

impl<B: OverflowingDivRem<B, Output = B>> OverflowingDivRem<Object<B>> for Object<B> {
    type Output = Self;

    fn overflowing_div_rem(self, other: Self) -> ((Self::Output, Self::Output), bool) {
        let ((a, b), overflow) = <B as OverflowingDivRem<B>>::overflowing_div_rem(self.0, other.0);

        ((Object(a), Object(b)), overflow)
    }
}

impl<B: OverflowingDivRem<B, Output = B>> Object<B> {
    /// Calculates the quotient and remainder when `self` is divided by `other`.
    pub fn overflowing_div_rem(self, other: Self) -> ((Self, Self), bool) {
        let ((a, b), overflow) = <B as OverflowingDivRem<B>>::overflowing_div_rem(self.0, other.0);

        ((Object(a), Object(b)), overflow)
    }
}

/// Bundle of overflowing Euclidean division operations.
pub trait OverflowingEuclidOps<Rhs>:
    OverflowingDivEuclid<Rhs> + OverflowingRemEuclid<Rhs> + OverflowingDivRemEuclid<Rhs>
{
}
impl<Rhs, T: OverflowingDivEuclid<Rhs> + OverflowingRemEuclid<Rhs> + OverflowingDivRemEuclid<Rhs>>
    OverflowingEuclidOps<Rhs> for T
{
}

/// Calculates the quotient of Euclidean division `self.div_euclid(other)`.
///
/// Returns a tuple of the divisor along with a boolean indicating whether an arithmetic overflow would
/// occur. If an overflow would occur then `self` is returned.
///
/// # Panics
///
/// This function will panic if `other` is zero.
pub trait OverflowingDivEuclid<Rhs = Self> {
    /// The resulting type after applying the operation.
    type Output;

    /// Calculates the quotient of Euclidean division `self.div_euclid(other)`.
    fn overflowing_div_euclid(self, other: Rhs) -> (Self::Output, bool);
}

impl<B: OverflowingDivEuclid<Output = B>> OverflowingDivEuclid for Object<B> {
    type Output = Self;

    fn overflowing_div_euclid(self, other: Self) -> (Self::Output, bool) {
        let (value, overflow) =
            <B as OverflowingDivEuclid>::overflowing_div_euclid(self.0, other.0);

        (Object(value), overflow)
    }
}

impl<B: OverflowingDivEuclid<Output = B>> Object<B> {
    /// Calculates the quotient of Euclidean division `self.div_euclid(other)`.
    pub fn overflowing_div_euclid(self, other: Self) -> (Self, bool) {
        let (value, overflow) =
            <B as OverflowingDivEuclid>::overflowing_div_euclid(self.0, other.0);

        (Object(value), overflow)
    }
}

/// Overflowing Euclidean remainder. Calculates `self.rem_euclid(other)`.
///
/// Returns a tuple of the remainder after dividing along with a boolean indicating whether an
/// arithmetic overflow would occur. If an overflow would occur then 0 is returned.
///
/// # Panics
///
/// This function will panic if `other` is zero.
pub trait OverflowingRemEuclid<Rhs = Self> {
    /// The resulting type after applying the operation.
    type Output;

    /// Overflowing Euclidean remainder. Calculates `self.rem_euclid(other)`.
    fn overflowing_rem_euclid(self, other: Rhs) -> (Self::Output, bool);
}

impl<B: OverflowingRemEuclid<Output = B>> OverflowingRemEuclid for Object<B> {
    type Output = Self;

    fn overflowing_rem_euclid(self, other: Self) -> (Self::Output, bool) {
        let (value, overflow) =
            <B as OverflowingRemEuclid>::overflowing_rem_euclid(self.0, other.0);

        (Object(value), overflow)
    }
}

impl<B: OverflowingRemEuclid<Output = B>> Object<B> {
    /// Overflowing Euclidean remainder. Calculates `self.rem_euclid(other)`.
    pub fn overflowing_rem_euclid(self, other: Self) -> (Self, bool) {
        let (value, overflow) =
            <B as OverflowingRemEuclid>::overflowing_rem_euclid(self.0, other.0);

        (Object(value), overflow)
    }
}

/// Calculates the Euclidean quotient and remainder when `self` is divided by `other`.
///
/// Returns a tuple of `(quotient, remainder)` along with a boolean indicating whether an
/// arithmetic overflow would occur.
///
/// # Panics
///
/// This function will panic if `other` is zero.
pub trait OverflowingDivRemEuclid<Rhs = Self> {
    /// The resulting type after applying the operation.
    type Output;

    /// Calculates the Euclidean quotient and remainder when `self` is divided by `other`.
    fn overflowing_div_rem_euclid(self, other: Rhs) -> ((Self::Output, Self::Output), bool);
}

impl<B: OverflowingDivRemEuclid<Output = B>> OverflowingDivRemEuclid for Object<B> {
    type Output = Self;

    fn overflowing_div_rem_euclid(self, other: Self) -> ((Self::Output, Self::Output), bool) {
        let ((a, b), overflow) =
            <B as OverflowingDivRemEuclid>::overflowing_div_rem_euclid(self.0, other.0);

        ((Object(a), Object(b)), overflow)
    }
}

impl<B: OverflowingDivRemEuclid<Output = B>> Object<B> {
    /// Calculates the Euclidean quotient and remainder when `self` is divided by `other`.
    pub fn overflowing_div_rem_euclid(self, other: Self) -> ((Self, Self), bool) {
        let ((a, b), overflow) =
            <B as OverflowingDivRemEuclid>::overflowing_div_rem_euclid(self.0, other.0);

        ((Object(a), Object(b)), overflow)
    }
}

/// Bundle of overflowing field operations.
pub trait OverflowingFieldOps<Rhs = Self>:
    OverflowingRingOps<Rhs> + OverflowingDiv<Rhs, Output = Self>
{
}
impl<Rhs, T: OverflowingRingOps<Rhs> + OverflowingDiv<Rhs, Output = Self>> OverflowingFieldOps<Rhs>
    for T
{
}

/// Calculates the divisor when `self` is divided by `other`.
///
/// Returns a tuple of the divisor along with a boolean indicating whether an arithmetic overflow would
/// occur. If an overflow would occur then self is returned.
///
/// # Panics
///
/// This function will panic if `other` is zero.
pub trait OverflowingDiv<Rhs = Self> {
    /// The resulting type after applying the operation.
    type Output;

    /// Calculates the divisor when `self` is divided by `other`.
    fn overflowing_div(self, other: Rhs) -> (Self::Output, bool);
}

impl<B: OverflowingDiv<Output = B>> OverflowingDiv for Object<B> {
    type Output = Self;

    fn overflowing_div(self, other: Self) -> (Self::Output, bool) {
        let (value, overflow) = <B as OverflowingDiv>::overflowing_div(self.0, other.0);

        (Object(value), overflow)
    }
}

impl<B: OverflowingDiv<Output = B>> Object<B> {
    /// Calculates the divisor when `self` is divided by `other`.
    pub fn overflowing_div(self, other: Self) -> (Self, bool) {
        let (value, overflow) = <B as OverflowingDiv>::overflowing_div(self.0, other.0);

        (Object(value), overflow)
    }
}

/// Bundle of overflowing ring operations.
pub trait OverflowingRingOps<Rhs = Self>:
    OverflowingAdd<Rhs, Output = Self>
    + OverflowingSub<Rhs, Output = Self>
    + OverflowingMul<Rhs, Output = Self>
    + OverflowingNeg<Output = Self>
{
}
impl<
    Rhs,
    T: OverflowingAdd<Rhs, Output = Self>
        + OverflowingSub<Rhs, Output = Self>
        + OverflowingMul<Rhs, Output = Self>
        + OverflowingNeg<Output = Self>,
> OverflowingRingOps<Rhs> for T
{
}

/// Returns a tuple of the addition along with a boolean indicating
/// whether an arithmetic overflow would occur. If an overflow would have
/// occurred then the wrapped value is returned.
pub trait OverflowingAdd<Rhs = Self> {
    /// The resulting type after applying the operation.
    type Output;

    /// Returns a tuple of the addition along with a boolean indicating
    /// whether an arithmetic overflow would occur. If an overflow would have
    /// occurred then the wrapped value is returned.
    fn overflowing_add(self, other: Rhs) -> (Self::Output, bool);
}

impl<B: OverflowingAdd<Output = B>> OverflowingAdd for Object<B> {
    type Output = Self;

    fn overflowing_add(self, other: Self) -> (Self::Output, bool) {
        let (value, overflow) = <B as OverflowingAdd>::overflowing_add(self.0, other.0);

        (Object(value), overflow)
    }
}

impl<B: OverflowingAdd<Output = B>> Object<B> {
    /// Returns a tuple of the addition along with a boolean indicating
    /// whether an arithmetic overflow would occur. If an overflow would have
    /// occurred then the wrapped value is returned.
    pub fn overflowing_add(self, other: Self) -> (Self, bool) {
        let (value, overflow) = <B as OverflowingAdd>::overflowing_add(self.0, other.0);

        (Object(value), overflow)
    }
}

/// Returns a tuple of the subtraction along with a boolean indicating whether an arithmetic overflow
/// would occur. If an overflow would have occurred then the wrapped value is returned.
pub trait OverflowingSub<Rhs = Self> {
    /// The resulting type after applying the operation.
    type Output;

    /// Returns a tuple of the subtraction along with a boolean indicating whether an arithmetic overflow
    /// would occur. If an overflow would have occurred then the wrapped value is returned.
    fn overflowing_sub(self, other: Rhs) -> (Self::Output, bool);
}

impl<B: OverflowingSub<Output = B>> OverflowingSub for Object<B> {
    type Output = Self;

    fn overflowing_sub(self, other: Self) -> (Self::Output, bool) {
        let (value, overflow) = <B as OverflowingSub>::overflowing_sub(self.0, other.0);

        (Object(value), overflow)
    }
}

impl<B: OverflowingSub<Output = B>> Object<B> {
    /// Returns a tuple of the subtraction along with a boolean indicating whether an arithmetic overflow
    /// would occur. If an overflow would have occurred then the wrapped value is returned.
    pub fn overflowing_sub(self, other: Self) -> (Self, bool) {
        let (value, overflow) = <B as OverflowingSub>::overflowing_sub(self.0, other.0);

        (Object(value), overflow)
    }
}

/// Calculates the multiplication of `self` and `other`.
///
/// Returns a tuple of the multiplication along with a boolean indicating whether an arithmetic overflow
/// would occur. If an overflow would have occurred then the wrapped value is returned.
pub trait OverflowingMul<Rhs = Self> {
    /// The resulting type after applying the operation.
    type Output;

    /// Calculates the multiplication of `self` and `other`.
    fn overflowing_mul(self, other: Rhs) -> (Self::Output, bool);
}

impl<B: OverflowingMul<Output = B>> OverflowingMul for Object<B> {
    type Output = Self;

    fn overflowing_mul(self, other: Self) -> (Self::Output, bool) {
        let (value, overflow) = <B as OverflowingMul>::overflowing_mul(self.0, other.0);

        (Object(value), overflow)
    }
}

impl<B: OverflowingMul<Output = B>> Object<B> {
    /// Calculates the multiplication of `self` and `other`.
    pub fn overflowing_mul(self, other: Self) -> (Self, bool) {
        let (value, overflow) = <B as OverflowingMul>::overflowing_mul(self.0, other.0);

        (Object(value), overflow)
    }
}

/// Returns a tuple of the negated version of self along with a boolean indicating whether an overflow
/// happened. If `self` is the minimum value, then the minimum value will be returned again and `true`
/// will be returned for an overflow happening.
pub trait OverflowingNeg {
    /// The resulting type after applying the operation.
    type Output;

    /// Returns a tuple of the negated version of self along with a boolean indicating whether an overflow
    /// happened. If `self` is the minimum value, then the minimum value will be returned again and `true`
    /// will be returned for an overflow happening.
    fn overflowing_neg(self) -> (Self::Output, bool);
}

impl<B: OverflowingNeg<Output = B>> OverflowingNeg for Object<B> {
    type Output = Self;

    fn overflowing_neg(self) -> (Self::Output, bool) {
        let (value, overflow) = <B as OverflowingNeg>::overflowing_neg(self.0);

        (Object(value), overflow)
    }
}

impl<B: OverflowingNeg<Output = B>> Object<B> {
    /// Returns a tuple of the negated version of self along with a boolean indicating whether an overflow
    /// happened. If `self` is the minimum value, then the minimum value will be returned again and `true`
    /// will be returned for an overflow happening.
    pub fn overflowing_neg(self) -> (Self, bool) {
        let (value, overflow) = <B as OverflowingNeg>::overflowing_neg(self.0);

        (Object(value), overflow)
    }
}

macro_rules! impl_overflowing_field_traits_for_ints {
    ($($ty:ty),*) => {
        $(
            impl OverflowingAdd<$ty> for $ty {
                type Output = $ty;

                #[inline]
                fn overflowing_add(self, other: $ty) -> (Self::Output, bool) {
                    <$ty>::overflowing_add(self, other)
                }
            }

            impl OverflowingSub<$ty> for $ty {
                type Output = $ty;

                #[inline]
                fn overflowing_sub(self, other: $ty) -> (Self::Output, bool) {
                    <$ty>::overflowing_sub(self, other)
                }
            }

            impl OverflowingMul<$ty> for $ty {
                type Output = $ty;

                #[inline]
                fn overflowing_mul(self, other: $ty) -> (Self::Output, bool) {
                    <$ty>::overflowing_mul(self, other)
                }
            }

            impl OverflowingDiv<$ty> for $ty {
                type Output = $ty;

                #[inline]
                fn overflowing_div(self, other: $ty) -> (Self::Output, bool) {
                    <$ty>::overflowing_div(self, other)
                }
            }

            impl OverflowingRem<$ty> for $ty {
                type Output = $ty;

                #[inline]
                fn overflowing_rem(self, other: $ty) -> (Self::Output, bool) {
                    <$ty>::overflowing_rem(self, other)
                }
            }

            impl OverflowingNeg for $ty {
                type Output = $ty;

                #[inline]
                fn overflowing_neg(self) -> (Self::Output, bool) {
                    <$ty>::overflowing_neg(self)
                }
            }
        )*
    };
}

macro_rules! impl_overflowing_euclid_traits {
    ($($ty:ty),*) => {
        $(
            impl OverflowingDivEuclid<$ty> for $ty {
                type Output = $ty;

                #[inline]
                fn overflowing_div_euclid(self, other: $ty) -> (Self::Output, bool) {
                    <$ty>::overflowing_div_euclid(self, other)
                }
            }

            impl OverflowingRemEuclid<$ty> for $ty {
                type Output = $ty;

                #[inline]
                fn overflowing_rem_euclid(self, other: $ty) -> (Self::Output, bool) {
                    <$ty>::overflowing_rem_euclid(self, other)
                }
            }

            impl OverflowingDivRemEuclid<$ty> for $ty {
                type Output = $ty;

                #[inline]
                fn overflowing_div_rem_euclid(self, other: $ty) -> ((Self::Output, Self::Output), bool) {
                    let (div, div_overflow) = <$ty>::overflowing_div_euclid(self, other);
                    let (rem, rem_overflow) = <$ty>::overflowing_rem_euclid(self, other);

                    ((div, rem), div_overflow | rem_overflow)
                }
            }
        )*
    };
}

macro_rules! impl_overflowing_div_rem_trait {
    ($($ty:ty),*) => {
        $(
            impl OverflowingDivRem<$ty> for $ty {
                type Output = $ty;

                #[inline]
                fn overflowing_div_rem(self, other: $ty) -> ((Self::Output, Self::Output), bool) {
                    let (div, div_overflow) = <$ty>::overflowing_div(self, other);
                    let (rem, rem_overflow) = <$ty>::overflowing_rem(self, other);

                    ((div, rem), div_overflow | rem_overflow)
                }
            }
        )*
    };
}

macro_rules! impl_overflowing_mul_add_trait {
    ($($ty:ty),*) => {
        $(
            impl OverflowingMulAdd<$ty, $ty> for $ty {
                type Output = $ty;

                #[inline]
                fn overflowing_mul_add(self, a: $ty, b: $ty) -> (Self::Output, bool) {
                    let (product, mul_overflow) = <$ty>::overflowing_mul(self, a);
                    let (result, add_overflow) = <$ty>::overflowing_add(product, b);

                    (result, mul_overflow | add_overflow)
                }
            }
        )*
    };
}

macro_rules! impl_overflowing_arith_traits_for_signed_ints {
    ($ty:ty $(, $rest:ident)*) => {
        impl_overflowing_field_traits_for_ints!($ty);
        impl_overflowing_div_rem_trait!($ty);
        impl_overflowing_euclid_traits!($ty);
        impl_overflowing_mul_add_trait!($ty);
    };
}

macro_rules! impl_overflowing_arith_traits_for_unsigned_ints {
    ($ty:ty $(, $rest:ident)*) => {
        impl_overflowing_field_traits_for_ints!($ty);
        impl_overflowing_div_rem_trait!($ty);
        impl_overflowing_euclid_traits!($ty);
        impl_overflowing_mul_add_trait!($ty);
    };
}

macro_rules! impl_overflowing_field_traits_via_inner {
    ($($ty:ty),*) => {
        $(
            impl OverflowingAdd<$ty> for $ty {
                type Output = $ty;

                #[inline]
                fn overflowing_add(self, other: $ty) -> (Self::Output, bool) {
                    let (value, overflowed) = self.0.overflowing_add(other.0);

                    (Self(value), overflowed)
                }
            }

            impl OverflowingSub<$ty> for $ty {
                type Output = $ty;

                #[inline]
                fn overflowing_sub(self, other: $ty) -> (Self::Output, bool) {
                    let (value, overflowed) = self.0.overflowing_sub(other.0);

                    (Self(value), overflowed)
                }
            }

            impl OverflowingMul<$ty> for $ty {
                type Output = $ty;

                #[inline]
                fn overflowing_mul(self, other: $ty) -> (Self::Output, bool) {
                    let (value, overflowed) = self.0.overflowing_mul(other.0);

                    (Self(value), overflowed)
                }
            }

            impl OverflowingDiv<$ty> for $ty {
                type Output = $ty;

                #[inline]
                fn overflowing_div(self, other: $ty) -> (Self::Output, bool) {
                    let (value, overflowed) = self.0.overflowing_div(other.0);

                    (Self(value), overflowed)
                }
            }

            impl OverflowingRem<$ty> for $ty {
                type Output = $ty;

                #[inline]
                fn overflowing_rem(self, other: $ty) -> (Self::Output, bool) {
                    let (value, overflowed) = self.0.overflowing_rem(other.0);

                    (Self(value), overflowed)
                }
            }

            impl OverflowingNeg for $ty {
                type Output = $ty;

                #[inline]
                fn overflowing_neg(self) -> (Self::Output, bool) {
                    let (value, overflowed) = self.0.overflowing_neg();

                    (Self(value), overflowed)
                }
            }
        )*
    };
}

macro_rules! impl_overflowing_euclid_traits_via_inner {
    ($($ty:ty),*) => {
        $(
            impl OverflowingDivEuclid<$ty> for $ty {
                type Output = $ty;

                #[inline]
                fn overflowing_div_euclid(self, other: $ty) -> (Self::Output, bool) {
                    let (value, overflowed) = self.0.overflowing_div_euclid(other.0);

                    (Self(value), overflowed)
                }
            }

            impl OverflowingRemEuclid<$ty> for $ty {
                type Output = $ty;

                #[inline]
                fn overflowing_rem_euclid(self, other: $ty) -> (Self::Output, bool) {
                    let (value, overflowed) = self.0.overflowing_rem_euclid(other.0);

                    (Self(value), overflowed)
                }
            }

            impl OverflowingDivRemEuclid<$ty> for $ty {
                type Output = $ty;

                #[inline]
                fn overflowing_div_rem_euclid(
                    self,
                    other: $ty,
                ) -> ((Self::Output, Self::Output), bool) {
                    let (div, div_overflow) = self.0.overflowing_div_euclid(other.0);
                    let (rem, rem_overflow) = self.0.overflowing_rem_euclid(other.0);

                    ((Self(div), Self(rem)), div_overflow | rem_overflow)
                }
            }
        )*
    };
}

macro_rules! impl_overflowing_div_rem_trait_via_inner {
    ($($ty:ty),*) => {
        $(
            impl OverflowingDivRem<$ty> for $ty {
                type Output = $ty;

                #[inline]
                fn overflowing_div_rem(self, other: $ty) -> ((Self::Output, Self::Output), bool) {
                    let (div, div_overflow) = self.0.overflowing_div(other.0);
                    let (rem, rem_overflow) = self.0.overflowing_rem(other.0);

                    ((Self(div), Self(rem)), div_overflow | rem_overflow)
                }
            }
        )*
    };
}

macro_rules! impl_overflowing_mul_add_trait_via_inner {
    ($($ty:ty),*) => {
        $(
            impl OverflowingMulAdd<$ty, $ty> for $ty {
                type Output = $ty;

                #[inline]
                fn overflowing_mul_add(self, a: $ty, b: $ty) -> (Self::Output, bool) {
                    let (product, mul_overflow) = self.0.overflowing_mul(a.0);
                    let (result, add_overflow) = product.overflowing_add(b.0);

                    (Self(result), mul_overflow | add_overflow)
                }
            }
        )*
    };
}

macro_rules! impl_overflowing_arith_traits_via_inner {
    ($ty:ty $(, $rest:ident)*) => {
        impl_overflowing_field_traits_via_inner!($ty);
        impl_overflowing_div_rem_trait_via_inner!($ty);
        impl_overflowing_euclid_traits_via_inner!($ty);
        impl_overflowing_mul_add_trait_via_inner!($ty);
    };
}

/*macro_rules! impl_overflowing_field_traits_for_floats {
    ($($ty:ty),*) => {
        $(
            impl OverflowingAdd<$ty> for $ty {
                type Output = $ty;

                #[inline]
                fn overflowing_add(self, other: $ty) -> (Self::Output, bool) {
                    (self + other, false)
                }
            }

            impl OverflowingSub<$ty> for $ty {
                type Output = $ty;

                #[inline]
                fn overflowing_sub(self, other: $ty) -> (Self::Output, bool) {
                    (self - other, false)
                }
            }

            impl OverflowingMul<$ty> for $ty {
                type Output = $ty;

                #[inline]
                fn overflowing_mul(self, other: $ty) -> (Self::Output, bool) {
                    (self * other, false)
                }
            }

            impl OverflowingDiv<$ty> for $ty {
                type Output = $ty;

                #[inline]
                fn overflowing_div(self, other: $ty) -> (Self::Output, bool) {
                    (self / other, false)
                }
            }

            impl OverflowingRem<$ty> for $ty {
                type Output = $ty;

                #[inline]
                fn overflowing_rem(self, other: $ty) -> (Self::Output, bool) {
                    (self % other, false)
                }
            }

            impl OverflowingNeg for $ty {
                type Output = $ty;

                #[inline]
                fn overflowing_neg(self) -> (Self::Output, bool) {
                    (-self, false)
                }
            }
        )*
    };
}

macro_rules! impl_overflowing_div_rem_trait_for_floats {
    ($($ty:ty),*) => {
        $(
            impl OverflowingDivRem<$ty> for $ty {
                type Output = $ty;

                #[inline]
                fn overflowing_div_rem(self, other: $ty) -> ((Self::Output, Self::Output), bool) {
                    ((self / other, self % other), false)
                }
            }
        )*
    };
}*/

/*#[cfg(feature = "std")]
macro_rules! impl_overflowing_euclid_traits_for_floats {
    ($($ty:ty),*) => {
        $(
            impl OverflowingDivEuclid<$ty> for $ty {
                type Output = $ty;

                #[inline]
                fn overflowing_div_euclid(self, other: $ty) -> (Self::Output, bool) {
                    (<$ty>::div_euclid(self, other), false)
                }
            }

            impl OverflowingRemEuclid<$ty> for $ty {
                type Output = $ty;

                #[inline]
                fn overflowing_rem_euclid(self, other: $ty) -> (Self::Output, bool) {
                    (<$ty>::rem_euclid(self, other), false)
                }
            }

            impl OverflowingDivRemEuclid<$ty> for $ty {
                type Output = $ty;

                #[inline]
                fn overflowing_div_rem_euclid(self, other: $ty) -> ((Self::Output, Self::Output), bool) {
                    (
                        (<$ty>::div_euclid(self, other), <$ty>::rem_euclid(self, other)),
                        false,
                    )
                }
            }
        )*
    };
}*/

/*macro_rules! impl_overflowing_mul_add_trait_for_floats {
    ($($ty:ty),*) => {
        $(
            impl OverflowingMulAdd<$ty, $ty> for $ty {
                type Output = $ty;

                #[inline]
                fn overflowing_mul_add(self, a: $ty, b: $ty) -> (Self::Output, bool) {
                    (MulAdd::mul_add(self, a, b), false)
                }
            }
        )*
    };
}*/

/*macro_rules! impl_overflowing_arith_traits_for_floats {
    ($($ty:ty),*) => {
        impl_overflowing_field_traits_for_floats!($($ty),*);
        impl_overflowing_div_rem_trait_for_floats!($($ty),*);
        #[cfg(feature = "std")]
        impl_overflowing_euclid_traits_for_floats!($($ty),*);
    };
}*/

for_signed_ints!(impl_overflowing_arith_traits_for_signed_ints);
for_unsigned_ints!(impl_overflowing_arith_traits_for_unsigned_ints);
for_wrapping_signed_ints!(impl_overflowing_arith_traits_via_inner);
for_wrapping_unsigned_ints!(impl_overflowing_arith_traits_via_inner);
for_saturating_signed_ints!(impl_overflowing_arith_traits_via_inner);
for_saturating_unsigned_ints!(impl_overflowing_arith_traits_via_inner);
//impl_overflowing_arith_traits_for_floats!(f32, f64);

//impl_overflowing_mul_add_trait_for_floats!(f32, f64);

macro_rules! test_overflowing_arith_traits_int_nonnegative {
    ($ty:ty, $tests_mod:ident) => {
        #[cfg(test)]
        mod $tests_mod {
            use crate::elem::*;
            use crate::ops::*;
            use crate::types::Object;

            #[test]
            fn test_div_rem() {
                let zero = <$ty as AssocConstZero>::ZERO;
                let one = <$ty as AssocConstOne>::ONE;
                let two = one + one;
                let three = two + one;
                let four = two + two;
                let five = four + one;

                assert_eq!(
                    OverflowingDivRem::overflowing_div_rem(four, two),
                    ((two, zero), false)
                );
                assert_eq!(
                    OverflowingDivRem::overflowing_div_rem(five, two),
                    ((two, one), false)
                );
                assert_eq!(
                    OverflowingDivRem::overflowing_div_rem(three, two),
                    ((one, one), false)
                );
                assert_eq!(Object::new(five).overflowing_div_rem(Object::new(two)), {
                    let ((a, b), o) = OverflowingDivRem::overflowing_div_rem(five, two);

                    ((Object::new(a), Object::new(b)), o)
                });
            }

            #[test]
            fn test_div_euclid() {
                let one = <$ty as AssocConstOne>::ONE;
                let two = one + one;
                let three = two + one;
                let four = two + two;
                let five = four + one;

                assert_eq!(
                    OverflowingDivEuclid::overflowing_div_euclid(four, two),
                    (two, false)
                );
                assert_eq!(
                    OverflowingDivEuclid::overflowing_div_euclid(five, two),
                    (two, false)
                );
                assert_eq!(
                    OverflowingDivEuclid::overflowing_div_euclid(three, two),
                    (one, false)
                );
                assert_eq!(
                    Object::new(five).overflowing_div_euclid(Object::new(two)),
                    {
                        let (v, o) = OverflowingDivEuclid::overflowing_div_euclid(five, two);

                        (Object::new(v), o)
                    }
                );
            }

            #[test]
            fn test_rem_euclid() {
                let zero = <$ty as AssocConstZero>::ZERO;
                let one = <$ty as AssocConstOne>::ONE;
                let two = one + one;
                let three = two + one;
                let four = two + two;
                let five = four + one;

                assert_eq!(
                    OverflowingRemEuclid::overflowing_rem_euclid(four, two),
                    (zero, false)
                );
                assert_eq!(
                    OverflowingRemEuclid::overflowing_rem_euclid(five, two),
                    (one, false)
                );
                assert_eq!(
                    OverflowingRemEuclid::overflowing_rem_euclid(three, two),
                    (one, false)
                );
                assert_eq!(
                    Object::new(five).overflowing_rem_euclid(Object::new(two)),
                    {
                        let (v, o) = OverflowingRemEuclid::overflowing_rem_euclid(five, two);

                        (Object::new(v), o)
                    }
                );
            }

            #[test]
            fn test_div_rem_euclid() {
                let zero = <$ty as AssocConstZero>::ZERO;
                let one = <$ty as AssocConstOne>::ONE;
                let two = one + one;
                let four = two + two;
                let five = four + one;

                assert_eq!(
                    OverflowingDivRemEuclid::overflowing_div_rem_euclid(five, two),
                    ((two, one), false)
                );
                assert_eq!(
                    OverflowingDivRemEuclid::overflowing_div_rem_euclid(four, two),
                    ((two, zero), false)
                );
                assert_eq!(
                    Object::new(five).overflowing_div_rem_euclid(Object::new(two)),
                    {
                        let ((a, b), o) =
                            OverflowingDivRemEuclid::overflowing_div_rem_euclid(five, two);

                        ((Object::new(a), Object::new(b)), o)
                    }
                );
            }

            #[test]
            fn test_mul_add() {
                let zero = <$ty as AssocConstZero>::ZERO;
                let one = <$ty as AssocConstOne>::ONE;
                let two = one + one;
                let four = two + two;

                assert_eq!(
                    OverflowingMulAdd::overflowing_mul_add(two, two, zero),
                    (four, false)
                );
                assert_eq!(
                    OverflowingMulAdd::overflowing_mul_add(one, one, one),
                    (two, false)
                );
                assert_eq!(
                    OverflowingMulAdd::overflowing_mul_add(two, one, two),
                    (four, false)
                );
                assert_eq!(
                    Object::new(two).overflowing_mul_add(Object::new(one), Object::new(two)),
                    {
                        let (v, o) = OverflowingMulAdd::overflowing_mul_add(two, one, two);

                        (Object::new(v), o)
                    }
                );
            }
        }
    };
}

macro_rules! test_overflowing_arith_traits_int_negative {
    ($ty:ty, $tests_mod:ident) => {
        #[cfg(test)]
        mod $tests_mod {
            use crate::elem::*;
            use crate::limits::*;
            use crate::ops::*;
            use crate::types::Object;

            #[test]
            fn test_div_rem() {
                let zero = <$ty as AssocConstZero>::ZERO;
                let one = <$ty as AssocConstOne>::ONE;
                let two = one + one;
                let four = two + two;
                let five = four + one;

                assert_eq!(
                    OverflowingDivRem::overflowing_div_rem(-five, two),
                    ((-two, -one), false)
                );
                assert_eq!(
                    OverflowingDivRem::overflowing_div_rem(-four, two),
                    ((-two, zero), false)
                );
                assert_eq!(
                    OverflowingDivRem::overflowing_div_rem(<$ty as MinFinite>::MIN_FINITE, -one),
                    ((<$ty as MinFinite>::MIN_FINITE, zero), true)
                );
                assert_eq!(Object::new(five).overflowing_div_rem(Object::new(two)), {
                    let ((a, b), o) = OverflowingDivRem::overflowing_div_rem(five, two);

                    ((Object::new(a), Object::new(b)), o)
                });
            }

            #[test]
            fn test_div_euclid() {
                let one = <$ty as AssocConstOne>::ONE;
                let two = one + one;
                let three = two + one;
                let five = two + two + one;

                assert_eq!(
                    OverflowingDivEuclid::overflowing_div_euclid(-five, two),
                    (-three, false)
                );
                assert_eq!(
                    OverflowingDivEuclid::overflowing_div_euclid(-five, -two),
                    (three, false)
                );
                assert_eq!(
                    Object::new(five).overflowing_div_euclid(Object::new(two)),
                    {
                        let (v, o) = OverflowingDivEuclid::overflowing_div_euclid(five, two);

                        (Object::new(v), o)
                    }
                );
            }

            #[test]
            fn test_rem_euclid() {
                let zero = <$ty as AssocConstZero>::ZERO;
                let one = <$ty as AssocConstOne>::ONE;
                let two = one + one;
                let four = two + two;
                let five = four + one;

                assert_eq!(
                    OverflowingRemEuclid::overflowing_rem_euclid(-five, two),
                    (one, false)
                );
                assert_eq!(
                    OverflowingRemEuclid::overflowing_rem_euclid(-four, two),
                    (zero, false)
                );
                assert_eq!(
                    Object::new(five).overflowing_rem_euclid(Object::new(two)),
                    {
                        let (v, o) = OverflowingRemEuclid::overflowing_rem_euclid(five, two);

                        (Object::new(v), o)
                    }
                );
            }

            #[test]
            fn test_div_rem_euclid() {
                let one = <$ty as AssocConstOne>::ONE;
                let two = one + one;
                let three = two + one;
                let five = two + two + one;

                assert_eq!(
                    OverflowingDivRemEuclid::overflowing_div_rem_euclid(-five, two),
                    ((-three, one), false)
                );
                assert_eq!(
                    Object::new(five).overflowing_div_rem_euclid(Object::new(two)),
                    {
                        let ((a, b), o) =
                            OverflowingDivRemEuclid::overflowing_div_rem_euclid(five, two);

                        ((Object::new(a), Object::new(b)), o)
                    }
                );
            }

            #[test]
            fn test_neg() {
                let zero = <$ty as AssocConstZero>::ZERO;
                let one = <$ty as AssocConstOne>::ONE;

                assert_eq!(OverflowingNeg::overflowing_neg(one), (-one, false));
                assert_eq!(OverflowingNeg::overflowing_neg(-one), (one, false));
                assert_eq!(OverflowingNeg::overflowing_neg(zero), (zero, false));
                assert_eq!(
                    OverflowingNeg::overflowing_neg(<$ty as MinFinite>::MIN_FINITE),
                    (<$ty as MinFinite>::MIN_FINITE, true)
                );
                assert_eq!(Object::new(one).overflowing_neg(), {
                    let (v, o) = OverflowingNeg::overflowing_neg(one);

                    (Object::new(v), o)
                });
            }
        }
    };
}

for_signed_ints!(
    test_overflowing_arith_traits_int_nonnegative; test_overflowing_arith_traits_int_negative
);
for_unsigned_ints!(test_overflowing_arith_traits_int_nonnegative);
