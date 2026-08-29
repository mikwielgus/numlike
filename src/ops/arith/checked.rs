// SPDX-FileCopyrightText: 2026 numlike contributors
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use super::plain::MulAdd;

pub trait CheckedPlainArithOps<Rhs = Self>:
    CheckedArithOps<Rhs> + CheckedFusedArithOps<Rhs>
{
}

pub trait CheckedFusedArithOps<Rhs = Self>: CheckedMulAdd<Rhs, Rhs> {}
impl<Rhs, T: CheckedMulAdd<Rhs, Rhs>> CheckedFusedArithOps<Rhs> for T {}

pub trait CheckedMulAdd<A = Self, B = Self> {
    type Output;

    fn checked_mul_add(self, a: A, b: B) -> Option<Self::Output>;
}

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

pub trait CheckedRem<Rhs = Self> {
    type Output;

    fn checked_rem(self, other: Rhs) -> Option<Self::Output>;
}

pub trait CheckedDivRem<Rhs> {
    type Output;

    fn checked_div_rem(self, other: Rhs) -> Option<(Self::Output, Self::Output)>;
}

pub trait CheckedEuclidOps<Rhs>:
    CheckedDivEuclid<Rhs> + CheckedRemEuclid<Rhs> + CheckedDivRemEuclid<Rhs>
{
}
impl<Rhs, T: CheckedDivEuclid<Rhs> + CheckedRemEuclid<Rhs> + CheckedDivRemEuclid<Rhs>>
    CheckedEuclidOps<Rhs> for T
{
}

pub trait CheckedDivEuclid<Rhs = Self> {
    type Output;

    fn checked_div_euclid(self, other: Rhs) -> Option<Self::Output>;
}

pub trait CheckedRemEuclid<Rhs = Self> {
    type Output;

    fn checked_rem_euclid(self, other: Rhs) -> Option<Self::Output>;
}

pub trait CheckedDivRemEuclid<Rhs = Self> {
    type Output;

    fn checked_div_rem_euclid(self, other: Rhs) -> Option<(Self::Output, Self::Output)>;
}

pub trait CheckedFieldOps<Rhs = Self>:
    CheckedRingOps<Rhs> + CheckedDiv<Rhs, Output = Self>
{
}
impl<Rhs, T: CheckedRingOps<Rhs> + CheckedDiv<Rhs, Output = Self>> CheckedFieldOps<Rhs> for T {}

pub trait CheckedDiv<Rhs = Self> {
    type Output;

    fn checked_div(self, other: Rhs) -> Option<Self::Output>;
}

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

pub trait CheckedAdd<Rhs = Self> {
    type Output;

    fn checked_add(self, other: Rhs) -> Option<Self::Output>;
}

pub trait CheckedSub<Rhs = Self> {
    type Output;

    fn checked_sub(self, other: Rhs) -> Option<Self::Output>;
}

pub trait CheckedMul<Rhs = Self> {
    type Output;

    fn checked_mul(self, other: Rhs) -> Option<Self::Output>;
}

pub trait CheckedNeg {
    type Output;

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
