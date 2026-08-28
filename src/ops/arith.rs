// SPDX-FileCopyrightText: 2026 numlike contributors
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use core::ops::{
    Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Rem, RemAssign, Sub, SubAssign,
};

pub trait NegAssign {
    fn neg_assign(&mut self);
}

pub trait AllArithmeticOps<Rhs = Self>: ArithmeticOps<Rhs> + ArithmeticAssignOps<Rhs> {}

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

pub trait ArithmeticAssignOps<Rhs = Self>:
    AddAssign<Rhs> + SubAssign<Rhs> + MulAssign<Rhs> + DivAssign<Rhs> + NegAssign + RemAssign<Rhs>
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

pub trait RingAssignOps<Rhs = Self>:
    AddAssign<Rhs> + SubAssign<Rhs> + MulAssign<Rhs> + NegAssign
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

pub trait FieldAssignOps<Rhs = Self>:
    AddAssign<Rhs> + SubAssign<Rhs> + MulAssign<Rhs> + DivAssign<Rhs> + NegAssign
{
}
impl<Rhs, T: AddAssign<Rhs> + SubAssign<Rhs> + MulAssign<Rhs> + DivAssign<Rhs> + NegAssign>
    FieldAssignOps<Rhs> for T
{
}
