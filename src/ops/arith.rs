// SPDX-FileCopyrightText: 2026 numlike contributors
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use core::ops::{Add, Div, Mul, Neg, Rem, Sub};

pub trait ArithmeticOps<Rhs = Self>:
    Add<Rhs, Output = Self>
    + Sub<Rhs, Output = Self>
    + Mul<Rhs, Output = Self>
    + Div<Rhs, Output = Self>
    + Neg<Output = Self>
    + Rem<Rhs, Output = Self>
{
}
impl<
    Rhs,
    T: Add<Rhs, Output = Self>
        + Sub<Rhs, Output = Self>
        + Mul<Rhs, Output = Self>
        + Div<Rhs, Output = Self>
        + Neg<Output = Self>
        + Rem<Rhs, Output = Self>,
> ArithmeticOps<Rhs> for T
{
}

pub trait RingOps<Rhs = Self>:
    Add<Rhs, Output = Self> + Sub<Rhs, Output = Self> + Mul<Rhs, Output = Self> + Neg<Output = Self>
{
}
impl<
    Rhs,
    T: Add<Rhs, Output = Self>
        + Sub<Rhs, Output = Self>
        + Mul<Rhs, Output = Self>
        + Neg<Output = Self>,
> RingOps<Rhs> for T
{
}

pub trait FieldOps<Rhs = Self>:
    Add<Rhs, Output = Self>
    + Sub<Rhs, Output = Self>
    + Mul<Rhs, Output = Self>
    + Div<Rhs, Output = Self>
    + Neg<Output = Self>
{
}
impl<
    Rhs,
    T: Add<Rhs, Output = Self>
        + Sub<Rhs, Output = Self>
        + Mul<Rhs, Output = Self>
        + Div<Rhs, Output = Self>
        + Neg<Output = Self>,
> FieldOps<Rhs> for T
{
}
