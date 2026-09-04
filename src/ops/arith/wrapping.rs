// SPDX-FileCopyrightText: 2026 numlike contributors
//
// SPDX-License-Identifier: MIT OR Apache-2.0

/// Bundle of wrapping arithmetic and fused arithmetic operations.
pub trait FullWrappingArithOps<Rhs = Self>:
    WrappingArithOps<Rhs> + WrappingFusedArithOps<Rhs>
{
}

/// Bundle of wrapping fused arithmetic operations.
pub trait WrappingFusedArithOps<Rhs = Self>: WrappingMulAdd<Rhs, Rhs> {}
impl<Rhs, T: WrappingMulAdd<Rhs, Rhs>> WrappingFusedArithOps<Rhs> for T {}

/// Wrapping (modular) fused multiply-add. Computes `(self * a) + b`,
/// wrapping around at the boundary of the type.
pub trait WrappingMulAdd<A = Self, B = Self> {
    /// The resulting type after applying the operation.
    type Output;

    /// Wrapping (modular) fused multiply-add. Computes `(self * a) + b`,
    /// wrapping around at the boundary of the type.
    fn wrapping_mul_add(self, a: A, b: B) -> Self::Output;
}

/// Bundle of wrapping arithmetic operations.
pub trait WrappingArithOps<Rhs = Self>:
    WrappingEuclidOps<Rhs>
    + WrappingFieldOps<Rhs>
    + WrappingRem<Rhs, Output = Self>
    + WrappingDivRem<Rhs, Output = Self>
{
}
impl<
    Rhs,
    T: WrappingEuclidOps<Rhs>
        + WrappingFieldOps<Rhs>
        + WrappingRem<Rhs, Output = Self>
        + WrappingDivRem<Rhs, Output = Self>,
> WrappingArithOps<Rhs> for T
{
}

/// Wrapping (modular) remainder. Computes `self % other`, wrapping around at the
/// boundary of the type.
///
/// Such wrap-around never actually occurs mathematically; implementation artifacts make `x % y`
/// invalid for `MIN / -1` on a signed type (where `MIN` is the negative minimal value). In such a case,
/// this function returns `0`.
///
/// # Panics
///
/// This function will panic if `other` is zero.
pub trait WrappingRem<Rhs = Self> {
    /// The resulting type after applying the operation.
    type Output;

    /// Wrapping (modular) remainder. Computes `self % other`, wrapping around at the
    /// boundary of the type.
    fn wrapping_rem(self, other: Rhs) -> Self::Output;
}

/// Wrapping simultaneous quotient and remainder. Computes
/// `(self / other, self % other)`, wrapping around at the boundary of the type.
///
/// # Panics
///
/// This function will panic if `other` is zero.
pub trait WrappingDivRem<Rhs> {
    /// The resulting type after applying the operation.
    type Output;

    /// Wrapping simultaneous quotient and remainder. Computes
    /// `(self / other, self % other)`, wrapping around at the boundary of the type.
    fn wrapping_div_rem(self, other: Rhs) -> (Self::Output, Self::Output);
}

/// Bundle of wrapping Euclidean division operations.
pub trait WrappingEuclidOps<Rhs>:
    WrappingDivEuclid<Rhs> + WrappingRemEuclid<Rhs> + WrappingDivRemEuclid<Rhs>
{
}
impl<Rhs, T: WrappingDivEuclid<Rhs> + WrappingRemEuclid<Rhs> + WrappingDivRemEuclid<Rhs>>
    WrappingEuclidOps<Rhs> for T
{
}

/// Wrapping Euclidean division. Computes `self.div_euclid(other)`,
/// wrapping around at the boundary of the type.
///
/// Wrapping will only occur in `MIN / -1` on a signed type (where `MIN` is the negative minimal value
/// for the type). This is equivalent to `-MIN`, a positive value that is too large to represent in the
/// type. In this case, this method returns `MIN` itself.
///
/// # Panics
///
/// This function will panic if `other` is zero.
pub trait WrappingDivEuclid<Rhs = Self> {
    /// The resulting type after applying the operation.
    type Output;

    /// Wrapping Euclidean division. Computes `self.div_euclid(other)`,
    /// wrapping around at the boundary of the type.
    fn wrapping_div_euclid(self, other: Rhs) -> Self::Output;
}

/// Wrapping Euclidean remainder. Computes `self.rem_euclid(other)`, wrapping around
/// at the boundary of the type.
///
/// Wrapping will only occur in `MIN % -1` on a signed type (where `MIN` is the negative minimal value
/// for the type). In this case, this method returns `0`.
///
/// # Panics
///
/// This function will panic if `other` is zero.
pub trait WrappingRemEuclid<Rhs = Self> {
    /// The resulting type after applying the operation.
    type Output;

    /// Wrapping Euclidean remainder. Computes `self.rem_euclid(other)`, wrapping around
    /// at the boundary of the type.
    fn wrapping_rem_euclid(self, other: Rhs) -> Self::Output;
}

/// Wrapping simultaneous Euclidean quotient and remainder. Computes
/// `(self.div_euclid(other), self.rem_euclid(other))`, wrapping around at the
/// boundary of the type.
///
/// # Panics
///
/// This function will panic if `other` is zero.
pub trait WrappingDivRemEuclid<Rhs = Self> {
    /// The resulting type after applying the operation.
    type Output;

    /// Wrapping simultaneous Euclidean quotient and remainder. Computes
    /// `(self.div_euclid(other), self.rem_euclid(other))`, wrapping around at the
    /// boundary of the type.
    fn wrapping_div_rem_euclid(self, other: Rhs) -> (Self::Output, Self::Output);
}

/// Bundle of wrapping field operations.
pub trait WrappingFieldOps<Rhs = Self>:
    WrappingRingOps<Rhs> + WrappingDiv<Rhs, Output = Self>
{
}
impl<Rhs, T: WrappingRingOps<Rhs> + WrappingDiv<Rhs, Output = Self>> WrappingFieldOps<Rhs> for T {}

/// Wrapping (modular) division. Computes `self / other`, wrapping around at the
/// boundary of the type.
///
/// The only case where such wrapping can occur is when one divides `MIN / -1` on a signed type (where
/// `MIN` is the negative minimal value for the type); this is equivalent to `-MIN`, a positive value
/// that is too large to represent in the type. In such a case, this function returns `MIN` itself.
///
/// # Panics
///
/// This function will panic if `other` is zero.
pub trait WrappingDiv<Rhs = Self> {
    /// The resulting type after applying the operation.
    type Output;

    /// Wrapping (modular) division. Computes `self / other`, wrapping around at the
    /// boundary of the type.
    fn wrapping_div(self, other: Rhs) -> Self::Output;
}

/// Bundle of wrapping ring operations.
pub trait WrappingRingOps<Rhs = Self>:
    WrappingAdd<Rhs, Output = Self>
    + WrappingSub<Rhs, Output = Self>
    + WrappingMul<Rhs, Output = Self>
    + WrappingNeg<Output = Self>
{
}
impl<
    Rhs,
    T: WrappingAdd<Rhs, Output = Self>
        + WrappingSub<Rhs, Output = Self>
        + WrappingMul<Rhs, Output = Self>
        + WrappingNeg<Output = Self>,
> WrappingRingOps<Rhs> for T
{
}

/// Wrapping (modular) addition. Computes `self + other`, wrapping around at the
/// boundary of the type.
pub trait WrappingAdd<Rhs = Self> {
    /// The resulting type after applying the operation.
    type Output;

    /// Wrapping (modular) addition. Computes `self + other`, wrapping around at the
    /// boundary of the type.
    fn wrapping_add(self, other: Rhs) -> Self::Output;
}

/// Wrapping (modular) subtraction. Computes `self - other`, wrapping around at the
/// boundary of the type.
pub trait WrappingSub<Rhs = Self> {
    /// The resulting type after applying the operation.
    type Output;

    /// Wrapping (modular) subtraction. Computes `self - other`, wrapping around at the
    /// boundary of the type.
    fn wrapping_sub(self, other: Rhs) -> Self::Output;
}

/// Wrapping (modular) multiplication. Computes `self * other`, wrapping around at
/// the boundary of the type.
pub trait WrappingMul<Rhs = Self> {
    /// The resulting type after applying the operation.
    type Output;

    /// Wrapping (modular) multiplication. Computes `self * other`, wrapping around at
    /// the boundary of the type.
    fn wrapping_mul(self, other: Rhs) -> Self::Output;
}

/// Wrapping (modular) negation. Computes `-self`, wrapping around at the boundary
/// of the type.
///
/// The only case where such wrapping can occur is when one negates `MIN` on a signed type (where `MIN`
/// is the negative minimal value for the type); this is a positive value that is too large to represent
/// in the type. In such a case, this function returns `MIN` itself.
pub trait WrappingNeg {
    /// The resulting type after applying the operation.
    type Output;

    /// Wrapping (modular) negation. Computes `-self`, wrapping around at the boundary
    /// of the type.
    fn wrapping_neg(self) -> Self::Output;
}

macro_rules! impl_wrapping_ring_field_traits_for_ints {
    ($($ty:ty),*) => {
        $(
            impl WrappingAdd<$ty> for $ty {
                type Output = $ty;

                #[inline]
                fn wrapping_add(self, other: $ty) -> Self::Output {
                    <$ty>::wrapping_add(self, other)
                }
            }

            impl WrappingSub<$ty> for $ty {
                type Output = $ty;

                #[inline]
                fn wrapping_sub(self, other: $ty) -> Self::Output {
                    <$ty>::wrapping_sub(self, other)
                }
            }

            impl WrappingMul<$ty> for $ty {
                type Output = $ty;

                #[inline]
                fn wrapping_mul(self, other: $ty) -> Self::Output {
                    <$ty>::wrapping_mul(self, other)
                }
            }

            impl WrappingDiv<$ty> for $ty {
                type Output = $ty;

                #[inline]
                fn wrapping_div(self, other: $ty) -> Self::Output {
                    <$ty>::wrapping_div(self, other)
                }
            }

            impl WrappingRem<$ty> for $ty {
                type Output = $ty;

                #[inline]
                fn wrapping_rem(self, other: $ty) -> Self::Output {
                    <$ty>::wrapping_rem(self, other)
                }
            }

            impl WrappingNeg for $ty {
                type Output = $ty;

                #[inline]
                fn wrapping_neg(self) -> Self::Output {
                    <$ty>::wrapping_neg(self)
                }
            }
        )*
    };
}

macro_rules! impl_wrapping_euclid_traits {
    ($($ty:ty),*) => {
        $(
            impl WrappingDivEuclid<$ty> for $ty {
                type Output = $ty;

                #[inline]
                fn wrapping_div_euclid(self, other: $ty) -> Self::Output {
                    <$ty>::wrapping_div_euclid(self, other)
                }
            }

            impl WrappingRemEuclid<$ty> for $ty {
                type Output = $ty;

                #[inline]
                fn wrapping_rem_euclid(self, other: $ty) -> Self::Output {
                    <$ty>::wrapping_rem_euclid(self, other)
                }
            }

            impl WrappingDivRemEuclid<$ty> for $ty {
                type Output = $ty;

                #[inline]
                fn wrapping_div_rem_euclid(self, other: $ty) -> (Self::Output, Self::Output) {
                    (
                        <$ty>::wrapping_div_euclid(self, other),
                        <$ty>::wrapping_rem_euclid(self, other),
                    )
                }
            }
        )*
    };
}

macro_rules! impl_wrapping_div_rem_trait {
    ($($ty:ty),*) => {
        $(
            impl WrappingDivRem<$ty> for $ty {
                type Output = $ty;

                #[inline]
                fn wrapping_div_rem(self, other: $ty) -> (Self::Output, Self::Output) {
                    (
                        <$ty>::wrapping_div(self, other),
                        <$ty>::wrapping_rem(self, other),
                    )
                }
            }
        )*
    };
}

macro_rules! impl_wrapping_mul_add_trait {
    ($($ty:ty),*) => {
        $(
            impl WrappingMulAdd<$ty, $ty> for $ty {
                type Output = $ty;

                #[inline]
                fn wrapping_mul_add(self, a: $ty, b: $ty) -> Self::Output {
                    <$ty>::wrapping_mul(self, a).wrapping_add(b)
                }
            }
        )*
    };
}

macro_rules! impl_wrapping_arith_traits_for_signed_ints {
    ($($ty:ty),*) => {
        impl_wrapping_ring_field_traits_for_ints!($($ty),*);
        impl_wrapping_div_rem_trait!($($ty),*);
        impl_wrapping_euclid_traits!($($ty),*);
        impl_wrapping_mul_add_trait!($($ty),*);
    };
}

macro_rules! impl_wrapping_arith_traits_for_unsigned_ints {
    ($($ty:ty),*) => {
        impl_wrapping_ring_field_traits_for_ints!($($ty),*);
        impl_wrapping_div_rem_trait!($($ty),*);
        impl_wrapping_euclid_traits!($($ty),*);
        impl_wrapping_mul_add_trait!($($ty),*);
    };
}

/*macro_rules! impl_wrapping_ring_field_traits_for_floats {
    ($($ty:ty),*) => {
        $(
            impl WrappingAdd<$ty> for $ty {
                type Output = $ty;

                #[inline]
                fn wrapping_add(self, other: $ty) -> Self::Output {
                    self + other
                }
            }

            impl WrappingSub<$ty> for $ty {
                type Output = $ty;

                #[inline]
                fn wrapping_sub(self, other: $ty) -> Self::Output {
                    self - other
                }
            }

            impl WrappingMul<$ty> for $ty {
                type Output = $ty;

                #[inline]
                fn wrapping_mul(self, other: $ty) -> Self::Output {
                    self * other
                }
            }

            impl WrappingDiv<$ty> for $ty {
                type Output = $ty;

                #[inline]
                fn wrapping_div(self, other: $ty) -> Self::Output {
                    self / other
                }
            }

            impl WrappingRem<$ty> for $ty {
                type Output = $ty;

                #[inline]
                fn wrapping_rem(self, other: $ty) -> Self::Output {
                    self % other
                }
            }

            impl WrappingNeg for $ty {
                type Output = $ty;

                #[inline]
                fn wrapping_neg(self) -> Self::Output {
                    -self
                }
            }
        )*
    };
}

macro_rules! impl_wrapping_div_rem_trait_for_floats {
    ($($ty:ty),*) => {
        $(
            impl WrappingDivRem<$ty> for $ty {
                type Output = $ty;

                #[inline]
                fn wrapping_div_rem(self, other: $ty) -> (Self::Output, Self::Output) {
                    (self / other, self % other)
                }
            }
        )*
    };
}*/

/*#[cfg(feature = "std")]
macro_rules! impl_wrapping_euclid_traits_for_floats {
    ($($ty:ty),*) => {
        $(
            impl WrappingDivEuclid<$ty> for $ty {
                type Output = $ty;

                #[inline]
                fn wrapping_div_euclid(self, other: $ty) -> Self::Output {
                    <$ty>::div_euclid(self, other)
                }
            }

            impl WrappingRemEuclid<$ty> for $ty {
                type Output = $ty;

                #[inline]
                fn wrapping_rem_euclid(self, other: $ty) -> Self::Output {
                    <$ty>::rem_euclid(self, other)
                }
            }

            impl WrappingDivRemEuclid<$ty> for $ty {
                type Output = $ty;

                #[inline]
                fn wrapping_div_rem_euclid(self, other: $ty) -> (Self::Output, Self::Output) {
                    (
                        <$ty>::div_euclid(self, other),
                        <$ty>::rem_euclid(self, other),
                    )
                }
            }
        )*
    };
}*/

/*macro_rules! impl_wrapping_mul_add_trait_for_floats {
    ($($ty:ty),*) => {
        $(
            impl WrappingMulAdd<$ty, $ty> for $ty {
                type Output = $ty;

                #[inline]
                fn wrapping_mul_add(self, a: $ty, b: $ty) -> Self::Output {
                    MulAdd::mul_add(self, a, b)
                }
            }
        )*
    };
}*/

/*macro_rules! impl_wrapping_arith_traits_for_floats {
    ($($ty:ty),*) => {
        impl_wrapping_ring_field_traits_for_floats!($($ty),*);
        impl_wrapping_div_rem_trait_for_floats!($($ty),*);
        #[cfg(feature = "std")]
        impl_wrapping_euclid_traits_for_floats!($($ty),*);
    };
}*/

impl_wrapping_arith_traits_for_signed_ints!(i8, i16, i32, i64, i128, isize);
impl_wrapping_arith_traits_for_unsigned_ints!(u8, u16, u32, u64, u128, usize);
//impl_wrapping_arith_traits_for_floats!(f32, f64);

//impl_wrapping_mul_add_trait_for_floats!(f32, f64);
