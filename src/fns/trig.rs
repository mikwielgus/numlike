// SPDX-FileCopyrightText: 2026 numlike contributors
//
// SPDX-License-Identifier: MIT OR Apache-2.0

/*pub trait FullTrigFns<Rhs = Self>: TrigFns + InvTrigFns<Rhs> {}
impl<Rhs, T: TrigFns + InvTrigFns<Rhs>> FullTrigFns<Rhs> for T {}*/

/// Bundle of trigonometric functions.
pub trait TrigFns: ClassicalTrigFns + SinCos {}
impl<T: ClassicalTrigFns + SinCos> TrigFns for T {}

/// Simultaneously computes the sine and cosine of the number, `x`. Returns
/// `(sin(x), cos(x))`.
pub trait SinCos {
    /// The resulting type after applying the operation.
    type Output;

    /// Simultaneously computes the sine and cosine of the number, `x`. Returns
    /// `(sin(x), cos(x))`.
    fn sin_cos(self) -> (Self::Output, Self::Output);
}

/// Bundle of classical trigonometric functions.
pub trait ClassicalTrigFns: Sin + Cos + Tan {}
impl<T: Sin + Cos + Tan> ClassicalTrigFns for T {}

/// Computes the sine of a number (in radians).
pub trait Sin {
    /// The resulting type after applying the operation.
    type Output;

    /// Computes the sine of a number (in radians).
    fn sin(self) -> Self::Output;
}

/// Computes the cosine of a number (in radians).
pub trait Cos {
    /// The resulting type after applying the operation.
    type Output;

    /// Computes the cosine of a number (in radians).
    fn cos(self) -> Self::Output;
}

/// Computes the tangent of a number (in radians).
pub trait Tan {
    /// The resulting type after applying the operation.
    type Output;

    /// Computes the tangent of a number (in radians).
    fn tan(self) -> Self::Output;
}

/// Computes the tangent of a number (in radians).
///
/// Returns `None` if the result is not finite.
pub trait CheckedTan {
    /// The resulting type after applying the operation.
    type Output;

    /// Computes the tangent of a number (in radians).
    fn checked_tan(self) -> Option<Self::Output>;
}

/// Bundle of inverse trigonometric functions.
pub trait InvTrigFns<Rhs = Self>: ClassicalInvTrigFns<Rhs> + Atan2<Rhs> {}
impl<Rhs, T: ClassicalInvTrigFns<Rhs> + Atan2<Rhs>> InvTrigFns<Rhs> for T {}

/// Computes the four quadrant arctangent of `self` (`y`) and `rhs` (`x`) in radians.
pub trait Atan2<Rhs> {
    /// The resulting type after applying the operation.
    type Output;

    /// Computes the four quadrant arctangent of `self` (`y`) and `rhs` (`x`) in radians.
    fn atan2(self, rhs: Rhs) -> Self::Output;
}

/// Bundle of classical inverse trigonometric functions.
pub trait ClassicalInvTrigFns<Rhs = Self>: Asin + Acos + Atan {}
impl<Rhs, T: Asin + Acos + Atan> ClassicalInvTrigFns<Rhs> for T {}

/// Computes the arcsine of a number. Return value is in radians in
/// the range [-pi/2, pi/2] or NaN if the number is outside the range
/// [-1, 1].
pub trait Asin {
    /// The resulting type after applying the operation.
    type Output;

    /// Computes the arcsine of a number. Return value is in radians in
    /// the range [-pi/2, pi/2] or NaN if the number is outside the range
    /// [-1, 1].
    fn asin(self) -> Self::Output;
}

/// Computes the arccosine of a number. Return value is in radians in
/// the range [0, pi] or NaN if the number is outside the range
/// [-1, 1].
pub trait Acos {
    /// The resulting type after applying the operation.
    type Output;

    /// Computes the arccosine of a number. Return value is in radians in
    /// the range [0, pi] or NaN if the number is outside the range
    /// [-1, 1].
    fn acos(self) -> Self::Output;
}

/// Computes the arctangent of a number. Return value is in radians in the
/// range [-pi/2, pi/2];
pub trait Atan {
    /// The resulting type after applying the operation.
    type Output;

    /// Computes the arctangent of a number. Return value is in radians in the
    /// range [-pi/2, pi/2];
    fn atan(self) -> Self::Output;
}

/// Bundle of checked classical inverse trigonometric functions.
pub trait CheckedClassicalInvTrigFns<Rhs = Self>: CheckedAsin + CheckedAcos {}
impl<Rhs, T: CheckedAsin + CheckedAcos> CheckedClassicalInvTrigFns<Rhs> for T {}

/// Computes the arcsine of a number. Return value is in radians in
/// the range [-pi/2, pi/2].
///
/// Returns `None` if the number is outside the range [-1, 1], or if the
/// result is not finite.
pub trait CheckedAsin {
    /// The resulting type after applying the operation.
    type Output;

    /// Computes the arcsine of a number. Return value is in radians in
    /// the range [-pi/2, pi/2].
    fn checked_asin(self) -> Option<Self::Output>;
}

/// Computes the arccosine of a number. Return value is in radians in
/// the range [0, pi].
///
/// Returns `None` if the number is outside the range [-1, 1], or if the
/// result is not finite.
pub trait CheckedAcos {
    /// The resulting type after applying the operation.
    type Output;

    /// Computes the arccosine of a number. Return value is in radians in
    /// the range [0, pi].
    fn checked_acos(self) -> Option<Self::Output>;
}

#[cfg(any(feature = "std", feature = "libm"))]
macro_rules! impl_trig_traits_for_float {
    (
        $ty:ty,
        $sin_cos:path,
        $sin:path,
        $cos:path,
        $tan:path,
        $atan2:path,
        $asin:path,
        $acos:path,
        $atan:path
    ) => {
        impl SinCos for $ty {
            type Output = $ty;

            #[inline]
            fn sin_cos(self) -> (Self::Output, Self::Output) {
                $sin_cos(self)
            }
        }

        impl Sin for $ty {
            type Output = $ty;

            #[inline]
            fn sin(self) -> Self::Output {
                $sin(self)
            }
        }

        impl Cos for $ty {
            type Output = $ty;

            #[inline]
            fn cos(self) -> Self::Output {
                $cos(self)
            }
        }

        impl Tan for $ty {
            type Output = $ty;

            #[inline]
            fn tan(self) -> Self::Output {
                $tan(self)
            }
        }

        impl CheckedTan for $ty {
            type Output = $ty;

            #[inline]
            fn checked_tan(self) -> Option<Self::Output> {
                let result = $tan(self);

                result.is_finite().then_some(result)
            }
        }

        impl Atan2<$ty> for $ty {
            type Output = $ty;

            #[inline]
            fn atan2(self, other: $ty) -> Self::Output {
                $atan2(self, other)
            }
        }

        impl Asin for $ty {
            type Output = $ty;

            #[inline]
            fn asin(self) -> Self::Output {
                $asin(self)
            }
        }

        impl Acos for $ty {
            type Output = $ty;

            #[inline]
            fn acos(self) -> Self::Output {
                $acos(self)
            }
        }

        impl Atan for $ty {
            type Output = $ty;

            #[inline]
            fn atan(self) -> Self::Output {
                $atan(self)
            }
        }

        impl CheckedAsin for $ty {
            type Output = $ty;

            #[inline]
            fn checked_asin(self) -> Option<Self::Output> {
                let result = $asin(self);

                result.is_finite().then_some(result)
            }
        }

        impl CheckedAcos for $ty {
            type Output = $ty;

            #[inline]
            fn checked_acos(self) -> Option<Self::Output> {
                let result = $acos(self);

                result.is_finite().then_some(result)
            }
        }
    };
}

#[cfg(feature = "std")]
impl_trig_traits_for_float!(
    f32,
    f32::sin_cos,
    f32::sin,
    f32::cos,
    f32::tan,
    f32::atan2,
    f32::asin,
    f32::acos,
    f32::atan
);
#[cfg(feature = "std")]
impl_trig_traits_for_float!(
    f64,
    f64::sin_cos,
    f64::sin,
    f64::cos,
    f64::tan,
    f64::atan2,
    f64::asin,
    f64::acos,
    f64::atan
);
#[cfg(all(not(feature = "std"), feature = "libm"))]
impl_trig_traits_for_float!(
    f32,
    libm::sincosf,
    libm::sinf,
    libm::cosf,
    libm::tanf,
    libm::atan2f,
    libm::asinf,
    libm::acosf,
    libm::atanf
);
#[cfg(all(not(feature = "std"), feature = "libm"))]
impl_trig_traits_for_float!(
    f64,
    libm::sincos,
    libm::sin,
    libm::cos,
    libm::tan,
    libm::atan2,
    libm::asin,
    libm::acos,
    libm::atan
);
