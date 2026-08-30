// SPDX-FileCopyrightText: 2026 numlike contributors
//
// SPDX-License-Identifier: MIT OR Apache-2.0

/*pub trait FullHypFns<Rhs = Self>: HypFns + InvHypFns<Rhs> {}
impl<Rhs, T: HypFns + InvHypFns<Rhs>> FullHypFns<Rhs> for T {}*/

pub trait HypFns: Sinh + Cosh + Tanh {}
impl<T: Sinh + Cosh + Tanh> HypFns for T {}

pub trait Sinh {
    type Output;

    fn sinh(self) -> Self::Output;
}

pub trait Cosh {
    type Output;

    fn cosh(self) -> Self::Output;
}

pub trait Tanh {
    type Output;

    fn tanh(self) -> Self::Output;
}

pub trait InvHypFns<Rhs = Self>: Asinh + Acosh + Atanh {}
impl<Rhs, T: Asinh + Acosh + Atanh> InvHypFns<Rhs> for T {}

pub trait Asinh {
    type Output;

    fn asinh(self) -> Self::Output;
}

pub trait Acosh {
    type Output;

    fn acosh(self) -> Self::Output;
}

pub trait Atanh {
    type Output;

    fn atanh(self) -> Self::Output;
}

macro_rules! impl_trig_traits_for_floats {
    ($($ty:ty),*) => {
        $(
            impl Sinh for $ty {
                type Output = $ty;

                #[inline]
                fn sinh(self) -> Self::Output {
                    <$ty>::sinh(self)
                }
            }

            impl Cosh for $ty {
                type Output = $ty;

                #[inline]
                fn cosh(self) -> Self::Output {
                    <$ty>::cosh(self)
                }
            }

            impl Tanh for $ty {
                type Output = $ty;

                #[inline]
                fn tanh(self) -> Self::Output {
                    <$ty>::tanh(self)
                }
            }

            impl Asinh for $ty {
                type Output = $ty;

                #[inline]
                fn asinh(self) -> Self::Output {
                    <$ty>::asinh(self)
                }
            }

            impl Acosh for $ty {
                type Output = $ty;

                #[inline]
                fn acosh(self) -> Self::Output {
                    <$ty>::acosh(self)
                }
            }

            impl Atanh for $ty {
                type Output = $ty;

                #[inline]
                fn atanh(self) -> Self::Output {
                    <$ty>::atanh(self)
                }
            }
        )*
    }
}

impl_trig_traits_for_floats!(f32, f64);
