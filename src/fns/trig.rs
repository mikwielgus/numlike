// SPDX-FileCopyrightText: 2026 numlike contributors
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use crate::types::Object;

/*pub trait FullTrigFns<Rhs = Self>: TrigFns + InvTrigFns<Rhs> {}
impl<Rhs, T: TrigFns + InvTrigFns<Rhs>> FullTrigFns<Rhs> for T {}*/

/// Bundle of trigonometric functions.
pub trait TrigFns: ClassicalTrigFns + SinCos {}
impl<T: ClassicalTrigFns + SinCos> TrigFns for T {}

/// Simultaneously computes the sine and cosine of the number, `x`. Returns
/// `(sin(x), cos(x))`.
pub trait SinCos {
    /// The resulting type after evaluating the function.
    type Output;

    /// Simultaneously computes the sine and cosine of the number, `x`. Returns
    /// `(sin(x), cos(x))`.
    fn sin_cos(self) -> (Self::Output, Self::Output);
}

impl<B: SinCos> SinCos for Object<B> {
    type Output = Object<<B as SinCos>::Output>;

    fn sin_cos(self) -> (Self::Output, Self::Output) {
        let (sin, cos) = <B as SinCos>::sin_cos(self.0);

        (Object(sin), Object(cos))
    }
}

impl<B: SinCos> Object<B> {
    /// Simultaneously computes the sine and cosine of the number, `x`. Returns
    /// `(sin(x), cos(x))`.
    pub fn sin_cos(self) -> (Object<<B as SinCos>::Output>, Object<<B as SinCos>::Output>) {
        let (sin, cos) = <B as SinCos>::sin_cos(self.0);

        (Object(sin), Object(cos))
    }
}

/// Bundle of classical trigonometric functions.
pub trait ClassicalTrigFns: Sin + Cos + Tan {}
impl<T: Sin + Cos + Tan> ClassicalTrigFns for T {}

/// Computes the sine of a number (in radians).
pub trait Sin {
    /// The resulting type after evaluating the function.
    type Output;

    /// Computes the sine of a number (in radians).
    fn sin(self) -> Self::Output;
}

impl<B: Sin> Sin for Object<B> {
    type Output = Object<<B as Sin>::Output>;

    fn sin(self) -> Self::Output {
        Object(<B as Sin>::sin(self.0))
    }
}

impl<B: Sin> Object<B> {
    /// Computes the sine of a number (in radians).
    pub fn sin(self) -> Object<<B as Sin>::Output> {
        Object(<B as Sin>::sin(self.0))
    }
}

/// Computes the cosine of a number (in radians).
pub trait Cos {
    /// The resulting type after evaluating the function.
    type Output;

    /// Computes the cosine of a number (in radians).
    fn cos(self) -> Self::Output;
}

impl<B: Cos> Cos for Object<B> {
    type Output = Object<<B as Cos>::Output>;

    fn cos(self) -> Self::Output {
        Object(<B as Cos>::cos(self.0))
    }
}

impl<B: Cos> Object<B> {
    /// Computes the cosine of a number (in radians).
    pub fn cos(self) -> Object<<B as Cos>::Output> {
        Object(<B as Cos>::cos(self.0))
    }
}

/// Computes the tangent of a number (in radians).
pub trait Tan {
    /// The resulting type after evaluating the function.
    type Output;

    /// Computes the tangent of a number (in radians).
    fn tan(self) -> Self::Output;
}

impl<B: Tan> Tan for Object<B> {
    type Output = Object<<B as Tan>::Output>;

    fn tan(self) -> Self::Output {
        Object(<B as Tan>::tan(self.0))
    }
}

impl<B: Tan> Object<B> {
    /// Computes the tangent of a number (in radians).
    pub fn tan(self) -> Object<<B as Tan>::Output> {
        Object(<B as Tan>::tan(self.0))
    }
}

/// Computes the tangent of a number (in radians).
///
/// Returns `None` if the result is not finite.
pub trait CheckedTan {
    /// The resulting type after evaluating the function.
    type Output;

    /// Computes the tangent of a number (in radians).
    fn checked_tan(self) -> Option<Self::Output>;
}

impl<B: CheckedTan> CheckedTan for Object<B> {
    type Output = Object<<B as CheckedTan>::Output>;

    fn checked_tan(self) -> Option<Self::Output> {
        <B as CheckedTan>::checked_tan(self.0).map(Object)
    }
}

impl<B: CheckedTan> Object<B> {
    /// Computes the tangent of a number (in radians).
    ///
    /// Returns `None` if the result is not finite.
    pub fn checked_tan(self) -> Option<Object<<B as CheckedTan>::Output>> {
        <B as CheckedTan>::checked_tan(self.0).map(Object)
    }
}

/// Bundle of inverse trigonometric functions.
pub trait InvTrigFns<Rhs = Self>: ClassicalInvTrigFns<Rhs> + Atan2<Rhs> {}
impl<Rhs, T: ClassicalInvTrigFns<Rhs> + Atan2<Rhs>> InvTrigFns<Rhs> for T {}

/// Computes the four quadrant arctangent of `self` (`y`) and `rhs` (`x`) in radians.
pub trait Atan2<Rhs> {
    /// The resulting type after evaluating the function.
    type Output;

    /// Computes the four quadrant arctangent of `self` (`y`) and `rhs` (`x`) in radians.
    fn atan2(self, rhs: Rhs) -> Self::Output;
}

impl<B, Rhs> Atan2<Rhs> for Object<B>
where
    B: Atan2<Rhs>,
{
    type Output = Object<<B as Atan2<Rhs>>::Output>;

    fn atan2(self, rhs: Rhs) -> Self::Output {
        Object(<B as Atan2<Rhs>>::atan2(self.0, rhs))
    }
}

impl<B> Object<B> {
    /// Computes the four quadrant arctangent of `self` (`y`) and `rhs` (`x`) in radians.
    pub fn atan2<Rhs>(self, rhs: Rhs) -> Object<<B as Atan2<Rhs>>::Output>
    where
        B: Atan2<Rhs>,
    {
        Object(<B as Atan2<Rhs>>::atan2(self.0, rhs))
    }
}

/// Bundle of classical inverse trigonometric functions.
pub trait ClassicalInvTrigFns<Rhs = Self>: Asin + Acos + Atan {}
impl<Rhs, T: Asin + Acos + Atan> ClassicalInvTrigFns<Rhs> for T {}

/// Computes the arcsine of a number. Return value is in radians in
/// the range [-pi/2, pi/2] or NaN if the number is outside the range
/// [-1, 1].
pub trait Asin {
    /// The resulting type after evaluating the function.
    type Output;

    /// Computes the arcsine of a number. Return value is in radians in
    /// the range [-pi/2, pi/2] or NaN if the number is outside the range
    /// [-1, 1].
    fn asin(self) -> Self::Output;
}

impl<B: Asin> Asin for Object<B> {
    type Output = Object<<B as Asin>::Output>;

    fn asin(self) -> Self::Output {
        Object(<B as Asin>::asin(self.0))
    }
}

impl<B: Asin> Object<B> {
    /// Computes the arcsine of a number. Return value is in radians in
    /// the range [-pi/2, pi/2] or NaN if the number is outside the range
    /// [-1, 1].
    pub fn asin(self) -> Object<<B as Asin>::Output> {
        Object(<B as Asin>::asin(self.0))
    }
}

/// Computes the arccosine of a number. Return value is in radians in
/// the range [0, pi] or NaN if the number is outside the range
/// [-1, 1].
pub trait Acos {
    /// The resulting type after evaluating the function.
    type Output;

    /// Computes the arccosine of a number. Return value is in radians in
    /// the range [0, pi] or NaN if the number is outside the range
    /// [-1, 1].
    fn acos(self) -> Self::Output;
}

impl<B: Acos> Acos for Object<B> {
    type Output = Object<<B as Acos>::Output>;

    fn acos(self) -> Self::Output {
        Object(<B as Acos>::acos(self.0))
    }
}

impl<B: Acos> Object<B> {
    /// Computes the arccosine of a number. Return value is in radians in
    /// the range [0, pi] or NaN if the number is outside the range
    /// [-1, 1].
    pub fn acos(self) -> Object<<B as Acos>::Output> {
        Object(<B as Acos>::acos(self.0))
    }
}

/// Computes the arctangent of a number. Return value is in radians in the
/// range [-pi/2, pi/2];
pub trait Atan {
    /// The resulting type after evaluating the function.
    type Output;

    /// Computes the arctangent of a number. Return value is in radians in the
    /// range [-pi/2, pi/2];
    fn atan(self) -> Self::Output;
}

impl<B: Atan> Atan for Object<B> {
    type Output = Object<<B as Atan>::Output>;

    fn atan(self) -> Self::Output {
        Object(<B as Atan>::atan(self.0))
    }
}

impl<B: Atan> Object<B> {
    /// Computes the arctangent of a number. Return value is in radians in the
    /// range [-pi/2, pi/2];
    pub fn atan(self) -> Object<<B as Atan>::Output> {
        Object(<B as Atan>::atan(self.0))
    }
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
    /// The resulting type after evaluating the function.
    type Output;

    /// Computes the arcsine of a number. Return value is in radians in
    /// the range [-pi/2, pi/2].
    fn checked_asin(self) -> Option<Self::Output>;
}

impl<B: CheckedAsin> CheckedAsin for Object<B> {
    type Output = Object<<B as CheckedAsin>::Output>;

    fn checked_asin(self) -> Option<Self::Output> {
        <B as CheckedAsin>::checked_asin(self.0).map(Object)
    }
}

impl<B: CheckedAsin> Object<B> {
    /// Computes the arcsine of a number. Return value is in radians in
    /// the range [-pi/2, pi/2].
    ///
    /// Returns `None` if the number is outside the range [-1, 1], or if the
    /// result is not finite.
    pub fn checked_asin(self) -> Option<Object<<B as CheckedAsin>::Output>> {
        <B as CheckedAsin>::checked_asin(self.0).map(Object)
    }
}

/// Computes the arccosine of a number. Return value is in radians in
/// the range [0, pi].
///
/// Returns `None` if the number is outside the range [-1, 1], or if the
/// result is not finite.
pub trait CheckedAcos {
    /// The resulting type after evaluating the function.
    type Output;

    /// Computes the arccosine of a number. Return value is in radians in
    /// the range [0, pi].
    fn checked_acos(self) -> Option<Self::Output>;
}

impl<B: CheckedAcos> CheckedAcos for Object<B> {
    type Output = Object<<B as CheckedAcos>::Output>;

    fn checked_acos(self) -> Option<Self::Output> {
        <B as CheckedAcos>::checked_acos(self.0).map(Object)
    }
}

impl<B: CheckedAcos> Object<B> {
    /// Computes the arccosine of a number. Return value is in radians in
    /// the range [0, pi].
    ///
    /// Returns `None` if the number is outside the range [-1, 1], or if the
    /// result is not finite.
    pub fn checked_acos(self) -> Option<Object<<B as CheckedAcos>::Output>> {
        <B as CheckedAcos>::checked_acos(self.0).map(Object)
    }
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
    (
        $ty:ty,
        $sin_cos:path,
        $sin:path,
        $cos:path,
        $tan:path,
        $atan2:path,
        $asin:path,
        $acos:path,
        $atan:path,
        $nonnegative_tests_mod:ident,
        $negative_tests_mod:ident
    ) => {
        impl_trig_traits_for_float!($ty, $sin_cos, $sin, $cos, $tan, $atan2, $asin, $acos, $atan);

        test_trig_traits_nonnegative!($ty, $nonnegative_tests_mod);
        test_trig_traits_negative!($ty, $negative_tests_mod);
    };
}

#[cfg(any(feature = "std", feature = "libm"))]
macro_rules! test_trig_traits_nonnegative {
    ($ty:ty, $tests_mod:ident) => {
        #[cfg(test)]
        mod $tests_mod {
            use crate::elem::*;
            use crate::fns::*;
            use crate::limits::*;
            use crate::types::Object;

            #[test]
            fn test_sin() {
                let zero = <$ty as AssocConstZero>::ZERO;
                let one = <$ty as AssocConstOne>::ONE;
                let two = one + one;
                let four = two + two;
                let half = one / two;
                let sixteen = four * four;
                let sixteenth = one / sixteen;

                assert_eq!(Sin::sin(zero), zero);
                assert_eq!(Object::new(zero).sin(), Object::new(Sin::sin(zero)));

                let sin_half = Sin::sin(half);

                assert!(sin_half > zero);
                assert!(sin_half < one);
                assert!(Abs::abs(Asin::asin(sin_half) - half) < sixteenth);
            }

            #[test]
            fn test_cos() {
                let zero = <$ty as AssocConstZero>::ZERO;
                let one = <$ty as AssocConstOne>::ONE;
                let two = one + one;
                let four = two + two;
                let half = one / two;
                let sixteen = four * four;
                let sixteenth = one / sixteen;

                assert_eq!(Cos::cos(zero), one);
                assert_eq!(Object::new(zero).cos(), Object::new(Cos::cos(zero)));

                let cos_half = Cos::cos(half);

                assert!(cos_half > zero);
                assert!(cos_half < one);
                assert!(Abs::abs(Acos::acos(cos_half) - half) < sixteenth);
            }

            #[test]
            fn test_tan() {
                let zero = <$ty as AssocConstZero>::ZERO;
                let one = <$ty as AssocConstOne>::ONE;
                let two = one + one;
                let four = two + two;
                let half = one / two;
                let sixteen = four * four;
                let sixteenth = one / sixteen;

                assert_eq!(Tan::tan(zero), zero);
                assert_eq!(Object::new(zero).tan(), Object::new(Tan::tan(zero)));

                let tan_half = Tan::tan(half);

                assert!(tan_half > zero);
                assert!(Abs::abs(Atan::atan(tan_half) - half) < sixteenth);
            }

            #[test]
            fn test_sin_cos() {
                let zero = <$ty as AssocConstZero>::ZERO;
                let one = <$ty as AssocConstOne>::ONE;
                let two = one + one;
                let four = two + two;
                let half = one / two;
                let sixteen = four * four;
                let sixteenth = one / sixteen;

                assert_eq!(SinCos::sin_cos(zero), (zero, one));

                let (sin_zero, cos_zero) = SinCos::sin_cos(zero);

                assert_eq!(
                    Object::new(zero).sin_cos(),
                    (Object::new(sin_zero), Object::new(cos_zero))
                );

                let (sin_half, cos_half) = SinCos::sin_cos(half);

                assert!(Abs::abs(sin_half - Sin::sin(half)) < sixteenth);
                assert!(Abs::abs(cos_half - Cos::cos(half)) < sixteenth);
            }

            #[test]
            fn test_asin() {
                let zero = <$ty as AssocConstZero>::ZERO;
                let one = <$ty as AssocConstOne>::ONE;
                let two = one + one;
                let four = two + two;
                let half = one / two;
                let sixteen = four * four;
                let sixteenth = one / sixteen;

                assert_eq!(Asin::asin(zero), zero);
                assert_eq!(Object::new(zero).asin(), Object::new(Asin::asin(zero)));

                let asin_half = Asin::asin(half);

                assert!(asin_half > zero);
                assert!(asin_half < one);
                assert!(Abs::abs(Sin::sin(asin_half) - half) < sixteenth);
            }

            #[test]
            fn test_acos() {
                let zero = <$ty as AssocConstZero>::ZERO;
                let one = <$ty as AssocConstOne>::ONE;
                let two = one + one;
                let four = two + two;
                let half = one / two;
                let sixteen = four * four;
                let sixteenth = one / sixteen;

                assert_eq!(Acos::acos(one), zero);
                assert_eq!(Object::new(one).acos(), Object::new(Acos::acos(one)));

                let acos_half = Acos::acos(half);

                assert!(acos_half > one);
                assert!(acos_half < two);
                assert!(Abs::abs(Cos::cos(acos_half) - half) < sixteenth);
            }

            #[test]
            fn test_atan() {
                let zero = <$ty as AssocConstZero>::ZERO;
                let one = <$ty as AssocConstOne>::ONE;
                let two = one + one;
                let four = two + two;
                let half = one / two;
                let sixteen = four * four;
                let sixteenth = one / sixteen;

                assert_eq!(Atan::atan(zero), zero);
                assert_eq!(Object::new(zero).atan(), Object::new(Atan::atan(zero)));

                let atan_half = Atan::atan(half);

                assert!(atan_half > zero);
                assert!(atan_half < one);
                assert!(Abs::abs(Tan::tan(atan_half) - half) < sixteenth);
            }

            #[test]
            fn test_atan2() {
                let zero = <$ty as AssocConstZero>::ZERO;
                let one = <$ty as AssocConstOne>::ONE;
                let two = one + one;
                let four = two + two;
                let sixteen = four * four;
                let sixteenth = one / sixteen;

                assert_eq!(Atan2::atan2(zero, one), zero);
                assert_eq!(
                    Object::new(zero).atan2(one),
                    Object::new(Atan2::atan2(zero, one))
                );

                let atan2_one_zero = Atan2::atan2(one, zero);

                assert!(atan2_one_zero > one);
                assert!(atan2_one_zero < two);
                assert!(Abs::abs(atan2_one_zero - Asin::asin(one)) < sixteenth);
                assert!(Abs::abs(Atan2::atan2(one, one) - Atan::atan(one)) < sixteenth);
            }

            #[test]
            fn test_checked_tan() {
                let zero = <$ty as AssocConstZero>::ZERO;
                let one = <$ty as AssocConstOne>::ONE;
                let two = one + one;
                let four = two + two;
                let half = one / two;
                let sixteen = four * four;
                let sixteenth = one / sixteen;

                assert_eq!(CheckedTan::checked_tan(zero), Some(zero));
                assert_eq!(
                    Object::new(zero).checked_tan(),
                    CheckedTan::checked_tan(zero).map(Object::new)
                );

                let tan_half = CheckedTan::checked_tan(half).unwrap();

                assert!(tan_half > zero);
                assert!(Abs::abs(Atan::atan(tan_half) - half) < sixteenth);
                assert_eq!(
                    CheckedTan::checked_tan(<$ty as MaxExtended>::MAX_EXTENDED),
                    None
                );
            }

            #[test]
            fn test_checked_asin() {
                let zero = <$ty as AssocConstZero>::ZERO;
                let one = <$ty as AssocConstOne>::ONE;
                let two = one + one;
                let half = one / two;

                assert_eq!(CheckedAsin::checked_asin(zero), Some(zero));
                assert_eq!(
                    Object::new(zero).checked_asin(),
                    CheckedAsin::checked_asin(zero).map(Object::new)
                );
                assert!(CheckedAsin::checked_asin(half).unwrap() > zero);
                assert_eq!(CheckedAsin::checked_asin(two), None);
            }

            #[test]
            fn test_checked_acos() {
                let zero = <$ty as AssocConstZero>::ZERO;
                let one = <$ty as AssocConstOne>::ONE;
                let two = one + one;
                let half = one / two;

                assert_eq!(CheckedAcos::checked_acos(one), Some(zero));
                assert_eq!(
                    Object::new(one).checked_acos(),
                    CheckedAcos::checked_acos(one).map(Object::new)
                );
                assert!(CheckedAcos::checked_acos(half).unwrap() > one);
                assert_eq!(CheckedAcos::checked_acos(two), None);
            }
        }
    };
}

#[cfg(any(feature = "std", feature = "libm"))]
macro_rules! test_trig_traits_negative {
    ($ty:ty, $tests_mod:ident) => {
        #[cfg(test)]
        mod $tests_mod {
            use crate::elem::*;
            use crate::fns::*;
            use crate::limits::*;
            use crate::types::Object;

            #[test]
            fn test_sin() {
                let one = <$ty as AssocConstOne>::ONE;
                let two = one + one;
                let four = two + two;
                let half = one / two;
                let sixteen = four * four;
                let sixteenth = one / sixteen;

                assert_eq!(Object::new(-half).sin(), Object::new(Sin::sin(-half)));
                assert!(Abs::abs(Sin::sin(-half) + Sin::sin(half)) < sixteenth);
            }

            #[test]
            fn test_cos() {
                let one = <$ty as AssocConstOne>::ONE;
                let two = one + one;
                let four = two + two;
                let half = one / two;
                let sixteen = four * four;
                let sixteenth = one / sixteen;

                assert_eq!(Object::new(-half).cos(), Object::new(Cos::cos(-half)));
                assert!(Abs::abs(Cos::cos(-half) - Cos::cos(half)) < sixteenth);
            }

            #[test]
            fn test_tan() {
                let one = <$ty as AssocConstOne>::ONE;
                let two = one + one;
                let four = two + two;
                let half = one / two;
                let sixteen = four * four;
                let sixteenth = one / sixteen;

                assert_eq!(Object::new(-half).tan(), Object::new(Tan::tan(-half)));
                assert!(Abs::abs(Tan::tan(-half) + Tan::tan(half)) < sixteenth);
            }

            #[test]
            fn test_sin_cos() {
                let one = <$ty as AssocConstOne>::ONE;
                let two = one + one;
                let four = two + two;
                let half = one / two;
                let sixteen = four * four;
                let sixteenth = one / sixteen;

                let (sin_neg_half, cos_neg_half) = SinCos::sin_cos(-half);

                assert_eq!(
                    Object::new(-half).sin_cos(),
                    (Object::new(sin_neg_half), Object::new(cos_neg_half))
                );
                assert!(Abs::abs(sin_neg_half + Sin::sin(half)) < sixteenth);
                assert!(Abs::abs(cos_neg_half - Cos::cos(half)) < sixteenth);
            }

            #[test]
            fn test_asin() {
                let one = <$ty as AssocConstOne>::ONE;
                let two = one + one;
                let four = two + two;
                let half = one / two;
                let sixteen = four * four;
                let sixteenth = one / sixteen;

                assert_eq!(Object::new(-half).asin(), Object::new(Asin::asin(-half)));
                assert!(Abs::abs(Asin::asin(-half) + Asin::asin(half)) < sixteenth);
            }

            #[test]
            fn test_acos() {
                let zero = <$ty as AssocConstZero>::ZERO;
                let one = <$ty as AssocConstOne>::ONE;
                let two = one + one;
                let four = two + two;
                let half = one / two;
                let sixteen = four * four;
                let sixteenth = one / sixteen;

                let acos_neg_half = Acos::acos(-half);

                assert_eq!(Object::new(-half).acos(), Object::new(Acos::acos(-half)));
                assert!(acos_neg_half > one);
                assert!(
                    Abs::abs((Acos::acos(half) + acos_neg_half) - Acos::acos(-one)) < sixteenth
                );
                assert!(Abs::abs(Acos::acos(-one) - two * Acos::acos(zero)) < sixteenth);
            }

            #[test]
            fn test_atan() {
                let one = <$ty as AssocConstOne>::ONE;
                let two = one + one;
                let four = two + two;
                let half = one / two;
                let sixteen = four * four;
                let sixteenth = one / sixteen;

                assert_eq!(Object::new(-half).atan(), Object::new(Atan::atan(-half)));
                assert!(Abs::abs(Atan::atan(-half) + Atan::atan(half)) < sixteenth);
            }

            #[test]
            fn test_atan2() {
                let zero = <$ty as AssocConstZero>::ZERO;
                let one = <$ty as AssocConstOne>::ONE;
                let two = one + one;
                let four = two + two;
                let sixteen = four * four;
                let sixteenth = one / sixteen;

                let atan2_neg_half = Atan2::atan2(-one, zero);

                assert_eq!(
                    Object::new(-one).atan2(zero),
                    Object::new(Atan2::atan2(-one, zero))
                );
                assert!(atan2_neg_half < zero);
                assert!(Abs::abs(atan2_neg_half + Asin::asin(one)) < sixteenth);
                assert!(Abs::abs(Atan2::atan2(-one, one) + Atan::atan(one)) < sixteenth);
            }

            #[test]
            fn test_checked_tan() {
                let one = <$ty as AssocConstOne>::ONE;
                let two = one + one;
                let four = two + two;
                let half = one / two;
                let sixteen = four * four;
                let sixteenth = one / sixteen;

                assert_eq!(
                    Object::new(-half).checked_tan(),
                    CheckedTan::checked_tan(-half).map(Object::new)
                );
                assert!(
                    Abs::abs(CheckedTan::checked_tan(-half).unwrap() + Tan::tan(half)) < sixteenth
                );
                assert_eq!(
                    CheckedTan::checked_tan(<$ty as MinExtended>::MIN_EXTENDED),
                    None
                );
            }

            #[test]
            fn test_checked_asin() {
                let one = <$ty as AssocConstOne>::ONE;
                let two = one + one;
                let half = one / two;

                assert_eq!(
                    Object::new(-half).checked_asin(),
                    CheckedAsin::checked_asin(-half).map(Object::new)
                );
                assert!(CheckedAsin::checked_asin(-half).unwrap() < <$ty as AssocConstZero>::ZERO);
                assert_eq!(CheckedAsin::checked_asin(-two), None);
            }

            #[test]
            fn test_checked_acos() {
                let one = <$ty as AssocConstOne>::ONE;
                let two = one + one;

                assert_eq!(
                    Object::new(-one).checked_acos(),
                    CheckedAcos::checked_acos(-one).map(Object::new)
                );
                assert!(CheckedAcos::checked_acos(-one).unwrap() > one);
                assert_eq!(CheckedAcos::checked_acos(-two), None);
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
    f32::atan,
    f32_nonnegative_tests,
    f32_negative_tests
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
    f64::atan,
    f64_nonnegative_tests,
    f64_negative_tests
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
    libm::atanf,
    f32_nonnegative_tests,
    f32_negative_tests
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
    libm::atan,
    f64_nonnegative_tests,
    f64_negative_tests
);
