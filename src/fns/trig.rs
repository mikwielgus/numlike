// SPDX-FileCopyrightText: 2026 numlike contributors
//
// SPDX-License-Identifier: MIT OR Apache-2.0

pub trait FullTrigFns<Rhs = Self>: TrigFns + InvTrigFns<Rhs> {}
impl<Rhs, T: TrigFns + InvTrigFns<Rhs>> FullTrigFns<Rhs> for T {}

pub trait TrigFns: ClassicalTrigFns + SinCos {}
impl<T: ClassicalTrigFns + SinCos> TrigFns for T {}

pub trait SinCos {
    type Output;

    fn sin_cos(self) -> (Self::Output, Self::Output);
}

pub trait ClassicalTrigFns: Sin + Cos + Tan {}
impl<T: Sin + Cos + Tan> ClassicalTrigFns for T {}

pub trait Sin {
    type Output;

    fn sin(self) -> Self::Output;
}

pub trait Cos {
    type Output;

    fn cos(self) -> Self::Output;
}

pub trait Tan {
    type Output;

    fn tan(self) -> Self::Output;
}

pub trait InvTrigFns<Rhs = Self>: ClassicalInvTrigFns<Rhs> + Atan2<Rhs> {}
impl<Rhs, T: ClassicalInvTrigFns<Rhs> + Atan2<Rhs>> InvTrigFns<Rhs> for T {}

pub trait Atan2<Rhs> {
    type Output;

    fn atan2(self, rhs: Rhs) -> Self::Output;
}

pub trait ClassicalInvTrigFns<Rhs = Self>: Asin + Acos + Atan {}
impl<Rhs, T: Asin + Acos + Atan> ClassicalInvTrigFns<Rhs> for T {}

pub trait Asin {
    type Output;

    fn asin(self) -> Self::Output;
}

pub trait Acos {
    type Output;

    fn acos(self) -> Self::Output;
}

pub trait Atan {
    type Output;

    fn atan(self) -> Self::Output;
}

macro_rules! impl_trig_traits_for_floats {
    ($($ty:ty),*) => {
        $(
            impl SinCos for $ty {
                type Output = $ty;

                #[inline]
                fn sin_cos(self) -> (Self::Output, Self::Output) {
                    <$ty>::sin_cos(self)
                }
            }

            impl Sin for $ty {
                type Output = $ty;

                #[inline]
                fn sin(self) -> Self::Output {
                    <$ty>::sin(self)
                }
            }

            impl Cos for $ty {
                type Output = $ty;

                #[inline]
                fn cos(self) -> Self::Output {
                    <$ty>::cos(self)
                }
            }

            impl Tan for $ty {
                type Output = $ty;

                #[inline]
                fn tan(self) -> Self::Output {
                    <$ty>::tan(self)
                }
            }

            impl Atan2<$ty> for $ty {
                type Output = $ty;

                #[inline]
                fn atan2(self, other: $ty) -> Self::Output {
                    <$ty>::atan2(self, other)
                }
            }

            impl Asin for $ty {
                type Output = $ty;

                #[inline]
                fn asin(self) -> Self::Output {
                    <$ty>::asin(self)
                }
            }

            impl Acos for $ty {
                type Output = $ty;

                #[inline]
                fn acos(self) -> Self::Output {
                    <$ty>::acos(self)
                }
            }

            impl Atan for $ty {
                type Output = $ty;

                #[inline]
                fn atan(self) -> Self::Output {
                    <$ty>::atan(self)
                }
            }
        )*
    }
}

impl_trig_traits_for_floats!(f32, f64);
