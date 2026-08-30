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

pub trait Ilog<Rhs = Self> {
    type Output;

    fn ilog(self, base: Rhs) -> Self::Output;
}

pub trait Ilog2 {
    type Output;

    fn ilog2(self) -> Self::Output;
}

pub trait Ilog10 {
    type Output;

    fn ilog10(self) -> Self::Output;
}

pub trait CheckedIlog<Rhs = Self> {
    type Output;

    fn checked_ilog(self, base: Rhs) -> Option<Self::Output>;
}

pub trait CheckedIlog2 {
    type Output;

    fn checked_ilog2(self) -> Option<Self::Output>;
}

pub trait CheckedIlog10 {
    type Output;

    fn checked_ilog10(self) -> Option<Self::Output>;
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

macro_rules! impl_ilog_traits_for_ints {
    ($($ty:ty),*) => {
        $(
            impl Ilog<$ty> for $ty {
                type Output = u32;

                #[inline]
                fn ilog(self, base: $ty) -> Self::Output {
                    <$ty>::ilog(self, base)
                }
            }

            impl Ilog2 for $ty {
                type Output = u32;

                #[inline]
                fn ilog2(self) -> Self::Output {
                    <$ty>::ilog2(self)
                }
            }

            impl Ilog10 for $ty {
                type Output = u32;

                #[inline]
                fn ilog10(self) -> Self::Output {
                    <$ty>::ilog10(self)
                }
            }

            impl CheckedIlog<$ty> for $ty {
                type Output = u32;

                #[inline]
                fn checked_ilog(self, base: $ty) -> Option<Self::Output> {
                    <$ty>::checked_ilog(self, base)
                }
            }

            impl CheckedIlog2 for $ty {
                type Output = u32;

                #[inline]
                fn checked_ilog2(self) -> Option<Self::Output> {
                    <$ty>::checked_ilog2(self)
                }
            }

            impl CheckedIlog10 for $ty {
                type Output = u32;

                #[inline]
                fn checked_ilog10(self) -> Option<Self::Output> {
                    <$ty>::checked_ilog10(self)
                }
            }
        )*
    };
}

impl_ilog_traits_for_ints!(i8, i16, i32, i64, i128, isize);
impl_ilog_traits_for_ints!(u8, u16, u32, u64, u128, usize);
