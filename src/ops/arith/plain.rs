// SPDX-FileCopyrightText: 2026 numlike contributors
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use core::ops::{
    Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Rem, RemAssign, Sub, SubAssign,
};

pub trait PlainArithOps<Rhs = Self>: ArithOps<Rhs> + ArithmeticAssignOps<Rhs> {}

pub trait ArithOps<Rhs = Self>: EuclidOps<Rhs> + FieldOps<Rhs> + Rem<Rhs, Output = Self> {}
impl<Rhs, T: EuclidOps<Rhs> + FieldOps<Rhs> + Rem<Rhs, Output = Self>> ArithOps<Rhs> for T {}

pub trait EuclidOps<Rhs>: DivEuclid<Rhs> + RemEuclid<Rhs> + DivRemEuclid<Rhs> {}
impl<Rhs, T: DivEuclid<Rhs> + RemEuclid<Rhs> + DivRemEuclid<Rhs>> EuclidOps<Rhs> for T {}

pub trait DivEuclid<Rhs = Self> {
    type Output;

    fn div_euclid(self, other: Rhs) -> Self::Output;
}

pub trait DivEuclidAssign<Rhs = Self> {
    fn div_euclid_assign(&mut self, other: Rhs);
}

pub trait RemEuclid<Rhs = Self> {
    type Output;

    fn rem_euclid(self, other: Rhs) -> Self::Output;
}

pub trait RemEuclidAssign<Rhs = Self> {
    fn rem_euclid_assign(&mut self, other: Rhs);
}

pub trait DivRemEuclid<Rhs = Self> {
    type Output;

    fn div_rem_euclid(self, other: Rhs) -> (Self::Output, Self::Output);
}

pub trait ArithmeticAssignOps<Rhs = Self>: FieldAssignOps<Rhs> + RemAssign<Rhs> {}
impl<Rhs, T: FieldAssignOps<Rhs> + RemAssign<Rhs>> ArithmeticAssignOps<Rhs> for T {}

pub trait FieldOps<Rhs = Self>: RingOps<Rhs> + Div<Rhs, Output = Self> {}
impl<Rhs, T: RingOps<Rhs> + Div<Rhs, Output = Self>> FieldOps<Rhs> for T {}

pub trait FieldAssignOps<Rhs = Self>: RingAssignOps<Rhs> + DivAssign<Rhs> {}
impl<Rhs, T: RingAssignOps<Rhs> + DivAssign<Rhs>> FieldAssignOps<Rhs> for T {}

pub trait DivRem<Rhs> {
    type Output;

    fn div_rem(self, other: Rhs) -> (Self::Output, Self::Output);
}

pub trait RingOps<Rhs = Self>:
    Add<Rhs, Output = Self> + Sub<Rhs, Output = Self> + Mul<Rhs, Output = Self> + Neg<Output = Self>
{
}
impl<
    Rhs,
    T: Add<Rhs, Output = Self>
        + Sub<Rhs, Output = Self>
        + Mul<Rhs, Output = Self>
        + Neg<Output = Self>,
> RingOps<Rhs> for T
{
}

pub trait RingAssignOps<Rhs = Self>:
    AddAssign<Rhs> + SubAssign<Rhs> + MulAssign<Rhs> + NegAssign
{
}
impl<Rhs, T: AddAssign<Rhs> + SubAssign<Rhs> + MulAssign<Rhs> + NegAssign> RingAssignOps<Rhs>
    for T
{
}

pub trait NegAssign {
    fn neg_assign(&mut self);
}

macro_rules! impl_euclid_traits {
    ($($ty:ty),*) => {
        $(
            impl DivEuclid<$ty> for $ty {
                type Output = $ty;

                #[inline]
                fn div_euclid(self, other: $ty) -> Self::Output {
                    <$ty>::div_euclid(self, other)
                }
            }

            impl DivEuclidAssign<$ty> for $ty {
                #[inline]
                fn div_euclid_assign(&mut self, other: $ty) {
                    *self = <$ty>::div_euclid(*self, other);
                }
            }

            impl RemEuclid<$ty> for $ty {
                type Output = $ty;

                #[inline]
                fn rem_euclid(self, other: $ty) -> Self::Output {
                    <$ty>::rem_euclid(self, other)
                }
            }

            impl RemEuclidAssign<$ty> for $ty {
                #[inline]
                fn rem_euclid_assign(&mut self, other: $ty) {
                    *self = <$ty>::rem_euclid(*self, other);
                }
            }

            impl DivRemEuclid<$ty> for $ty {
                type Output = $ty;

                #[inline]
                fn div_rem_euclid(self, other: $ty) -> (Self::Output, Self::Output) {
                    (
                        <$ty>::div_euclid(self, other),
                        <$ty>::rem_euclid(self, other),
                    )
                }
            }
        )*
    };
}

macro_rules! impl_div_rem_trait {
    ($($ty:ty),*) => {
        $(
            impl DivRem<$ty> for $ty {
                type Output = $ty;

                #[inline]
                fn div_rem(self, other: $ty) -> (Self::Output, Self::Output) {
                    (
                        <$ty>::div(self, other),
                        <$ty>::rem(self, other),
                    )
                }
            }
        )*
    };
}

macro_rules! impl_neg_assign_trait {
    ($($ty:ty),*) => {
        $(
            impl NegAssign for $ty {
                #[inline]
                fn neg_assign(&mut self) {
                    *self = Neg::neg(*self);
                }
            }
        )*
    };
}

macro_rules! impl_plain_arith_traits_for_signed_ints {
    ($($ty:ty),*) => {
        impl_div_rem_trait!($($ty),*);
        impl_euclid_traits!($($ty),*);
        impl_neg_assign_trait!($($ty),*);
    };
}

macro_rules! impl_plain_arith_traits_for_unsigned_ints {
    ($($ty:ty),*) => {
        impl_div_rem_trait!($($ty),*);
        impl_euclid_traits!($($ty),*);
    };
}

macro_rules! impl_plain_arith_traits_for_floats {
    ($($ty:ty),*) => {
        impl_div_rem_trait!($($ty),*);
        #[cfg(feature = "std")]
        impl_euclid_traits!($($ty),*);
        impl_neg_assign_trait!($($ty),*);
    };
}

impl_plain_arith_traits_for_signed_ints!(i8, i16, i32, i64, i128, isize);
impl_plain_arith_traits_for_unsigned_ints!(u8, u16, u32, u64, u128, usize);
impl_plain_arith_traits_for_floats!(f32, f64);
