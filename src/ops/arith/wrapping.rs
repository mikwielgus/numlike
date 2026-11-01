// SPDX-FileCopyrightText: 2026 numlike contributors
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use crate::types::Object;

/// Bundle of wrapping arithmetic and fused arithmetic operations.
pub trait FullWrappingArithOps<Rhs = Self>:
    WrappingArithOps<Rhs> + WrappingFusedArithOps<Rhs>
{
}

/// Bundle of wrapping fused arithmetic operations.
pub trait WrappingFusedArithOps<Rhs = Self>: WrappingMulAdd<Rhs, Rhs> {}
impl<Rhs, T: WrappingMulAdd<Rhs, Rhs>> WrappingFusedArithOps<Rhs> for T {}

/// Wrapping (modular) fused multiply-add. Computes `(self * a) + b`,
/// wrapping around at the boundary of the type.
pub trait WrappingMulAdd<A = Self, B = Self> {
    /// The resulting type after applying the operation.
    type Output;

    /// Wrapping (modular) fused multiply-add. Computes `(self * a) + b`,
    /// wrapping around at the boundary of the type.
    fn wrapping_mul_add(self, a: A, b: B) -> Self::Output;
}

impl<B: WrappingMulAdd<Output = B>> WrappingMulAdd for Object<B> {
    type Output = Self;

    fn wrapping_mul_add(self, a: Self, b: Self) -> Self::Output {
        Object(<B as WrappingMulAdd>::wrapping_mul_add(self.0, a.0, b.0))
    }
}

impl<B: WrappingMulAdd<Output = B>> Object<B> {
    /// Wrapping (modular) fused multiply-add. Computes `(self * a) + b`,
    /// wrapping around at the boundary of the type.
    pub fn wrapping_mul_add(self, a: Self, b: Self) -> Self {
        Object(<B as WrappingMulAdd>::wrapping_mul_add(self.0, a.0, b.0))
    }
}

/// Bundle of wrapping arithmetic operations.
pub trait WrappingArithOps<Rhs = Self>:
    WrappingEuclidOps<Rhs>
    + WrappingFieldOps<Rhs>
    + WrappingRem<Rhs, Output = Self>
    + WrappingDivRem<Rhs, Output = Self>
{
}
impl<
    Rhs,
    T: WrappingEuclidOps<Rhs>
        + WrappingFieldOps<Rhs>
        + WrappingRem<Rhs, Output = Self>
        + WrappingDivRem<Rhs, Output = Self>,
> WrappingArithOps<Rhs> for T
{
}

/// Wrapping (modular) remainder. Computes `self % other`, wrapping around at the
/// boundary of the type.
///
/// Such wrap-around never actually occurs mathematically; implementation artifacts make `x % y`
/// invalid for `MIN / -1` on a signed type (where `MIN` is the negative minimal value). In such a case,
/// this function returns `0`.
///
/// # Panics
///
/// This function will panic if `other` is zero.
pub trait WrappingRem<Rhs = Self> {
    /// The resulting type after applying the operation.
    type Output;

    /// Wrapping (modular) remainder. Computes `self % other`, wrapping around at the
    /// boundary of the type.
    fn wrapping_rem(self, other: Rhs) -> Self::Output;
}

impl<B: WrappingRem<Output = B>> WrappingRem for Object<B> {
    type Output = Self;

    fn wrapping_rem(self, other: Self) -> Self::Output {
        Object(<B as WrappingRem>::wrapping_rem(self.0, other.0))
    }
}

impl<B: WrappingRem<Output = B>> Object<B> {
    /// Wrapping (modular) remainder. Computes `self % other`, wrapping around at the
    /// boundary of the type.
    pub fn wrapping_rem(self, other: Self) -> Self {
        Object(<B as WrappingRem>::wrapping_rem(self.0, other.0))
    }
}

/// Wrapping simultaneous quotient and remainder. Computes
/// `(self / other, self % other)`, wrapping around at the boundary of the type.
///
/// # Panics
///
/// This function will panic if `other` is zero.
pub trait WrappingDivRem<Rhs> {
    /// The resulting type after applying the operation.
    type Output;

    /// Wrapping simultaneous quotient and remainder. Computes
    /// `(self / other, self % other)`, wrapping around at the boundary of the type.
    fn wrapping_div_rem(self, other: Rhs) -> (Self::Output, Self::Output);
}

impl<B: WrappingDivRem<B, Output = B>> WrappingDivRem<Object<B>> for Object<B> {
    type Output = Self;

    fn wrapping_div_rem(self, other: Self) -> (Self::Output, Self::Output) {
        let (a, b) = <B as WrappingDivRem<B>>::wrapping_div_rem(self.0, other.0);

        (Object(a), Object(b))
    }
}

impl<B: WrappingDivRem<B, Output = B>> Object<B> {
    /// Wrapping simultaneous quotient and remainder. Computes
    /// `(self / other, self % other)`, wrapping around at the boundary of the type.
    pub fn wrapping_div_rem(self, other: Self) -> (Self, Self) {
        let (a, b) = <B as WrappingDivRem<B>>::wrapping_div_rem(self.0, other.0);

        (Object(a), Object(b))
    }
}

/// Bundle of wrapping Euclidean division operations.
pub trait WrappingEuclidOps<Rhs>:
    WrappingDivEuclid<Rhs> + WrappingRemEuclid<Rhs> + WrappingDivRemEuclid<Rhs>
{
}
impl<Rhs, T: WrappingDivEuclid<Rhs> + WrappingRemEuclid<Rhs> + WrappingDivRemEuclid<Rhs>>
    WrappingEuclidOps<Rhs> for T
{
}

/// Wrapping Euclidean division. Computes `self.div_euclid(other)`,
/// wrapping around at the boundary of the type.
///
/// Wrapping will only occur in `MIN / -1` on a signed type (where `MIN` is the negative minimal value
/// for the type). This is equivalent to `-MIN`, a positive value that is too large to represent in the
/// type. In this case, this method returns `MIN` itself.
///
/// # Panics
///
/// This function will panic if `other` is zero.
pub trait WrappingDivEuclid<Rhs = Self> {
    /// The resulting type after applying the operation.
    type Output;

    /// Wrapping Euclidean division. Computes `self.div_euclid(other)`,
    /// wrapping around at the boundary of the type.
    fn wrapping_div_euclid(self, other: Rhs) -> Self::Output;
}

impl<B: WrappingDivEuclid<Output = B>> WrappingDivEuclid for Object<B> {
    type Output = Self;

    fn wrapping_div_euclid(self, other: Self) -> Self::Output {
        Object(<B as WrappingDivEuclid>::wrapping_div_euclid(
            self.0, other.0,
        ))
    }
}

impl<B: WrappingDivEuclid<Output = B>> Object<B> {
    /// Wrapping Euclidean division. Computes `self.div_euclid(other)`,
    /// wrapping around at the boundary of the type.
    pub fn wrapping_div_euclid(self, other: Self) -> Self {
        Object(<B as WrappingDivEuclid>::wrapping_div_euclid(
            self.0, other.0,
        ))
    }
}

/// Wrapping Euclidean remainder. Computes `self.rem_euclid(other)`, wrapping around
/// at the boundary of the type.
///
/// Wrapping will only occur in `MIN % -1` on a signed type (where `MIN` is the negative minimal value
/// for the type). In this case, this method returns `0`.
///
/// # Panics
///
/// This function will panic if `other` is zero.
pub trait WrappingRemEuclid<Rhs = Self> {
    /// The resulting type after applying the operation.
    type Output;

    /// Wrapping Euclidean remainder. Computes `self.rem_euclid(other)`, wrapping around
    /// at the boundary of the type.
    fn wrapping_rem_euclid(self, other: Rhs) -> Self::Output;
}

impl<B: WrappingRemEuclid<Output = B>> WrappingRemEuclid for Object<B> {
    type Output = Self;

    fn wrapping_rem_euclid(self, other: Self) -> Self::Output {
        Object(<B as WrappingRemEuclid>::wrapping_rem_euclid(
            self.0, other.0,
        ))
    }
}

impl<B: WrappingRemEuclid<Output = B>> Object<B> {
    /// Wrapping Euclidean remainder. Computes `self.rem_euclid(other)`, wrapping around
    /// at the boundary of the type.
    pub fn wrapping_rem_euclid(self, other: Self) -> Self {
        Object(<B as WrappingRemEuclid>::wrapping_rem_euclid(
            self.0, other.0,
        ))
    }
}

/// Wrapping simultaneous Euclidean quotient and remainder. Computes
/// `(self.div_euclid(other), self.rem_euclid(other))`, wrapping around at the
/// boundary of the type.
///
/// # Panics
///
/// This function will panic if `other` is zero.
pub trait WrappingDivRemEuclid<Rhs = Self> {
    /// The resulting type after applying the operation.
    type Output;

    /// Wrapping simultaneous Euclidean quotient and remainder. Computes
    /// `(self.div_euclid(other), self.rem_euclid(other))`, wrapping around at the
    /// boundary of the type.
    fn wrapping_div_rem_euclid(self, other: Rhs) -> (Self::Output, Self::Output);
}

impl<B: WrappingDivRemEuclid<Output = B>> WrappingDivRemEuclid for Object<B> {
    type Output = Self;

    fn wrapping_div_rem_euclid(self, other: Self) -> (Self::Output, Self::Output) {
        let (a, b) = <B as WrappingDivRemEuclid>::wrapping_div_rem_euclid(self.0, other.0);

        (Object(a), Object(b))
    }
}

impl<B: WrappingDivRemEuclid<Output = B>> Object<B> {
    /// Wrapping simultaneous Euclidean quotient and remainder. Computes
    /// `(self.div_euclid(other), self.rem_euclid(other))`, wrapping around at the
    /// boundary of the type.
    pub fn wrapping_div_rem_euclid(self, other: Self) -> (Self, Self) {
        let (a, b) = <B as WrappingDivRemEuclid>::wrapping_div_rem_euclid(self.0, other.0);

        (Object(a), Object(b))
    }
}

/// Bundle of wrapping field operations.
pub trait WrappingFieldOps<Rhs = Self>:
    WrappingRingOps<Rhs> + WrappingDiv<Rhs, Output = Self>
{
}
impl<Rhs, T: WrappingRingOps<Rhs> + WrappingDiv<Rhs, Output = Self>> WrappingFieldOps<Rhs> for T {}

/// Wrapping (modular) division. Computes `self / other`, wrapping around at the
/// boundary of the type.
///
/// The only case where such wrapping can occur is when one divides `MIN / -1` on a signed type (where
/// `MIN` is the negative minimal value for the type); this is equivalent to `-MIN`, a positive value
/// that is too large to represent in the type. In such a case, this function returns `MIN` itself.
///
/// # Panics
///
/// This function will panic if `other` is zero.
pub trait WrappingDiv<Rhs = Self> {
    /// The resulting type after applying the operation.
    type Output;

    /// Wrapping (modular) division. Computes `self / other`, wrapping around at the
    /// boundary of the type.
    fn wrapping_div(self, other: Rhs) -> Self::Output;
}

impl<B: WrappingDiv<Output = B>> WrappingDiv for Object<B> {
    type Output = Self;

    fn wrapping_div(self, other: Self) -> Self::Output {
        Object(<B as WrappingDiv>::wrapping_div(self.0, other.0))
    }
}

impl<B: WrappingDiv<Output = B>> Object<B> {
    /// Wrapping (modular) division. Computes `self / other`, wrapping around at the
    /// boundary of the type.
    pub fn wrapping_div(self, other: Self) -> Self {
        Object(<B as WrappingDiv>::wrapping_div(self.0, other.0))
    }
}

/// Bundle of wrapping ring operations.
pub trait WrappingRingOps<Rhs = Self>:
    WrappingAdd<Rhs, Output = Self>
    + WrappingSub<Rhs, Output = Self>
    + WrappingMul<Rhs, Output = Self>
    + WrappingNeg<Output = Self>
{
}
impl<
    Rhs,
    T: WrappingAdd<Rhs, Output = Self>
        + WrappingSub<Rhs, Output = Self>
        + WrappingMul<Rhs, Output = Self>
        + WrappingNeg<Output = Self>,
> WrappingRingOps<Rhs> for T
{
}

/// Wrapping (modular) addition. Computes `self + other`, wrapping around at the
/// boundary of the type.
pub trait WrappingAdd<Rhs = Self> {
    /// The resulting type after applying the operation.
    type Output;

    /// Wrapping (modular) addition. Computes `self + other`, wrapping around at the
    /// boundary of the type.
    fn wrapping_add(self, other: Rhs) -> Self::Output;
}

impl<B: WrappingAdd<Output = B>> WrappingAdd for Object<B> {
    type Output = Self;

    fn wrapping_add(self, other: Self) -> Self::Output {
        Object(<B as WrappingAdd>::wrapping_add(self.0, other.0))
    }
}

impl<B: WrappingAdd<Output = B>> Object<B> {
    /// Wrapping (modular) addition. Computes `self + other`, wrapping around at the
    /// boundary of the type.
    pub fn wrapping_add(self, other: Self) -> Self {
        Object(<B as WrappingAdd>::wrapping_add(self.0, other.0))
    }
}

/// Wrapping (modular) subtraction. Computes `self - other`, wrapping around at the
/// boundary of the type.
pub trait WrappingSub<Rhs = Self> {
    /// The resulting type after applying the operation.
    type Output;

    /// Wrapping (modular) subtraction. Computes `self - other`, wrapping around at the
    /// boundary of the type.
    fn wrapping_sub(self, other: Rhs) -> Self::Output;
}

impl<B: WrappingSub<Output = B>> WrappingSub for Object<B> {
    type Output = Self;

    fn wrapping_sub(self, other: Self) -> Self::Output {
        Object(<B as WrappingSub>::wrapping_sub(self.0, other.0))
    }
}

impl<B: WrappingSub<Output = B>> Object<B> {
    /// Wrapping (modular) subtraction. Computes `self - other`, wrapping around at the
    /// boundary of the type.
    pub fn wrapping_sub(self, other: Self) -> Self {
        Object(<B as WrappingSub>::wrapping_sub(self.0, other.0))
    }
}

/// Wrapping (modular) multiplication. Computes `self * other`, wrapping around at
/// the boundary of the type.
pub trait WrappingMul<Rhs = Self> {
    /// The resulting type after applying the operation.
    type Output;

    /// Wrapping (modular) multiplication. Computes `self * other`, wrapping around at
    /// the boundary of the type.
    fn wrapping_mul(self, other: Rhs) -> Self::Output;
}

impl<B: WrappingMul<Output = B>> WrappingMul for Object<B> {
    type Output = Self;

    fn wrapping_mul(self, other: Self) -> Self::Output {
        Object(<B as WrappingMul>::wrapping_mul(self.0, other.0))
    }
}

impl<B: WrappingMul<Output = B>> Object<B> {
    /// Wrapping (modular) multiplication. Computes `self * other`, wrapping around at
    /// the boundary of the type.
    pub fn wrapping_mul(self, other: Self) -> Self {
        Object(<B as WrappingMul>::wrapping_mul(self.0, other.0))
    }
}

/// Wrapping (modular) negation. Computes `-self`, wrapping around at the boundary
/// of the type.
///
/// The only case where such wrapping can occur is when one negates `MIN` on a signed type (where `MIN`
/// is the negative minimal value for the type); this is a positive value that is too large to represent
/// in the type. In such a case, this function returns `MIN` itself.
pub trait WrappingNeg {
    /// The resulting type after applying the operation.
    type Output;

    /// Wrapping (modular) negation. Computes `-self`, wrapping around at the boundary
    /// of the type.
    fn wrapping_neg(self) -> Self::Output;
}

impl<B: WrappingNeg<Output = B>> WrappingNeg for Object<B> {
    type Output = Self;

    fn wrapping_neg(self) -> Self::Output {
        Object(<B as WrappingNeg>::wrapping_neg(self.0))
    }
}

impl<B: WrappingNeg<Output = B>> Object<B> {
    /// Wrapping (modular) negation. Computes `-self`, wrapping around at the boundary
    /// of the type.
    pub fn wrapping_neg(self) -> Self {
        Object(<B as WrappingNeg>::wrapping_neg(self.0))
    }
}

macro_rules! impl_wrapping_field_traits_for_ints {
    ($($ty:ty),*) => {
        $(
            impl WrappingAdd<$ty> for $ty {
                type Output = $ty;

                #[inline]
                fn wrapping_add(self, other: $ty) -> Self::Output {
                    <$ty>::wrapping_add(self, other)
                }
            }

            impl WrappingSub<$ty> for $ty {
                type Output = $ty;

                #[inline]
                fn wrapping_sub(self, other: $ty) -> Self::Output {
                    <$ty>::wrapping_sub(self, other)
                }
            }

            impl WrappingMul<$ty> for $ty {
                type Output = $ty;

                #[inline]
                fn wrapping_mul(self, other: $ty) -> Self::Output {
                    <$ty>::wrapping_mul(self, other)
                }
            }

            impl WrappingDiv<$ty> for $ty {
                type Output = $ty;

                #[inline]
                fn wrapping_div(self, other: $ty) -> Self::Output {
                    <$ty>::wrapping_div(self, other)
                }
            }

            impl WrappingRem<$ty> for $ty {
                type Output = $ty;

                #[inline]
                fn wrapping_rem(self, other: $ty) -> Self::Output {
                    <$ty>::wrapping_rem(self, other)
                }
            }

            impl WrappingNeg for $ty {
                type Output = $ty;

                #[inline]
                fn wrapping_neg(self) -> Self::Output {
                    <$ty>::wrapping_neg(self)
                }
            }
        )*
    };
}

macro_rules! impl_wrapping_euclid_traits {
    ($($ty:ty),*) => {
        $(
            impl WrappingDivEuclid<$ty> for $ty {
                type Output = $ty;

                #[inline]
                fn wrapping_div_euclid(self, other: $ty) -> Self::Output {
                    <$ty>::wrapping_div_euclid(self, other)
                }
            }

            impl WrappingRemEuclid<$ty> for $ty {
                type Output = $ty;

                #[inline]
                fn wrapping_rem_euclid(self, other: $ty) -> Self::Output {
                    <$ty>::wrapping_rem_euclid(self, other)
                }
            }

            impl WrappingDivRemEuclid<$ty> for $ty {
                type Output = $ty;

                #[inline]
                fn wrapping_div_rem_euclid(self, other: $ty) -> (Self::Output, Self::Output) {
                    (
                        <$ty>::wrapping_div_euclid(self, other),
                        <$ty>::wrapping_rem_euclid(self, other),
                    )
                }
            }
        )*
    };
}

macro_rules! impl_wrapping_div_rem_trait {
    ($($ty:ty),*) => {
        $(
            impl WrappingDivRem<$ty> for $ty {
                type Output = $ty;

                #[inline]
                fn wrapping_div_rem(self, other: $ty) -> (Self::Output, Self::Output) {
                    (
                        <$ty>::wrapping_div(self, other),
                        <$ty>::wrapping_rem(self, other),
                    )
                }
            }
        )*
    };
}

macro_rules! impl_wrapping_mul_add_trait {
    ($($ty:ty),*) => {
        $(
            impl WrappingMulAdd<$ty, $ty> for $ty {
                type Output = $ty;

                #[inline]
                fn wrapping_mul_add(self, a: $ty, b: $ty) -> Self::Output {
                    <$ty>::wrapping_mul(self, a).wrapping_add(b)
                }
            }
        )*
    };
}

macro_rules! impl_wrapping_arith_traits_for_signed_ints {
    ($ty:ty $(, $rest:ident)*) => {
        impl_wrapping_field_traits_for_ints!($ty);
        impl_wrapping_div_rem_trait!($ty);
        impl_wrapping_euclid_traits!($ty);
        impl_wrapping_mul_add_trait!($ty);
    };
}

macro_rules! impl_wrapping_arith_traits_for_unsigned_ints {
    ($ty:ty $(, $rest:ident)*) => {
        impl_wrapping_field_traits_for_ints!($ty);
        impl_wrapping_div_rem_trait!($ty);
        impl_wrapping_euclid_traits!($ty);
        impl_wrapping_mul_add_trait!($ty);
    };
}

macro_rules! impl_wrapping_field_traits_via_inner {
    ($($ty:ty),*) => {
        $(
            impl WrappingAdd<$ty> for $ty {
                type Output = $ty;

                #[inline]
                fn wrapping_add(self, other: $ty) -> Self::Output {
                    Self(self.0.wrapping_add(other.0))
                }
            }

            impl WrappingSub<$ty> for $ty {
                type Output = $ty;

                #[inline]
                fn wrapping_sub(self, other: $ty) -> Self::Output {
                    Self(self.0.wrapping_sub(other.0))
                }
            }

            impl WrappingMul<$ty> for $ty {
                type Output = $ty;

                #[inline]
                fn wrapping_mul(self, other: $ty) -> Self::Output {
                    Self(self.0.wrapping_mul(other.0))
                }
            }

            impl WrappingDiv<$ty> for $ty {
                type Output = $ty;

                #[inline]
                fn wrapping_div(self, other: $ty) -> Self::Output {
                    Self(self.0.wrapping_div(other.0))
                }
            }

            impl WrappingRem<$ty> for $ty {
                type Output = $ty;

                #[inline]
                fn wrapping_rem(self, other: $ty) -> Self::Output {
                    Self(self.0.wrapping_rem(other.0))
                }
            }

            impl WrappingNeg for $ty {
                type Output = $ty;

                #[inline]
                fn wrapping_neg(self) -> Self::Output {
                    Self(self.0.wrapping_neg())
                }
            }
        )*
    };
}

macro_rules! impl_wrapping_euclid_traits_via_inner {
    ($($ty:ty),*) => {
        $(
            impl WrappingDivEuclid<$ty> for $ty {
                type Output = $ty;

                #[inline]
                fn wrapping_div_euclid(self, other: $ty) -> Self::Output {
                    Self(self.0.wrapping_div_euclid(other.0))
                }
            }

            impl WrappingRemEuclid<$ty> for $ty {
                type Output = $ty;

                #[inline]
                fn wrapping_rem_euclid(self, other: $ty) -> Self::Output {
                    Self(self.0.wrapping_rem_euclid(other.0))
                }
            }

            impl WrappingDivRemEuclid<$ty> for $ty {
                type Output = $ty;

                #[inline]
                fn wrapping_div_rem_euclid(self, other: $ty) -> (Self::Output, Self::Output) {
                    (
                        Self(self.0.wrapping_div_euclid(other.0)),
                        Self(self.0.wrapping_rem_euclid(other.0)),
                    )
                }
            }
        )*
    };
}

macro_rules! impl_wrapping_div_rem_trait_via_inner {
    ($($ty:ty),*) => {
        $(
            impl WrappingDivRem<$ty> for $ty {
                type Output = $ty;

                #[inline]
                fn wrapping_div_rem(self, other: $ty) -> (Self::Output, Self::Output) {
                    (
                        Self(self.0.wrapping_div(other.0)),
                        Self(self.0.wrapping_rem(other.0)),
                    )
                }
            }
        )*
    };
}

macro_rules! impl_wrapping_mul_add_trait_via_inner {
    ($($ty:ty),*) => {
        $(
            impl WrappingMulAdd<$ty, $ty> for $ty {
                type Output = $ty;

                #[inline]
                fn wrapping_mul_add(self, a: $ty, b: $ty) -> Self::Output {
                    Self(self.0.wrapping_mul(a.0).wrapping_add(b.0))
                }
            }
        )*
    };
}

macro_rules! impl_wrapping_arith_traits_via_inner {
    ($ty:ty $(, $rest:ident)*) => {
        impl_wrapping_field_traits_via_inner!($ty);
        impl_wrapping_div_rem_trait_via_inner!($ty);
        impl_wrapping_euclid_traits_via_inner!($ty);
        impl_wrapping_mul_add_trait_via_inner!($ty);
    };
}

/*macro_rules! impl_wrapping_field_traits_for_floats {
    ($($ty:ty),*) => {
        $(
            impl WrappingAdd<$ty> for $ty {
                type Output = $ty;

                #[inline]
                fn wrapping_add(self, other: $ty) -> Self::Output {
                    self + other
                }
            }

            impl WrappingSub<$ty> for $ty {
                type Output = $ty;

                #[inline]
                fn wrapping_sub(self, other: $ty) -> Self::Output {
                    self - other
                }
            }

            impl WrappingMul<$ty> for $ty {
                type Output = $ty;

                #[inline]
                fn wrapping_mul(self, other: $ty) -> Self::Output {
                    self * other
                }
            }

            impl WrappingDiv<$ty> for $ty {
                type Output = $ty;

                #[inline]
                fn wrapping_div(self, other: $ty) -> Self::Output {
                    self / other
                }
            }

            impl WrappingRem<$ty> for $ty {
                type Output = $ty;

                #[inline]
                fn wrapping_rem(self, other: $ty) -> Self::Output {
                    self % other
                }
            }

            impl WrappingNeg for $ty {
                type Output = $ty;

                #[inline]
                fn wrapping_neg(self) -> Self::Output {
                    -self
                }
            }
        )*
    };
}

macro_rules! impl_wrapping_div_rem_trait_for_floats {
    ($($ty:ty),*) => {
        $(
            impl WrappingDivRem<$ty> for $ty {
                type Output = $ty;

                #[inline]
                fn wrapping_div_rem(self, other: $ty) -> (Self::Output, Self::Output) {
                    (self / other, self % other)
                }
            }
        )*
    };
}*/

/*#[cfg(feature = "std")]
macro_rules! impl_wrapping_euclid_traits_for_floats {
    ($($ty:ty),*) => {
        $(
            impl WrappingDivEuclid<$ty> for $ty {
                type Output = $ty;

                #[inline]
                fn wrapping_div_euclid(self, other: $ty) -> Self::Output {
                    <$ty>::div_euclid(self, other)
                }
            }

            impl WrappingRemEuclid<$ty> for $ty {
                type Output = $ty;

                #[inline]
                fn wrapping_rem_euclid(self, other: $ty) -> Self::Output {
                    <$ty>::rem_euclid(self, other)
                }
            }

            impl WrappingDivRemEuclid<$ty> for $ty {
                type Output = $ty;

                #[inline]
                fn wrapping_div_rem_euclid(self, other: $ty) -> (Self::Output, Self::Output) {
                    (
                        <$ty>::div_euclid(self, other),
                        <$ty>::rem_euclid(self, other),
                    )
                }
            }
        )*
    };
}*/

/*macro_rules! impl_wrapping_mul_add_trait_for_floats {
    ($($ty:ty),*) => {
        $(
            impl WrappingMulAdd<$ty, $ty> for $ty {
                type Output = $ty;

                #[inline]
                fn wrapping_mul_add(self, a: $ty, b: $ty) -> Self::Output {
                    MulAdd::mul_add(self, a, b)
                }
            }
        )*
    };
}*/

/*macro_rules! impl_wrapping_arith_traits_for_floats {
    ($($ty:ty),*) => {
        impl_wrapping_field_traits_for_floats!($($ty),*);
        impl_wrapping_div_rem_trait_for_floats!($($ty),*);
        #[cfg(feature = "std")]
        impl_wrapping_euclid_traits_for_floats!($($ty),*);
    };
}*/

for_signed_ints!(impl_wrapping_arith_traits_for_signed_ints);
for_unsigned_ints!(impl_wrapping_arith_traits_for_unsigned_ints);
for_wrapping_signed_ints!(impl_wrapping_arith_traits_via_inner);
for_wrapping_unsigned_ints!(impl_wrapping_arith_traits_via_inner);
for_saturating_signed_ints!(impl_wrapping_arith_traits_via_inner);
for_saturating_unsigned_ints!(impl_wrapping_arith_traits_via_inner);
//impl_wrapping_arith_traits_for_floats!(f32, f64);

//impl_wrapping_mul_add_trait_for_floats!(f32, f64);

macro_rules! test_wrapping_arith_traits_int_nonnegative {
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

                assert_eq!(WrappingDivRem::wrapping_div_rem(four, two), (two, zero));
                assert_eq!(WrappingDivRem::wrapping_div_rem(five, two), (two, one));
                assert_eq!(WrappingDivRem::wrapping_div_rem(three, two), (one, one));
                assert_eq!(Object::new(five).wrapping_div_rem(Object::new(two)), {
                    let (a, b) = WrappingDivRem::wrapping_div_rem(five, two);

                    (Object::new(a), Object::new(b))
                });
            }

            #[test]
            fn test_div_euclid() {
                let one = <$ty as AssocConstOne>::ONE;
                let two = one + one;
                let three = two + one;
                let four = two + two;
                let five = four + one;

                assert_eq!(WrappingDivEuclid::wrapping_div_euclid(four, two), two);
                assert_eq!(WrappingDivEuclid::wrapping_div_euclid(five, two), two);
                assert_eq!(WrappingDivEuclid::wrapping_div_euclid(three, two), one);
                assert_eq!(
                    Object::new(five).wrapping_div_euclid(Object::new(two)),
                    Object::new(WrappingDivEuclid::wrapping_div_euclid(five, two))
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

                assert_eq!(WrappingRemEuclid::wrapping_rem_euclid(four, two), zero);
                assert_eq!(WrappingRemEuclid::wrapping_rem_euclid(five, two), one);
                assert_eq!(WrappingRemEuclid::wrapping_rem_euclid(three, two), one);
                assert_eq!(
                    Object::new(five).wrapping_rem_euclid(Object::new(two)),
                    Object::new(WrappingRemEuclid::wrapping_rem_euclid(five, two))
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
                    WrappingDivRemEuclid::wrapping_div_rem_euclid(five, two),
                    (two, one)
                );
                assert_eq!(
                    WrappingDivRemEuclid::wrapping_div_rem_euclid(four, two),
                    (two, zero)
                );
                assert_eq!(
                    Object::new(five).wrapping_div_rem_euclid(Object::new(two)),
                    {
                        let (a, b) = WrappingDivRemEuclid::wrapping_div_rem_euclid(five, two);

                        (Object::new(a), Object::new(b))
                    }
                );
            }

            #[test]
            fn test_mul_add() {
                let zero = <$ty as AssocConstZero>::ZERO;
                let one = <$ty as AssocConstOne>::ONE;
                let two = one + one;
                let four = two + two;

                assert_eq!(WrappingMulAdd::wrapping_mul_add(two, two, zero), four);
                assert_eq!(WrappingMulAdd::wrapping_mul_add(one, one, one), two);
                assert_eq!(WrappingMulAdd::wrapping_mul_add(two, one, two), four);
                assert_eq!(
                    Object::new(two).wrapping_mul_add(Object::new(one), Object::new(two)),
                    Object::new(WrappingMulAdd::wrapping_mul_add(two, one, two))
                );
            }
        }
    };
}

macro_rules! test_wrapping_arith_traits_int_negative {
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

                assert_eq!(WrappingDivRem::wrapping_div_rem(-five, two), (-two, -one));
                assert_eq!(WrappingDivRem::wrapping_div_rem(-four, two), (-two, zero));
                assert_eq!(
                    WrappingDivRem::wrapping_div_rem(<$ty as MinFinite>::MIN_FINITE, -one),
                    (<$ty as MinFinite>::MIN_FINITE, zero)
                );
                assert_eq!(Object::new(five).wrapping_div_rem(Object::new(two)), {
                    let (a, b) = WrappingDivRem::wrapping_div_rem(five, two);

                    (Object::new(a), Object::new(b))
                });
            }

            #[test]
            fn test_div_euclid() {
                let one = <$ty as AssocConstOne>::ONE;
                let two = one + one;
                let three = two + one;
                let five = two + two + one;

                assert_eq!(WrappingDivEuclid::wrapping_div_euclid(-five, two), -three);
                assert_eq!(WrappingDivEuclid::wrapping_div_euclid(-five, -two), three);
                assert_eq!(
                    Object::new(five).wrapping_div_euclid(Object::new(two)),
                    Object::new(WrappingDivEuclid::wrapping_div_euclid(five, two))
                );
            }

            #[test]
            fn test_rem_euclid() {
                let zero = <$ty as AssocConstZero>::ZERO;
                let one = <$ty as AssocConstOne>::ONE;
                let two = one + one;
                let four = two + two;
                let five = four + one;

                assert_eq!(WrappingRemEuclid::wrapping_rem_euclid(-five, two), one);
                assert_eq!(WrappingRemEuclid::wrapping_rem_euclid(-four, two), zero);
                assert_eq!(
                    Object::new(five).wrapping_rem_euclid(Object::new(two)),
                    Object::new(WrappingRemEuclid::wrapping_rem_euclid(five, two))
                );
            }

            #[test]
            fn test_div_rem_euclid() {
                let one = <$ty as AssocConstOne>::ONE;
                let two = one + one;
                let three = two + one;
                let five = two + two + one;

                assert_eq!(
                    WrappingDivRemEuclid::wrapping_div_rem_euclid(-five, two),
                    (-three, one)
                );
                assert_eq!(
                    Object::new(five).wrapping_div_rem_euclid(Object::new(two)),
                    {
                        let (a, b) = WrappingDivRemEuclid::wrapping_div_rem_euclid(five, two);

                        (Object::new(a), Object::new(b))
                    }
                );
            }

            #[test]
            fn test_neg() {
                let zero = <$ty as AssocConstZero>::ZERO;
                let one = <$ty as AssocConstOne>::ONE;

                assert_eq!(WrappingNeg::wrapping_neg(one), -one);
                assert_eq!(WrappingNeg::wrapping_neg(-one), one);
                assert_eq!(WrappingNeg::wrapping_neg(zero), zero);
                assert_eq!(
                    WrappingNeg::wrapping_neg(<$ty as MinFinite>::MIN_FINITE),
                    <$ty as MinFinite>::MIN_FINITE
                );
                assert_eq!(
                    Object::new(one).wrapping_neg(),
                    Object::new(WrappingNeg::wrapping_neg(one))
                );
            }
        }
    };
}

for_signed_ints!(
    test_wrapping_arith_traits_int_nonnegative; test_wrapping_arith_traits_int_negative
);
for_unsigned_ints!(test_wrapping_arith_traits_int_nonnegative);
