// SPDX-FileCopyrightText: 2026 numlike contributors
//
// SPDX-License-Identifier: MIT OR Apache-2.0

//! Trait bundles for common algebraic structures.

use core::fmt::Debug;

use crate::{
    elem::{One, Zero},
    ops::{AddGroupOps, FieldOps, RingOps},
};

/// Supertraits shared by all `*Kit` and `*Sensible` traits.
pub trait Sensible: Clone + Debug + Default + PartialEq {}
impl<T: Clone + Debug + Default + PartialEq> Sensible for T {}

/// Has total order, has field operations and identity elements.
///
/// Having field operations alone doesn't imply being a field -- it may as well
/// be a meadow or skewfield, or something else entirely. This is why there's
/// "bundle" in this trait's name.
pub trait TofieldOpsIds: Ord + FieldOpsIds {}
impl<T: Ord + FieldOpsIds> TofieldOpsIds for T {}

/// Has total order, has field operations and identity elements, has common
/// sensible num traits.
///
/// Having field operations alone doesn't imply being a field -- it may as well
/// be a meadow or skewfield, or something else entirely. This is why there's
/// "bundle" in this trait's name.
pub trait TofieldKit: Sensible + TofieldOpsIds {}
impl<T: Sensible + TofieldOpsIds> TofieldKit for T {}

/// Has partial order, has field operations and identity elements.
///
/// Having field operations alone doesn't imply being a field -- it may as well
/// be a meadow or skewfield, or something else entirely. This is why there's
/// "bundle" in this trait's name.
pub trait PofieldOpsIds: PartialOrd + FieldOpsIds {}
impl<T: PartialOrd + FieldOpsIds> PofieldOpsIds for T {}

/// Has partial order, has field operations and identity elements, has common
/// sensible num traits.
///
/// Having field operations alone doesn't imply being a field -- it may as well
/// be a meadow or skewfield, or something else entirely. This is why there's
/// "bundle" in this trait's name.
pub trait PofieldKit: Sensible + PofieldOpsIds {}
impl<T: Sensible + PofieldOpsIds> PofieldKit for T {}

/// Has field operations and identity elements.
///
/// Having field operations alone doesn't imply being a field -- it may as well
/// be a meadow or skewfield, or something else entirely. This is why there's
/// "bundle" in this trait's name.
pub trait FieldOpsIds: Zero + One + FieldOps + Sized {}
impl<T: Zero + One + FieldOps + Sized> FieldOpsIds for T {}

/// Has field operations and identity elements, has common sensible num traits.
///
/// Having field operations alone doesn't imply being a field -- it may as well
/// be a meadow or skewfield, or something else entirely. This is why there's
/// "bundle" in this trait's name.
pub trait FieldKit: Sensible + FieldOpsIds {}
impl<T: Sensible + TofieldOpsIds> FieldKit for T {}

/// Kit of field operations and identity elements and common sensible num traits.
pub trait FieldOpsSensible: Sensible + FieldOps {}
impl<T: Sensible + FieldOps> FieldOpsSensible for T {}

/// Has total order, has ring operations and identity elements.
///
/// Having ring operations alone doesn't imply being a ring -- it may as well
/// be a near-ring, semiring, hemiring, or something else entirely. This is why
/// there's "bundle" in this trait's name.
pub trait ToringOpsIds: Ord + RingOpsIds {}
impl<T: Ord + RingOpsIds> ToringOpsIds for T {}

/// Has total order, has ring operations and identity elements, has common
/// sensible num traits.
///
/// Having ring operations alone doesn't imply being a ring -- it may as well
/// be a near-ring, semiring, hemiring, or something else entirely. This is why
/// there's "bundle" in this trait's name.
pub trait ToringKit: Sensible + ToringOpsIds {}
impl<T: Sensible + ToringOpsIds> ToringKit for T {}

/// Has partial order, has ring operations and identity elements.
///
/// Having ring operations alone doesn't imply being a ring -- it may as well
/// be a near-ring, semiring, hemiring, or something else entirely. This is why
/// there's "bundle" in this trait's name.
pub trait PoringOpsIds: PartialOrd + RingOpsIds {}
impl<T: PartialOrd + RingOpsIds> PoringOpsIds for T {}

/// Has partial order, has ring operations and identity elements, has common
/// sensible num traits.
///
/// Having ring operations alone doesn't imply being a ring -- it may as well
/// be a near-ring, semiring, hemiring, or something else entirely. This is why
/// there's "bundle" in this trait's name.
pub trait PoringKit: Sensible + PoringOpsIds {}
impl<T: Sensible + PoringOpsIds> PoringKit for T {}

/// Has ring operations and identity elements.
///
/// Having ring operations alone doesn't imply being a ring -- it may as well
/// be a near-ring, semiring, hemiring, or something else entirely. This is why
/// there's "bundle" in this trait's name.
pub trait RingOpsIds: Zero + One + RingOps + Sized {}
impl<T: Zero + One + RingOps + Sized> RingOpsIds for T {}

/// Has ring operations and identity elements, has common sensible num traits.
///
/// Having ring operations alone doesn't imply being a ring -- it may as well
/// be a near-ring, semiring, hemiring, or something else entirely. This is why
/// there's "bundle" in this trait's name.
pub trait RingKit: Sensible + RingOpsIds {}
impl<T: Sensible + RingOpsIds> RingKit for T {}

/// Kit of ring operations and common sensible num traits.
pub trait RingOpsSensible: Sensible + RingOps {}
impl<T: Sensible + RingOps> RingOpsSensible for T {}

/// Kit of group operations and common sensible num traits.
pub trait AddGroupOpsSensible: Sensible + AddGroupOps {}
impl<T: Sensible + AddGroupOps> AddGroupOpsSensible for T {}
