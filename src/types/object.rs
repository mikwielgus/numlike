// SPDX-FileCopyrightText: 2026 numlike contributors
//
// SPDX-License-Identifier: MIT OR Apache-2.0

//! A wrapper to turn this library's trait methods into inherent methods.

use core::ops::{
    Add, AddAssign, BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Div, DivAssign,
    Mul, MulAssign, Neg, Not, Rem, RemAssign, Shl, ShlAssign, Shr, ShrAssign, Sub, SubAssign,
};

/// A wrapper to turn this library's trait methods into inherent methods.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Object<B>(pub(crate) B);

impl<B> Object<B> {
    /// Wrap the backend inside an object type.
    #[inline(always)]
    pub fn new(backend: B) -> Self {
        Self(backend)
    }

    /// Unwrap the backend out of the object wrapper type.
    #[inline(always)]
    pub fn dissolve(self) -> B {
        self.0
    }
}

impl<B: Add<Output = B>> Add for Object<B> {
    type Output = Self;

    #[inline(always)]
    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0)
    }
}

impl<B: AddAssign> AddAssign for Object<B> {
    #[inline(always)]
    fn add_assign(&mut self, rhs: Self) {
        self.0 += rhs.0;
    }
}

impl<B: Sub<Output = B>> Sub for Object<B> {
    type Output = Self;

    #[inline(always)]
    fn sub(self, rhs: Self) -> Self::Output {
        Self(self.0 - rhs.0)
    }
}

impl<B: SubAssign> SubAssign for Object<B> {
    #[inline(always)]
    fn sub_assign(&mut self, rhs: Self) {
        self.0 -= rhs.0;
    }
}

impl<B: Mul<Output = B>> Mul for Object<B> {
    type Output = Self;

    #[inline(always)]
    fn mul(self, rhs: Self) -> Self::Output {
        Self(self.0 * rhs.0)
    }
}

impl<B: MulAssign> MulAssign for Object<B> {
    #[inline(always)]
    fn mul_assign(&mut self, rhs: Self) {
        self.0 *= rhs.0;
    }
}

impl<B: Div<Output = B>> Div for Object<B> {
    type Output = Self;

    #[inline(always)]
    fn div(self, rhs: Self) -> Self::Output {
        Self(self.0 / rhs.0)
    }
}

impl<B: DivAssign> DivAssign for Object<B> {
    #[inline(always)]
    fn div_assign(&mut self, rhs: Self) {
        self.0 /= rhs.0;
    }
}

impl<B: Rem<Output = B>> Rem for Object<B> {
    type Output = Self;

    #[inline(always)]
    fn rem(self, rhs: Self) -> Self::Output {
        Self(self.0 % rhs.0)
    }
}

impl<B: RemAssign> RemAssign for Object<B> {
    #[inline(always)]
    fn rem_assign(&mut self, rhs: Self) {
        self.0 %= rhs.0;
    }
}

impl<B: Neg<Output = B>> Neg for Object<B> {
    type Output = Self;

    #[inline(always)]
    fn neg(self) -> Self::Output {
        Self(-self.0)
    }
}

impl<B: BitAnd<Output = B>> BitAnd for Object<B> {
    type Output = Self;

    #[inline(always)]
    fn bitand(self, rhs: Self) -> Self::Output {
        Self(self.0 & rhs.0)
    }
}

impl<B: BitAndAssign> BitAndAssign for Object<B> {
    #[inline(always)]
    fn bitand_assign(&mut self, rhs: Self) {
        self.0 &= rhs.0;
    }
}

impl<B: BitOr<Output = B>> BitOr for Object<B> {
    type Output = Self;

    #[inline(always)]
    fn bitor(self, rhs: Self) -> Self::Output {
        Self(self.0 | rhs.0)
    }
}

impl<B: BitOrAssign> BitOrAssign for Object<B> {
    #[inline(always)]
    fn bitor_assign(&mut self, rhs: Self) {
        self.0 |= rhs.0;
    }
}

impl<B: BitXor<Output = B>> BitXor for Object<B> {
    type Output = Self;

    #[inline(always)]
    fn bitxor(self, rhs: Self) -> Self::Output {
        Self(self.0 ^ rhs.0)
    }
}

impl<B: BitXorAssign> BitXorAssign for Object<B> {
    #[inline(always)]
    fn bitxor_assign(&mut self, rhs: Self) {
        self.0 ^= rhs.0;
    }
}

impl<B: Not<Output = B>> Not for Object<B> {
    type Output = Self;

    #[inline(always)]
    fn not(self) -> Self::Output {
        Self(!self.0)
    }
}

impl<B: Shl<Output = B>> Shl for Object<B> {
    type Output = Self;

    #[inline(always)]
    fn shl(self, rhs: Self) -> Self::Output {
        Self(self.0 << rhs.0)
    }
}

impl<B: ShlAssign> ShlAssign for Object<B> {
    #[inline(always)]
    fn shl_assign(&mut self, rhs: Self) {
        self.0 <<= rhs.0;
    }
}

impl<B: Shr<Output = B>> Shr for Object<B> {
    type Output = Self;

    #[inline(always)]
    fn shr(self, rhs: Self) -> Self::Output {
        Self(self.0 >> rhs.0)
    }
}

impl<B: ShrAssign> ShrAssign for Object<B> {
    #[inline(always)]
    fn shr_assign(&mut self, rhs: Self) {
        self.0 >>= rhs.0;
    }
}

#[cfg(feature = "maplike")]
impl_maplike_traits_for_scalar!(T, Object);

#[cfg(feature = "undoredo")]
impl_undoredo_traits_for_scalar!(T, Object);

// TODO: the remaining traits.
