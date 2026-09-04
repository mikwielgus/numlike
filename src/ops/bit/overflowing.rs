// SPDX-FileCopyrightText: 2026 numlike contributors
//
// SPDX-License-Identifier: MIT OR Apache-2.0

/// Bundle of overflowing bitwise shift operations.
pub trait OverflowingBitOps: OverflowingShl + OverflowingShr {}
impl<T: OverflowingShl + OverflowingShr> OverflowingBitOps for T {}

/// Shifts self left by `rhs` bits.
///
/// Returns a tuple of the shifted version of self along with a boolean indicating whether the shift
/// value was larger than or equal to the number of bits. If the shift value is too large, then value is
/// masked (N-1) where N is the number of bits, and this value is then used to perform the shift.
pub trait OverflowingShl {
    /// The resulting type after applying the operation.
    type Output;

    /// Shifts self left by `rhs` bits.
    fn overflowing_shl(self, rhs: u32) -> (Self::Output, bool);
}

/// Shifts self right by `rhs` bits.
///
/// Returns a tuple of the shifted version of self along with a boolean indicating whether the shift
/// value was larger than or equal to the number of bits. If the shift value is too large, then value is
/// masked (N-1) where N is the number of bits, and this value is then used to perform the shift.
pub trait OverflowingShr {
    /// The resulting type after applying the operation.
    type Output;

    /// Shifts self right by `rhs` bits.
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
