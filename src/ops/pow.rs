// SPDX-FileCopyrightText: 2026 numlike contributors
//
// SPDX-License-Identifier: MIT OR Apache-2.0

pub trait Pow<Rhs> {
    type Output;

    fn pow(self, rhs: Rhs) -> Self::Output;
}

pub trait CheckedPow<Rhs> {
    type Output;

    fn checked_pow(self, rhs: Rhs) -> Option<Self::Output>;
}

macro_rules! impl_pow_traits_for_ints {
    ($($ty:ty),*) => {
        $(
            impl Pow<u32> for $ty {
                type Output = $ty;

                #[inline]
                fn pow(self, rhs: u32) -> Self::Output {
                    <$ty>::pow(self, rhs)
                }
            }

            impl CheckedPow<u32> for $ty {
                type Output = $ty;

                #[inline]
                fn checked_pow(self, rhs: u32) -> Option<Self::Output> {
                    <$ty>::checked_pow(self, rhs)
                }
            }
        )*
    };
}

#[cfg(feature = "std")]
macro_rules! impl_pow_traits_for_floats {
    ($($ty:ty),*) => {
        $(
            impl Pow<i32> for $ty {
                type Output = $ty;

                #[inline]
                fn pow(self, rhs: i32) -> Self::Output {
                    <$ty>::powi(self, rhs)
                }
            }

            impl CheckedPow<i32> for $ty {
                type Output = $ty;

                #[inline]
                fn checked_pow(self, rhs: i32) -> Option<Self::Output> {
                    let result = <$ty>::powi(self, rhs);

                    result.is_finite().then_some(result)
                }
            }

            impl Pow<$ty> for $ty {
                type Output = $ty;

                #[inline]
                fn pow(self, rhs: $ty) -> Self::Output {
                    <$ty>::powf(self, rhs)
                }
            }

            impl CheckedPow<$ty> for $ty {
                type Output = $ty;

                #[inline]
                fn checked_pow(self, rhs: $ty) -> Option<Self::Output> {
                    let result = <$ty>::powf(self, rhs);

                    result.is_finite().then_some(result)
                }
            }
        )*
    };
}

impl_pow_traits_for_ints!(i8, i16, i32, i64, i128, isize);
impl_pow_traits_for_ints!(u8, u16, u32, u64, u128, usize);

#[cfg(feature = "std")]
impl_pow_traits_for_floats!(f32, f64);
