// SPDX-FileCopyrightText: 2026 numlike contributors
//
// SPDX-License-Identifier: MIT OR Apache-2.0

pub trait RoundFns: Round + Trunc + RoundTiesEven + Floor + Ceil {}
impl<T: Round + Trunc + RoundTiesEven + Floor + Ceil> RoundFns for T {}

pub trait Round {
    type Output;

    fn round(self) -> Self::Output;
}

pub trait Trunc {
    type Output;

    fn trunc(self) -> Self::Output;
}

pub trait RoundTiesEven {
    type Output;

    fn round_ties_even(self) -> Self::Output;
}

pub trait Floor {
    type Output;

    fn floor(self) -> Self::Output;
}

pub trait Ceil {
    type Output;

    fn ceil(self) -> Self::Output;
}

macro_rules! impl_round_traits_for_floats {
    ($($ty:ty),*) => {
        $(
            impl Round for $ty {
                type Output = $ty;

                #[inline]
                fn round(self) -> Self::Output {
                    <$ty>::round(self)
                }
            }

            impl Trunc for $ty {
                type Output = $ty;

                #[inline]
                fn trunc(self) -> Self::Output {
                    <$ty>::trunc(self)
                }
            }

            impl RoundTiesEven for $ty {
                type Output = $ty;

                #[inline]
                fn round_ties_even(self) -> Self::Output {
                    <$ty>::round_ties_even(self)
                }
            }

            impl Floor for $ty {
                type Output = $ty;

                #[inline]
                fn floor(self) -> Self::Output {
                    <$ty>::floor(self)
                }
            }

            impl Ceil for $ty {
                type Output = $ty;

                #[inline]
                fn ceil(self) -> Self::Output {
                    <$ty>::ceil(self)
                }
            }
        )*
    };
}

impl_round_traits_for_floats!(f32, f64);
