// SPDX-FileCopyrightText: 2026 numlike contributors
//
// SPDX-License-Identifier: MIT OR Apache-2.0

pub trait OverflowingBitOps: OverflowingShl + OverflowingShr {}
impl<T: OverflowingShl + OverflowingShr> OverflowingBitOps for T {}

pub trait OverflowingShl {
    type Output;

    fn overflowing_shl(self, rhs: u32) -> (Self::Output, bool);
}

pub trait OverflowingShr {
    type Output;

    fn overflowing_shr(self, rhs: u32) -> (Self::Output, bool);
}

macro_rules! impl_overflowing_shift_traits_for_ints {
    ($($ty:ty),*) => {
        $(
            impl OverflowingShl for $ty {
                type Output = $ty;

                #[inline]
                fn overflowing_shl(self, rhs: u32) -> (Self::Output, bool) {
                    <$ty>::overflowing_shl(self, rhs)
                }
            }

            impl OverflowingShr for $ty {
                type Output = $ty;

                #[inline]
                fn overflowing_shr(self, rhs: u32) -> (Self::Output, bool) {
                    <$ty>::overflowing_shr(self, rhs)
                }
            }
        )*
    };
}

impl_overflowing_shift_traits_for_ints!(i8, i16, i32, i64, i128, isize);
impl_overflowing_shift_traits_for_ints!(u8, u16, u32, u64, u128, usize);
