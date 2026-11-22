// SPDX-FileCopyrightText: 2026 numlike contributors
//
// SPDX-License-Identifier: MIT OR Apache-2.0

//! A two-dimensional vector, with arbitrary numeric type as field.

use core::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};

use crate::ops::{CheckedAdd, CheckedDiv, CheckedNeg, CheckedSub, NegAssign};

/// A two-dimensional vector, with arbitrary numeric type as field.
#[derive(Clone, Copy, Debug)]
pub struct Vector2<T> {
    x: T,
    y: T,
}

impl<T: Clone> Vector2<T> {
    /// Returns the vector's X coordinate.
    #[inline]
    pub fn x(&self) -> T {
        self.x.clone()
    }

    /// Returns the vector's Y coordinate.
    #[inline]
    pub fn y(&self) -> T {
        self.y.clone()
    }
}

impl<T> Vector2<T> {
    /// Returns the vector's X coordinate as immutable reference.
    #[inline]
    pub fn x_ref(&self) -> &T {
        &self.x
    }

    /// Returns the vector's Y coordinate as immutable reference.
    #[inline]
    pub fn y_ref(&self) -> &T {
        &self.y
    }

    /// Create a vector from its components.
    #[inline]
    pub fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

impl<T: Add<Output = T>> Add for Vector2<T> {
    type Output = Self;

    #[inline]
    fn add(self, rhs: Self) -> Self {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl<T: Add<Output = T> + Clone> AddAssign for Vector2<T> {
    #[inline]
    fn add_assign(&mut self, rhs: Self) {
        *self = self.clone() + rhs
    }
}

impl<T: CheckedAdd<Output = T>> CheckedAdd for Vector2<T> {
    type Output = Self;

    #[inline]
    fn checked_add(self, rhs: Self) -> Option<Self> {
        Some(Self {
            x: self.x.checked_add(rhs.x)?,
            y: self.y.checked_add(rhs.y)?,
        })
    }
}

impl<T: Sub<Output = T>> Sub for Vector2<T> {
    type Output = Self;

    #[inline]
    fn sub(self, rhs: Self) -> Self {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl<T: Sub<Output = T> + Clone> SubAssign for Vector2<T> {
    #[inline]
    fn sub_assign(&mut self, rhs: Self) {
        *self = self.clone() - rhs
    }
}

impl<T: CheckedSub<Output = T>> CheckedSub for Vector2<T> {
    type Output = Self;

    #[inline]
    fn checked_sub(self, rhs: Self) -> Option<Self> {
        Some(Self {
            x: self.x.checked_sub(rhs.x)?,
            y: self.y.checked_sub(rhs.y)?,
        })
    }
}

impl<T: Clone + Mul<Output = T>> Mul<T> for Vector2<T> {
    type Output = Self;

    #[inline]
    fn mul(self, rhs: T) -> Self {
        Self {
            x: self.x * rhs.clone(),
            y: self.y * rhs,
        }
    }
}

impl<T: Clone + Mul<Output = T>> MulAssign<T> for Vector2<T> {
    #[inline]
    fn mul_assign(&mut self, rhs: T) {
        *self = self.clone() * rhs
    }
}

impl<T: Clone + Div<Output = T>> Div<T> for Vector2<T> {
    type Output = Self;

    #[inline]
    fn div(self, rhs: T) -> Self {
        Self {
            x: self.x / rhs.clone(),
            y: self.y / rhs,
        }
    }
}

impl<T: Clone + Div<Output = T>> DivAssign<T> for Vector2<T> {
    #[inline]
    fn div_assign(&mut self, rhs: T) {
        *self = self.clone() / rhs
    }
}

impl<T: Clone + CheckedDiv<Output = T>> CheckedDiv<T> for Vector2<T> {
    type Output = Vector2<T>;

    #[inline]
    fn checked_div(self, rhs: T) -> Option<Vector2<T>> {
        Some(Self {
            x: self.x.checked_div(rhs.clone())?,
            y: self.y.checked_div(rhs)?,
        })
    }
}

impl<T: Neg<Output = T>> Neg for Vector2<T> {
    type Output = Vector2<T>;

    #[inline]
    fn neg(self) -> Vector2<T> {
        Vector2 {
            x: -self.x,
            y: -self.y,
        }
    }
}

impl<T: Clone + Neg<Output = T>> NegAssign for Vector2<T> {
    #[inline]
    fn neg_assign(&mut self) {
        *self = -self.clone();
    }
}

impl<T: CheckedNeg<Output = T> + Clone> CheckedNeg for Vector2<T> {
    type Output = Vector2<T>;

    #[inline]
    fn checked_neg(self) -> Option<Vector2<T>> {
        Some(Vector2 {
            x: self.x.clone().checked_neg()?,
            y: self.y.checked_neg()?,
        })
    }
}

impl<T: PartialEq> PartialEq for Vector2<T> {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}

impl<T: Eq> Eq for Vector2<T> {}

#[cfg(feature = "maplike")]
impl_maplike_traits_for_scalar!(T, Vector2);

#[cfg(feature = "undoredo")]
impl_undoredo_traits_for_scalar!(T, Vector2);

// TODO: the remaining traits.
