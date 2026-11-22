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
pub trait NumCommon: Clone + Bounds + Debug + Default + PartialEq {}
impl<T: Clone + Bounds + Debug + Default + PartialEq> NumCommon for T {}

/// Has total order, has field operations.
///
/// Having field operations alone doesn't imply being a field -- it may as well
/// be a meadow or skewfield, or something else entirely. This is why there's
/// "bundle" in this trait's name.
pub trait TofieldKit: Ord + FieldKit {}
impl<T: Ord + FieldKit> TofieldKit for T {}

/// Has total order, has field operations, has common sensible num traits.
///
/// Having field operations alone doesn't imply being a field -- it may as well
/// be a meadow or skewfield, or something else entirely. This is why there's
/// "bundle" in this trait's name.
pub trait TofieldKitNum: NumCommon + TofieldKit {}
impl<T: NumCommon + TofieldKit> TofieldKitNum for T {}

/// Has partial order, has field operations.
///
/// Having field operations alone doesn't imply being a field -- it may as well
/// be a meadow or skewfield, or something else entirely. This is why there's
/// "bundle" in this trait's name.
pub trait PofieldKit: PartialOrd + FieldKit {}
impl<T: PartialOrd + FieldKit> PofieldKit for T {}

/// Has partial order, has field operations, has common sensible num traits.
///
/// Having field operations alone doesn't imply being a field -- it may as well
/// be a meadow or skewfield, or something else entirely. This is why there's
/// "bundle" in this trait's name.
pub trait PofieldKitNum: NumCommon + PofieldKit {}
impl<T: NumCommon + PofieldKit> PofieldKitNum for T {}

/// Has field operations.
///
/// Having field operations alone doesn't imply being a field -- it may as well
/// be a meadow or skewfield, or something else entirely. This is why there's
/// "bundle" in this trait's name.
pub trait FieldKit: Zero + One + FieldOps + Sized {}
impl<T: Zero + One + FieldOps + Sized> FieldKit for T {}

/// Has field operations, has common sensible num traits.
///
/// Having field operations alone doesn't imply being a field -- it may as well
/// be a meadow or skewfield, or something else entirely. This is why there's
/// "bundle" in this trait's name.
pub trait FieldKitNum: NumCommon + FieldKit {}
impl<T: NumCommon + TofieldKit> FieldKitNum for T {}

/// Set of field operations and common sensible num traits.
pub trait FieldOpsNum: NumCommon + FieldOps {}
impl<T: NumCommon + FieldOps> FieldOpsNum for T {}

/// Has total order, has ring operations.
///
/// Having ring operations alone doesn't imply being a ring -- it may as well
/// be a near-ring, semiring, hemiring, or something else entirely. This is why
/// there's "bundle" in this trait's name.
pub trait ToringKit: Ord + RingKit {}
impl<T: Ord + RingKit> ToringKit for T {}

/// Has total order, has ring operations, has common sensible num traits.
///
/// Having ring operations alone doesn't imply being a ring -- it may as well
/// be a near-ring, semiring, hemiring, or something else entirely. This is why
/// there's "bundle" in this trait's name.
pub trait ToringKitNum: NumCommon + ToringKit {}
impl<T: NumCommon + ToringKit> ToringKitNum for T {}

/// Has partial order, has ring operations.
///
/// Having ring operations alone doesn't imply being a ring -- it may as well
/// be a near-ring, semiring, hemiring, or something else entirely. This is why
/// there's "bundle" in this trait's name.
pub trait PoringKit: PartialOrd + RingKit {}
impl<T: PartialOrd + RingKit> PoringKit for T {}

/// Has partial order, has ring operations, has common sensible num traits.
///
/// Having ring operations alone doesn't imply being a ring -- it may as well
/// be a near-ring, semiring, hemiring, or something else entirely. This is why
/// there's "bundle" in this trait's name.
pub trait PoringKitNum: NumCommon + PoringKit {}
impl<T: NumCommon + PoringKit> PoringKitNum for T {}

/// Has ring operations.
///
/// Having ring operations alone doesn't imply being a ring -- it may as well
/// be a near-ring, semiring, hemiring, or something else entirely. This is why
/// there's "bundle" in this trait's name.
pub trait RingKit: Zero + One + RingOps + Sized {}
impl<T: Zero + One + RingOps + Sized> RingKit for T {}

/// Has ring operations, has common sensible num traits.
///
/// Having ring operations alone doesn't imply being a ring -- it may as well
/// be a near-ring, semiring, hemiring, or something else entirely. This is why
/// there's "bundle" in this trait's name.
pub trait RingKitNum: NumCommon + RingKit {}
impl<T: NumCommon + PoringKit> RingKitNum for T {}

/// Set of ring operations and common sensible num traits.
pub trait RingOpsNum: NumCommon + RingOps {}
impl<T: NumCommon + RingOps> RingOpsNum for T {}
