// SPDX-FileCopyrightText: 2026 numlike contributors
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use core::ops::{
    Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Rem, RemAssign, Sub, SubAssign,
};

/// Bundle of arithmetic and fused arithmetic operations, including assign variants.
pub trait FullArithOps<Rhs = Self>:
    ArithOps<Rhs> + ArithAssignOps<Rhs> + FusedArithOps<Rhs> + FusedArithAssignOps<Rhs>
{
}

/// Bundle of fused arithmetic operations.
pub trait FusedArithOps<Rhs = Self>: MulAdd<Rhs, Rhs> {}
impl<Rhs, T: MulAdd<Rhs, Rhs>> FusedArithOps<Rhs> for T {}

/// Fused multiply-add. Computes `(self * a) + b` with only one rounding
/// error, yielding a more accurate result than an unfused multiply-add.
///
/// Using `mul_add` *may* be more performant than an unfused multiply-add if
/// the target architecture has a dedicated `fma` CPU instruction. However,
/// this is not always true, and will be heavily dependant on designing
/// algorithms with specific target hardware in mind.
pub trait MulAdd<A = Self, B = Self> {
    /// The resulting type after applying the operation.
    type Output;

    /// Fused multiply-add. Computes `(self * a) + b` with only one rounding
    /// error, yielding a more accurate result than an unfused multiply-add.
    fn mul_add(self, a: A, b: B) -> Self::Output;
}

/// Bundle of fused arithmetic assignment operations.
pub trait FusedArithAssignOps<Rhs = Self>: MulAddAssign<Rhs, Rhs> {}
impl<Rhs, T: MulAddAssign<Rhs, Rhs>> FusedArithAssignOps<Rhs> for T {}

/// Fused multiply-add assignment. Computes `*self = (*self * a) + b` with
/// only one rounding error, yielding a more accurate result than an unfused
/// multiply-add.
pub trait MulAddAssign<A = Self, B = Self> {
    /// Fused multiply-add assignment. Computes `*self = (*self * a) + b` with
    /// only one rounding error, yielding a more accurate result than an unfused
    /// multiply-add.
    fn mul_add_assign(&mut self, a: A, b: B);
}

/// Bundle of arithmetic operations.
pub trait ArithOps<Rhs = Self>:
    EuclidOps<Rhs> + FieldOps<Rhs> + Rem<Rhs, Output = Self> + DivRem<Rhs, Output = Self>
{
}
impl<Rhs, T: EuclidOps<Rhs> + FieldOps<Rhs> + Rem<Rhs, Output = Self> + DivRem<Rhs, Output = Self>>
    ArithOps<Rhs> for T
{
}

/// Simultaneously computes the quotient and remainder of `self` divided by
/// `other`.
///
/// Returns `(self / other, self % other)`.
pub trait DivRem<Rhs> {
    /// The resulting type after applying the operation.
    type Output;

    /// Simultaneously computes the quotient and remainder of `self` divided by
    /// `other`.
    fn div_rem(self, other: Rhs) -> (Self::Output, Self::Output);
}

/// Bundle of Euclidean division operations.
pub trait EuclidOps<Rhs>: DivEuclid<Rhs> + RemEuclid<Rhs> + DivRemEuclid<Rhs> {}
impl<Rhs, T: DivEuclid<Rhs> + RemEuclid<Rhs> + DivRemEuclid<Rhs>> EuclidOps<Rhs> for T {}

/// Calculates the quotient of Euclidean division of `self` by `other`.
///
/// This computes the integer `q` such that `self = q * other + r`, with
/// `r = self.rem_euclid(other)` and `0 <= r < abs(other)`.
///
/// In other words, the result is `self / other` rounded to the integer `q`
/// such that `self >= q * other`.
/// If `self > 0`, this is equal to rounding towards zero (the default in Rust);
/// if `self < 0`, this is equal to rounding away from zero (towards +/- infinity).
/// If `other > 0`, this is equal to rounding towards -infinity;
/// if `other < 0`, this is equal to rounding towards +infinity.
///
/// # Panics
///
/// This function will panic if `other` is zero or if `self` is `Self::MIN`
/// and `other` is -1. This behavior is not affected by the `overflow-checks` flag.
pub trait DivEuclid<Rhs = Self> {
    /// The resulting type after applying the operation.
    type Output;

    /// Calculates the quotient of Euclidean division of `self` by `other`.
    fn div_euclid(self, other: Rhs) -> Self::Output;
}

/// Calculates the quotient of Euclidean division of `self` by `other`,
/// writing the result back to `self`.
///
/// See [`DivEuclid::div_euclid`] for details.
pub trait DivEuclidAssign<Rhs = Self> {
    /// Calculates the quotient of Euclidean division of `self` by `other`,
    /// writing the result back to `self`.
    fn div_euclid_assign(&mut self, other: Rhs);
}

/// Calculates the least nonnegative remainder of `self` when
/// divided by `other`.
///
/// This is done as if by the Euclidean division algorithm -- given
/// `r = self.rem_euclid(other)`, the result satisfies
/// `self = other * self.div_euclid(other) + r` and `0 <= r < abs(other)`.
///
/// # Panics
///
/// This function will panic if `other` is zero or if `self` is `Self::MIN`
/// and `other` is -1. This behavior is not affected by the `overflow-checks` flag.
pub trait RemEuclid<Rhs = Self> {
    /// The resulting type after applying the operation.
    type Output;

    /// Calculates the least nonnegative remainder of `self` when
    /// divided by `other`.
    fn rem_euclid(self, other: Rhs) -> Self::Output;
}

/// Calculates the least nonnegative remainder of `self` when divided by
/// `other`, writing the result back to `self`.
///
/// See [`RemEuclid::rem_euclid`] for details.
pub trait RemEuclidAssign<Rhs = Self> {
    /// Calculates the least nonnegative remainder of `self` when divided by
    /// `other`, writing the result back to `self`.
    fn rem_euclid_assign(&mut self, other: Rhs);
}

/// Simultaneously computes the Euclidean quotient and remainder of `self`
/// divided by `other`.
///
/// Returns `(self.div_euclid(other), self.rem_euclid(other))`.
pub trait DivRemEuclid<Rhs = Self> {
    /// The resulting type after applying the operation.
    type Output;

    /// Simultaneously computes the Euclidean quotient and remainder of `self`
    /// divided by `other`.
    fn div_rem_euclid(self, other: Rhs) -> (Self::Output, Self::Output);
}

/// Bundle of arithmetic assignment operations.
pub trait ArithAssignOps<Rhs = Self>: FieldAssignOps<Rhs> + RemAssign<Rhs> {}
impl<Rhs, T: FieldAssignOps<Rhs> + RemAssign<Rhs>> ArithAssignOps<Rhs> for T {}

/// Bundle of field operations (ring operations plus division).
pub trait FieldOps<Rhs = Self>: RingOps<Rhs> + Div<Rhs, Output = Self> {}
impl<Rhs, T: RingOps<Rhs> + Div<Rhs, Output = Self>> FieldOps<Rhs> for T {}

/// Bundle of field assignment operations.
pub trait FieldAssignOps<Rhs = Self>: RingAssignOps<Rhs> + DivAssign<Rhs> {}
impl<Rhs, T: RingAssignOps<Rhs> + DivAssign<Rhs>> FieldAssignOps<Rhs> for T {}

/// Bundle of ring operations (addition, subtraction, multiplication, and negation).
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

/// Bundle of ring assignment operations.
pub trait RingAssignOps<Rhs = Self>:
    AddAssign<Rhs> + SubAssign<Rhs> + MulAssign<Rhs> + NegAssign
{
}
impl<Rhs, T: AddAssign<Rhs> + SubAssign<Rhs> + MulAssign<Rhs> + NegAssign> RingAssignOps<Rhs>
    for T
{
}

/// Negates `self` in place.
pub trait NegAssign {
    /// Negates `self` in place.
    fn neg_assign(&mut self);
}

macro_rules! impl_euclid_traits {
    ($($ty:ty),*) => {
        $(
            impl DivEuclid<$ty> for $ty {
                type Output = $ty;

                #[inline]
                fn div_euclid(self, other: $ty) -> Self::Output {
                    <$ty>::div_euclid(self, other)
                }
            }

            impl DivEuclidAssign<$ty> for $ty {
                #[inline]
                fn div_euclid_assign(&mut self, other: $ty) {
                    *self = <$ty>::div_euclid(*self, other);
                }
            }

            impl RemEuclid<$ty> for $ty {
                type Output = $ty;

                #[inline]
                fn rem_euclid(self, other: $ty) -> Self::Output {
                    <$ty>::rem_euclid(self, other)
                }
            }

            impl RemEuclidAssign<$ty> for $ty {
                #[inline]
                fn rem_euclid_assign(&mut self, other: $ty) {
                    *self = <$ty>::rem_euclid(*self, other);
                }
            }

            impl DivRemEuclid<$ty> for $ty {
                type Output = $ty;

                #[inline]
                fn div_rem_euclid(self, other: $ty) -> (Self::Output, Self::Output) {
                    (
                        <$ty>::div_euclid(self, other),
                        <$ty>::rem_euclid(self, other),
                    )
                }
            }
        )*
    };
}

macro_rules! impl_div_rem_trait {
    ($($ty:ty),*) => {
        $(
            impl DivRem<$ty> for $ty {
                type Output = $ty;

                #[inline]
                fn div_rem(self, other: $ty) -> (Self::Output, Self::Output) {
                    (
                        <$ty>::div(self, other),
                        <$ty>::rem(self, other),
                    )
                }
            }
        )*
    };
}

macro_rules! impl_mul_add_trait {
    ($($ty:ty),*) => {
        $(
            impl MulAdd<$ty, $ty> for $ty {
                type Output = $ty;

                #[inline]
                fn mul_add(self, a: $ty, b: $ty) -> Self::Output {
                    (self * a) + b
                }
            }

            impl MulAddAssign<$ty, $ty> for $ty {
                #[inline]
                fn mul_add_assign(&mut self, a: $ty, b: $ty) {
                    *self = MulAdd::mul_add(*self, a, b);
                }
            }
        )*
    };
}

macro_rules! impl_neg_assign_trait {
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

macro_rules! impl_plain_arith_traits_for_signed_ints {
    ($($ty:ty),*) => {
        impl_div_rem_trait!($($ty),*);
        impl_euclid_traits!($($ty),*);
        impl_mul_add_trait!($($ty),*);
        impl_neg_assign_trait!($($ty),*);
    };
}

macro_rules! impl_plain_arith_traits_for_unsigned_ints {
    ($($ty:ty),*) => {
        impl_div_rem_trait!($($ty),*);
        impl_euclid_traits!($($ty),*);
        impl_mul_add_trait!($($ty),*);
    };
}

macro_rules! impl_plain_arith_traits_for_floats {
    ($($ty:ty),*) => {
        impl_div_rem_trait!($($ty),*);
        #[cfg(feature = "std")]
        impl_euclid_traits!($($ty),*);
        impl_neg_assign_trait!($($ty),*);
    };
}

impl_plain_arith_traits_for_signed_ints!(i8, i16, i32, i64, i128, isize);
impl_plain_arith_traits_for_unsigned_ints!(u8, u16, u32, u64, u128, usize);
impl_plain_arith_traits_for_floats!(f32, f64);

#[cfg(feature = "std")]
macro_rules! impl_std_mul_add_for_floats {
    ($($ty:ty),*) => {
        $(
            impl MulAdd<$ty, $ty> for $ty {
                type Output = $ty;

                #[inline]
                fn mul_add(self, a: $ty, b: $ty) -> Self::Output {
                    <$ty>::mul_add(self, a, b)
                }
            }

            impl MulAddAssign<$ty, $ty> for $ty {
                #[inline]
                fn mul_add_assign(&mut self, a: $ty, b: $ty) {
                    *self = MulAdd::mul_add(*self, a, b);
                }
            }
        )*
    };
}

#[cfg(feature = "std")]
impl_std_mul_add_for_floats!(f32, f64);

#[cfg(all(not(feature = "std"), feature = "libm"))]
macro_rules! impl_libm_mul_add_for_float {
    ($ty:ty, $fma:path) => {
        impl MulAdd<$ty, $ty> for $ty {
            type Output = $ty;

            #[inline]
            fn mul_add(self, a: $ty, b: $ty) -> Self::Output {
                $fma(self, a, b)
            }
        }

        impl MulAddAssign<$ty, $ty> for $ty {
            #[inline]
            fn mul_add_assign(&mut self, a: $ty, b: $ty) {
                *self = MulAdd::mul_add(*self, a, b);
            }
        }
    };
}

#[cfg(all(not(feature = "std"), feature = "libm"))]
impl_libm_mul_add_for_float!(f32, libm::fmaf);
#[cfg(all(not(feature = "std"), feature = "libm"))]
impl_libm_mul_add_for_float!(f64, libm::fma);

#[cfg(all(not(feature = "std"), not(feature = "libm")))]
macro_rules! impl_unfused_mul_add_for_floats {
    ($($ty:ty),*) => {
        $(
            impl MulAdd<$ty, $ty> for $ty {
                type Output = $ty;

                #[inline]
                fn mul_add(self, a: $ty, b: $ty) -> Self::Output {
                    (self * a) + b
                }
            }

            impl MulAddAssign<$ty, $ty> for $ty {
                #[inline]
                fn mul_add_assign(&mut self, a: $ty, b: $ty) {
                    *self = MulAdd::mul_add(*self, a, b);
                }
            }
        )*
    };
}

#[cfg(all(not(feature = "std"), not(feature = "libm")))]
impl_unfused_mul_add_for_floats!(f32, f64);
