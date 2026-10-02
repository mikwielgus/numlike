// SPDX-FileCopyrightText: 2026 numlike contributors
//
// SPDX-License-Identifier: MIT OR Apache-2.0

//! Abstract algebraic structures.

use crate::{
    elem::{One, Zero},
    ops::{FieldOps, RingOps},
};

/// Element of a totally-ordered field (i.e. of a tofield).
pub trait TofieldBundle: Ord + FieldBundle {}
impl<T: Ord + FieldBundle> TofieldBundle for T {}

/// Element of a partially-ordered field (i.e. of a pofield).
pub trait PofieldBundle: PartialOrd + FieldBundle {}
impl<T: PartialOrd + FieldBundle> PofieldBundle for T {}

/// Element of a field.
pub trait FieldBundle: Zero + One + FieldOps + Sized {}
impl<T: Zero + One + FieldOps + Sized> FieldBundle for T {}

/// Element of a totally-ordered ring (i.e. of a toring).
pub trait ToringBundle: Ord + RingBundle {}
impl<T: Ord + RingBundle> ToringBundle for T {}

/// Element of a partially-ordered ring (i.e. of a poring).
pub trait PoringBundle: PartialOrd + RingBundle {}
impl<T: PartialOrd + RingBundle> PoringBundle for T {}

/// Element of a ring.
pub trait RingBundle: Zero + One + RingOps + Sized {}
impl<T: Zero + One + RingOps + Sized> RingBundle for T {}
