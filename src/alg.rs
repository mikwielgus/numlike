// SPDX-FileCopyrightText: 2026 numlike contributors
//
// SPDX-License-Identifier: MIT OR Apache-2.0

//! Abstract algebraic structures.

use crate::{
    elem::{One, Zero},
    ops::{FieldOps, RingOps},
};

/// Cloneable, has total order, has field operations.
pub trait TofieldBundle: Clone + Ord + FieldBundle {}
impl<T: Clone + Ord + FieldBundle> TofieldBundle for T {}

/// Cloneable, has partial order, has field operations.
pub trait PofieldBundle: Clone + PartialOrd + FieldBundle {}
impl<T: Clone + PartialOrd + FieldBundle> PofieldBundle for T {}

/// Cloneable, has field operations.
pub trait FieldBundle: Clone + Zero + One + FieldOps + Sized {}
impl<T: Clone + Zero + One + FieldOps + Sized> FieldBundle for T {}

/// Cloneable, has total order, has ring operations.
pub trait ToringBundle: Clone + Ord + RingBundle {}
impl<T: Clone + Ord + RingBundle> ToringBundle for T {}

/// Cloneable, has partial order, has ring operations.
pub trait PoringBundle: Clone + PartialOrd + RingBundle {}
impl<T: Clone + PartialOrd + RingBundle> PoringBundle for T {}

/// Cloneable, has ring operations.
pub trait RingBundle: Clone + Zero + One + RingOps + Sized {}
impl<T: Clone + Zero + One + RingOps + Sized> RingBundle for T {}
