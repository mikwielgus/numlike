// SPDX-FileCopyrightText: 2026 numlike contributors
//
// SPDX-License-Identifier: MIT OR Apache-2.0

/// Bundle of exponential functions.
pub trait ExpFns: Exp + Exp2 + ExpM1 {}
impl<T: Exp + Exp2 + ExpM1> ExpFns for T {}

/// Returns `e^(self)`, (the exponential function).
pub trait Exp {
    /// The resulting type after applying the operation.
    type Output;

    /// Returns `e^(self)`, (the exponential function).
    fn exp(self) -> Self::Output;
}

/// Returns `2^(self)`.
pub trait Exp2 {
    /// The resulting type after applying the operation.
    type Output;

    /// Returns `2^(self)`.
    fn exp2(self) -> Self::Output;
}

/// Returns `e^(self) - 1` in a way that is accurate even if the
/// number is close to zero.
pub trait ExpM1 {
    /// The resulting type after applying the operation.
    type Output;

    /// Returns `e^(self) - 1` in a way that is accurate even if the
    /// number is close to zero.
    fn exp_m1(self) -> Self::Output;
}

/// Bundle of checked exponential functions.
pub trait CheckedExpFns: CheckedExp + CheckedExp2 + CheckedExpM1 {}
impl<T: CheckedExp + CheckedExp2 + CheckedExpM1> CheckedExpFns for T {}

/// Returns `e^(self)`, (the exponential function).
///
/// Returns `None` if the result is not finite.
pub trait CheckedExp {
    /// The resulting type after applying the operation.
    type Output;

    /// Returns `e^(self)`, (the exponential function).
    fn checked_exp(self) -> Option<Self::Output>;
}

/// Returns `2^(self)`.
///
/// Returns `None` if the result is not finite.
pub trait CheckedExp2 {
    /// The resulting type after applying the operation.
    type Output;

    /// Returns `2^(self)`.
    fn checked_exp2(self) -> Option<Self::Output>;
}

/// Returns `e^(self) - 1` in a way that is accurate even if the
/// number is close to zero.
///
/// Returns `None` if the result is not finite.
pub trait CheckedExpM1 {
    /// The resulting type after applying the operation.
    type Output;

    /// Returns `e^(self) - 1` in a way that is accurate even if the
    /// number is close to zero.
    fn checked_exp_m1(self) -> Option<Self::Output>;
}

#[cfg(any(feature = "std", feature = "libm"))]
macro_rules! impl_exp_traits_for_float {
    ($ty:ty, $exp:path, $exp2:path, $expm1:path) => {
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
    };
}

#[cfg(feature = "std")]
impl_exp_traits_for_float!(f32, f32::exp, f32::exp2, f32::exp_m1);
#[cfg(feature = "std")]
impl_exp_traits_for_float!(f64, f64::exp, f64::exp2, f64::exp_m1);
#[cfg(all(not(feature = "std"), feature = "libm"))]
impl_exp_traits_for_float!(f32, libm::expf, libm::exp2f, libm::expm1f);
#[cfg(all(not(feature = "std"), feature = "libm"))]
impl_exp_traits_for_float!(f64, libm::exp, libm::exp2, libm::expm1);
