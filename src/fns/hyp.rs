// SPDX-FileCopyrightText: 2026 numlike contributors
//
// SPDX-License-Identifier: MIT OR Apache-2.0

/*pub trait FullHypFns<Rhs = Self>: HypFns + InvHypFns<Rhs> {}
impl<Rhs, T: HypFns + InvHypFns<Rhs>> FullHypFns<Rhs> for T {}*/

/// Bundle of hyperbolic functions.
pub trait HypFns: Sinh + Cosh + Tanh {}
impl<T: Sinh + Cosh + Tanh> HypFns for T {}

/// Hyperbolic sine function.
pub trait Sinh {
    /// The resulting type after applying the operation.
    type Output;

    /// Hyperbolic sine function.
    fn sinh(self) -> Self::Output;
}

/// Hyperbolic cosine function.
pub trait Cosh {
    /// The resulting type after applying the operation.
    type Output;

    /// Hyperbolic cosine function.
    fn cosh(self) -> Self::Output;
}

/// Hyperbolic tangent function.
pub trait Tanh {
    /// The resulting type after applying the operation.
    type Output;

    /// Hyperbolic tangent function.
    fn tanh(self) -> Self::Output;
}

/// Bundle of checked hyperbolic functions.
pub trait CheckedHypFns: CheckedSinh + CheckedCosh {}
impl<T: CheckedSinh + CheckedCosh> CheckedHypFns for T {}

/// Hyperbolic sine function.
///
/// Returns `None` if the result is not finite.
pub trait CheckedSinh {
    /// The resulting type after applying the operation.
    type Output;

    /// Hyperbolic sine function.
    fn checked_sinh(self) -> Option<Self::Output>;
}

/// Hyperbolic cosine function.
///
/// Returns `None` if the result is not finite.
pub trait CheckedCosh {
    /// The resulting type after applying the operation.
    type Output;

    /// Hyperbolic cosine function.
    fn checked_cosh(self) -> Option<Self::Output>;
}

// No checked `atanh` because it does not have restricted domain.

/// Bundle of inverse hyperbolic functions.
pub trait InvHypFns<Rhs = Self>: Asinh + Acosh + Atanh {}
impl<Rhs, T: Asinh + Acosh + Atanh> InvHypFns<Rhs> for T {}

/// Inverse hyperbolic sine function.
pub trait Asinh {
    /// The resulting type after applying the operation.
    type Output;

    /// Inverse hyperbolic sine function.
    fn asinh(self) -> Self::Output;
}

/// Inverse hyperbolic cosine function.
pub trait Acosh {
    /// The resulting type after applying the operation.
    type Output;

    /// Inverse hyperbolic cosine function.
    fn acosh(self) -> Self::Output;
}

/// Inverse hyperbolic tangent function.
pub trait Atanh {
    /// The resulting type after applying the operation.
    type Output;

    /// Inverse hyperbolic tangent function.
    fn atanh(self) -> Self::Output;
}

/// Bundle of checked inverse hyperbolic functions.
pub trait CheckedInvHypFns<Rhs = Self>: /*CheckedAsinh +*/ CheckedAcosh + CheckedAtanh {}
impl<Rhs, T: CheckedAcosh + CheckedAtanh> CheckedInvHypFns<Rhs> for T {}

// No need for checked asinh, since it's defined for all reals.

/*pub trait CheckedAsinh {
    /// The resulting type after applying the operation.
    type Output;

    fn checked_asinh(self) -> Option<Self::Output>;
}*/

/// Inverse hyperbolic cosine function.
///
/// Returns `None` if the result is not finite.
pub trait CheckedAcosh {
    /// The resulting type after applying the operation.
    type Output;

    /// Inverse hyperbolic cosine function.
    fn checked_acosh(self) -> Option<Self::Output>;
}

/// Inverse hyperbolic tangent function.
///
/// Returns `None` if the result is not finite.
pub trait CheckedAtanh {
    /// The resulting type after applying the operation.
    type Output;

    /// Inverse hyperbolic tangent function.
    fn checked_atanh(self) -> Option<Self::Output>;
}

#[cfg(any(feature = "std", feature = "libm"))]
macro_rules! impl_hyp_traits_for_float {
    (
        $ty:ty,
        $sinh:path,
        $cosh:path,
        $tanh:path,
        $asinh:path,
        $acosh:path,
        $atanh:path
    ) => {
        impl Sinh for $ty {
            type Output = $ty;

            #[inline]
            fn sinh(self) -> Self::Output {
                $sinh(self)
            }
        }

        impl Cosh for $ty {
            type Output = $ty;

            #[inline]
            fn cosh(self) -> Self::Output {
                $cosh(self)
            }
        }

        impl Tanh for $ty {
            type Output = $ty;

            #[inline]
            fn tanh(self) -> Self::Output {
                $tanh(self)
            }
        }

        impl CheckedSinh for $ty {
            type Output = $ty;

            #[inline]
            fn checked_sinh(self) -> Option<Self::Output> {
                let result = $sinh(self);

                result.is_finite().then_some(result)
            }
        }

        impl CheckedCosh for $ty {
            type Output = $ty;

            #[inline]
            fn checked_cosh(self) -> Option<Self::Output> {
                let result = $cosh(self);

                result.is_finite().then_some(result)
            }
        }

        impl Asinh for $ty {
            type Output = $ty;

            #[inline]
            fn asinh(self) -> Self::Output {
                $asinh(self)
            }
        }

        impl Acosh for $ty {
            type Output = $ty;

            #[inline]
            fn acosh(self) -> Self::Output {
                $acosh(self)
            }
        }

        impl Atanh for $ty {
            type Output = $ty;

            #[inline]
            fn atanh(self) -> Self::Output {
                $atanh(self)
            }
        }

        /*impl CheckedAsinh for $ty {
            type Output = $ty;

            #[inline]
            fn checked_asinh(self) -> Option<Self::Output> {
                let result = $asinh(self);

                result.is_finite().then_some(result)
            }
        }*/

        impl CheckedAcosh for $ty {
            type Output = $ty;

            #[inline]
            fn checked_acosh(self) -> Option<Self::Output> {
                let result = $acosh(self);

                result.is_finite().then_some(result)
            }
        }

        impl CheckedAtanh for $ty {
            type Output = $ty;

            #[inline]
            fn checked_atanh(self) -> Option<Self::Output> {
                let result = $atanh(self);

                result.is_finite().then_some(result)
            }
        }
    };
    (
        $ty:ty,
        $sinh:path,
        $cosh:path,
        $tanh:path,
        $asinh:path,
        $acosh:path,
        $atanh:path,
        $nonnegative_tests_mod:ident,
        $negative_tests_mod:ident
    ) => {
        impl_hyp_traits_for_float!($ty, $sinh, $cosh, $tanh, $asinh, $acosh, $atanh);

        test_hyp_traits_nonnegative!($ty, $nonnegative_tests_mod);
        test_hyp_traits_negative!($ty, $negative_tests_mod);
    };
}

#[cfg(any(feature = "std", feature = "libm"))]
macro_rules! test_hyp_traits_nonnegative {
    ($ty:ty, $tests_mod:ident) => {
        #[cfg(test)]
        mod $tests_mod {
            use crate::elem::*;
            use crate::fns::*;
            use crate::limits::*;

            #[test]
            fn test_sinh() {
                let zero = <$ty as Zero>::ZERO;
                let one = <$ty as One>::ONE;
                let two = one + one;
                let four = two + two;
                let sixteen = four * four;
                let sixteenth = one / sixteen;

                assert_eq!(Sinh::sinh(zero), zero);

                let sinh_one = Sinh::sinh(one);

                assert!(sinh_one > one);
                assert!(sinh_one < two);
                assert!(Abs::abs(Asinh::asinh(sinh_one) - one) < sixteenth);
            }

            #[test]
            fn test_cosh() {
                let zero = <$ty as Zero>::ZERO;
                let one = <$ty as One>::ONE;
                let two = one + one;
                let four = two + two;
                let sixteen = four * four;
                let sixteenth = one / sixteen;

                assert_eq!(Cosh::cosh(zero), one);

                let cosh_one = Cosh::cosh(one);

                assert!(cosh_one > one);
                assert!(cosh_one < two);
                assert!(Abs::abs(Acosh::acosh(cosh_one) - one) < sixteenth);
            }

            #[test]
            fn test_tanh() {
                let zero = <$ty as Zero>::ZERO;
                let one = <$ty as One>::ONE;
                let two = one + one;
                let four = two + two;
                let sixteen = four * four;
                let sixteenth = one / sixteen;

                assert_eq!(Tanh::tanh(zero), zero);

                let tanh_one = Tanh::tanh(one);

                assert!(tanh_one > zero);
                assert!(tanh_one < one);
                assert!(Abs::abs(Atanh::atanh(tanh_one) - one) < sixteenth);
            }

            #[test]
            fn test_asinh() {
                let zero = <$ty as Zero>::ZERO;
                let one = <$ty as One>::ONE;
                let two = one + one;
                let four = two + two;
                let sixteen = four * four;
                let sixteenth = one / sixteen;

                assert_eq!(Asinh::asinh(zero), zero);

                let asinh_one = Asinh::asinh(one);

                assert!(asinh_one > zero);
                assert!(asinh_one < one);
                assert!(Abs::abs(Sinh::sinh(asinh_one) - one) < sixteenth);
            }

            #[test]
            fn test_acosh() {
                let zero = <$ty as Zero>::ZERO;
                let one = <$ty as One>::ONE;
                let two = one + one;
                let four = two + two;
                let sixteen = four * four;
                let sixteenth = one / sixteen;

                assert_eq!(Acosh::acosh(one), zero);

                let acosh_two = Acosh::acosh(two);

                assert!(acosh_two > one);
                assert!(acosh_two < two);
                assert!(Abs::abs(Cosh::cosh(acosh_two) - two) < sixteenth);
            }

            #[test]
            fn test_atanh() {
                let zero = <$ty as Zero>::ZERO;
                let one = <$ty as One>::ONE;
                let two = one + one;
                let four = two + two;
                let half = one / two;
                let sixteen = four * four;
                let sixteenth = one / sixteen;

                assert_eq!(Atanh::atanh(zero), zero);

                let atanh_half = Atanh::atanh(half);

                assert!(atanh_half > zero);
                assert!(atanh_half < one);
                assert!(Abs::abs(Tanh::tanh(atanh_half) - half) < sixteenth);
            }

            #[test]
            fn test_checked_sinh() {
                let zero = <$ty as Zero>::ZERO;
                let one = <$ty as One>::ONE;

                assert_eq!(CheckedSinh::checked_sinh(zero), Some(zero));
                assert!(CheckedSinh::checked_sinh(one).unwrap() > one);
                assert_eq!(
                    CheckedSinh::checked_sinh(<$ty as MaxFinite>::MAX_FINITE),
                    None
                );
            }

            #[test]
            fn test_checked_cosh() {
                let zero = <$ty as Zero>::ZERO;
                let one = <$ty as One>::ONE;

                assert_eq!(CheckedCosh::checked_cosh(zero), Some(one));
                assert!(CheckedCosh::checked_cosh(one).unwrap() > one);
                assert_eq!(
                    CheckedCosh::checked_cosh(<$ty as MaxFinite>::MAX_FINITE),
                    None
                );
            }

            #[test]
            fn test_checked_acosh() {
                let zero = <$ty as Zero>::ZERO;
                let one = <$ty as One>::ONE;
                let two = one + one;

                assert_eq!(CheckedAcosh::checked_acosh(one), Some(zero));
                assert!(CheckedAcosh::checked_acosh(two).unwrap() > one);
                assert_eq!(CheckedAcosh::checked_acosh(zero), None);
            }

            #[test]
            fn test_checked_atanh() {
                let zero = <$ty as Zero>::ZERO;
                let one = <$ty as One>::ONE;
                let two = one + one;
                let half = one / two;

                assert_eq!(CheckedAtanh::checked_atanh(zero), Some(zero));
                assert!(CheckedAtanh::checked_atanh(half).unwrap() > zero);
                assert_eq!(CheckedAtanh::checked_atanh(one), None);
            }
        }
    };
}

#[cfg(any(feature = "std", feature = "libm"))]
macro_rules! test_hyp_traits_negative {
    ($ty:ty, $tests_mod:ident) => {
        #[cfg(test)]
        mod $tests_mod {
            use crate::elem::*;
            use crate::fns::*;
            use crate::limits::*;

            #[test]
            fn test_sinh() {
                let one = <$ty as One>::ONE;
                let two = one + one;
                let four = two + two;
                let sixteen = four * four;
                let sixteenth = one / sixteen;

                assert!(Abs::abs(Sinh::sinh(-one) + Sinh::sinh(one)) < sixteenth);
            }

            #[test]
            fn test_cosh() {
                let one = <$ty as One>::ONE;
                let two = one + one;
                let four = two + two;
                let sixteen = four * four;
                let sixteenth = one / sixteen;

                assert!(Abs::abs(Cosh::cosh(-one) - Cosh::cosh(one)) < sixteenth);
            }

            #[test]
            fn test_tanh() {
                let one = <$ty as One>::ONE;
                let two = one + one;
                let four = two + two;
                let sixteen = four * four;
                let sixteenth = one / sixteen;

                assert!(Abs::abs(Tanh::tanh(-one) + Tanh::tanh(one)) < sixteenth);
            }

            #[test]
            fn test_asinh() {
                let one = <$ty as One>::ONE;
                let two = one + one;
                let four = two + two;
                let sixteen = four * four;
                let sixteenth = one / sixteen;

                assert!(Abs::abs(Asinh::asinh(-one) + Asinh::asinh(one)) < sixteenth);
            }

            #[test]
            fn test_atanh() {
                let one = <$ty as One>::ONE;
                let two = one + one;
                let four = two + two;
                let half = one / two;
                let sixteen = four * four;
                let sixteenth = one / sixteen;

                assert!(Abs::abs(Atanh::atanh(-half) + Atanh::atanh(half)) < sixteenth);
            }

            #[test]
            fn test_checked_sinh() {
                let one = <$ty as One>::ONE;

                assert!(CheckedSinh::checked_sinh(-one).unwrap() < -one);
                assert_eq!(
                    CheckedSinh::checked_sinh(-<$ty as MaxFinite>::MAX_FINITE),
                    None
                );
            }

            #[test]
            fn test_checked_cosh() {
                let one = <$ty as One>::ONE;

                assert!(CheckedCosh::checked_cosh(-one).unwrap() > one);
                assert_eq!(
                    CheckedCosh::checked_cosh(-<$ty as MaxFinite>::MAX_FINITE),
                    None
                );
            }

            #[test]
            fn test_checked_acosh() {
                let one = <$ty as One>::ONE;

                assert_eq!(CheckedAcosh::checked_acosh(-one), None);
            }

            #[test]
            fn test_checked_atanh() {
                let one = <$ty as One>::ONE;
                let two = one + one;
                let half = one / two;

                assert!(CheckedAtanh::checked_atanh(-half).unwrap() < <$ty as Zero>::ZERO);
                assert_eq!(CheckedAtanh::checked_atanh(-one), None);
            }
        }
    };
}

#[cfg(feature = "std")]
impl_hyp_traits_for_float!(
    f32,
    f32::sinh,
    f32::cosh,
    f32::tanh,
    f32::asinh,
    f32::acosh,
    f32::atanh,
    f32_nonnegative_tests,
    f32_negative_tests
);
#[cfg(feature = "std")]
impl_hyp_traits_for_float!(
    f64,
    f64::sinh,
    f64::cosh,
    f64::tanh,
    f64::asinh,
    f64::acosh,
    f64::atanh,
    f64_nonnegative_tests,
    f64_negative_tests
);
#[cfg(all(not(feature = "std"), feature = "libm"))]
impl_hyp_traits_for_float!(
    f32,
    libm::sinhf,
    libm::coshf,
    libm::tanhf,
    libm::asinhf,
    libm::acoshf,
    libm::atanhf,
    f32_nonnegative_tests,
    f32_negative_tests
);
#[cfg(all(not(feature = "std"), feature = "libm"))]
impl_hyp_traits_for_float!(
    f64,
    libm::sinh,
    libm::cosh,
    libm::tanh,
    libm::asinh,
    libm::acosh,
    libm::atanh,
    f64_nonnegative_tests,
    f64_negative_tests
);
