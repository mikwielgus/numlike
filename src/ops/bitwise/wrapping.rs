// SPDX-FileCopyrightText: 2026 numlike contributors
//
// SPDX-License-Identifier: MIT OR Apache-2.0

pub trait WrappingShl {
    type Output;

    fn wrapping_shl(self, rhs: u32) -> Self::Output;
}

pub trait WrappingShr {
    type Output;

    fn wrapping_shr(self, rhs: u32) -> Self::Output;
}

macro_rules! impl_wrapping_shift_traits_for_ints {
    ($($ty:ty),*) => {
        $(
            impl WrappingShl for $ty {
                type Output = $ty;

                #[inline]
                fn wrapping_shl(self, rhs: u32) -> Self::Output {
                    <$ty>::wrapping_shl(self, rhs)
                }
            }

            impl WrappingShr for $ty {
                type Output = $ty;

                #[inline]
                fn wrapping_shr(self, rhs: u32) -> Self::Output {
                    <$ty>::wrapping_shr(self, rhs)
                }
            }
        )*
    };
}

impl_wrapping_shift_traits_for_ints!(i8, i16, i32, i64, i128, isize);
impl_wrapping_shift_traits_for_ints!(u8, u16, u32, u64, u128, usize);
