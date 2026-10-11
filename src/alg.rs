// SPDX-FileCopyrightText: 2026 numlike contributors
//
// SPDX-License-Identifier: MIT OR Apache-2.0

//! Abstract algebraic structures.

use crate::{
    elem::{One, Zero},
    ops::{FieldOps, RingOps},
};

/// Cloneable element of a totally-ordered field (i.e. of a tofield).
pub trait TofieldNum: Clone + TofieldBundle {}
impl<T: Clone + TofieldBundle> TofieldNum for T {}

/// Element of a totally-ordered field (i.e. of a tofield).
pub trait TofieldBundle: Ord + FieldBundle {}
impl<T: Ord + FieldBundle> TofieldBundle for T {}

/// Cloneable element of a partially-ordered field (i.e. of a pofield).
pub trait PofieldNum: Clone + PofieldBundle {}
impl<T: Clone + PofieldBundle> PofieldNum for T {}

/// Element of a partially-ordered field (i.e. of a pofield).
pub trait PofieldBundle: PartialOrd + FieldBundle {}
impl<T: PartialOrd + FieldBundle> PofieldBundle for T {}

/// Cloneable element of a field.
pub trait FieldNum: Clone + FieldBundle {}
impl<T: Clone + FieldBundle> FieldNum for T {}

/// Element of a field.
pub trait FieldBundle: Zero + One + FieldOps + Sized {}
impl<T: Zero + One + FieldOps + Sized> FieldBundle for T {}

/// Cloneable element of a totally-ordered ring (i.e. of a toring).
pub trait ToringNum: Clone + ToringBundle {}
impl<T: Clone + ToringBundle> ToringNum for T {}

/// Element of a totally-ordered ring (i.e. of a toring).
pub trait ToringBundle: Ord + RingBundle {}
impl<T: Ord + RingBundle> ToringBundle for T {}

/// Cloneable element of a partially-ordered ring (i.e. of a poring).
pub trait PoringNum: Clone + PoringBundle {}
impl<T: Clone + PoringBundle> PoringNum for T {}

/// Element of a partially-ordered ring (i.e. of a poring).
pub trait PoringBundle: PartialOrd + RingBundle {}
impl<T: PartialOrd + RingBundle> PoringBundle for T {}

/// Cloneable element of a ring.
pub trait RingNum: Clone + RingBundle {}
impl<T: Clone + RingBundle> RingNum for T {}

/// Element of a ring.
pub trait RingBundle: Zero + One + RingOps + Sized {}
impl<T: Zero + One + RingOps + Sized> RingBundle for T {}
