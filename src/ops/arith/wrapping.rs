// SPDX-FileCopyrightText: 2026 numlike contributors
//
// SPDX-License-Identifier: MIT OR Apache-2.0

pub trait WrappingPlainArithOps<Rhs = Self>:
    WrappingArithOps<Rhs> + WrappingFusedArithOps<Rhs>
{
}

pub trait WrappingFusedArithOps<Rhs = Self>: WrappingMulAdd<Rhs, Rhs> {}
impl<Rhs, T: WrappingMulAdd<Rhs, Rhs>> WrappingFusedArithOps<Rhs> for T {}

pub trait WrappingMulAdd<A = Self, B = Self> {
    type Output;

    fn wrapping_mul_add(self, a: A, b: B) -> Self::Output;
}

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

pub trait WrappingRem<Rhs = Self> {
    type Output;

    fn wrapping_rem(self, other: Rhs) -> Self::Output;
}

pub trait WrappingDivRem<Rhs> {
    type Output;

    fn wrapping_div_rem(self, other: Rhs) -> (Self::Output, Self::Output);
}

pub trait WrappingEuclidOps<Rhs>:
    WrappingDivEuclid<Rhs> + WrappingRemEuclid<Rhs> + WrappingDivRemEuclid<Rhs>
{
}
impl<Rhs, T: WrappingDivEuclid<Rhs> + WrappingRemEuclid<Rhs> + WrappingDivRemEuclid<Rhs>>
    WrappingEuclidOps<Rhs> for T
{
}

pub trait WrappingDivEuclid<Rhs = Self> {
    type Output;

    fn wrapping_div_euclid(self, other: Rhs) -> Self::Output;
}

pub trait WrappingRemEuclid<Rhs = Self> {
    type Output;

    fn wrapping_rem_euclid(self, other: Rhs) -> Self::Output;
}

pub trait WrappingDivRemEuclid<Rhs = Self> {
    type Output;

    fn wrapping_div_rem_euclid(self, other: Rhs) -> (Self::Output, Self::Output);
}

pub trait WrappingFieldOps<Rhs = Self>:
    WrappingRingOps<Rhs> + WrappingDiv<Rhs, Output = Self>
{
}
impl<Rhs, T: WrappingRingOps<Rhs> + WrappingDiv<Rhs, Output = Self>> WrappingFieldOps<Rhs> for T {}

pub trait WrappingDiv<Rhs = Self> {
    type Output;

    fn wrapping_div(self, other: Rhs) -> Self::Output;
}

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

pub trait WrappingAdd<Rhs = Self> {
    type Output;

    fn wrapping_add(self, other: Rhs) -> Self::Output;
}

pub trait WrappingSub<Rhs = Self> {
    type Output;

    fn wrapping_sub(self, other: Rhs) -> Self::Output;
}

pub trait WrappingMul<Rhs = Self> {
    type Output;

    fn wrapping_mul(self, other: Rhs) -> Self::Output;
}

pub trait WrappingNeg {
    type Output;

    fn wrapping_neg(self) -> Self::Output;
}

macro_rules! impl_wrapping_ring_field_traits_for_ints {
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
    ($($ty:ty),*) => {
        impl_wrapping_ring_field_traits_for_ints!($($ty),*);
        impl_wrapping_div_rem_trait!($($ty),*);
        impl_wrapping_euclid_traits!($($ty),*);
        impl_wrapping_mul_add_trait!($($ty),*);
    };
}

macro_rules! impl_wrapping_arith_traits_for_unsigned_ints {
    ($($ty:ty),*) => {
        impl_wrapping_ring_field_traits_for_ints!($($ty),*);
        impl_wrapping_div_rem_trait!($($ty),*);
        impl_wrapping_euclid_traits!($($ty),*);
        impl_wrapping_mul_add_trait!($($ty),*);
    };
}

/*macro_rules! impl_wrapping_ring_field_traits_for_floats {
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
        impl_wrapping_ring_field_traits_for_floats!($($ty),*);
        impl_wrapping_div_rem_trait_for_floats!($($ty),*);
        #[cfg(feature = "std")]
        impl_wrapping_euclid_traits_for_floats!($($ty),*);
    };
}*/

impl_wrapping_arith_traits_for_signed_ints!(i8, i16, i32, i64, i128, isize);
impl_wrapping_arith_traits_for_unsigned_ints!(u8, u16, u32, u64, u128, usize);
//impl_wrapping_arith_traits_for_floats!(f32, f64);

//impl_wrapping_mul_add_trait_for_floats!(f32, f64);
