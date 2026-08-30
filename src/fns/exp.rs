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
        )*
    }
}

impl_exp_traits_for_floats!(f32, f64);
