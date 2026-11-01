// SPDX-FileCopyrightText: 2026 numlike contributors
//
// SPDX-License-Identifier: MIT OR Apache-2.0

//! Abstract algebraic structures.

use crate::{
    elem::{One, Zero},
    limits::Bounds,
    ops::{FieldOps, RingOps},
};

/// Supertraits shared by all `*Num` traits.
pub trait NumCommon: Clone + Default + Bounds {}
impl<T: Clone + Default + Bounds> NumCommon for T {}

/// Has total order, has field operations.
pub trait TofieldBundle: Ord + FieldBundle {}
impl<T: Ord + FieldBundle> TofieldBundle for T {}

/// Has total order, has field operations, has other common num traits.
pub trait TofieldBundleNum: NumCommon + TofieldBundle {}
impl<T: NumCommon + TofieldBundle> TofieldBundleNum for T {}

/// Has partial order, has field operations.
pub trait PofieldBundle: PartialOrd + FieldBundle {}
impl<T: PartialOrd + FieldBundle> PofieldBundle for T {}

/// Has partial order, has field operations, has other common num traits.
pub trait PofieldBundleNum: NumCommon + PofieldBundle {}
impl<T: NumCommon + PofieldBundle> PofieldBundleNum for T {}

/// Has field operations.
pub trait FieldBundle: Zero + One + FieldOps + Sized {}
impl<T: Zero + One + FieldOps + Sized> FieldBundle for T {}

/// Has field operations, has other common num traits.
pub trait FieldBundleNum: NumCommon + FieldBundle {}
impl<T: NumCommon + TofieldBundle> FieldBundleNum for T {}

/// Has total order, has ring operations.
pub trait ToringBundle: Ord + RingBundle {}
impl<T: Ord + RingBundle> ToringBundle for T {}

/// Has total order, has ring operations, has other common num traits.
pub trait ToringBundleNum: NumCommon + ToringBundle {}
impl<T: NumCommon + ToringBundle> ToringBundleNum for T {}

/// Has partial order, has ring operations.
pub trait PoringBundle: PartialOrd + RingBundle {}
impl<T: PartialOrd + RingBundle> PoringBundle for T {}

/// Has partial order, has ring operations, has other common num traits.
pub trait PoringBundleNum: NumCommon + PoringBundle {}
impl<T: NumCommon + PoringBundle> PoringBundleNum for T {}

/// Has ring operations.
pub trait RingBundle: Zero + One + RingOps + Sized {}
impl<T: Zero + One + RingOps + Sized> RingBundle for T {}

/// Has ring operations, has other common num traits.
pub trait RingBundleNum: NumCommon + RingBundle {}
impl<T: NumCommon + PoringBundle> RingBundleNum for T {}
