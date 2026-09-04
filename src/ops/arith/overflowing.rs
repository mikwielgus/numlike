// SPDX-FileCopyrightText: 2026 numlike contributors
//
// SPDX-License-Identifier: MIT OR Apache-2.0

/// Bundle of overflowing arithmetic and fused arithmetic operations.
pub trait FullOverflowingArithOps<Rhs = Self>:
    OverflowingArithOps<Rhs> + OverflowingFusedArithOps<Rhs>
{
}

/// Bundle of overflowing fused arithmetic operations.
pub trait OverflowingFusedArithOps<Rhs = Self>: OverflowingMulAdd<Rhs, Rhs> {}
impl<Rhs, T: OverflowingMulAdd<Rhs, Rhs>> OverflowingFusedArithOps<Rhs> for T {}

/// Overflowing fused multiply-add. Computes `(self * a) + b`.
///
/// Returns a tuple of the result along with a boolean indicating whether an
/// arithmetic overflow would occur. If an overflow would have occurred then
/// the wrapped value is returned.
pub trait OverflowingMulAdd<A = Self, B = Self> {
    /// The resulting type after applying the operation.
    type Output;

    /// Overflowing fused multiply-add. Computes `(self * a) + b`.
    fn overflowing_mul_add(self, a: A, b: B) -> (Self::Output, bool);
}

/// Bundle of overflowing arithmetic operations.
pub trait OverflowingArithOps<Rhs = Self>:
    OverflowingEuclidOps<Rhs>
    + OverflowingFieldOps<Rhs>
    + OverflowingRem<Rhs, Output = Self>
    + OverflowingDivRem<Rhs, Output = Self>
{
}
impl<
    Rhs,
    T: OverflowingEuclidOps<Rhs>
        + OverflowingFieldOps<Rhs>
        + OverflowingRem<Rhs, Output = Self>
        + OverflowingDivRem<Rhs, Output = Self>,
> OverflowingArithOps<Rhs> for T
{
}

/// Calculates the remainder when `self` is divided by `other`.
///
/// Returns a tuple of the remainder after dividing along with a boolean indicating whether an
/// arithmetic overflow would occur. If an overflow would occur then zero is returned.
///
/// # Panics
///
/// This function will panic if `other` is zero.
pub trait OverflowingRem<Rhs = Self> {
    /// The resulting type after applying the operation.
    type Output;

    /// Calculates the remainder when `self` is divided by `other`.
    fn overflowing_rem(self, other: Rhs) -> (Self::Output, bool);
}

/// Calculates the quotient and remainder when `self` is divided by `other`.
///
/// Returns a tuple of `(quotient, remainder)` along with a boolean indicating whether an
/// arithmetic overflow would occur.
///
/// # Panics
///
/// This function will panic if `other` is zero.
pub trait OverflowingDivRem<Rhs> {
    /// The resulting type after applying the operation.
    type Output;

    /// Calculates the quotient and remainder when `self` is divided by `other`.
    fn overflowing_div_rem(self, other: Rhs) -> ((Self::Output, Self::Output), bool);
}

/// Bundle of overflowing Euclidean division operations.
pub trait OverflowingEuclidOps<Rhs>:
    OverflowingDivEuclid<Rhs> + OverflowingRemEuclid<Rhs> + OverflowingDivRemEuclid<Rhs>
{
}
impl<Rhs, T: OverflowingDivEuclid<Rhs> + OverflowingRemEuclid<Rhs> + OverflowingDivRemEuclid<Rhs>>
    OverflowingEuclidOps<Rhs> for T
{
}

/// Calculates the quotient of Euclidean division `self.div_euclid(other)`.
///
/// Returns a tuple of the divisor along with a boolean indicating whether an arithmetic overflow would
/// occur. If an overflow would occur then `self` is returned.
///
/// # Panics
///
/// This function will panic if `other` is zero.
pub trait OverflowingDivEuclid<Rhs = Self> {
    /// The resulting type after applying the operation.
    type Output;

    /// Calculates the quotient of Euclidean division `self.div_euclid(other)`.
    fn overflowing_div_euclid(self, other: Rhs) -> (Self::Output, bool);
}

/// Overflowing Euclidean remainder. Calculates `self.rem_euclid(other)`.
///
/// Returns a tuple of the remainder after dividing along with a boolean indicating whether an
/// arithmetic overflow would occur. If an overflow would occur then 0 is returned.
///
/// # Panics
///
/// This function will panic if `other` is zero.
pub trait OverflowingRemEuclid<Rhs = Self> {
    /// The resulting type after applying the operation.
    type Output;

    /// Overflowing Euclidean remainder. Calculates `self.rem_euclid(other)`.
    fn overflowing_rem_euclid(self, other: Rhs) -> (Self::Output, bool);
}

/// Calculates the Euclidean quotient and remainder when `self` is divided by `other`.
///
/// Returns a tuple of `(quotient, remainder)` along with a boolean indicating whether an
/// arithmetic overflow would occur.
///
/// # Panics
///
/// This function will panic if `other` is zero.
pub trait OverflowingDivRemEuclid<Rhs = Self> {
    /// The resulting type after applying the operation.
    type Output;

    /// Calculates the Euclidean quotient and remainder when `self` is divided by `other`.
    fn overflowing_div_rem_euclid(self, other: Rhs) -> ((Self::Output, Self::Output), bool);
}

/// Bundle of overflowing field operations.
pub trait OverflowingFieldOps<Rhs = Self>:
    OverflowingRingOps<Rhs> + OverflowingDiv<Rhs, Output = Self>
{
}
impl<Rhs, T: OverflowingRingOps<Rhs> + OverflowingDiv<Rhs, Output = Self>> OverflowingFieldOps<Rhs>
    for T
{
}

/// Calculates the divisor when `self` is divided by `other`.
///
/// Returns a tuple of the divisor along with a boolean indicating whether an arithmetic overflow would
/// occur. If an overflow would occur then self is returned.
///
/// # Panics
///
/// This function will panic if `other` is zero.
pub trait OverflowingDiv<Rhs = Self> {
    /// The resulting type after applying the operation.
    type Output;

    /// Calculates the divisor when `self` is divided by `other`.
    fn overflowing_div(self, other: Rhs) -> (Self::Output, bool);
}

/// Bundle of overflowing ring operations.
pub trait OverflowingRingOps<Rhs = Self>:
    OverflowingAdd<Rhs, Output = Self>
    + OverflowingSub<Rhs, Output = Self>
    + OverflowingMul<Rhs, Output = Self>
    + OverflowingNeg<Output = Self>
{
}
impl<
    Rhs,
    T: OverflowingAdd<Rhs, Output = Self>
        + OverflowingSub<Rhs, Output = Self>
        + OverflowingMul<Rhs, Output = Self>
        + OverflowingNeg<Output = Self>,
> OverflowingRingOps<Rhs> for T
{
}

/// Returns a tuple of the addition along with a boolean indicating
/// whether an arithmetic overflow would occur. If an overflow would have
/// occurred then the wrapped value is returned.
pub trait OverflowingAdd<Rhs = Self> {
    /// The resulting type after applying the operation.
    type Output;

    /// Returns a tuple of the addition along with a boolean indicating
    /// whether an arithmetic overflow would occur. If an overflow would have
    /// occurred then the wrapped value is returned.
    fn overflowing_add(self, other: Rhs) -> (Self::Output, bool);
}

/// Returns a tuple of the subtraction along with a boolean indicating whether an arithmetic overflow
/// would occur. If an overflow would have occurred then the wrapped value is returned.
pub trait OverflowingSub<Rhs = Self> {
    /// The resulting type after applying the operation.
    type Output;

    /// Returns a tuple of the subtraction along with a boolean indicating whether an arithmetic overflow
    /// would occur. If an overflow would have occurred then the wrapped value is returned.
    fn overflowing_sub(self, other: Rhs) -> (Self::Output, bool);
}

/// Calculates the multiplication of `self` and `other`.
///
/// Returns a tuple of the multiplication along with a boolean indicating whether an arithmetic overflow
/// would occur. If an overflow would have occurred then the wrapped value is returned.
pub trait OverflowingMul<Rhs = Self> {
    /// The resulting type after applying the operation.
    type Output;

    /// Calculates the multiplication of `self` and `other`.
    fn overflowing_mul(self, other: Rhs) -> (Self::Output, bool);
}

/// Returns a tuple of the negated version of self along with a boolean indicating whether an overflow
/// happened. If `self` is the minimum value, then the minimum value will be returned again and `true`
/// will be returned for an overflow happening.
pub trait OverflowingNeg {
    /// The resulting type after applying the operation.
    type Output;

    /// Returns a tuple of the negated version of self along with a boolean indicating whether an overflow
    /// happened. If `self` is the minimum value, then the minimum value will be returned again and `true`
    /// will be returned for an overflow happening.
    fn overflowing_neg(self) -> (Self::Output, bool);
}

macro_rules! impl_overflowing_ring_field_traits_for_ints {
    ($($ty:ty),*) => {
        $(
            impl OverflowingAdd<$ty> for $ty {
                type Output = $ty;

                #[inline]
                fn overflowing_add(self, other: $ty) -> (Self::Output, bool) {
                    <$ty>::overflowing_add(self, other)
                }
            }

            impl OverflowingSub<$ty> for $ty {
                type Output = $ty;

                #[inline]
                fn overflowing_sub(self, other: $ty) -> (Self::Output, bool) {
                    <$ty>::overflowing_sub(self, other)
                }
            }

            impl OverflowingMul<$ty> for $ty {
                type Output = $ty;

                #[inline]
                fn overflowing_mul(self, other: $ty) -> (Self::Output, bool) {
                    <$ty>::overflowing_mul(self, other)
                }
            }

            impl OverflowingDiv<$ty> for $ty {
                type Output = $ty;

                #[inline]
                fn overflowing_div(self, other: $ty) -> (Self::Output, bool) {
                    <$ty>::overflowing_div(self, other)
                }
            }

            impl OverflowingRem<$ty> for $ty {
                type Output = $ty;

                #[inline]
                fn overflowing_rem(self, other: $ty) -> (Self::Output, bool) {
                    <$ty>::overflowing_rem(self, other)
                }
            }

            impl OverflowingNeg for $ty {
                type Output = $ty;

                #[inline]
                fn overflowing_neg(self) -> (Self::Output, bool) {
                    <$ty>::overflowing_neg(self)
                }
            }
        )*
    };
}

macro_rules! impl_overflowing_euclid_traits {
    ($($ty:ty),*) => {
        $(
            impl OverflowingDivEuclid<$ty> for $ty {
                type Output = $ty;

                #[inline]
                fn overflowing_div_euclid(self, other: $ty) -> (Self::Output, bool) {
                    <$ty>::overflowing_div_euclid(self, other)
                }
            }

            impl OverflowingRemEuclid<$ty> for $ty {
                type Output = $ty;

                #[inline]
                fn overflowing_rem_euclid(self, other: $ty) -> (Self::Output, bool) {
                    <$ty>::overflowing_rem_euclid(self, other)
                }
            }

            impl OverflowingDivRemEuclid<$ty> for $ty {
                type Output = $ty;

                #[inline]
                fn overflowing_div_rem_euclid(self, other: $ty) -> ((Self::Output, Self::Output), bool) {
                    let (div, div_overflow) = <$ty>::overflowing_div_euclid(self, other);
                    let (rem, rem_overflow) = <$ty>::overflowing_rem_euclid(self, other);

                    ((div, rem), div_overflow | rem_overflow)
                }
            }
        )*
    };
}

macro_rules! impl_overflowing_div_rem_trait {
    ($($ty:ty),*) => {
        $(
            impl OverflowingDivRem<$ty> for $ty {
                type Output = $ty;

                #[inline]
                fn overflowing_div_rem(self, other: $ty) -> ((Self::Output, Self::Output), bool) {
                    let (div, div_overflow) = <$ty>::overflowing_div(self, other);
                    let (rem, rem_overflow) = <$ty>::overflowing_rem(self, other);

                    ((div, rem), div_overflow | rem_overflow)
                }
            }
        )*
    };
}

macro_rules! impl_overflowing_mul_add_trait {
    ($($ty:ty),*) => {
        $(
            impl OverflowingMulAdd<$ty, $ty> for $ty {
                type Output = $ty;

                #[inline]
                fn overflowing_mul_add(self, a: $ty, b: $ty) -> (Self::Output, bool) {
                    let (product, mul_overflow) = <$ty>::overflowing_mul(self, a);
                    let (result, add_overflow) = <$ty>::overflowing_add(product, b);

                    (result, mul_overflow | add_overflow)
                }
            }
        )*
    };
}

macro_rules! impl_overflowing_arith_traits_for_signed_ints {
    ($($ty:ty),*) => {
        impl_overflowing_ring_field_traits_for_ints!($($ty),*);
        impl_overflowing_div_rem_trait!($($ty),*);
        impl_overflowing_euclid_traits!($($ty),*);
        impl_overflowing_mul_add_trait!($($ty),*);
    };
}

macro_rules! impl_overflowing_arith_traits_for_unsigned_ints {
    ($($ty:ty),*) => {
        impl_overflowing_ring_field_traits_for_ints!($($ty),*);
        impl_overflowing_div_rem_trait!($($ty),*);
        impl_overflowing_euclid_traits!($($ty),*);
        impl_overflowing_mul_add_trait!($($ty),*);
    };
}

/*macro_rules! impl_overflowing_ring_field_traits_for_floats {
    ($($ty:ty),*) => {
        $(
            impl OverflowingAdd<$ty> for $ty {
                type Output = $ty;

                #[inline]
                fn overflowing_add(self, other: $ty) -> (Self::Output, bool) {
                    (self + other, false)
                }
            }

            impl OverflowingSub<$ty> for $ty {
                type Output = $ty;

                #[inline]
                fn overflowing_sub(self, other: $ty) -> (Self::Output, bool) {
                    (self - other, false)
                }
            }

            impl OverflowingMul<$ty> for $ty {
                type Output = $ty;

                #[inline]
                fn overflowing_mul(self, other: $ty) -> (Self::Output, bool) {
                    (self * other, false)
                }
            }

            impl OverflowingDiv<$ty> for $ty {
                type Output = $ty;

                #[inline]
                fn overflowing_div(self, other: $ty) -> (Self::Output, bool) {
                    (self / other, false)
                }
            }

            impl OverflowingRem<$ty> for $ty {
                type Output = $ty;

                #[inline]
                fn overflowing_rem(self, other: $ty) -> (Self::Output, bool) {
                    (self % other, false)
                }
            }

            impl OverflowingNeg for $ty {
                type Output = $ty;

                #[inline]
                fn overflowing_neg(self) -> (Self::Output, bool) {
                    (-self, false)
                }
            }
        )*
    };
}

macro_rules! impl_overflowing_div_rem_trait_for_floats {
    ($($ty:ty),*) => {
        $(
            impl OverflowingDivRem<$ty> for $ty {
                type Output = $ty;

                #[inline]
                fn overflowing_div_rem(self, other: $ty) -> ((Self::Output, Self::Output), bool) {
                    ((self / other, self % other), false)
                }
            }
        )*
    };
}*/

/*#[cfg(feature = "std")]
macro_rules! impl_overflowing_euclid_traits_for_floats {
    ($($ty:ty),*) => {
        $(
            impl OverflowingDivEuclid<$ty> for $ty {
                type Output = $ty;

                #[inline]
                fn overflowing_div_euclid(self, other: $ty) -> (Self::Output, bool) {
                    (<$ty>::div_euclid(self, other), false)
                }
            }

            impl OverflowingRemEuclid<$ty> for $ty {
                type Output = $ty;

                #[inline]
                fn overflowing_rem_euclid(self, other: $ty) -> (Self::Output, bool) {
                    (<$ty>::rem_euclid(self, other), false)
                }
            }

            impl OverflowingDivRemEuclid<$ty> for $ty {
                type Output = $ty;

                #[inline]
                fn overflowing_div_rem_euclid(self, other: $ty) -> ((Self::Output, Self::Output), bool) {
                    (
                        (<$ty>::div_euclid(self, other), <$ty>::rem_euclid(self, other)),
                        false,
                    )
                }
            }
        )*
    };
}*/

/*macro_rules! impl_overflowing_mul_add_trait_for_floats {
    ($($ty:ty),*) => {
        $(
            impl OverflowingMulAdd<$ty, $ty> for $ty {
                type Output = $ty;

                #[inline]
                fn overflowing_mul_add(self, a: $ty, b: $ty) -> (Self::Output, bool) {
                    (MulAdd::mul_add(self, a, b), false)
                }
            }
        )*
    };
}*/

/*macro_rules! impl_overflowing_arith_traits_for_floats {
    ($($ty:ty),*) => {
        impl_overflowing_ring_field_traits_for_floats!($($ty),*);
        impl_overflowing_div_rem_trait_for_floats!($($ty),*);
        #[cfg(feature = "std")]
        impl_overflowing_euclid_traits_for_floats!($($ty),*);
    };
}*/

impl_overflowing_arith_traits_for_signed_ints!(i8, i16, i32, i64, i128, isize);
impl_overflowing_arith_traits_for_unsigned_ints!(u8, u16, u32, u64, u128, usize);
//impl_overflowing_arith_traits_for_floats!(f32, f64);

//impl_overflowing_mul_add_trait_for_floats!(f32, f64);
