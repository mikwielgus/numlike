// SPDX-FileCopyrightText: 2026 numlike contributors
//
// SPDX-License-Identifier: MIT OR Apache-2.0

/// Bundle of checked bitwise shift operations.
pub trait CheckedBitOps: CheckedShl + CheckedShr {}
impl<T: CheckedShl + CheckedShr> CheckedBitOps for T {}

/// Checked shift left. Computes `self << rhs`, returning `None` if `rhs` is larger
/// than or equal to the number of bits in `self`.
pub trait CheckedShl {
    /// The resulting type after applying the operation.
    type Output;

    /// Checked shift left. Computes `self << rhs`, returning `None` if `rhs` is larger
    /// than or equal to the number of bits in `self`.
    fn checked_shl(self, rhs: u32) -> Option<Self::Output>;
}

/// Checked shift right. Computes `self >> rhs`, returning `None` if `rhs` is
/// larger than or equal to the number of bits in `self`.
pub trait CheckedShr {
    /// The resulting type after applying the operation.
    type Output;

    /// Checked shift right. Computes `self >> rhs`, returning `None` if `rhs` is
    /// larger than or equal to the number of bits in `self`.
    fn checked_shr(self, rhs: u32) -> Option<Self::Output>;
}

macro_rules! impl_checked_shift_traits_for_ints {
    ($($ty:ty),*) => {
        $(
            impl CheckedShl for $ty {
                type Output = $ty;

                #[inline]
                fn checked_shl(self, rhs: u32) -> Option<Self::Output> {
                    <$ty>::checked_shl(self, rhs)
                }
            }

            impl CheckedShr for $ty {
                type Output = $ty;

                #[inline]
                fn checked_shr(self, rhs: u32) -> Option<Self::Output> {
                    <$ty>::checked_shr(self, rhs)
                }
            }
        )*
    };
}

impl_checked_shift_traits_for_ints!(i8, i16, i32, i64, i128, isize);
impl_checked_shift_traits_for_ints!(u8, u16, u32, u64, u128, usize);
