// SPDX-FileCopyrightText: 2026 numlike contributors
//
// SPDX-License-Identifier: MIT OR Apache-2.0

pub trait ExpFns: Exp + Exp2 + ExpM1 {}
impl<T: Exp + Exp2 + ExpM1> ExpFns for T {}

pub trait Exp {
    type Output;

    fn exp(self) -> Self::Output;
}

pub trait Exp2 {
    type Output;

    fn exp2(self) -> Self::Output;
}

pub trait ExpM1 {
    type Output;

    fn exp_m1(self) -> Self::Output;
}

pub trait CheckedExpFns: CheckedExp + CheckedExp2 + CheckedExpM1 {}
impl<T: CheckedExp + CheckedExp2 + CheckedExpM1> CheckedExpFns for T {}

pub trait CheckedExp {
    type Output;

    fn checked_exp(self) -> Option<Self::Output>;
}

pub trait CheckedExp2 {
    type Output;

    fn checked_exp2(self) -> Option<Self::Output>;
}

pub trait CheckedExpM1 {
    type Output;

    fn checked_exp_m1(self) -> Option<Self::Output>;
}

macro_rules! impl_exp_traits_for_floats {
    ($($ty:ty),*) => {
        $(
            impl Exp for $ty {
                type Output = $ty;

                #[inline]
                fn exp(self) -> Self::Output {
                    <$ty>::exp(self)
                }
            }

            impl Exp2 for $ty {
                type Output = $ty;

                #[inline]
                fn exp2(self) -> Self::Output {
                    <$ty>::exp2(self)
                }
            }

            impl ExpM1 for $ty {
                type Output = $ty;

                #[inline]
                fn exp_m1(self) -> Self::Output {
                    <$ty>::exp_m1(self)
                }
            }

            impl CheckedExp for $ty {
                type Output = $ty;

                #[inline]
                fn checked_exp(self) -> Option<Self::Output> {
                    let result = <$ty>::exp(self);

                    result.is_finite().then_some(result)
                }
            }

            impl CheckedExp2 for $ty {
                type Output = $ty;

                #[inline]
                fn checked_exp2(self) -> Option<Self::Output> {
                    let result = <$ty>::exp2(self);

                    result.is_finite().then_some(result)
                }
            }

            impl CheckedExpM1 for $ty {
                type Output = $ty;

                #[inline]
                fn checked_exp_m1(self) -> Option<Self::Output> {
                    let result = <$ty>::exp_m1(self);

                    result.is_finite().then_some(result)
                }
            }
        )*
    }
}

impl_exp_traits_for_floats!(f32, f64);
