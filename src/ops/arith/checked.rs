// SPDX-FileCopyrightText: 2026 numlike contributors
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use super::plain::MulAdd;

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

/// Checked Euclidean remainder. Computes `self.rem_euclid(other)`, returning
/// `None` if `other == 0` or the division results in overflow.
pub trait CheckedRemEuclid<Rhs = Self> {
    /// The resulting type after applying the operation.
    type Output;

    /// Checked Euclidean remainder. Computes `self.rem_euclid(other)`,
    /// returning `None` if `other == 0` or the division results in overflow.
    fn checked_rem_euclid(self, other: Rhs) -> Option<Self::Output>;
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

/// Checked subtraction. Computes `self - other`, returning `None` if
/// overflow occurred.
pub trait CheckedSub<Rhs = Self> {
    /// The resulting type after applying the operation.
    type Output;

    /// Checked subtraction. Computes `self - other`, returning `None` if
    /// overflow occurred.
    fn checked_sub(self, other: Rhs) -> Option<Self::Output>;
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

/// Checked negation. Computes `-self`, returning `None` if `self == MIN`.
pub trait CheckedNeg {
    /// The resulting type after applying the operation.
    type Output;

    /// Checked negation. Computes `-self`, returning `None` if `self == MIN`.
    fn checked_neg(self) -> Option<Self::Output>;
}

macro_rules! impl_checked_ring_field_traits_for_ints {
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
    ($($ty:ty),*) => {
        impl_checked_ring_field_traits_for_ints!($($ty),*);
        impl_checked_div_rem_trait!($($ty),*);
        impl_checked_euclid_traits!($($ty),*);
        impl_checked_mul_add_trait!($($ty),*);
    };
}

macro_rules! impl_checked_arith_traits_for_unsigned_ints {
    ($($ty:ty),*) => {
        impl_checked_ring_field_traits_for_ints!($($ty),*);
        impl_checked_div_rem_trait!($($ty),*);
        impl_checked_euclid_traits!($($ty),*);
        impl_checked_mul_add_trait!($($ty),*);
    };
}

macro_rules! impl_checked_ring_field_traits_for_floats {
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

#[cfg(feature = "std")]
macro_rules! impl_checked_euclid_traits_for_floats {
    ($($ty:ty),*) => {
        $(
            impl CheckedDivEuclid<$ty> for $ty {
                type Output = $ty;

                #[inline]
                fn checked_div_euclid(self, other: $ty) -> Option<Self::Output> {
                    let result = <$ty>::div_euclid(self, other);

                    result.is_finite().then_some(result)
                }
            }

            impl CheckedRemEuclid<$ty> for $ty {
                type Output = $ty;

                #[inline]
                fn checked_rem_euclid(self, other: $ty) -> Option<Self::Output> {
                    let result = <$ty>::rem_euclid(self, other);

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
                    let div = <$ty>::div_euclid(self, other);
                    let rem = <$ty>::rem_euclid(self, other);

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
        impl_checked_ring_field_traits_for_floats!($($ty),*);
        impl_checked_div_rem_trait_for_floats!($($ty),*);
        #[cfg(feature = "std")]
        impl_checked_euclid_traits_for_floats!($($ty),*);
    };
}

impl_checked_arith_traits_for_signed_ints!(i8, i16, i32, i64, i128, isize);
impl_checked_arith_traits_for_unsigned_ints!(u8, u16, u32, u64, u128, usize);
impl_checked_arith_traits_for_floats!(f32, f64);

impl_checked_mul_add_trait_for_floats!(f32, f64);
