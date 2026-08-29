// SPDX-FileCopyrightText: 2026 numlike contributors
//
// SPDX-License-Identifier: MIT OR Apache-2.0

pub trait CheckedArithmeticOps<Rhs = Self>:
    CheckedFieldOps<Rhs> + CheckedRem<Rhs, Output = Self>
{
}
impl<Rhs, T: CheckedFieldOps<Rhs> + CheckedRem<Rhs, Output = Self>> CheckedArithmeticOps<Rhs>
    for T
{
}

pub trait CheckedFieldOps<Rhs = Self>:
    CheckedRingOps<Rhs> + CheckedDiv<Rhs, Output = Self>
{
}
impl<Rhs, T: CheckedRingOps<Rhs> + CheckedDiv<Rhs, Output = Self>> CheckedFieldOps<Rhs> for T {}

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

pub trait CheckedDiv<Rhs = Self> {
    type Output;

    fn checked_div(self, other: Rhs) -> Option<Self::Output>;
}

pub trait CheckedRem<Rhs = Self> {
    type Output;

    fn checked_rem(self, other: Rhs) -> Option<Self::Output>;
}

pub trait CheckedNeg {
    type Output;

    fn checked_neg(self) -> Option<Self::Output>;
}

pub trait CheckedEuclid: Sized + CheckedDivEuclid + CheckedRemEuclid + CheckedDivRemEuclid {}
impl<T: CheckedDivEuclid + CheckedRemEuclid + CheckedDivRemEuclid> CheckedEuclid for T {}

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

            #[cfg(feature = "std")]
            impl CheckedDivEuclid<$ty> for $ty {
                type Output = $ty;

                #[inline]
                fn checked_div_euclid(self, other: $ty) -> Option<Self::Output> {
                    let result = <$ty>::div_euclid(self, other);

                    result.is_finite().then_some(result)
                }
            }

            #[cfg(feature = "std")]
            impl CheckedRemEuclid<$ty> for $ty {
                type Output = $ty;

                #[inline]
                fn checked_rem_euclid(self, other: $ty) -> Option<Self::Output> {
                    let result = <$ty>::rem_euclid(self, other);

                    result.is_finite().then_some(result)
                }
            }

            #[cfg(feature = "std")]
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

impl_checked_traits_for_floats!(f32, f64);
