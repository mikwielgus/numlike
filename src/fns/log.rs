// SPDX-FileCopyrightText: 2026 numlike contributors
//
// SPDX-License-Identifier: MIT OR Apache-2.0

pub trait LogFns<Rhs = Self>: Ln + Log2 + Log10 + Ln1p + Log<Rhs> {}
impl<Rhs, T: Ln + Log2 + Log10 + Ln1p + Log<Rhs>> LogFns<Rhs> for T {}

pub trait Ln {
    type Output;

    fn ln(self) -> Self::Output;
}

pub trait Log2 {
    type Output;

    fn log2(self) -> Self::Output;
}

pub trait Log10 {
    type Output;

    fn log10(self) -> Self::Output;
}

pub trait Ln1p {
    type Output;

    fn ln_1p(self) -> Self::Output;
}

pub trait Log<Rhs = Self> {
    type Output;

    fn log(self, base: Rhs) -> Self::Output;
}

macro_rules! impl_log_traits_for_floats {
    ($($ty:ty),*) => {
        $(
            impl Ln for $ty {
                type Output = $ty;

                #[inline]
                fn ln(self) -> Self::Output {
                    <$ty>::ln(self)
                }
            }

            impl Log2 for $ty {
                type Output = $ty;

                #[inline]
                fn log2(self) -> Self::Output {
                    <$ty>::log2(self)
                }
            }

            impl Log10 for $ty {
                type Output = $ty;

                #[inline]
                fn log10(self) -> Self::Output {
                    <$ty>::log10(self)
                }
            }

            impl Ln1p for $ty {
                type Output = $ty;

                #[inline]
                fn ln_1p(self) -> Self::Output {
                    <$ty>::ln_1p(self)
                }
            }

            impl Log<$ty> for $ty {
                type Output = $ty;

                #[inline]
                fn log(self, base: $ty) -> Self::Output {
                    <$ty>::log(self, base)
                }
            }
        )*
    };
}

impl_log_traits_for_floats!(f32, f64);
