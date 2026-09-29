// SPDX-FileCopyrightText: 2026 numlike contributors
//
// SPDX-License-Identifier: MIT OR Apache-2.0

//! Abstract algebraic structures.

use crate::{
    elem::{One, Zero},
    ops::{FieldOps, RingOps},
};

/// Element of a totally-ordered field (i.e. of a tofield).
pub trait Tofield: Ord + Field {}
impl<T: Ord + Field> Tofield for T {}

/// Element of a partially-ordered field (i.e. of a pofield).
pub trait Pofield: PartialOrd + Field {}
impl<T: PartialOrd + Field> Pofield for T {}

/// Element of a field.
pub trait Field: Zero + One + FieldOps + Sized {}
impl<T: Zero + One + FieldOps + Sized> Field for T {}

/// Element of a totally-ordered ring (i.e. of a toring).
pub trait Toring: Ord + Ring {}
impl<T: Ord + Ring> Toring for T {}

/// Element of a partially-ordered ring (i.e. of a poring).
pub trait Poring: PartialOrd + Ring {}
impl<T: PartialOrd + Ring> Poring for T {}

/// Element of a ring.
pub trait Ring: Zero + One + RingOps + Sized {}
impl<T: Zero + One + RingOps + Sized> Ring for T {}
