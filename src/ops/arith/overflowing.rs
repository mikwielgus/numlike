// SPDX-FileCopyrightText: 2026 numlike contributors
//
// SPDX-License-Identifier: MIT OR Apache-2.0

pub trait OverflowingPlainArithOps<Rhs = Self>:
    OverflowingArithOps<Rhs> + OverflowingFusedArithOps<Rhs>
{
}

pub trait OverflowingFusedArithOps<Rhs = Self>: OverflowingMulAdd<Rhs, Rhs> {}
impl<Rhs, T: OverflowingMulAdd<Rhs, Rhs>> OverflowingFusedArithOps<Rhs> for T {}

pub trait OverflowingMulAdd<A = Self, B = Self> {
    type Output;

    fn overflowing_mul_add(self, a: A, b: B) -> (Self::Output, bool);
}

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

pub trait OverflowingRem<Rhs = Self> {
    type Output;

    fn overflowing_rem(self, other: Rhs) -> (Self::Output, bool);
}

pub trait OverflowingDivRem<Rhs> {
    type Output;

    fn overflowing_div_rem(self, other: Rhs) -> ((Self::Output, Self::Output), bool);
}

pub trait OverflowingEuclidOps<Rhs>:
    OverflowingDivEuclid<Rhs> + OverflowingRemEuclid<Rhs> + OverflowingDivRemEuclid<Rhs>
{
}
impl<Rhs, T: OverflowingDivEuclid<Rhs> + OverflowingRemEuclid<Rhs> + OverflowingDivRemEuclid<Rhs>>
    OverflowingEuclidOps<Rhs> for T
{
}

pub trait OverflowingDivEuclid<Rhs = Self> {
    type Output;

    fn overflowing_div_euclid(self, other: Rhs) -> (Self::Output, bool);
}

pub trait OverflowingRemEuclid<Rhs = Self> {
    type Output;

    fn overflowing_rem_euclid(self, other: Rhs) -> (Self::Output, bool);
}

pub trait OverflowingDivRemEuclid<Rhs = Self> {
    type Output;

    fn overflowing_div_rem_euclid(self, other: Rhs) -> ((Self::Output, Self::Output), bool);
}

pub trait OverflowingFieldOps<Rhs = Self>:
    OverflowingRingOps<Rhs> + OverflowingDiv<Rhs, Output = Self>
{
}
impl<Rhs, T: OverflowingRingOps<Rhs> + OverflowingDiv<Rhs, Output = Self>> OverflowingFieldOps<Rhs>
    for T
{
}

pub trait OverflowingDiv<Rhs = Self> {
    type Output;

    fn overflowing_div(self, other: Rhs) -> (Self::Output, bool);
}

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

pub trait OverflowingAdd<Rhs = Self> {
    type Output;

    fn overflowing_add(self, other: Rhs) -> (Self::Output, bool);
}

pub trait OverflowingSub<Rhs = Self> {
    type Output;

    fn overflowing_sub(self, other: Rhs) -> (Self::Output, bool);
}

pub trait OverflowingMul<Rhs = Self> {
    type Output;

    fn overflowing_mul(self, other: Rhs) -> (Self::Output, bool);
}

pub trait OverflowingNeg {
    type Output;

    fn overflowing_neg(self) -> (Self::Output, bool);
}

macro_rules! impl_overflowing_ring_field_traits_for_ints {
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
    ($($ty:ty),*) => {
        impl_overflowing_ring_field_traits_for_ints!($($ty),*);
        impl_overflowing_div_rem_trait!($($ty),*);
        impl_overflowing_euclid_traits!($($ty),*);
        impl_overflowing_mul_add_trait!($($ty),*);
    };
}

macro_rules! impl_overflowing_arith_traits_for_unsigned_ints {
    ($($ty:ty),*) => {
        impl_overflowing_ring_field_traits_for_ints!($($ty),*);
        impl_overflowing_div_rem_trait!($($ty),*);
        impl_overflowing_euclid_traits!($($ty),*);
        impl_overflowing_mul_add_trait!($($ty),*);
    };
}

/*macro_rules! impl_overflowing_ring_field_traits_for_floats {
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
        impl_overflowing_ring_field_traits_for_floats!($($ty),*);
        impl_overflowing_div_rem_trait_for_floats!($($ty),*);
        #[cfg(feature = "std")]
        impl_overflowing_euclid_traits_for_floats!($($ty),*);
    };
}*/

impl_overflowing_arith_traits_for_signed_ints!(i8, i16, i32, i64, i128, isize);
impl_overflowing_arith_traits_for_unsigned_ints!(u8, u16, u32, u64, u128, usize);
//impl_overflowing_arith_traits_for_floats!(f32, f64);

//impl_overflowing_mul_add_trait_for_floats!(f32, f64);
