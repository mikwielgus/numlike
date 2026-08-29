// SPDX-FileCopyrightText: 2026 numlike contributors
//
// SPDX-License-Identifier: MIT OR Apache-2.0

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

macro_rules! impl_wrapping_traits_for_ints {
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

impl_wrapping_traits_for_ints!(i8, i16, i32, i64, i128, isize);
impl_wrapping_traits_for_ints!(u8, u16, u32, u64, u128, usize);
