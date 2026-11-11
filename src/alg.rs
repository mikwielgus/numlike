// SPDX-FileCopyrightText: 2026 numlike contributors
//
// SPDX-License-Identifier: MIT OR Apache-2.0

//! Trait bundles for common algebraic structures.

use core::fmt::Debug;

use crate::{
    elem::{One, Zero},
    limits::Bounds,
    ops::{FieldOps, RingOps},
};

/// Supertraits shared by all `*Num` traits.
pub trait NumCommon: Clone + Debug + Default + Bounds {}
impl<T: Clone + Debug + Default + Bounds> NumCommon for T {}

/// Has total order, has field operations.
///
/// Having field operations alone doesn't imply being a field -- it may as well
/// be a meadow or skewfield, or something else entirely. This is why there's
/// "bundle" in this trait's name.
pub trait TofieldBundle: Ord + FieldBundle {}
impl<T: Ord + FieldBundle> TofieldBundle for T {}

/// Has total order, has field operations, has common sensible num traits.
///
/// Having field operations alone doesn't imply being a field -- it may as well
/// be a meadow or skewfield, or something else entirely. This is why there's
/// "bundle" in this trait's name.
pub trait TofieldBundleNum: NumCommon + TofieldBundle {}
impl<T: NumCommon + TofieldBundle> TofieldBundleNum for T {}

/// Has partial order, has field operations.
///
/// Having field operations alone doesn't imply being a field -- it may as well
/// be a meadow or skewfield, or something else entirely. This is why there's
/// "bundle" in this trait's name.
pub trait PofieldBundle: PartialOrd + FieldBundle {}
impl<T: PartialOrd + FieldBundle> PofieldBundle for T {}

/// Has partial order, has field operations, has common sensible num traits.
///
/// Having field operations alone doesn't imply being a field -- it may as well
/// be a meadow or skewfield, or something else entirely. This is why there's
/// "bundle" in this trait's name.
pub trait PofieldBundleNum: NumCommon + PofieldBundle {}
impl<T: NumCommon + PofieldBundle> PofieldBundleNum for T {}

/// Has field operations.
///
/// Having field operations alone doesn't imply being a field -- it may as well
/// be a meadow or skewfield, or something else entirely. This is why there's
/// "bundle" in this trait's name.
pub trait FieldBundle: Zero + One + FieldOps + Sized {}
impl<T: Zero + One + FieldOps + Sized> FieldBundle for T {}

/// Has field operations, has common sensible num traits.
///
/// Having field operations alone doesn't imply being a field -- it may as well
/// be a meadow or skewfield, or something else entirely. This is why there's
/// "bundle" in this trait's name.
pub trait FieldBundleNum: NumCommon + FieldBundle {}
impl<T: NumCommon + TofieldBundle> FieldBundleNum for T {}

/// Has total order, has ring operations.
///
/// Having ring operations alone doesn't imply being a ring -- it may as well
/// be a near-ring, semiring, hemiring, or something else entirely. This is why
/// there's "bundle" in this trait's name.
pub trait ToringBundle: Ord + RingBundle {}
impl<T: Ord + RingBundle> ToringBundle for T {}

/// Has total order, has ring operations, has common sensible num traits.
///
/// Having ring operations alone doesn't imply being a ring -- it may as well
/// be a near-ring, semiring, hemiring, or something else entirely. This is why
/// there's "bundle" in this trait's name.
pub trait ToringBundleNum: NumCommon + ToringBundle {}
impl<T: NumCommon + ToringBundle> ToringBundleNum for T {}

/// Has partial order, has ring operations.
///
/// Having ring operations alone doesn't imply being a ring -- it may as well
/// be a near-ring, semiring, hemiring, or something else entirely. This is why
/// there's "bundle" in this trait's name.
pub trait PoringBundle: PartialOrd + RingBundle {}
impl<T: PartialOrd + RingBundle> PoringBundle for T {}

/// Has partial order, has ring operations, has common sensible num traits.
///
/// Having ring operations alone doesn't imply being a ring -- it may as well
/// be a near-ring, semiring, hemiring, or something else entirely. This is why
/// there's "bundle" in this trait's name.
pub trait PoringBundleNum: NumCommon + PoringBundle {}
impl<T: NumCommon + PoringBundle> PoringBundleNum for T {}

/// Has ring operations.
///
/// Having ring operations alone doesn't imply being a ring -- it may as well
/// be a near-ring, semiring, hemiring, or something else entirely. This is why
/// there's "bundle" in this trait's name.
pub trait RingBundle: Zero + One + RingOps + Sized {}
impl<T: Zero + One + RingOps + Sized> RingBundle for T {}

/// Has ring operations, has common sensible num traits.
///
/// Having ring operations alone doesn't imply being a ring -- it may as well
/// be a near-ring, semiring, hemiring, or something else entirely. This is why
/// there's "bundle" in this trait's name.
pub trait RingBundleNum: NumCommon + RingBundle {}
impl<T: NumCommon + PoringBundle> RingBundleNum for T {}
