// SPDX-FileCopyrightText: 2026 numlike contributors
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use crate::ops::MulAdd;

#[cfg(any(feature = "std", feature = "libm"))]
use crate::ops::{DivEuclid, RemEuclid};
use crate::types::Object;

/// Bundle of checked arithmetic and fused arithmetic operations.
pub trait CheckedPlainArithOps<Rhs = Self>:
    CheckedArithOps<Rhs> + CheckedFusedArithOps<Rhs>
{
}

/// Bundle of checked fused arithmetic operations.
pub trait CheckedFusedArithOps<Rhs = Self>: CheckedMulAdd<Rhs, Rhs> {}
impl<Rhs, T: CheckedMulAdd<Rhs, Rhs>> CheckedFusedArithOps<Rhs> for T {}

/// Checked fused multiply-add. Computes `(self * a) + b`, returning `None` if
/// overflow occurred or the result is not finite.
pub trait CheckedMulAdd<A = Self, B = Self> {
    /// The resulting type after applying the operation.
    type Output;

    /// Checked fused multiply-add. Computes `(self * a) + b`, returning `None`
    /// if overflow occurred or the result is not finite.
    fn checked_mul_add(self, a: A, b: B) -> Option<Self::Output>;
}

impl<B: CheckedMulAdd<Output = B>> CheckedMulAdd for Object<B> {
    type Output = Self;

    fn checked_mul_add(self, a: Self, b: Self) -> Option<Self::Output> {
        <B as CheckedMulAdd>::checked_mul_add(self.0, a.0, b.0).map(Object)
    }
}

impl<B: CheckedMulAdd<Output = B>> Object<B> {
    /// Checked fused multiply-add. Computes `(self * a) + b`, returning `None`
    /// if overflow occurred or the result is not finite.
    pub fn checked_mul_add(self, a: Self, b: Self) -> Option<Self> {
        <B as CheckedMulAdd>::checked_mul_add(self.0, a.0, b.0).map(Object)
    }
}

/// Bundle of checked arithmetic operations.
pub trait CheckedArithOps<Rhs = Self>:
    CheckedEuclidOps<Rhs>
    + CheckedFieldOps<Rhs>
    + CheckedRem<Rhs, Output = Self>
    + CheckedDivRem<Rhs, Output = Self>
{
}
impl<
    Rhs,
    T: CheckedEuclidOps<Rhs>
        + CheckedFieldOps<Rhs>
        + CheckedRem<Rhs, Output = Self>
        + CheckedDivRem<Rhs, Output = Self>,
> CheckedArithOps<Rhs> for T
{
}

/// Checked remainder. Computes `self % other`, returning `None` if `other == 0`
/// or the division results in overflow.
pub trait CheckedRem<Rhs = Self> {
    /// The resulting type after applying the operation.
    type Output;

    /// Checked remainder. Computes `self % other`, returning `None` if `other
    /// == 0` or the division results in overflow.
    fn checked_rem(self, other: Rhs) -> Option<Self::Output>;
}

impl<B: CheckedRem<Output = B>> CheckedRem for Object<B> {
    type Output = Self;

    fn checked_rem(self, other: Self) -> Option<Self::Output> {
        <B as CheckedRem>::checked_rem(self.0, other.0).map(Object)
    }
}

impl<B: CheckedRem<Output = B>> Object<B> {
    /// Checked remainder. Computes `self % other`, returning `None` if `other
    /// == 0` or the division results in overflow.
    pub fn checked_rem(self, other: Self) -> Option<Self> {
        <B as CheckedRem>::checked_rem(self.0, other.0).map(Object)
    }
}

/// Checked simultaneous quotient and remainder. Computes
/// `(self / other, self % other)`, returning `None` if `other == 0` or the
/// division results in overflow.
pub trait CheckedDivRem<Rhs> {
    /// The resulting type after applying the operation.
    type Output;

    /// Checked simultaneous quotient and remainder. Computes
    /// `(self / other, self % other)`, returning `None` if `other == 0` or the
    /// division results in overflow.
    fn checked_div_rem(self, other: Rhs) -> Option<(Self::Output, Self::Output)>;
}

impl<B: CheckedDivRem<B, Output = B>> CheckedDivRem<Object<B>> for Object<B> {
    type Output = Self;

    fn checked_div_rem(self, other: Self) -> Option<(Self::Output, Self::Output)> {
        <B as CheckedDivRem<B>>::checked_div_rem(self.0, other.0)
            .map(|(a, b)| (Object(a), Object(b)))
    }
}

impl<B: CheckedDivRem<B, Output = B>> Object<B> {
    /// Checked simultaneous quotient and remainder. Computes
    /// `(self / other, self % other)`, returning `None` if `other == 0` or the
    /// division results in overflow.
    pub fn checked_div_rem(self, other: Self) -> Option<(Self, Self)> {
        <B as CheckedDivRem<B>>::checked_div_rem(self.0, other.0)
            .map(|(a, b)| (Object(a), Object(b)))
    }
}

/// Bundle of checked Euclidean division operations.
pub trait CheckedEuclidOps<Rhs>:
    CheckedDivEuclid<Rhs> + CheckedRemEuclid<Rhs> + CheckedDivRemEuclid<Rhs>
{
}
impl<Rhs, T: CheckedDivEuclid<Rhs> + CheckedRemEuclid<Rhs> + CheckedDivRemEuclid<Rhs>>
    CheckedEuclidOps<Rhs> for T
{
}

/// Checked Euclidean division. Computes `self.div_euclid(other)`, returning
/// `None` if `other == 0` or the division results in overflow.
pub trait CheckedDivEuclid<Rhs = Self> {
    /// The resulting type after applying the operation.
    type Output;

    /// Checked Euclidean division. Computes `self.div_euclid(other)`,
    /// returning `None` if `other == 0` or the division results in overflow.
    fn checked_div_euclid(self, other: Rhs) -> Option<Self::Output>;
}

impl<B: CheckedDivEuclid<Output = B>> CheckedDivEuclid for Object<B> {
    type Output = Self;

    fn checked_div_euclid(self, other: Self) -> Option<Self::Output> {
        <B as CheckedDivEuclid>::checked_div_euclid(self.0, other.0).map(Object)
    }
}

impl<B: CheckedDivEuclid<Output = B>> Object<B> {
    /// Checked Euclidean division. Computes `self.div_euclid(other)`,
    /// returning `None` if `other == 0` or the division results in overflow.
    pub fn checked_div_euclid(self, other: Self) -> Option<Self> {
        <B as CheckedDivEuclid>::checked_div_euclid(self.0, other.0).map(Object)
    }
}

/// Checked Euclidean remainder. Computes `self.rem_euclid(other)`, returning
/// `None` if `other == 0` or the division results in overflow.
pub trait CheckedRemEuclid<Rhs = Self> {
    /// The resulting type after applying the operation.
    type Output;

    /// Checked Euclidean remainder. Computes `self.rem_euclid(other)`,
    /// returning `None` if `other == 0` or the division results in overflow.
    fn checked_rem_euclid(self, other: Rhs) -> Option<Self::Output>;
}

impl<B: CheckedRemEuclid<Output = B>> CheckedRemEuclid for Object<B> {
    type Output = Self;

    fn checked_rem_euclid(self, other: Self) -> Option<Self::Output> {
        <B as CheckedRemEuclid>::checked_rem_euclid(self.0, other.0).map(Object)
    }
}

impl<B: CheckedRemEuclid<Output = B>> Object<B> {
    /// Checked Euclidean remainder. Computes `self.rem_euclid(other)`,
    /// returning `None` if `other == 0` or the division results in overflow.
    pub fn checked_rem_euclid(self, other: Self) -> Option<Self> {
        <B as CheckedRemEuclid>::checked_rem_euclid(self.0, other.0).map(Object)
    }
}

/// Checked simultaneous Euclidean quotient and remainder. Computes
/// `(self.div_euclid(other), self.rem_euclid(other))`, returning `None` if
/// `other == 0` or the division results in overflow.
pub trait CheckedDivRemEuclid<Rhs = Self> {
    /// The resulting type after applying the operation.
    type Output;

    /// Checked simultaneous Euclidean quotient and remainder. Computes
    /// `(self.div_euclid(other), self.rem_euclid(other))`, returning `None` if
    /// `other == 0` or the division results in overflow.
    fn checked_div_rem_euclid(self, other: Rhs) -> Option<(Self::Output, Self::Output)>;
}

impl<B: CheckedDivRemEuclid<Output = B>> CheckedDivRemEuclid for Object<B> {
    type Output = Self;

    fn checked_div_rem_euclid(self, other: Self) -> Option<(Self::Output, Self::Output)> {
        <B as CheckedDivRemEuclid>::checked_div_rem_euclid(self.0, other.0)
            .map(|(a, b)| (Object(a), Object(b)))
    }
}

impl<B: CheckedDivRemEuclid<Output = B>> Object<B> {
    /// Checked simultaneous Euclidean quotient and remainder. Computes
    /// `(self.div_euclid(other), self.rem_euclid(other))`, returning `None` if
    /// `other == 0` or the division results in overflow.
    pub fn checked_div_rem_euclid(self, other: Self) -> Option<(Self, Self)> {
        <B as CheckedDivRemEuclid>::checked_div_rem_euclid(self.0, other.0)
            .map(|(a, b)| (Object(a), Object(b)))
    }
}

/// Bundle of checked field operations.
pub trait CheckedFieldOps<Rhs = Self>:
    CheckedRingOps<Rhs> + CheckedDiv<Rhs, Output = Self>
{
}
impl<Rhs, T: CheckedRingOps<Rhs> + CheckedDiv<Rhs, Output = Self>> CheckedFieldOps<Rhs> for T {}

/// Checked division. Computes `self / other`, returning `None` if `other ==
/// 0` or the division results in overflow.
pub trait CheckedDiv<Rhs = Self> {
    /// The resulting type after applying the operation.
    type Output;

    /// Checked division. Computes `self / other`, returning `None` if `other ==
    /// 0` or the division results in overflow.
    fn checked_div(self, other: Rhs) -> Option<Self::Output>;
}

impl<B: CheckedDiv<Output = B>> CheckedDiv for Object<B> {
    type Output = Self;

    fn checked_div(self, other: Self) -> Option<Self::Output> {
        <B as CheckedDiv>::checked_div(self.0, other.0).map(Object)
    }
}

impl<B: CheckedDiv<Output = B>> Object<B> {
    /// Checked division. Computes `self / other`, returning `None` if `other ==
    /// 0` or the division results in overflow.
    pub fn checked_div(self, other: Self) -> Option<Self> {
        <B as CheckedDiv>::checked_div(self.0, other.0).map(Object)
    }
}

/// Bundle of checked ring operations.
pub trait CheckedRingOps<Rhs = Self>:
    CheckedAdd<Rhs, Output = Self>
    + CheckedSub<Rhs, Output = Self>
    + CheckedMul<Rhs, Output = Self>
    + CheckedNeg<Output = Self>
{
}
impl<
    Rhs,
    T: CheckedAdd<Rhs, Output = Self>
        + CheckedSub<Rhs, Output = Self>
        + CheckedMul<Rhs, Output = Self>
        + CheckedNeg<Output = Self>,
> CheckedRingOps<Rhs> for T
{
}

/// Checked addition. Computes `self + other`, returning `None` if overflow
/// or occurred.
pub trait CheckedAdd<Rhs = Self> {
    /// The resulting type after applying the operation.
    type Output;

    /// Checked addition. Computes `self + other`, returning `None` if overflow
    /// or occurred.
    fn checked_add(self, other: Rhs) -> Option<Self::Output>;
}

impl<B: CheckedAdd<Output = B>> CheckedAdd for Object<B> {
    type Output = Self;

    fn checked_add(self, other: Self) -> Option<Self::Output> {
        <B as CheckedAdd>::checked_add(self.0, other.0).map(Object)
    }
}

impl<B: CheckedAdd<Output = B>> Object<B> {
    /// Checked addition. Computes `self + other`, returning `None` if overflow
    /// or occurred.
    pub fn checked_add(self, other: Self) -> Option<Self> {
        <B as CheckedAdd>::checked_add(self.0, other.0).map(Object)
    }
}

/// Checked subtraction. Computes `self - other`, returning `None` if
/// overflow occurred.
pub trait CheckedSub<Rhs = Self> {
    /// The resulting type after applying the operation.
    type Output;

    /// Checked subtraction. Computes `self - other`, returning `None` if
    /// overflow occurred.
    fn checked_sub(self, other: Rhs) -> Option<Self::Output>;
}

impl<B: CheckedSub<Output = B>> CheckedSub for Object<B> {
    type Output = Self;

    fn checked_sub(self, other: Self) -> Option<Self::Output> {
        <B as CheckedSub>::checked_sub(self.0, other.0).map(Object)
    }
}

impl<B: CheckedSub<Output = B>> Object<B> {
    /// Checked subtraction. Computes `self - other`, returning `None` if
    /// overflow occurred.
    pub fn checked_sub(self, other: Self) -> Option<Self> {
        <B as CheckedSub>::checked_sub(self.0, other.0).map(Object)
    }
}

/// Checked multiplication. Computes `self * other`, returning `None` if
/// overflow occurred.
pub trait CheckedMul<Rhs = Self> {
    /// The resulting type after applying the operation.
    type Output;

    /// Checked multiplication. Computes `self * other`, returning `None` if
    /// overflow occurred.
    fn checked_mul(self, other: Rhs) -> Option<Self::Output>;
}

impl<B: CheckedMul<Output = B>> CheckedMul for Object<B> {
    type Output = Self;

    fn checked_mul(self, other: Self) -> Option<Self::Output> {
        <B as CheckedMul>::checked_mul(self.0, other.0).map(Object)
    }
}

impl<B: CheckedMul<Output = B>> Object<B> {
    /// Checked multiplication. Computes `self * other`, returning `None` if
    /// overflow occurred.
    pub fn checked_mul(self, other: Self) -> Option<Self> {
        <B as CheckedMul>::checked_mul(self.0, other.0).map(Object)
    }
}

/// Checked negation. Computes `-self`, returning `None` if `self == MIN`.
pub trait CheckedNeg {
    /// The resulting type after applying the operation.
    type Output;

    /// Checked negation. Computes `-self`, returning `None` if `self == MIN`.
    fn checked_neg(self) -> Option<Self::Output>;
}

impl<B: CheckedNeg<Output = B>> CheckedNeg for Object<B> {
    type Output = Self;

    fn checked_neg(self) -> Option<Self::Output> {
        <B as CheckedNeg>::checked_neg(self.0).map(Object)
    }
}

impl<B: CheckedNeg<Output = B>> Object<B> {
    /// Checked negation. Computes `-self`, returning `None` if `self == MIN`.
    pub fn checked_neg(self) -> Option<Self> {
        <B as CheckedNeg>::checked_neg(self.0).map(Object)
    }
}

macro_rules! impl_checked_field_traits_for_ints {
    ($($ty:ty),*) => {
        $(
            impl CheckedAdd<$ty> for $ty {
                type Output = $ty;

                #[inline]
                fn checked_add(self, other: $ty) -> Option<Self::Output> {
                    <$ty>::checked_add(self, other)
                }
            }

            impl CheckedSub<$ty> for $ty {
                type Output = $ty;

                #[inline]
                fn checked_sub(self, other: $ty) -> Option<Self::Output> {
                    <$ty>::checked_sub(self, other)
                }
            }

            impl CheckedMul<$ty> for $ty {
                type Output = $ty;

                #[inline]
                fn checked_mul(self, other: $ty) -> Option<Self::Output> {
                    <$ty>::checked_mul(self, other)
                }
            }

            impl CheckedDiv<$ty> for $ty {
                type Output = $ty;

                #[inline]
                fn checked_div(self, other: $ty) -> Option<Self::Output> {
                    <$ty>::checked_div(self, other)
                }
            }

            impl CheckedRem<$ty> for $ty {
                type Output = $ty;

                #[inline]
                fn checked_rem(self, other: $ty) -> Option<Self::Output> {
                    <$ty>::checked_rem(self, other)
                }
            }

            impl CheckedNeg for $ty {
                type Output = $ty;

                #[inline]
                fn checked_neg(self) -> Option<Self::Output> {
                    <$ty>::checked_neg(self)
                }
            }
        )*
    };
}

macro_rules! impl_checked_euclid_traits {
    ($($ty:ty),*) => {
        $(
            impl CheckedDivEuclid<$ty> for $ty {
                type Output = $ty;

                #[inline]
                fn checked_div_euclid(self, other: $ty) -> Option<Self::Output> {
                    <$ty>::checked_div_euclid(self, other)
                }
            }

            impl CheckedRemEuclid<$ty> for $ty {
                type Output = $ty;

                #[inline]
                fn checked_rem_euclid(self, other: $ty) -> Option<Self::Output> {
                    <$ty>::checked_rem_euclid(self, other)
                }
            }

            impl CheckedDivRemEuclid<$ty> for $ty {
                type Output = $ty;

                #[inline]
                fn checked_div_rem_euclid(
                    self,
                    other: $ty,
                ) -> Option<(Self::Output, Self::Output)> {
                    Some((
                        <$ty>::checked_div_euclid(self, other)?,
                        <$ty>::checked_rem_euclid(self, other)?,
                    ))
                }
            }
        )*
    };
}

macro_rules! impl_checked_div_rem_trait {
    ($($ty:ty),*) => {
        $(
            impl CheckedDivRem<$ty> for $ty {
                type Output = $ty;

                #[inline]
                fn checked_div_rem(self, other: $ty) -> Option<(Self::Output, Self::Output)> {
                    Some((<$ty>::checked_div(self, other)?, <$ty>::checked_rem(self, other)?))
                }
            }
        )*
    };
}

macro_rules! impl_checked_mul_add_trait {
    ($($ty:ty),*) => {
        $(
            impl CheckedMulAdd<$ty, $ty> for $ty {
                type Output = $ty;

                #[inline]
                fn checked_mul_add(self, a: $ty, b: $ty) -> Option<Self::Output> {
                    <$ty>::checked_mul(self, a).and_then(|product| <$ty>::checked_add(product, b))
                }
            }
        )*
    };
}

macro_rules! impl_checked_arith_traits_for_signed_ints {
    ($ty:ty $(, $rest:ident)*) => {
        impl_checked_field_traits_for_ints!($ty);
        impl_checked_div_rem_trait!($ty);
        impl_checked_euclid_traits!($ty);
        impl_checked_mul_add_trait!($ty);
    };
}

macro_rules! impl_checked_arith_traits_for_unsigned_ints {
    ($ty:ty $(, $rest:ident)*) => {
        impl_checked_field_traits_for_ints!($ty);
        impl_checked_div_rem_trait!($ty);
        impl_checked_euclid_traits!($ty);
        impl_checked_mul_add_trait!($ty);
    };
}

macro_rules! impl_checked_field_traits_via_inner {
    ($($ty:ty),*) => {
        $(
            impl CheckedAdd<$ty> for $ty {
                type Output = $ty;

                #[inline]
                fn checked_add(self, other: $ty) -> Option<Self::Output> {
                    self.0.checked_add(other.0).map(Self)
                }
            }

            impl CheckedSub<$ty> for $ty {
                type Output = $ty;

                #[inline]
                fn checked_sub(self, other: $ty) -> Option<Self::Output> {
                    self.0.checked_sub(other.0).map(Self)
                }
            }

            impl CheckedMul<$ty> for $ty {
                type Output = $ty;

                #[inline]
                fn checked_mul(self, other: $ty) -> Option<Self::Output> {
                    self.0.checked_mul(other.0).map(Self)
                }
            }

            impl CheckedDiv<$ty> for $ty {
                type Output = $ty;

                #[inline]
                fn checked_div(self, other: $ty) -> Option<Self::Output> {
                    self.0.checked_div(other.0).map(Self)
                }
            }

            impl CheckedRem<$ty> for $ty {
                type Output = $ty;

                #[inline]
                fn checked_rem(self, other: $ty) -> Option<Self::Output> {
                    self.0.checked_rem(other.0).map(Self)
                }
            }

            impl CheckedNeg for $ty {
                type Output = $ty;

                #[inline]
                fn checked_neg(self) -> Option<Self::Output> {
                    self.0.checked_neg().map(Self)
                }
            }
        )*
    };
}

macro_rules! impl_checked_euclid_traits_via_inner {
    ($($ty:ty),*) => {
        $(
            impl CheckedDivEuclid<$ty> for $ty {
                type Output = $ty;

                #[inline]
                fn checked_div_euclid(self, other: $ty) -> Option<Self::Output> {
                    self.0.checked_div_euclid(other.0).map(Self)
                }
            }

            impl CheckedRemEuclid<$ty> for $ty {
                type Output = $ty;

                #[inline]
                fn checked_rem_euclid(self, other: $ty) -> Option<Self::Output> {
                    self.0.checked_rem_euclid(other.0).map(Self)
                }
            }

            impl CheckedDivRemEuclid<$ty> for $ty {
                type Output = $ty;

                #[inline]
                fn checked_div_rem_euclid(
                    self,
                    other: $ty,
                ) -> Option<(Self::Output, Self::Output)> {
                    Some((
                        Self(self.0.checked_div_euclid(other.0)?),
                        Self(self.0.checked_rem_euclid(other.0)?),
                    ))
                }
            }
        )*
    };
}

macro_rules! impl_checked_div_rem_trait_via_inner {
    ($($ty:ty),*) => {
        $(
            impl CheckedDivRem<$ty> for $ty {
                type Output = $ty;

                #[inline]
                fn checked_div_rem(self, other: $ty) -> Option<(Self::Output, Self::Output)> {
                    Some((
                        Self(self.0.checked_div(other.0)?),
                        Self(self.0.checked_rem(other.0)?),
                    ))
                }
            }
        )*
    };
}

macro_rules! impl_checked_mul_add_trait_via_inner {
    ($($ty:ty),*) => {
        $(
            impl CheckedMulAdd<$ty, $ty> for $ty {
                type Output = $ty;

                #[inline]
                fn checked_mul_add(self, a: $ty, b: $ty) -> Option<Self::Output> {
                    self.0
                        .checked_mul(a.0)
                        .and_then(|product| product.checked_add(b.0))
                        .map(Self)
                }
            }
        )*
    };
}

macro_rules! impl_checked_arith_traits_via_inner {
    ($ty:ty $(, $rest:ident)*) => {
        impl_checked_field_traits_via_inner!($ty);
        impl_checked_div_rem_trait_via_inner!($ty);
        impl_checked_euclid_traits_via_inner!($ty);
        impl_checked_mul_add_trait_via_inner!($ty);
    };
}

macro_rules! impl_checked_field_traits_for_floats {
    ($($ty:ty),*) => {
        $(
            impl CheckedAdd<$ty> for $ty {
                type Output = $ty;

                #[inline]
                fn checked_add(self, other: $ty) -> Option<Self::Output> {
                    let result = self + other;

                    result.is_finite().then_some(result)
                }
            }

            impl CheckedSub<$ty> for $ty {
                type Output = $ty;

                #[inline]
                fn checked_sub(self, other: $ty) -> Option<Self::Output> {
                    let result = self - other;

                    result.is_finite().then_some(result)
                }
            }

            impl CheckedMul<$ty> for $ty {
                type Output = $ty;

                #[inline]
                fn checked_mul(self, other: $ty) -> Option<Self::Output> {
                    let result = self * other;

                    result.is_finite().then_some(result)
                }
            }

            impl CheckedDiv<$ty> for $ty {
                type Output = $ty;

                #[inline]
                fn checked_div(self, other: $ty) -> Option<Self::Output> {
                    let result = self / other;

                    result.is_finite().then_some(result)
                }
            }

            impl CheckedRem<$ty> for $ty {
                type Output = $ty;

                #[inline]
                fn checked_rem(self, other: $ty) -> Option<Self::Output> {
                    let result = self % other;

                    result.is_finite().then_some(result)
                }
            }

            impl CheckedNeg for $ty {
                type Output = $ty;

                #[inline]
                fn checked_neg(self) -> Option<Self::Output> {
                    let result = -self;

                    result.is_finite().then_some(result)
                }
            }
        )*
    };
}

macro_rules! impl_checked_div_rem_trait_for_floats {
    ($($ty:ty),*) => {
        $(
            impl CheckedDivRem<$ty> for $ty {
                type Output = $ty;

                #[inline]
                fn checked_div_rem(self, other: $ty) -> Option<(Self::Output, Self::Output)> {
                    let div = self / other;
                    let rem = self % other;

                    (div.is_finite() && rem.is_finite()).then_some((div, rem))
                }
            }
        )*
    };
}

#[cfg(any(feature = "std", feature = "libm"))]
macro_rules! impl_checked_euclid_traits_for_floats {
    ($($ty:ty),*) => {
        $(
            impl CheckedDivEuclid<$ty> for $ty {
                type Output = $ty;

                #[inline]
                fn checked_div_euclid(self, other: $ty) -> Option<Self::Output> {
                    let result = DivEuclid::div_euclid(self, other);

                    result.is_finite().then_some(result)
                }
            }

            impl CheckedRemEuclid<$ty> for $ty {
                type Output = $ty;

                #[inline]
                fn checked_rem_euclid(self, other: $ty) -> Option<Self::Output> {
                    let result = RemEuclid::rem_euclid(self, other);

                    result.is_finite().then_some(result)
                }
            }

            impl CheckedDivRemEuclid<$ty> for $ty {
                type Output = $ty;

                #[inline]
                fn checked_div_rem_euclid(
                    self,
                    other: $ty,
                ) -> Option<(Self::Output, Self::Output)> {
                    let div = DivEuclid::div_euclid(self, other);
                    let rem = RemEuclid::rem_euclid(self, other);

                    (div.is_finite() && rem.is_finite()).then_some((div, rem))
                }
            }
        )*
    };
}

macro_rules! impl_checked_mul_add_trait_for_floats {
    ($($ty:ty),*) => {
        $(
            impl CheckedMulAdd<$ty, $ty> for $ty {
                type Output = $ty;

                #[inline]
                fn checked_mul_add(self, a: $ty, b: $ty) -> Option<Self::Output> {
                    let result = MulAdd::mul_add(self, a, b);

                    result.is_finite().then_some(result)
                }
            }
        )*
    };
}

macro_rules! impl_checked_arith_traits_for_floats {
    ($($ty:ty),*) => {
        impl_checked_field_traits_for_floats!($($ty),*);
        impl_checked_div_rem_trait_for_floats!($($ty),*);
        #[cfg(any(feature = "std", feature = "libm"))]
        impl_checked_euclid_traits_for_floats!($($ty),*);
    };
}

for_signed_ints!(impl_checked_arith_traits_for_signed_ints);
for_unsigned_ints!(impl_checked_arith_traits_for_unsigned_ints);
for_wrapping_signed_ints!(impl_checked_arith_traits_via_inner);
for_wrapping_unsigned_ints!(impl_checked_arith_traits_via_inner);
for_saturating_signed_ints!(impl_checked_arith_traits_via_inner);
for_saturating_unsigned_ints!(impl_checked_arith_traits_via_inner);
impl_checked_arith_traits_for_floats!(f32, f64);

impl_checked_mul_add_trait_for_floats!(f32, f64);

macro_rules! test_checked_arith_traits_int_nonnegative {
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

                assert_eq!(CheckedDivRem::checked_div_rem(four, two), Some((two, zero)));
                assert_eq!(CheckedDivRem::checked_div_rem(five, two), Some((two, one)));
                assert_eq!(CheckedDivRem::checked_div_rem(three, two), Some((one, one)));
                assert_eq!(CheckedDivRem::checked_div_rem(one, zero), None);
                assert_eq!(
                    Object::new(five).checked_div_rem(Object::new(two)),
                    CheckedDivRem::checked_div_rem(five, two)
                        .map(|(a, b)| (Object::new(a), Object::new(b)))
                );
            }

            #[test]
            fn test_div_euclid() {
                let zero = <$ty as AssocConstZero>::ZERO;
                let one = <$ty as AssocConstOne>::ONE;
                let two = one + one;
                let three = two + one;
                let four = two + two;
                let five = four + one;

                assert_eq!(CheckedDivEuclid::checked_div_euclid(four, two), Some(two));
                assert_eq!(CheckedDivEuclid::checked_div_euclid(five, two), Some(two));
                assert_eq!(CheckedDivEuclid::checked_div_euclid(three, two), Some(one));
                assert_eq!(CheckedDivEuclid::checked_div_euclid(one, zero), None);
                assert_eq!(
                    Object::new(five).checked_div_euclid(Object::new(two)),
                    CheckedDivEuclid::checked_div_euclid(five, two).map(Object::new)
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

                assert_eq!(CheckedRemEuclid::checked_rem_euclid(four, two), Some(zero));
                assert_eq!(CheckedRemEuclid::checked_rem_euclid(five, two), Some(one));
                assert_eq!(CheckedRemEuclid::checked_rem_euclid(three, two), Some(one));
                assert_eq!(CheckedRemEuclid::checked_rem_euclid(one, zero), None);
                assert_eq!(
                    Object::new(five).checked_rem_euclid(Object::new(two)),
                    CheckedRemEuclid::checked_rem_euclid(five, two).map(Object::new)
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
                    CheckedDivRemEuclid::checked_div_rem_euclid(five, two),
                    Some((two, one))
                );
                assert_eq!(
                    CheckedDivRemEuclid::checked_div_rem_euclid(four, two),
                    Some((two, zero))
                );
                assert_eq!(CheckedDivRemEuclid::checked_div_rem_euclid(one, zero), None);
                assert_eq!(
                    Object::new(five).checked_div_rem_euclid(Object::new(two)),
                    CheckedDivRemEuclid::checked_div_rem_euclid(five, two)
                        .map(|(a, b)| (Object::new(a), Object::new(b)))
                );
            }

            #[test]
            fn test_mul_add() {
                let zero = <$ty as AssocConstZero>::ZERO;
                let one = <$ty as AssocConstOne>::ONE;
                let two = one + one;
                let four = two + two;

                assert_eq!(CheckedMulAdd::checked_mul_add(two, two, zero), Some(four));
                assert_eq!(CheckedMulAdd::checked_mul_add(one, one, one), Some(two));
                assert_eq!(CheckedMulAdd::checked_mul_add(two, one, two), Some(four));
                assert_eq!(
                    Object::new(two).checked_mul_add(Object::new(one), Object::new(two)),
                    CheckedMulAdd::checked_mul_add(two, one, two).map(Object::new)
                );
            }
        }
    };
}

macro_rules! test_checked_arith_traits_int_negative {
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
                    CheckedDivRem::checked_div_rem(-five, two),
                    Some((-two, -one))
                );
                assert_eq!(
                    CheckedDivRem::checked_div_rem(-four, two),
                    Some((-two, zero))
                );
                assert_eq!(
                    CheckedDivRem::checked_div_rem(<$ty as MinFinite>::MIN_FINITE, -one),
                    None
                );
                assert_eq!(
                    Object::new(five).checked_div_rem(Object::new(two)),
                    CheckedDivRem::checked_div_rem(five, two)
                        .map(|(a, b)| (Object::new(a), Object::new(b)))
                );
            }

            #[test]
            fn test_div_euclid() {
                let one = <$ty as AssocConstOne>::ONE;
                let two = one + one;
                let three = two + one;
                let five = two + two + one;

                assert_eq!(
                    CheckedDivEuclid::checked_div_euclid(-five, two),
                    Some(-three)
                );
                assert_eq!(
                    CheckedDivEuclid::checked_div_euclid(-five, -two),
                    Some(three)
                );
                assert_eq!(
                    Object::new(five).checked_div_euclid(Object::new(two)),
                    CheckedDivEuclid::checked_div_euclid(five, two).map(Object::new)
                );
            }

            #[test]
            fn test_rem_euclid() {
                let zero = <$ty as AssocConstZero>::ZERO;
                let one = <$ty as AssocConstOne>::ONE;
                let two = one + one;
                let four = two + two;
                let five = four + one;

                assert_eq!(CheckedRemEuclid::checked_rem_euclid(-five, two), Some(one));
                assert_eq!(CheckedRemEuclid::checked_rem_euclid(-four, two), Some(zero));
                assert_eq!(
                    Object::new(five).checked_rem_euclid(Object::new(two)),
                    CheckedRemEuclid::checked_rem_euclid(five, two).map(Object::new)
                );
            }

            #[test]
            fn test_div_rem_euclid() {
                let one = <$ty as AssocConstOne>::ONE;
                let two = one + one;
                let three = two + one;
                let five = two + two + one;

                assert_eq!(
                    CheckedDivRemEuclid::checked_div_rem_euclid(-five, two),
                    Some((-three, one))
                );
                assert_eq!(
                    Object::new(five).checked_div_rem_euclid(Object::new(two)),
                    CheckedDivRemEuclid::checked_div_rem_euclid(five, two)
                        .map(|(a, b)| (Object::new(a), Object::new(b)))
                );
            }

            #[test]
            fn test_neg() {
                let zero = <$ty as AssocConstZero>::ZERO;
                let one = <$ty as AssocConstOne>::ONE;

                assert_eq!(CheckedNeg::checked_neg(one), Some(-one));
                assert_eq!(CheckedNeg::checked_neg(-one), Some(one));
                assert_eq!(CheckedNeg::checked_neg(zero), Some(zero));
                assert_eq!(
                    CheckedNeg::checked_neg(<$ty as MinFinite>::MIN_FINITE),
                    None
                );
                assert_eq!(
                    Object::new(one).checked_neg(),
                    CheckedNeg::checked_neg(one).map(Object::new)
                );
            }
        }
    };
}

macro_rules! test_checked_arith_traits_float_nonnegative {
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
                let four = two + two;

                assert_eq!(CheckedDivRem::checked_div_rem(four, two), Some((two, zero)));
                assert_eq!(CheckedDivRem::checked_div_rem(two, one), Some((two, zero)));
                assert_eq!(CheckedDivRem::checked_div_rem(one, zero), None);
                assert_eq!(
                    Object::new(four).checked_div_rem(Object::new(two)),
                    CheckedDivRem::checked_div_rem(four, two)
                        .map(|(a, b)| (Object::new(a), Object::new(b)))
                );
            }

            #[cfg(any(feature = "std", feature = "libm"))]
            #[test]
            fn test_div_euclid() {
                let zero = <$ty as AssocConstZero>::ZERO;
                let one = <$ty as AssocConstOne>::ONE;
                let two = one + one;
                let three = two + one;
                let four = two + two;
                let five = four + one;

                assert_eq!(CheckedDivEuclid::checked_div_euclid(four, two), Some(two));
                assert_eq!(CheckedDivEuclid::checked_div_euclid(five, two), Some(two));
                assert_eq!(CheckedDivEuclid::checked_div_euclid(three, two), Some(one));
                assert_eq!(CheckedDivEuclid::checked_div_euclid(one, zero), None);
                assert_eq!(
                    Object::new(five).checked_div_euclid(Object::new(two)),
                    CheckedDivEuclid::checked_div_euclid(five, two).map(Object::new)
                );
            }

            #[cfg(any(feature = "std", feature = "libm"))]
            #[test]
            fn test_rem_euclid() {
                let zero = <$ty as AssocConstZero>::ZERO;
                let one = <$ty as AssocConstOne>::ONE;
                let two = one + one;
                let three = two + one;
                let four = two + two;
                let five = four + one;

                assert_eq!(CheckedRemEuclid::checked_rem_euclid(four, two), Some(zero));
                assert_eq!(CheckedRemEuclid::checked_rem_euclid(five, two), Some(one));
                assert_eq!(CheckedRemEuclid::checked_rem_euclid(three, two), Some(one));
                assert_eq!(CheckedRemEuclid::checked_rem_euclid(one, zero), None);
                assert_eq!(
                    Object::new(five).checked_rem_euclid(Object::new(two)),
                    CheckedRemEuclid::checked_rem_euclid(five, two).map(Object::new)
                );
            }

            #[cfg(any(feature = "std", feature = "libm"))]
            #[test]
            fn test_div_rem_euclid() {
                let zero = <$ty as AssocConstZero>::ZERO;
                let one = <$ty as AssocConstOne>::ONE;
                let two = one + one;
                let four = two + two;
                let five = four + one;

                assert_eq!(
                    CheckedDivRemEuclid::checked_div_rem_euclid(five, two),
                    Some((two, one))
                );
                assert_eq!(
                    CheckedDivRemEuclid::checked_div_rem_euclid(four, two),
                    Some((two, zero))
                );
                assert_eq!(CheckedDivRemEuclid::checked_div_rem_euclid(one, zero), None);
                assert_eq!(
                    Object::new(five).checked_div_rem_euclid(Object::new(two)),
                    CheckedDivRemEuclid::checked_div_rem_euclid(five, two)
                        .map(|(a, b)| (Object::new(a), Object::new(b)))
                );
            }

            #[test]
            fn test_mul_add() {
                let zero = <$ty as AssocConstZero>::ZERO;
                let one = <$ty as AssocConstOne>::ONE;
                let two = one + one;
                let four = two + two;

                assert_eq!(CheckedMulAdd::checked_mul_add(two, two, zero), Some(four));
                assert_eq!(CheckedMulAdd::checked_mul_add(one, one, one), Some(two));
                assert_eq!(CheckedMulAdd::checked_mul_add(two, one, two), Some(four));
                assert_eq!(
                    Object::new(two).checked_mul_add(Object::new(one), Object::new(two)),
                    CheckedMulAdd::checked_mul_add(two, one, two).map(Object::new)
                );
            }
        }
    };
}

macro_rules! test_checked_arith_traits_float_negative {
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
                let four = two + two;

                assert_eq!(
                    CheckedDivRem::checked_div_rem(-four, two),
                    Some((-two, zero))
                );
                assert_eq!(
                    CheckedDivRem::checked_div_rem(-two, one),
                    Some((-two, zero))
                );
                assert_eq!(
                    Object::new(four).checked_div_rem(Object::new(two)),
                    CheckedDivRem::checked_div_rem(four, two)
                        .map(|(a, b)| (Object::new(a), Object::new(b)))
                );
            }

            #[cfg(any(feature = "std", feature = "libm"))]
            #[test]
            fn test_div_euclid() {
                let one = <$ty as AssocConstOne>::ONE;
                let two = one + one;
                let three = two + one;
                let five = two + two + one;

                assert_eq!(
                    CheckedDivEuclid::checked_div_euclid(-five, two),
                    Some(-three)
                );
                assert_eq!(
                    CheckedDivEuclid::checked_div_euclid(-five, -two),
                    Some(three)
                );
                assert_eq!(
                    Object::new(five).checked_div_euclid(Object::new(two)),
                    CheckedDivEuclid::checked_div_euclid(five, two).map(Object::new)
                );
            }

            #[cfg(any(feature = "std", feature = "libm"))]
            #[test]
            fn test_rem_euclid() {
                let zero = <$ty as AssocConstZero>::ZERO;
                let one = <$ty as AssocConstOne>::ONE;
                let two = one + one;
                let four = two + two;
                let five = four + one;

                assert_eq!(CheckedRemEuclid::checked_rem_euclid(-five, two), Some(one));
                assert_eq!(CheckedRemEuclid::checked_rem_euclid(-four, two), Some(zero));
                assert_eq!(
                    Object::new(five).checked_rem_euclid(Object::new(two)),
                    CheckedRemEuclid::checked_rem_euclid(five, two).map(Object::new)
                );
            }

            #[cfg(any(feature = "std", feature = "libm"))]
            #[test]
            fn test_div_rem_euclid() {
                let one = <$ty as AssocConstOne>::ONE;
                let two = one + one;
                let three = two + one;
                let five = two + two + one;

                assert_eq!(
                    CheckedDivRemEuclid::checked_div_rem_euclid(-five, two),
                    Some((-three, one))
                );
                assert_eq!(
                    Object::new(five).checked_div_rem_euclid(Object::new(two)),
                    CheckedDivRemEuclid::checked_div_rem_euclid(five, two)
                        .map(|(a, b)| (Object::new(a), Object::new(b)))
                );
            }

            #[test]
            fn test_neg() {
                let zero = <$ty as AssocConstZero>::ZERO;
                let one = <$ty as AssocConstOne>::ONE;

                assert_eq!(CheckedNeg::checked_neg(one), Some(-one));
                assert_eq!(CheckedNeg::checked_neg(-one), Some(one));
                assert_eq!(CheckedNeg::checked_neg(zero), Some(zero));
                assert_eq!(
                    Object::new(one).checked_neg(),
                    CheckedNeg::checked_neg(one).map(Object::new)
                );
            }
        }
    };
}

for_signed_ints!(
    test_checked_arith_traits_int_nonnegative; test_checked_arith_traits_int_negative
);
for_unsigned_ints!(test_checked_arith_traits_int_nonnegative);

test_checked_arith_traits_float_nonnegative!(f32, f32_nonnegative_tests);
test_checked_arith_traits_float_negative!(f32, f32_negative_tests);
test_checked_arith_traits_float_nonnegative!(f64, f64_nonnegative_tests);
test_checked_arith_traits_float_negative!(f64, f64_negative_tests);
