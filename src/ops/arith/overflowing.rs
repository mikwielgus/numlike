// SPDX-FileCopyrightText: 2026 numlike contributors
//
// SPDX-License-Identifier: MIT OR Apache-2.0

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

pub trait OverflowingDiv<Rhs = Self> {
    type Output;

    fn overflowing_div(self, other: Rhs) -> (Self::Output, bool);
}

// TODO: overflowing rem?

pub trait OverflowingNeg {
    type Output;

    fn overflowing_neg(self) -> (Self::Output, bool);
}

pub trait OverflowingArithmeticOps<Rhs = Self>:
    OverflowingAdd<Rhs, Output = Self>
    + OverflowingSub<Rhs, Output = Self>
    + OverflowingMul<Rhs, Output = Self>
    + OverflowingDiv<Rhs, Output = Self>
    //+ OverflowingRem?
    + OverflowingNeg<Output = Self> {}
impl<
    Rhs,
    T: OverflowingAdd<Rhs, Output = Self>
        + OverflowingSub<Rhs, Output = Self>
        + OverflowingMul<Rhs, Output = Self>
        + OverflowingDiv<Rhs, Output = Self>
        //+ OverflowingRem?
        + OverflowingNeg<Output = Self>,
> OverflowingArithmeticOps<Rhs> for T
{
}

pub trait OverflowingFieldOps<Rhs = Self>:
    OverflowingAdd<Rhs, Output = Self>
    + OverflowingSub<Rhs, Output = Self>
    + OverflowingMul<Rhs, Output = Self>
    + OverflowingDiv<Rhs, Output = Self>
    + OverflowingNeg<Output = Self>
{
}
impl<
    Rhs,
    T: OverflowingAdd<Rhs, Output = Self>
        + OverflowingSub<Rhs, Output = Self>
        + OverflowingMul<Rhs, Output = Self>
        + OverflowingDiv<Rhs, Output = Self>
        + OverflowingNeg<Output = Self>,
> OverflowingFieldOps<Rhs> for T
{
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

macro_rules! impl_overflowing_traits_for_ints {
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

impl_overflowing_traits_for_ints!(i8, i16, i32, i64, i128, isize);
impl_overflowing_traits_for_ints!(u8, u16, u32, u64, u128, usize);
