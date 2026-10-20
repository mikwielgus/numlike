// SPDX-FileCopyrightText: 2026 numlike contributors
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use crate::types::Object;

/// Bundle of exponential functions.
pub trait ExpFns: Exp + Exp2 + ExpM1 {}
impl<T: Exp + Exp2 + ExpM1> ExpFns for T {}

/// Returns `e^(self)` (the exponential function).
pub trait Exp {
    /// The resulting type after evaluating the function.
    type Output;

    /// Returns `e^(self)` (the exponential function).
    fn exp(self) -> Self::Output;
}

impl<B: Exp> Exp for Object<B> {
    type Output = Object<<B as Exp>::Output>;

    fn exp(self) -> Self::Output {
        Object(<B as Exp>::exp(self.0))
    }
}

impl<B: Exp> Object<B> {
    /// Returns `e^(self)` (the exponential function).
    pub fn exp(self) -> Object<<B as Exp>::Output> {
        Object(<B as Exp>::exp(self.0))
    }
}

/// Returns `2^(self)`.
pub trait Exp2 {
    /// The resulting type after evaluating the function.
    type Output;

    /// Returns `2^(self)`.
    fn exp2(self) -> Self::Output;
}

impl<B: Exp2> Exp2 for Object<B> {
    type Output = Object<<B as Exp2>::Output>;

    fn exp2(self) -> Self::Output {
        Object(<B as Exp2>::exp2(self.0))
    }
}

impl<B: Exp2> Object<B> {
    /// Returns `2^(self)`.
    pub fn exp2(self) -> Object<<B as Exp2>::Output> {
        Object(<B as Exp2>::exp2(self.0))
    }
}

/// Returns `e^(self) - 1` in a way that is accurate even if the
/// number is close to zero.
pub trait ExpM1 {
    /// The resulting type after evaluating the function.
    type Output;

    /// Returns `e^(self) - 1` in a way that is accurate even if the
    /// number is close to zero.
    fn exp_m1(self) -> Self::Output;
}

impl<B: ExpM1> ExpM1 for Object<B> {
    type Output = Object<<B as ExpM1>::Output>;

    fn exp_m1(self) -> Self::Output {
        Object(<B as ExpM1>::exp_m1(self.0))
    }
}

impl<B: ExpM1> Object<B> {
    /// Returns `e^(self) - 1` in a way that is accurate even if the
    /// number is close to zero.
    pub fn exp_m1(self) -> Object<<B as ExpM1>::Output> {
        Object(<B as ExpM1>::exp_m1(self.0))
    }
}

/// Bundle of checked exponential functions.
pub trait CheckedExpFns: CheckedExp + CheckedExp2 + CheckedExpM1 {}
impl<T: CheckedExp + CheckedExp2 + CheckedExpM1> CheckedExpFns for T {}

/// Returns `e^(self)`, (the exponential function).
///
/// Returns `None` if the result is not finite.
pub trait CheckedExp {
    /// The resulting type after evaluating the function.
    type Output;

    /// Returns `e^(self)`, (the exponential function).
    fn checked_exp(self) -> Option<Self::Output>;
}

impl<B: CheckedExp> CheckedExp for Object<B> {
    type Output = Object<<B as CheckedExp>::Output>;

    fn checked_exp(self) -> Option<Self::Output> {
        <B as CheckedExp>::checked_exp(self.0).map(Object)
    }
}

impl<B: CheckedExp> Object<B> {
    /// Returns `e^(self)`, (the exponential function).
    ///
    /// Returns `None` if the result is not finite.
    pub fn checked_exp(self) -> Option<Object<<B as CheckedExp>::Output>> {
        <B as CheckedExp>::checked_exp(self.0).map(Object)
    }
}

/// Returns `2^(self)`.
///
/// Returns `None` if the result is not finite.
pub trait CheckedExp2 {
    /// The resulting type after evaluating the function.
    type Output;

    /// Returns `2^(self)`.
    fn checked_exp2(self) -> Option<Self::Output>;
}

impl<B: CheckedExp2> CheckedExp2 for Object<B> {
    type Output = Object<<B as CheckedExp2>::Output>;

    fn checked_exp2(self) -> Option<Self::Output> {
        <B as CheckedExp2>::checked_exp2(self.0).map(Object)
    }
}

impl<B: CheckedExp2> Object<B> {
    /// Returns `2^(self)`.
    ///
    /// Returns `None` if the result is not finite.
    pub fn checked_exp2(self) -> Option<Object<<B as CheckedExp2>::Output>> {
        <B as CheckedExp2>::checked_exp2(self.0).map(Object)
    }
}

/// Returns `e^(self) - 1` in a way that is accurate even if the
/// number is close to zero.
///
/// Returns `None` if the result is not finite.
pub trait CheckedExpM1 {
    /// The resulting type after evaluating the function.
    type Output;

    /// Returns `e^(self) - 1` in a way that is accurate even if the
    /// number is close to zero.
    fn checked_exp_m1(self) -> Option<Self::Output>;
}

impl<B: CheckedExpM1> CheckedExpM1 for Object<B> {
    type Output = Object<<B as CheckedExpM1>::Output>;

    fn checked_exp_m1(self) -> Option<Self::Output> {
        <B as CheckedExpM1>::checked_exp_m1(self.0).map(Object)
    }
}

impl<B: CheckedExpM1> Object<B> {
    /// Returns `e^(self) - 1` in a way that is accurate even if the
    /// number is close to zero.
    ///
    /// Returns `None` if the result is not finite.
    pub fn checked_exp_m1(self) -> Option<Object<<B as CheckedExpM1>::Output>> {
        <B as CheckedExpM1>::checked_exp_m1(self.0).map(Object)
    }
}

#[cfg(any(feature = "std", feature = "libm"))]
macro_rules! impl_exp_traits_for_float {
    (
        $ty:ty,
        $exp:path,
        $exp2:path,
        $expm1:path,
        $nonnegative_tests_mod:ident,
        $negative_tests_mod:ident
    ) => {
        impl Exp for $ty {
            type Output = $ty;

            #[inline]
            fn exp(self) -> Self::Output {
                $exp(self)
            }
        }

        impl Exp2 for $ty {
            type Output = $ty;

            #[inline]
            fn exp2(self) -> Self::Output {
                $exp2(self)
            }
        }

        impl ExpM1 for $ty {
            type Output = $ty;

            #[inline]
            fn exp_m1(self) -> Self::Output {
                $expm1(self)
            }
        }

        impl CheckedExp for $ty {
            type Output = $ty;

            #[inline]
            fn checked_exp(self) -> Option<Self::Output> {
                let result = $exp(self);

                result.is_finite().then_some(result)
            }
        }

        impl CheckedExp2 for $ty {
            type Output = $ty;

            #[inline]
            fn checked_exp2(self) -> Option<Self::Output> {
                let result = $exp2(self);

                result.is_finite().then_some(result)
            }
        }

        impl CheckedExpM1 for $ty {
            type Output = $ty;

            #[inline]
            fn checked_exp_m1(self) -> Option<Self::Output> {
                let result = $expm1(self);

                result.is_finite().then_some(result)
            }
        }

        test_exp_traits_nonnegative!($ty, $nonnegative_tests_mod);
        test_exp_traits_negative!($ty, $negative_tests_mod);
    };
}

#[cfg(any(feature = "std", feature = "libm"))]
macro_rules! test_exp_traits_nonnegative {
    ($ty:ty, $tests_mod:ident) => {
        #[cfg(test)]
        mod $tests_mod {
            use crate::elem::*;
            use crate::fns::*;
            use crate::limits::*;
            use crate::types::Object;

            #[test]
            fn test_exp() {
                let zero = <$ty as ConstZero>::ZERO;
                let one = <$ty as ConstOne>::ONE;
                let two = one + one;

                assert_eq!(Exp::exp(zero), one);
                assert_eq!(Object::new(zero).exp(), Object::new(Exp::exp(zero)));

                let e = Exp::exp(one);

                assert!(e > two);
                assert!(e < two + one);
            }

            #[test]
            fn test_exp2() {
                let zero = <$ty as ConstZero>::ZERO;
                let one = <$ty as ConstOne>::ONE;
                let two = one + one;
                let four = two + two;

                assert_eq!(Exp2::exp2(zero), one);
                assert_eq!(Object::new(zero).exp2(), Object::new(Exp2::exp2(zero)));
                assert_eq!(Exp2::exp2(one), two);
                assert_eq!(Exp2::exp2(two), four);
            }

            #[test]
            fn test_exp_m1() {
                let zero = <$ty as ConstZero>::ZERO;
                let one = <$ty as ConstOne>::ONE;
                let two = one + one;
                let four = two + two;
                let sixteen = four * four;
                let sixteenth = one / sixteen;

                assert_eq!(ExpM1::exp_m1(zero), zero);
                assert_eq!(Object::new(zero).exp_m1(), Object::new(ExpM1::exp_m1(zero)));

                let e_m1 = ExpM1::exp_m1(one);

                assert!(e_m1 > one);
                assert!(e_m1 < two);
                assert!(Abs::abs((e_m1 + one) - Exp::exp(one)) < sixteenth);
            }

            #[test]
            fn test_checked_exp() {
                let zero = <$ty as ConstZero>::ZERO;
                let one = <$ty as ConstOne>::ONE;
                let two = one + one;

                assert_eq!(CheckedExp::checked_exp(zero), Some(one));
                assert_eq!(
                    Object::new(zero).checked_exp(),
                    CheckedExp::checked_exp(zero).map(Object::new)
                );

                let e = CheckedExp::checked_exp(one).unwrap();

                assert!(e > two);
                assert!(e < two + one);

                assert_eq!(
                    CheckedExp::checked_exp(<$ty as MaxFinite>::MAX_FINITE),
                    None
                );
            }

            #[test]
            fn test_checked_exp2() {
                let zero = <$ty as ConstZero>::ZERO;
                let one = <$ty as ConstOne>::ONE;
                let two = one + one;
                let four = two + two;

                assert_eq!(CheckedExp2::checked_exp2(zero), Some(one));
                assert_eq!(
                    Object::new(zero).checked_exp2(),
                    CheckedExp2::checked_exp2(zero).map(Object::new)
                );
                assert_eq!(CheckedExp2::checked_exp2(one), Some(two));
                assert_eq!(CheckedExp2::checked_exp2(two), Some(four));

                assert_eq!(
                    CheckedExp2::checked_exp2(<$ty as MaxFinite>::MAX_FINITE),
                    None
                );
            }

            #[test]
            fn test_checked_exp_m1() {
                let zero = <$ty as ConstZero>::ZERO;
                let one = <$ty as ConstOne>::ONE;
                let two = one + one;

                assert_eq!(CheckedExpM1::checked_exp_m1(zero), Some(zero));
                assert_eq!(
                    Object::new(zero).checked_exp_m1(),
                    CheckedExpM1::checked_exp_m1(zero).map(Object::new)
                );

                let e_m1 = CheckedExpM1::checked_exp_m1(one).unwrap();

                assert!(e_m1 > one);
                assert!(e_m1 < two);

                assert_eq!(
                    CheckedExpM1::checked_exp_m1(<$ty as MaxFinite>::MAX_FINITE),
                    None
                );
            }
        }
    };
}

#[cfg(any(feature = "std", feature = "libm"))]
macro_rules! test_exp_traits_negative {
    ($ty:ty, $tests_mod:ident) => {
        #[cfg(test)]
        mod $tests_mod {
            use crate::elem::*;
            use crate::fns::*;
            use crate::types::Object;

            #[test]
            fn test_exp() {
                let zero = <$ty as ConstZero>::ZERO;
                let one = <$ty as ConstOne>::ONE;
                let two = one + one;
                let four = two + two;
                let sixteen = four * four;
                let sixteenth = one / sixteen;

                let inv_e = Exp::exp(-one);

                assert_eq!(Object::new(-one).exp(), Object::new(Exp::exp(-one)));
                assert!(inv_e > zero);
                assert!(inv_e < one);
                assert!(Abs::abs(inv_e * Exp::exp(one) - one) < sixteenth);
            }

            #[test]
            fn test_exp2() {
                let one = <$ty as ConstOne>::ONE;
                let two = one + one;
                let four = two + two;
                let half = one / two;
                let quarter = one / four;

                assert_eq!(Exp2::exp2(-one), half);
                assert_eq!(Object::new(-one).exp2(), Object::new(Exp2::exp2(-one)));
                assert_eq!(Exp2::exp2(-two), quarter);
            }

            #[test]
            fn test_exp_m1() {
                let zero = <$ty as ConstZero>::ZERO;
                let one = <$ty as ConstOne>::ONE;
                let two = one + one;
                let four = two + two;
                let sixteen = four * four;
                let sixteenth = one / sixteen;

                let inv_e_m1 = ExpM1::exp_m1(-one);

                assert_eq!(Object::new(-one).exp_m1(), Object::new(ExpM1::exp_m1(-one)));
                assert!(inv_e_m1 < zero);
                assert!(inv_e_m1 > -one);
                assert!(Abs::abs((inv_e_m1 + one) - Exp::exp(-one)) < sixteenth);
            }

            #[test]
            fn test_checked_exp() {
                let zero = <$ty as ConstZero>::ZERO;
                let one = <$ty as ConstOne>::ONE;

                let inv_e = CheckedExp::checked_exp(-one).unwrap();

                assert_eq!(
                    Object::new(-one).checked_exp(),
                    CheckedExp::checked_exp(-one).map(Object::new)
                );
                assert!(inv_e > zero);
                assert!(inv_e < one);
            }

            #[test]
            fn test_checked_exp2() {
                let one = <$ty as ConstOne>::ONE;
                let two = one + one;
                let four = two + two;
                let half = one / two;
                let quarter = one / four;

                assert_eq!(CheckedExp2::checked_exp2(-one), Some(half));
                assert_eq!(
                    Object::new(-one).checked_exp2(),
                    CheckedExp2::checked_exp2(-one).map(Object::new)
                );
                assert_eq!(CheckedExp2::checked_exp2(-two), Some(quarter));
            }

            #[test]
            fn test_checked_exp_m1() {
                let zero = <$ty as ConstZero>::ZERO;
                let one = <$ty as ConstOne>::ONE;

                let inv_e_m1 = CheckedExpM1::checked_exp_m1(-one).unwrap();

                assert_eq!(
                    Object::new(-one).checked_exp_m1(),
                    CheckedExpM1::checked_exp_m1(-one).map(Object::new)
                );
                assert!(inv_e_m1 < zero);
                assert!(inv_e_m1 > -one);
            }
        }
    };
}

#[cfg(feature = "std")]
impl_exp_traits_for_float!(
    f32,
    f32::exp,
    f32::exp2,
    f32::exp_m1,
    f32_nonnegative_tests,
    f32_negative_tests
);
#[cfg(feature = "std")]
impl_exp_traits_for_float!(
    f64,
    f64::exp,
    f64::exp2,
    f64::exp_m1,
    f64_nonnegative_tests,
    f64_negative_tests
);
#[cfg(all(not(feature = "std"), feature = "libm"))]
impl_exp_traits_for_float!(
    f32,
    libm::expf,
    libm::exp2f,
    libm::expm1f,
    f32_nonnegative_tests,
    f32_negative_tests
);
#[cfg(all(not(feature = "std"), feature = "libm"))]
impl_exp_traits_for_float!(
    f64,
    libm::exp,
    libm::exp2,
    libm::expm1,
    f64_nonnegative_tests,
    f64_negative_tests
);
