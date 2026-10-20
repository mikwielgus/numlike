// SPDX-FileCopyrightText: 2026 numlike contributors
//
// SPDX-License-Identifier: MIT OR Apache-2.0

//! A wrapper to turn this library's trait methods into inherent methods.

use core::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};

/// A wrapper to turn this library's trait methods into inherent methods.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Object<B>(pub(crate) B);

impl<B> Object<B> {
    /// Wrap the backend inside an object type.
    pub fn new(backend: B) -> Self {
        Self(backend)
    }

    /// Unwrap the backend out of the object wrapper type.
    pub fn dissolve(self) -> B {
        self.0
    }
}

impl<B: Add<Output = B>> Add for Object<B> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0)
    }
}

impl<B: AddAssign> AddAssign for Object<B> {
    fn add_assign(&mut self, rhs: Self) {
        self.0 += rhs.0;
    }
}

impl<B: Sub<Output = B>> Sub for Object<B> {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self(self.0 - rhs.0)
    }
}

impl<B: SubAssign> SubAssign for Object<B> {
    fn sub_assign(&mut self, rhs: Self) {
        self.0 -= rhs.0;
    }
}

impl<B: Mul<Output = B>> Mul for Object<B> {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self(self.0 * rhs.0)
    }
}

impl<B: MulAssign> MulAssign for Object<B> {
    fn mul_assign(&mut self, rhs: Self) {
        self.0 *= rhs.0;
    }
}

impl<B: Div<Output = B>> Div for Object<B> {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        Self(self.0 / rhs.0)
    }
}

impl<B: DivAssign> DivAssign for Object<B> {
    fn div_assign(&mut self, rhs: Self) {
        self.0 /= rhs.0;
    }
}

impl<B: Neg<Output = B>> Neg for Object<B> {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self(-self.0)
    }
}
