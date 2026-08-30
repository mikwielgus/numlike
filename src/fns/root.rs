// SPDX-FileCopyrightText: 2026 numlike contributors
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use super::round::Floor;

pub trait RootFns: Sqrt + Cbrt {}
impl<T: Sqrt + Cbrt> RootFns for T {}

pub trait Sqrt {
    type Output;

    fn sqrt(self) -> Self::Output;
}

pub trait Cbrt {
    type Output;

    fn cbrt(self) -> Self::Output;
}

pub trait Isqrt {
    type Output;

    fn isqrt(self) -> Self::Output;
}

pub trait CheckedIsqrt {
    type Output;

    fn checked_isqrt(self) -> Option<Self::Output>;
}

macro_rules! impl_root_traits_for_floats {
    ($($ty:ty),*) => {
        $(
            impl Sqrt for $ty {
                type Output = $ty;

                #[inline]
                fn sqrt(self) -> Self::Output {
                    <$ty>::sqrt(self)
                }
            }

            impl Cbrt for $ty {
                type Output = $ty;

                #[inline]
                fn cbrt(self) -> Self::Output {
                    <$ty>::cbrt(self)
                }
            }

            impl Isqrt for $ty {
                type Output = $ty;

                #[inline]
                fn isqrt(self) -> Self::Output {
                    Floor::floor(<$ty>::sqrt(self))
                }
            }

            impl CheckedIsqrt for $ty {
                type Output = $ty;

                #[inline]
                fn checked_isqrt(self) -> Option<Self::Output> {
                    let result = Floor::floor(<$ty>::sqrt(self));

                    result.is_finite().then_some(result)
                }
            }
        )*
    };
}

macro_rules! impl_isqrt_trait_for_ints {
    ($($ty:ty),*) => {
        $(
            impl Isqrt for $ty {
                type Output = $ty;

                #[inline]
                fn isqrt(self) -> Self::Output {
                    <$ty>::isqrt(self)
                }
            }
        )*
    };
}

macro_rules! impl_checked_isqrt_trait_for_signed_ints {
    ($($ty:ty),*) => {
        $(
            impl CheckedIsqrt for $ty {
                type Output = $ty;

                #[inline]
                fn checked_isqrt(self) -> Option<Self::Output> {
                    <$ty>::checked_isqrt(self)
                }
            }
        )*
    };
}

macro_rules! impl_checked_isqrt_trait_for_unsigned_ints {
    ($($ty:ty),*) => {
        $(
            impl CheckedIsqrt for $ty {
                type Output = $ty;

                #[inline]
                fn checked_isqrt(self) -> Option<Self::Output> {
                    Some(<$ty>::isqrt(self))
                }
            }
        )*
    };
}

impl_root_traits_for_floats!(f32, f64);
impl_isqrt_trait_for_ints!(i8, i16, i32, i64, i128, isize);
impl_isqrt_trait_for_ints!(u8, u16, u32, u64, u128, usize);
impl_checked_isqrt_trait_for_signed_ints!(i8, i16, i32, i64, i128, isize);
impl_checked_isqrt_trait_for_unsigned_ints!(u8, u16, u32, u64, u128, usize);
