// SPDX-FileCopyrightText: 2026 numlike contributors
//
// SPDX-License-Identifier: MIT OR Apache-2.0

//! Abstract algebraic structures.

use crate::{
    elem::{One, Zero},
    ops::{FieldOps, RingOps},
};

/// Element of a totally-ordered field (i.e. of a tofield).
pub trait TofieldElem: Ord + FieldElem {}
impl<T: Ord + FieldElem> TofieldElem for T {}

/// Element of a partially-ordered field (i.e. of a pofield).
pub trait PofieldElem: PartialOrd + FieldElem {}
impl<T: PartialOrd + FieldElem> PofieldElem for T {}

/// Element of a field.
pub trait FieldElem: Zero + One + FieldOps + Sized {}
impl<T: Zero + One + FieldOps + Sized> FieldElem for T {}

/// Element of a totally-ordered ring (i.e. of a toring).
pub trait ToringElem: Ord + RingElem {}
impl<T: Ord + RingElem> ToringElem for T {}

/// Element of a partially-ordered ring (i.e. of a poring).
pub trait PoringElem: PartialOrd + RingElem {}
impl<T: PartialOrd + RingElem> PoringElem for T {}

/// Element of a ring.
pub trait RingElem: Zero + One + RingOps + Sized {}
impl<T: Zero + One + RingOps + Sized> RingElem for T {}
