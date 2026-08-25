// SPDX-FileCopyrightText: 2026 numlike contributors
//
// SPDX-License-Identifier: MIT OR Apache-2.0

pub trait DivEuclid<Rhs: ?Sized = Self> {
    type Output;

    fn div_euclid(self, other: Rhs) -> Self::Output;
}

pub trait RemEuclid<Rhs: ?Sized = Self> {
    type Output;

    fn rem_euclid(self, other: Rhs) -> Self::Output;
}

pub trait DivRemEuclid<Rhs: ?Sized = Self> {
    type Output;

    fn div_rem_euclid(self, other: Rhs) -> (Self::Output, Self::Output);
}

pub trait CheckedDivEuclid<Rhs: ?Sized = Self> {
    type Output;

    fn checked_div_euclid(self, other: Rhs) -> Option<Self::Output>;
}

pub trait CheckedRemEuclid<Rhs: ?Sized = Self> {
    type Output;

    fn checked_rem_euclid(self, other: Rhs) -> Option<Self::Output>;
}

pub trait CheckedDivRemEuclid<Rhs: ?Sized = Self> {
    type Output;

    fn checked_div_rem_euclid(self, other: Rhs) -> Option<(Self::Output, Self::Output)>;
}

macro_rules! impl_euclid_traits_for_ints {
    ($($ty:ty),*) => {
        $(
            impl DivEuclid<$ty> for $ty {
                type Output = $ty;

                #[inline]
                fn div_euclid(self, other: $ty) -> Self::Output {
                    <$ty>::div_euclid(self, other)
                }
            }

            impl RemEuclid<$ty> for $ty {
                type Output = $ty;

                #[inline]
                fn rem_euclid(self, other: $ty) -> Self::Output {
                    <$ty>::rem_euclid(self, other)
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

macro_rules! impl_euclid_traits_for_floats {
    ($($ty:ty),*) => {
        $(
            impl DivEuclid<$ty> for $ty {
                type Output = $ty;

                #[inline]
                fn div_euclid(self, other: $ty) -> Self::Output {
                    <$ty>::div_euclid(self, other)
                }
            }

            impl RemEuclid<$ty> for $ty {
                type Output = $ty;

                #[inline]
                fn rem_euclid(self, other: $ty) -> Self::Output {
                    <$ty>::rem_euclid(self, other)
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

impl_euclid_traits_for_ints!(i8, i16, i32, i64, i128, isize);
impl_euclid_traits_for_ints!(u8, u16, u32, u64, u128, usize);
impl_euclid_traits_for_floats!(f32, f64);
