// SPDX-FileCopyrightText: 2026 numlike contributors
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use core::ops::{
    BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Not, Shl, ShlAssign, Shr,
    ShrAssign,
};

/// Bundle of bitwise operations, including assign variants.
pub trait FullBitOps<Rhs = Self>: BitOps<Rhs> + BitAssignOps<Rhs> {}

/// Bundle of bitwise operations.
pub trait BitOps<Rhs = Self>:
    BitAnd<Rhs> + BitOr<Rhs> + BitXor<Rhs> + Not + Shl<Rhs> + Shr<Rhs>
{
}
impl<Rhs, T: BitAnd<Rhs> + BitOr<Rhs> + BitXor<Rhs> + Not + Shl<Rhs> + Shr<Rhs>> BitOps<Rhs> for T {}

/// Bundle of bitwise assignment operations.
pub trait BitAssignOps<Rhs = Self>:
    BitAndAssign<Rhs>
    + BitOrAssign<Rhs>
    + BitXorAssign<Rhs>
    + NotAssign
    + ShlAssign<Rhs>
    + ShrAssign<Rhs>
{
}
impl<
    Rhs,
    T: BitAndAssign<Rhs>
        + BitOrAssign<Rhs>
        + BitXorAssign<Rhs>
        + NotAssign
        + ShlAssign<Rhs>
        + ShrAssign<Rhs>,
> BitAssignOps<Rhs> for T
{
}

/// Inverts all bits of `self` in place.
pub trait NotAssign {
    /// Inverts all bits of `self` in place.
    fn not_assign(&mut self);
}

macro_rules! impl_not_assign_for_bitwisables {
    ($($ty:ty),*) => {
        $(
            impl NotAssign for $ty {
                #[inline]
                fn not_assign(&mut self) {
                    *self = Not::not(*self);
                }
            }
        )*
    };
}

impl_not_assign_for_bitwisables!(i8, i16, i32, i64, i128, isize);
impl_not_assign_for_bitwisables!(u8, u16, u32, u64, u128, usize);
impl_not_assign_for_bitwisables!(bool);
