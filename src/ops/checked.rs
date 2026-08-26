// SPDX-FileCopyrightText: 2026 numlike contributors
//
// SPDX-License-Identifier: MIT OR Apache-2.0

pub trait CheckedAdd<Rhs: ?Sized = Self> {
    type Output;

    fn checked_add(self, other: Rhs) -> Option<Self::Output>;
}

pub trait CheckedSub<Rhs: ?Sized = Self> {
    type Output;

    fn checked_sub(self, other: Rhs) -> Option<Self::Output>;
}

pub trait CheckedMul<Rhs: ?Sized = Self> {
    type Output;

    fn checked_mul(self, other: Rhs) -> Option<Self::Output>;
}

pub trait CheckedDiv<Rhs: ?Sized = Self> {
    type Output;

    fn checked_div(self, other: Rhs) -> Option<Self::Output>;
}

pub trait CheckedNeg {
    type Output;

    fn checked_neg(self) -> Option<Self::Output>;
}

pub trait CheckedRingOps<Rhs: ?Sized = Self>:
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

pub trait CheckedFieldOps<Rhs: ?Sized = Self>:
    CheckedAdd<Rhs, Output = Self>
    + CheckedSub<Rhs, Output = Self>
    + CheckedMul<Rhs, Output = Self>
    + CheckedDiv<Rhs, Output = Self>
    + CheckedNeg<Output = Self>
{
}
impl<
    Rhs,
    T: CheckedAdd<Rhs, Output = Self>
        + CheckedSub<Rhs, Output = Self>
        + CheckedMul<Rhs, Output = Self>
        + CheckedDiv<Rhs, Output = Self>
        + CheckedNeg<Output = Self>,
> CheckedFieldOps<Rhs> for T
{
}

pub trait CheckedRem<Rhs: ?Sized = Self> {
    type Output;

    fn checked_rem(self, other: Rhs) -> Option<Self::Output>;
}

pub trait CheckedShl {
    type Output;

    fn checked_shl(self, rhs: u32) -> Option<Self::Output>;
}

pub trait CheckedShr {
    type Output;

    fn checked_shr(self, rhs: u32) -> Option<Self::Output>;
}

macro_rules! impl_checked_traits_for_ints {
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

            impl CheckedShl for $ty {
                type Output = $ty;

                #[inline]
                fn checked_shl(self, rhs: u32) -> Option<Self::Output> {
                    <$ty>::checked_shl(self, rhs)
                }
            }

            impl CheckedShr for $ty {
                type Output = $ty;

                #[inline]
                fn checked_shr(self, rhs: u32) -> Option<Self::Output> {
                    <$ty>::checked_shr(self, rhs)
                }
            }
        )*
    };
}

impl_checked_traits_for_ints!(i8, i16, i32, i64, i128, isize);
impl_checked_traits_for_ints!(u8, u16, u32, u64, u128, usize);

macro_rules! impl_checked_traits_for_floats {
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

impl_checked_traits_for_floats!(f32, f64);
