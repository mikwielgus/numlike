// SPDX-FileCopyrightText: 2026 numlike contributors
//
// SPDX-License-Identifier: MIT OR Apache-2.0

//! Abstract algebraic structures.

use crate::{
    elem::{One, Zero},
    ops::{FieldOps, RingOps},
};

/// Has total order, has field operations.
pub trait TofieldBundle: Ord + FieldBundle {}
impl<T: Ord + FieldBundle> TofieldBundle for T {}

/// Has partial order, has field operations.
pub trait PofieldBundle: PartialOrd + FieldBundle {}
impl<T: PartialOrd + FieldBundle> PofieldBundle for T {}

/// Has field operations.
pub trait FieldBundle: Zero + One + FieldOps + Sized {}
impl<T: Zero + One + FieldOps + Sized> FieldBundle for T {}

/// Has total order, has ring operations.
pub trait ToringBundle: Ord + RingBundle {}
impl<T: Ord + RingBundle> ToringBundle for T {}

/// Has partial order, has ring operations.
pub trait PoringBundle: PartialOrd + RingBundle {}
impl<T: PartialOrd + RingBundle> PoringBundle for T {}

/// Has ring operations.
pub trait RingBundle: Zero + One + RingOps + Sized {}
impl<T: Zero + One + RingOps + Sized> RingBundle for T {}
