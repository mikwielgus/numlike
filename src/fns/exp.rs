// SPDX-FileCopyrightText: 2026 numlike contributors
//
// SPDX-License-Identifier: MIT OR Apache-2.0

/// Bundle of exponential functions.
pub trait ExpFns: Exp + Exp2 + ExpM1 {}
impl<T: Exp + Exp2 + ExpM1> ExpFns for T {}

pub trait Exp {
    type Output;

    /// Returns `e^(self)`, (the exponential function).
    fn exp(self) -> Self::Output;
}

pub trait Exp2 {
    type Output;

    /// Returns `2^(self)`.
    fn exp2(self) -> Self::Output;
}

pub trait ExpM1 {
    type Output;

    /// Returns `e^(self) - 1` in a way that is accurate even if the
    /// number is close to zero.
    fn exp_m1(self) -> Self::Output;
}

/// Bundle of checked exponential functions.
pub trait CheckedExpFns: CheckedExp + CheckedExp2 + CheckedExpM1 {}
impl<T: CheckedExp + CheckedExp2 + CheckedExpM1> CheckedExpFns for T {}

pub trait CheckedExp {
    type Output;

    /// Returns `e^(self)`, (the exponential function).
    ///
    /// Returns `None` if the result is not finite.
    fn checked_exp(self) -> Option<Self::Output>;
}

pub trait CheckedExp2 {
    type Output;

    /// Returns `2^(self)`.
    ///
    /// Returns `None` if the result is not finite.
    fn checked_exp2(self) -> Option<Self::Output>;
}

pub trait CheckedExpM1 {
    type Output;

    /// Returns `e^(self) - 1` in a way that is accurate even if the
    /// number is close to zero.
    ///
    /// Returns `None` if the result is not finite.
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
