// SPDX-FileCopyrightText: 2026 numlike contributors
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use crate::types::Object;

/// Bundle of floating-point logarithm functions.
pub trait LogFns<Rhs = Self>: Log<Rhs> + Ln + Log2 + Log10 + Ln1p {}
impl<Rhs, T: Log<Rhs> + Ln + Log2 + Log10 + Ln1p> LogFns<Rhs> for T {}

/// Returns the logarithm of the number with respect to an arbitrary base.
///
/// This returns NaN when the number is negative, and negative infinity when number is zero.
///
/// The result might not be correctly rounded owing to implementation details;
/// `self.log2()` can produce more accurate results for base 2, and
/// `self.log10()` can produce more accurate results for base 10.
pub trait Log<Rhs = Self> {
    /// The resulting type after evaluating the function.
    type Output;

    /// Returns the logarithm of the number with respect to an arbitrary base.
    fn log(self, base: Rhs) -> Self::Output;
}

impl<B> Object<B> {
    /// Returns the logarithm of the number with respect to an arbitrary base.
    fn log<Rhs>(self, base: Rhs) -> Object<<B as Log<Rhs>>::Output>
    where
        B: Log<Rhs>,
    {
        Object(<B as Log<Rhs>>::log(self.0, base))
    }
}

/// Returns the natural logarithm of the number.
///
/// This returns NaN when the number is negative, and negative infinity when number is zero.
pub trait Ln {
    /// The resulting type after evaluating the function.
    type Output;

    /// Returns the natural logarithm of the number.
    fn ln(self) -> Self::Output;
}

impl<B: Ln> Object<B> {
    /// Returns the natural logarithm of the number.
    fn ln(self) -> Object<<B as Ln>::Output> {
        Object(<B as Ln>::ln(self.0))
    }
}

/// Returns the base 2 logarithm of the number.
///
/// This returns NaN when the number is negative, and negative infinity when number is zero.
pub trait Log2 {
    /// The resulting type after evaluating the function.
    type Output;

    /// Returns the base 2 logarithm of the number.
    fn log2(self) -> Self::Output;
}

impl<B: Log2> Object<B> {
    /// Returns the base 2 logarithm of the number.
    fn log2(self) -> Object<<B as Log2>::Output> {
        Object(<B as Log2>::log2(self.0))
    }
}

/// Returns the base 10 logarithm of the number.
///
/// This returns NaN when the number is negative, and negative infinity when number is zero.
pub trait Log10 {
    /// The resulting type after evaluating the function.
    type Output;

    /// Returns the base 10 logarithm of the number.
    fn log10(self) -> Self::Output;
}

impl<B: Log10> Object<B> {
    /// Returns the base 10 logarithm of the number.
    fn log10(self) -> Object<<B as Log10>::Output> {
        Object(<B as Log10>::log10(self.0))
    }
}

/// Returns `ln(1+n)` (natural logarithm) more accurately than if
/// the operations were performed separately.
///
/// This returns NaN when `n < -1.0`, and negative infinity when `n == -1.0`.
pub trait Ln1p {
    /// The resulting type after evaluating the function.
    type Output;

    /// Returns `ln(1+n)` (natural logarithm) more accurately than if
    /// the operations were performed separately.
    fn ln_1p(self) -> Self::Output;
}

impl<B: Ln1p> Object<B> {
    /// Returns `ln(1+n)` (natural logarithm) more accurately than if
    /// the operations were performed separately.
    fn ln_1p(self) -> Object<<B as Ln1p>::Output> {
        Object(<B as Ln1p>::ln_1p(self.0))
    }
}

/// Bundle of checked floating-point logarithm functions.
pub trait CheckedLogFns<Rhs = Self>:
    CheckedLog<Rhs> + CheckedLn + CheckedLog2 + CheckedLog10 + CheckedLn1p
{
}
impl<Rhs, T: CheckedLog<Rhs> + CheckedLn + CheckedLog2 + CheckedLog10 + CheckedLn1p>
    CheckedLogFns<Rhs> for T
{
}

/// Returns the logarithm of the number with respect to an arbitrary base.
///
/// Returns `None` if the result is not finite.
///
/// The result might not be correctly rounded owing to implementation details;
/// `self.checked_log2()` can produce more accurate results for base 2, and
/// `self.checked_log10()` can produce more accurate results for base 10.
pub trait CheckedLog<Rhs = Self> {
    /// The resulting type after evaluating the function.
    type Output;

    /// Returns the logarithm of the number with respect to an arbitrary base.
    fn checked_log(self, base: Rhs) -> Option<Self::Output>;
}

impl<B> Object<B> {
    /// Returns the logarithm of the number with respect to an arbitrary base.
    ///
    /// Returns `None` if the result is not finite.
    fn checked_log<Rhs>(self, base: Rhs) -> Option<Object<<B as CheckedLog<Rhs>>::Output>>
    where
        B: CheckedLog<Rhs>,
    {
        <B as CheckedLog<Rhs>>::checked_log(self.0, base).map(Object)
    }
}

/// Returns the natural logarithm of the number.
///
/// Returns `None` if the result is not finite.
pub trait CheckedLn {
    /// The resulting type after evaluating the function.
    type Output;

    /// Returns the natural logarithm of the number.
    fn checked_ln(self) -> Option<Self::Output>;
}

impl<B: CheckedLn> Object<B> {
    /// Returns the natural logarithm of the number.
    ///
    /// Returns `None` if the result is not finite.
    fn checked_ln(self) -> Option<Object<<B as CheckedLn>::Output>> {
        <B as CheckedLn>::checked_ln(self.0).map(Object)
    }
}

/// Returns the base 2 logarithm of the number.
///
/// Returns `None` if the result is not finite.
pub trait CheckedLog2 {
    /// The resulting type after evaluating the function.
    type Output;

    /// Returns the base 2 logarithm of the number.
    fn checked_log2(self) -> Option<Self::Output>;
}

impl<B: CheckedLog2> Object<B> {
    /// Returns the base 2 logarithm of the number.
    ///
    /// Returns `None` if the result is not finite.
    fn checked_log2(self) -> Option<Object<<B as CheckedLog2>::Output>> {
        <B as CheckedLog2>::checked_log2(self.0).map(Object)
    }
}

/// Returns the base 10 logarithm of the number.
///
/// Returns `None` if the result is not finite.
pub trait CheckedLog10 {
    /// The resulting type after evaluating the function.
    type Output;

    /// Returns the base 10 logarithm of the number.
    fn checked_log10(self) -> Option<Self::Output>;
}

impl<B: CheckedLog10> Object<B> {
    /// Returns the base 10 logarithm of the number.
    ///
    /// Returns `None` if the result is not finite.
    fn checked_log10(self) -> Option<Object<<B as CheckedLog10>::Output>> {
        <B as CheckedLog10>::checked_log10(self.0).map(Object)
    }
}

/// Returns `ln(1+n)` (natural logarithm) more accurately than if
/// the operations were performed separately.
///
/// Returns `None` if the result is not finite.
pub trait CheckedLn1p {
    /// The resulting type after evaluating the function.
    type Output;

    /// Returns `ln(1+n)` (natural logarithm) more accurately than if
    /// the operations were performed separately.
    fn checked_ln_1p(self) -> Option<Self::Output>;
}

impl<B: CheckedLn1p> Object<B> {
    /// Returns `ln(1+n)` (natural logarithm) more accurately than if
    /// the operations were performed separately.
    ///
    /// Returns `None` if the result is not finite.
    fn checked_ln_1p(self) -> Option<Object<<B as CheckedLn1p>::Output>> {
        <B as CheckedLn1p>::checked_ln_1p(self.0).map(Object)
    }
}

/// Bundle of integer logarithm functions.
pub trait IlogFns<Rhs = Self>: Ilog<Rhs> + Ilog2 + Ilog10 {}
impl<Rhs, T: Ilog<Rhs> + Ilog2 + Ilog10> IlogFns<Rhs> for T {}

/// Returns the logarithm of the number with respect to an arbitrary base,
/// rounded down.
///
/// This method might not be optimized owing to implementation details;
/// `ilog2` can produce results more efficiently for base 2, and `ilog10`
/// can produce results more efficiently for base 10.
///
/// # Panics
///
/// This function will panic if `self` is less than or equal to zero,
/// or if `base` is less than 2.
pub trait Ilog<Rhs = Self> {
    /// The resulting type after evaluating the function.
    type Output;

    /// Returns the logarithm of the number with respect to an arbitrary base,
    /// rounded down.
    fn ilog(self, base: Rhs) -> Self::Output;
}

impl<B> Object<B> {
    /// Returns the logarithm of the number with respect to an arbitrary base,
    /// rounded down.
    fn ilog<Rhs>(self, base: Rhs) -> Object<<B as Ilog<Rhs>>::Output>
    where
        B: Ilog<Rhs>,
    {
        Object(<B as Ilog<Rhs>>::ilog(self.0, base))
    }
}

/// Returns the base 2 logarithm of the number, rounded down.
///
/// # Panics
///
/// This function will panic if `self` is less than or equal to zero.
pub trait Ilog2 {
    /// The resulting type after evaluating the function.
    type Output;

    /// Returns the base 2 logarithm of the number, rounded down.
    fn ilog2(self) -> Self::Output;
}

impl<B: Ilog2> Object<B> {
    /// Returns the base 2 logarithm of the number, rounded down.
    fn ilog2(self) -> Object<<B as Ilog2>::Output> {
        Object(<B as Ilog2>::ilog2(self.0))
    }
}

/// Returns the base 10 logarithm of the number, rounded down.
///
/// # Panics
///
/// This function will panic if `self` is less than or equal to zero.
pub trait Ilog10 {
    /// The resulting type after evaluating the function.
    type Output;

    /// Returns the base 10 logarithm of the number, rounded down.
    fn ilog10(self) -> Self::Output;
}

impl<B: Ilog10> Object<B> {
    /// Returns the base 10 logarithm of the number, rounded down.
    fn ilog10(self) -> Object<<B as Ilog10>::Output> {
        Object(<B as Ilog10>::ilog10(self.0))
    }
}

/// Bundle of checked integer logarithm functions.
pub trait CheckedIlogFns<Rhs = Self>: CheckedIlog<Rhs> + CheckedIlog2 + CheckedIlog10 {}
impl<Rhs, T: CheckedIlog<Rhs> + CheckedIlog2 + CheckedIlog10> CheckedIlogFns<Rhs> for T {}

/// Returns the logarithm of the number with respect to an arbitrary base,
/// rounded down.
///
/// Returns `None` if `self` is less than or equal to zero, or if the base is not at least 2.
///
/// This method might not be optimized owing to implementation details;
/// `checked_ilog2` can produce results more efficiently for base 2, and
/// `checked_ilog10` can produce results more efficiently for base 10.
pub trait CheckedIlog<Rhs = Self> {
    /// The resulting type after evaluating the function.
    type Output;

    /// Returns the logarithm of the number with respect to an arbitrary base,
    /// rounded down.
    fn checked_ilog(self, base: Rhs) -> Option<Self::Output>;
}

impl<B> Object<B> {
    /// Returns the logarithm of the number with respect to an arbitrary base,
    /// rounded down.
    ///
    /// Returns `None` if `self` is less than or equal to zero, or if the base is not at least 2.
    fn checked_ilog<Rhs>(self, base: Rhs) -> Option<Object<<B as CheckedIlog<Rhs>>::Output>>
    where
        B: CheckedIlog<Rhs>,
    {
        <B as CheckedIlog<Rhs>>::checked_ilog(self.0, base).map(Object)
    }
}

/// Returns the base 2 logarithm of the number, rounded down.
///
/// Returns `None` if `self` is less than or equal to zero.
pub trait CheckedIlog2 {
    /// The resulting type after evaluating the function.
    type Output;

    /// Returns the base 2 logarithm of the number, rounded down.
    fn checked_ilog2(self) -> Option<Self::Output>;
}

impl<B: CheckedIlog2> Object<B> {
    /// Returns the base 2 logarithm of the number, rounded down.
    ///
    /// Returns `None` if `self` is less than or equal to zero.
    fn checked_ilog2(self) -> Option<Object<<B as CheckedIlog2>::Output>> {
        <B as CheckedIlog2>::checked_ilog2(self.0).map(Object)
    }
}

/// Returns the base 10 logarithm of the number, rounded down.
///
/// Returns `None` if `self` is less than or equal to zero.
pub trait CheckedIlog10 {
    /// The resulting type after evaluating the function.
    type Output;

    /// Returns the base 10 logarithm of the number, rounded down.
    fn checked_ilog10(self) -> Option<Self::Output>;
}

impl<B: CheckedIlog10> Object<B> {
    /// Returns the base 10 logarithm of the number, rounded down.
    ///
    /// Returns `None` if `self` is less than or equal to zero.
    fn checked_ilog10(self) -> Option<Object<<B as CheckedIlog10>::Output>> {
        <B as CheckedIlog10>::checked_ilog10(self.0).map(Object)
    }
}

#[cfg(any(feature = "std", feature = "libm"))]
macro_rules! impl_log_traits_for_float {
    ($ty:ty, $ln:path, $log2:path, $log10:path, $ln_1p:path) => {
        impl Log<$ty> for $ty {
            type Output = $ty;

            #[inline]
            fn log(self, base: $ty) -> Self::Output {
                $ln(self) / $ln(base)
            }
        }

        impl Ln for $ty {
            type Output = $ty;

            #[inline]
            fn ln(self) -> Self::Output {
                $ln(self)
            }
        }

        impl Log2 for $ty {
            type Output = $ty;

            #[inline]
            fn log2(self) -> Self::Output {
                $log2(self)
            }
        }

        impl Log10 for $ty {
            type Output = $ty;

            #[inline]
            fn log10(self) -> Self::Output {
                $log10(self)
            }
        }

        impl Ln1p for $ty {
            type Output = $ty;

            #[inline]
            fn ln_1p(self) -> Self::Output {
                $ln_1p(self)
            }
        }

        impl CheckedLog<$ty> for $ty {
            type Output = $ty;

            #[inline]
            fn checked_log(self, base: $ty) -> Option<Self::Output> {
                let result = $ln(self) / $ln(base);

                result.is_finite().then_some(result)
            }
        }

        impl CheckedLn for $ty {
            type Output = $ty;

            #[inline]
            fn checked_ln(self) -> Option<Self::Output> {
                let result = $ln(self);

                result.is_finite().then_some(result)
            }
        }

        impl CheckedLog2 for $ty {
            type Output = $ty;

            #[inline]
            fn checked_log2(self) -> Option<Self::Output> {
                let result = $log2(self);

                result.is_finite().then_some(result)
            }
        }

        impl CheckedLog10 for $ty {
            type Output = $ty;

            #[inline]
            fn checked_log10(self) -> Option<Self::Output> {
                let result = $log10(self);

                result.is_finite().then_some(result)
            }
        }

        impl CheckedLn1p for $ty {
            type Output = $ty;

            #[inline]
            fn checked_ln_1p(self) -> Option<Self::Output> {
                let result = $ln_1p(self);

                result.is_finite().then_some(result)
            }
        }
    };
    (
        $ty:ty,
        $ln:path,
        $log2:path,
        $log10:path,
        $ln_1p:path,
        $nonnegative_tests_mod:ident,
        $negative_tests_mod:ident
    ) => {
        impl_log_traits_for_float!($ty, $ln, $log2, $log10, $ln_1p);

        test_log_traits_float_nonnegative!($ty, $nonnegative_tests_mod);
        test_log_traits_float_negative!($ty, $negative_tests_mod);
    };
}

#[cfg(any(feature = "std", feature = "libm"))]
macro_rules! test_log_traits_float_nonnegative {
    ($ty:ty, $tests_mod:ident) => {
        #[cfg(test)]
        mod $tests_mod {
            use crate::elem::*;
            use crate::fns::*;
            use crate::types::Object;

            #[test]
            fn test_ln() {
                let zero = <$ty as ConstZero>::ZERO;
                let one = <$ty as ConstOne>::ONE;
                let two = one + one;
                let four = two + two;
                let sixteen = four * four;
                let sixteenth = one / sixteen;

                assert_eq!(Ln::ln(one), zero);
                assert_eq!(Object::new(one).ln(), Object::new(Ln::ln(one)));

                let ln_e = Ln::ln(Exp::exp(one));
                assert!(Abs::abs(ln_e - one) < sixteenth);
            }

            #[test]
            fn test_log2() {
                let zero = <$ty as ConstZero>::ZERO;
                let one = <$ty as ConstOne>::ONE;
                let two = one + one;
                let four = two + two;
                let eight = four + four;

                assert_eq!(Log2::log2(one), zero);
                assert_eq!(Object::new(one).log2(), Object::new(Log2::log2(one)));
                assert_eq!(Log2::log2(two), one);
                assert_eq!(Log2::log2(four), two);
                assert_eq!(Log2::log2(eight), two + one);
            }

            #[test]
            fn test_log10() {
                let zero = <$ty as ConstZero>::ZERO;
                let one = <$ty as ConstOne>::ONE;
                let ten = 10 as $ty;

                assert_eq!(Log10::log10(one), zero);
                assert_eq!(Object::new(one).log10(), Object::new(Log10::log10(one)));
                assert_eq!(Log10::log10(ten), one);
                assert_eq!(Log10::log10(ten * ten), one + one);
            }

            #[test]
            fn test_log() {
                let one = <$ty as ConstOne>::ONE;
                let two = one + one;
                let four = two + two;
                let eight = four + four;

                assert_eq!(Log::log(one, two), <$ty as ConstZero>::ZERO);
                assert_eq!(Object::new(one).log(two), Object::new(Log::log(one, two)));
                assert_eq!(Log::log(two, two), one);
                assert_eq!(Log::log(four, two), two);
                assert_eq!(Log::log(eight, two), two + one);
                assert_eq!(Log::log(eight, four), one + one / two);
            }

            #[test]
            fn test_ln_1p() {
                let zero = <$ty as ConstZero>::ZERO;
                let one = <$ty as ConstOne>::ONE;
                let two = one + one;
                let four = two + two;
                let sixteen = four * four;
                let sixteenth = one / sixteen;

                assert_eq!(Ln1p::ln_1p(zero), zero);
                assert_eq!(Object::new(zero).ln_1p(), Object::new(Ln1p::ln_1p(zero)));

                let ln_2 = Ln1p::ln_1p(one);

                assert!(ln_2 > zero);
                assert!(ln_2 < one);
                assert!(Abs::abs(ln_2 - Ln::ln(two)) < sixteenth);
            }

            #[test]
            fn test_checked_ln() {
                let zero = <$ty as ConstZero>::ZERO;
                let one = <$ty as ConstOne>::ONE;

                assert_eq!(CheckedLn::checked_ln(one), Some(zero));
                assert_eq!(
                    Object::new(one).checked_ln(),
                    CheckedLn::checked_ln(one).map(Object::new)
                );
                assert_eq!(CheckedLn::checked_ln(zero), None);
            }

            #[test]
            fn test_checked_log2() {
                let zero = <$ty as ConstZero>::ZERO;
                let one = <$ty as ConstOne>::ONE;
                let two = one + one;
                let four = two + two;

                assert_eq!(CheckedLog2::checked_log2(one), Some(zero));
                assert_eq!(
                    Object::new(one).checked_log2(),
                    CheckedLog2::checked_log2(one).map(Object::new)
                );
                assert_eq!(CheckedLog2::checked_log2(two), Some(one));
                assert_eq!(CheckedLog2::checked_log2(four), Some(two));
                assert_eq!(CheckedLog2::checked_log2(zero), None);
            }

            #[test]
            fn test_checked_log10() {
                let zero = <$ty as ConstZero>::ZERO;
                let one = <$ty as ConstOne>::ONE;
                let ten = 10 as $ty;

                assert_eq!(CheckedLog10::checked_log10(one), Some(zero));
                assert_eq!(
                    Object::new(one).checked_log10(),
                    CheckedLog10::checked_log10(one).map(Object::new)
                );
                assert_eq!(CheckedLog10::checked_log10(ten), Some(one));
                assert_eq!(CheckedLog10::checked_log10(zero), None);
            }

            #[test]
            fn test_checked_log() {
                let zero = <$ty as ConstZero>::ZERO;
                let one = <$ty as ConstOne>::ONE;
                let two = one + one;
                let four = two + two;

                assert_eq!(CheckedLog::checked_log(four, two), Some(two));
                assert_eq!(
                    Object::new(four).checked_log(two),
                    CheckedLog::checked_log(four, two).map(Object::new)
                );
                assert_eq!(CheckedLog::checked_log(zero, two), None);
            }

            #[test]
            fn test_checked_ln_1p() {
                let zero = <$ty as ConstZero>::ZERO;
                let one = <$ty as ConstOne>::ONE;

                assert_eq!(CheckedLn1p::checked_ln_1p(zero), Some(zero));
                assert_eq!(
                    Object::new(zero).checked_ln_1p(),
                    CheckedLn1p::checked_ln_1p(zero).map(Object::new)
                );
                assert!(CheckedLn1p::checked_ln_1p(one).unwrap() > zero);
                assert_eq!(CheckedLn1p::checked_ln_1p(-one), None);
            }
        }
    };
}

#[cfg(any(feature = "std", feature = "libm"))]
macro_rules! test_log_traits_float_negative {
    ($ty:ty, $tests_mod:ident) => {
        #[cfg(test)]
        mod $tests_mod {
            use crate::elem::*;
            use crate::fns::*;
            use crate::types::Object;

            #[test]
            fn test_ln() {
                let zero = <$ty as ConstZero>::ZERO;
                let one = <$ty as ConstOne>::ONE;
                let two = one + one;
                let four = two + two;
                let half = one / two;
                let sixteen = four * four;
                let sixteenth = one / sixteen;

                let ln_half = Ln::ln(half);

                assert_eq!(Object::new(half).ln(), Object::new(Ln::ln(half)));
                assert!(ln_half < zero);
                assert!(Abs::abs(ln_half + Ln::ln(two)) < sixteenth);
            }

            #[test]
            fn test_log2() {
                let one = <$ty as ConstOne>::ONE;
                let two = one + one;
                let four = two + two;
                let half = one / two;
                let quarter = one / four;

                assert_eq!(Log2::log2(half), -one);
                assert_eq!(Object::new(half).log2(), Object::new(Log2::log2(half)));
                assert_eq!(Log2::log2(quarter), -two);
            }

            #[test]
            fn test_log10() {
                let one = <$ty as ConstOne>::ONE;
                let ten = 10 as $ty;
                let tenth = one / ten;

                assert_eq!(Log10::log10(tenth), -one);
                assert_eq!(Object::new(tenth).log10(), Object::new(Log10::log10(tenth)));
            }

            #[test]
            fn test_log() {
                let one = <$ty as ConstOne>::ONE;
                let two = one + one;
                let four = two + two;
                let half = one / two;
                let quarter = one / four;

                assert_eq!(Log::log(half, two), -one);
                assert_eq!(Object::new(half).log(two), Object::new(Log::log(half, two)));
                assert_eq!(Log::log(quarter, two), -two);
                assert_eq!(Log::log(half, four), -one / two);
            }

            #[test]
            fn test_ln_1p() {
                let zero = <$ty as ConstZero>::ZERO;
                let one = <$ty as ConstOne>::ONE;
                let two = one + one;
                let four = two + two;
                let half = one / two;
                let sixteen = four * four;
                let sixteenth = one / sixteen;

                let ln_1p = Ln1p::ln_1p(-half);

                assert_eq!(Object::new(-half).ln_1p(), Object::new(Ln1p::ln_1p(-half)));
                assert!(ln_1p < zero);
                assert!(Abs::abs(ln_1p - Ln::ln(half)) < sixteenth);
            }

            #[test]
            fn test_checked_ln() {
                let one = <$ty as ConstOne>::ONE;

                assert_eq!(
                    Object::new(-one).checked_ln(),
                    CheckedLn::checked_ln(-one).map(Object::new)
                );
                assert_eq!(CheckedLn::checked_ln(-one), None);
            }

            #[test]
            fn test_checked_log2() {
                let one = <$ty as ConstOne>::ONE;
                let two = one + one;
                let half = one / two;

                assert_eq!(CheckedLog2::checked_log2(half), Some(-one));
                assert_eq!(
                    Object::new(half).checked_log2(),
                    CheckedLog2::checked_log2(half).map(Object::new)
                );
                assert_eq!(CheckedLog2::checked_log2(-one), None);
            }

            #[test]
            fn test_checked_log10() {
                let one = <$ty as ConstOne>::ONE;
                let ten = 10 as $ty;
                let tenth = one / ten;

                assert_eq!(CheckedLog10::checked_log10(tenth), Some(-one));
                assert_eq!(
                    Object::new(tenth).checked_log10(),
                    CheckedLog10::checked_log10(tenth).map(Object::new)
                );
                assert_eq!(CheckedLog10::checked_log10(-one), None);
            }

            #[test]
            fn test_checked_log() {
                let one = <$ty as ConstOne>::ONE;
                let two = one + one;
                let half = one / two;

                assert_eq!(CheckedLog::checked_log(half, two), Some(-one));
                assert_eq!(
                    Object::new(half).checked_log(two),
                    CheckedLog::checked_log(half, two).map(Object::new)
                );
                assert_eq!(CheckedLog::checked_log(-one, two), None);
            }

            #[test]
            fn test_checked_ln_1p() {
                let zero = <$ty as ConstZero>::ZERO;
                let one = <$ty as ConstOne>::ONE;
                let two = one + one;
                let half = one / two;

                assert_eq!(
                    Object::new(-half).checked_ln_1p(),
                    CheckedLn1p::checked_ln_1p(-half).map(Object::new)
                );
                assert!(CheckedLn1p::checked_ln_1p(-half).unwrap() < zero);
                assert_eq!(CheckedLn1p::checked_ln_1p(-two), None);
            }
        }
    };
}

#[cfg(feature = "std")]
impl_log_traits_for_float!(
    f32,
    f32::ln,
    f32::log2,
    f32::log10,
    f32::ln_1p,
    f32_nonnegative_tests,
    f32_negative_tests
);
#[cfg(feature = "std")]
impl_log_traits_for_float!(
    f64,
    f64::ln,
    f64::log2,
    f64::log10,
    f64::ln_1p,
    f64_nonnegative_tests,
    f64_negative_tests
);
#[cfg(all(not(feature = "std"), feature = "libm"))]
impl_log_traits_for_float!(
    f32,
    libm::logf,
    libm::log2f,
    libm::log10f,
    libm::log1pf,
    f32_nonnegative_tests,
    f32_negative_tests
);
#[cfg(all(not(feature = "std"), feature = "libm"))]
impl_log_traits_for_float!(
    f64,
    libm::log,
    libm::log2,
    libm::log10,
    libm::log1p,
    f64_nonnegative_tests,
    f64_negative_tests
);

macro_rules! impl_ilog_traits_for_int {
    ($ty:ty) => {
        impl Ilog<$ty> for $ty {
            type Output = u32;

            #[inline]
            fn ilog(self, base: $ty) -> Self::Output {
                <$ty>::ilog(self, base)
            }
        }

        impl Ilog2 for $ty {
            type Output = u32;

            #[inline]
            fn ilog2(self) -> Self::Output {
                <$ty>::ilog2(self)
            }
        }

        impl Ilog10 for $ty {
            type Output = u32;

            #[inline]
            fn ilog10(self) -> Self::Output {
                <$ty>::ilog10(self)
            }
        }

        impl CheckedIlog<$ty> for $ty {
            type Output = u32;

            #[inline]
            fn checked_ilog(self, base: $ty) -> Option<Self::Output> {
                <$ty>::checked_ilog(self, base)
            }
        }

        impl CheckedIlog2 for $ty {
            type Output = u32;

            #[inline]
            fn checked_ilog2(self) -> Option<Self::Output> {
                <$ty>::checked_ilog2(self)
            }
        }

        impl CheckedIlog10 for $ty {
            type Output = u32;

            #[inline]
            fn checked_ilog10(self) -> Option<Self::Output> {
                <$ty>::checked_ilog10(self)
            }
        }
    };
    ($ty:ty, $nonnegative_tests_mod:ident) => {
        impl_ilog_traits_for_int!($ty);

        test_ilog_traits_nonnegative!($ty, $nonnegative_tests_mod);
    };
    ($ty:ty, $nonnegative_tests_mod:ident, $negative_tests_mod:ident) => {
        impl_ilog_traits_for_int!($ty, $nonnegative_tests_mod);

        test_ilog_traits_negative!($ty, $negative_tests_mod);
    };
}

macro_rules! test_ilog_traits_nonnegative {
    ($ty:ty, $tests_mod:ident) => {
        #[cfg(test)]
        mod $tests_mod {
            use crate::elem::*;
            use crate::fns::*;
            use crate::types::Object;

            #[test]
            fn test_ilog2() {
                let one = <$ty as ConstOne>::ONE;
                let two = one + one;
                let three = two + one;
                let four = two + two;
                let eight = four + four;

                assert_eq!(Ilog2::ilog2(one), 0);
                assert_eq!(Object::new(one).ilog2(), Object::new(Ilog2::ilog2(one)));
                assert_eq!(Ilog2::ilog2(two), 1);
                assert_eq!(Ilog2::ilog2(three), 1);
                assert_eq!(Ilog2::ilog2(four), 2);
                assert_eq!(Ilog2::ilog2(eight), 3);
            }

            #[test]
            fn test_ilog10() {
                let one = <$ty as ConstOne>::ONE;
                let ten = 10 as $ty;
                let hundred = ten * ten;

                assert_eq!(Ilog10::ilog10(one), 0);
                assert_eq!(Object::new(one).ilog10(), Object::new(Ilog10::ilog10(one)));
                assert_eq!(Ilog10::ilog10(ten), 1);
                assert_eq!(Ilog10::ilog10(hundred), 2);
            }

            #[test]
            fn test_ilog() {
                let one = <$ty as ConstOne>::ONE;
                let two = one + one;
                let three = two + one;
                let four = two + two;
                let eight = four + four;

                assert_eq!(Ilog::ilog(one, two), 0);
                assert_eq!(
                    Object::new(one).ilog(two),
                    Object::new(Ilog::ilog(one, two))
                );
                assert_eq!(Ilog::ilog(two, two), 1);
                assert_eq!(Ilog::ilog(three, two), 1);
                assert_eq!(Ilog::ilog(four, two), 2);
                assert_eq!(Ilog::ilog(eight, two), 3);
                assert_eq!(Ilog::ilog(eight, four), 1);
            }

            #[test]
            fn test_checked_ilog2() {
                let zero = <$ty as ConstZero>::ZERO;
                let one = <$ty as ConstOne>::ONE;
                let two = one + one;
                let four = two + two;

                assert_eq!(CheckedIlog2::checked_ilog2(one), Some(0));
                assert_eq!(
                    Object::new(one).checked_ilog2(),
                    CheckedIlog2::checked_ilog2(one).map(Object::new)
                );
                assert_eq!(CheckedIlog2::checked_ilog2(two), Some(1));
                assert_eq!(CheckedIlog2::checked_ilog2(four), Some(2));
                assert_eq!(CheckedIlog2::checked_ilog2(zero), None);
            }

            #[test]
            fn test_checked_ilog10() {
                let zero = <$ty as ConstZero>::ZERO;
                let one = <$ty as ConstOne>::ONE;
                let ten = 10 as $ty;

                assert_eq!(CheckedIlog10::checked_ilog10(one), Some(0));
                assert_eq!(
                    Object::new(one).checked_ilog10(),
                    CheckedIlog10::checked_ilog10(one).map(Object::new)
                );
                assert_eq!(CheckedIlog10::checked_ilog10(ten), Some(1));
                assert_eq!(CheckedIlog10::checked_ilog10(zero), None);
            }

            #[test]
            fn test_checked_ilog() {
                let zero = <$ty as ConstZero>::ZERO;
                let one = <$ty as ConstOne>::ONE;
                let two = one + one;
                let eight = (one + one) * (one + one) * (one + one);

                assert_eq!(CheckedIlog::checked_ilog(eight, two), Some(3));
                assert_eq!(
                    Object::new(eight).checked_ilog(two),
                    CheckedIlog::checked_ilog(eight, two).map(Object::new)
                );
                assert_eq!(CheckedIlog::checked_ilog(zero, two), None);
                assert_eq!(CheckedIlog::checked_ilog(two, one), None);
            }
        }
    };
}

macro_rules! test_ilog_traits_negative {
    ($ty:ty, $tests_mod:ident) => {
        #[cfg(test)]
        mod $tests_mod {
            use crate::elem::*;
            use crate::fns::*;
            use crate::types::Object;

            #[test]
            fn test_checked_ilog2() {
                let one = <$ty as ConstOne>::ONE;

                assert_eq!(
                    Object::new(-one).checked_ilog2(),
                    CheckedIlog2::checked_ilog2(-one).map(Object::new)
                );
                assert_eq!(CheckedIlog2::checked_ilog2(-one), None);
            }

            #[test]
            fn test_checked_ilog10() {
                let one = <$ty as ConstOne>::ONE;

                assert_eq!(
                    Object::new(-one).checked_ilog10(),
                    CheckedIlog10::checked_ilog10(-one).map(Object::new)
                );
                assert_eq!(CheckedIlog10::checked_ilog10(-one), None);
            }

            #[test]
            fn test_checked_ilog() {
                let one = <$ty as ConstOne>::ONE;
                let two = one + one;

                assert_eq!(
                    Object::new(-one).checked_ilog(two),
                    CheckedIlog::checked_ilog(-one, two).map(Object::new)
                );
                assert_eq!(CheckedIlog::checked_ilog(-one, two), None);
            }
        }
    };
}

for_signed_ints!(impl_ilog_traits_for_int);

for_unsigned_ints!(impl_ilog_traits_for_int);
