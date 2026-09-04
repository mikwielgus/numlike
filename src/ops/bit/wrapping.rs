// SPDX-FileCopyrightText: 2026 numlike contributors
//
// SPDX-License-Identifier: MIT OR Apache-2.0

/// Panic-free bitwise shift-left; yields `self << mask(rhs)`, where `mask`
/// removes any high-order bits of `rhs` that would cause the shift to
/// exceed the bitwidth of the type.
///
/// Beware that, unlike most other `wrapping_*` methods on integers, this
/// does *not* give the same result as doing the shift in infinite precision
/// then truncating as needed.  The behaviour matches what shift instructions
/// do on many processors, and is what the `<<` operator does when overflow
/// checks are disabled, but numerically it's weird.
///
/// Note that this is *not* the same as a rotate-left; the RHS of a wrapping
/// shift-left is restricted to the range of the type, rather than the bits
/// shifted out of the LHS being returned to the other end.
pub trait WrappingShl {
    /// The resulting type after applying the operation.
    type Output;

    /// Panic-free bitwise shift-left; yields `self << mask(rhs)`, where `mask`
    /// removes any high-order bits of `rhs` that would cause the shift to exceed
    /// the bitwidth of the type.
    fn wrapping_shl(self, rhs: u32) -> Self::Output;
}

/// Panic-free bitwise shift-right; yields `self >> mask(rhs)`, where `mask`
/// removes any high-order bits of `rhs` that would cause the shift to
/// exceed the bitwidth of the type.
///
/// Beware that, unlike most other `wrapping_*` methods on integers, this
/// does *not* give the same result as doing the shift in infinite precision
/// then truncating as needed.  The behaviour matches what shift instructions
/// do on many processors, and is what the `>>` operator does when overflow
/// checks are disabled, but numerically it's weird.
///
/// Note that this is *not* the same as a rotate-right; the RHS of a
/// wrapping shift-right is restricted to the range of the type, rather than
/// the bits shifted out of the LHS being returned to the other end.
pub trait WrappingShr {
    /// The resulting type after applying the operation.
    type Output;

    /// Panic-free bitwise shift-right; yields `self >> mask(rhs)`, where `mask`
    /// removes any high-order bits of `rhs` that would cause the shift to exceed
    /// the bitwidth of the type.
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
