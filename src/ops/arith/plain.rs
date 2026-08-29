// SPDX-FileCopyrightText: 2026 numlike contributors
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use core::ops::{
    Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Rem, RemAssign, Sub, SubAssign,
};

pub trait NegAssign {
    fn neg_assign(&mut self);
}

pub trait FullArithmeticOps<Rhs = Self>: ArithmeticOps<Rhs> + ArithmeticAssignOps<Rhs> {}

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
impl<
    Rhs,
    T: AddAssign<Rhs> + SubAssign<Rhs> + MulAssign<Rhs> + DivAssign<Rhs> + NegAssign + RemAssign<Rhs>,
> ArithmeticAssignOps<Rhs> for T
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
impl<Rhs, T: AddAssign<Rhs> + SubAssign<Rhs> + MulAssign<Rhs> + NegAssign> RingAssignOps<Rhs>
    for T
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

macro_rules! impl_neg_assign_for_signed_types {
    ($($ty:ty),*) => {
        $(
            impl NegAssign for $ty {
                #[inline]
                fn neg_assign(&mut self) {
                    *self = Neg::neg(*self);
                }
            }
        )*
    };
}

/*macro_rules! impl_neg_assign_for_unsigned_types {
    ($($ty:ty),*) => {
        $(
            impl NegAssign for $ty {
                #[inline]
                fn neg_assign(&mut self) {
                    *self = (*self).wrapping_neg();
                }
            }
        )*
    };
}*/

impl_neg_assign_for_signed_types!(i8, i16, i32, i64, i128, isize);
//impl_neg_assign_for_unsigned_types!(u8, u16, u32, u64, u128, usize);
impl_neg_assign_for_signed_types!(f32, f64);
