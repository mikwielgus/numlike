// SPDX-FileCopyrightText: 2026 numlike contributors
//
// SPDX-License-Identifier: MIT OR Apache-2.0

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
    /// The resulting type after applying the operation.
    type Output;

    /// Returns the logarithm of the number with respect to an arbitrary base.
    fn log(self, base: Rhs) -> Self::Output;
}

/// Returns the natural logarithm of the number.
///
/// This returns NaN when the number is negative, and negative infinity when number is zero.
pub trait Ln {
    /// The resulting type after applying the operation.
    type Output;

    /// Returns the natural logarithm of the number.
    fn ln(self) -> Self::Output;
}

/// Returns the base 2 logarithm of the number.
///
/// This returns NaN when the number is negative, and negative infinity when number is zero.
pub trait Log2 {
    /// The resulting type after applying the operation.
    type Output;

    /// Returns the base 2 logarithm of the number.
    fn log2(self) -> Self::Output;
}

/// Returns the base 10 logarithm of the number.
///
/// This returns NaN when the number is negative, and negative infinity when number is zero.
pub trait Log10 {
    /// The resulting type after applying the operation.
    type Output;

    /// Returns the base 10 logarithm of the number.
    fn log10(self) -> Self::Output;
}

/// Returns `ln(1+n)` (natural logarithm) more accurately than if
/// the operations were performed separately.
///
/// This returns NaN when `n < -1.0`, and negative infinity when `n == -1.0`.
pub trait Ln1p {
    /// The resulting type after applying the operation.
    type Output;

    /// Returns `ln(1+n)` (natural logarithm) more accurately than if
    /// the operations were performed separately.
    fn ln_1p(self) -> Self::Output;
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
    /// The resulting type after applying the operation.
    type Output;

    /// Returns the logarithm of the number with respect to an arbitrary base.
    fn checked_log(self, base: Rhs) -> Option<Self::Output>;
}

/// Returns the natural logarithm of the number.
///
/// Returns `None` if the result is not finite.
pub trait CheckedLn {
    /// The resulting type after applying the operation.
    type Output;

    /// Returns the natural logarithm of the number.
    fn checked_ln(self) -> Option<Self::Output>;
}

/// Returns the base 2 logarithm of the number.
///
/// Returns `None` if the result is not finite.
pub trait CheckedLog2 {
    /// The resulting type after applying the operation.
    type Output;

    /// Returns the base 2 logarithm of the number.
    fn checked_log2(self) -> Option<Self::Output>;
}

/// Returns the base 10 logarithm of the number.
///
/// Returns `None` if the result is not finite.
pub trait CheckedLog10 {
    /// The resulting type after applying the operation.
    type Output;

    /// Returns the base 10 logarithm of the number.
    fn checked_log10(self) -> Option<Self::Output>;
}

/// Returns `ln(1+n)` (natural logarithm) more accurately than if
/// the operations were performed separately.
///
/// Returns `None` if the result is not finite.
pub trait CheckedLn1p {
    /// The resulting type after applying the operation.
    type Output;

    /// Returns `ln(1+n)` (natural logarithm) more accurately than if
    /// the operations were performed separately.
    fn checked_ln_1p(self) -> Option<Self::Output>;
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
    /// The resulting type after applying the operation.
    type Output;

    /// Returns the logarithm of the number with respect to an arbitrary base,
    /// rounded down.
    fn ilog(self, base: Rhs) -> Self::Output;
}

/// Returns the base 2 logarithm of the number, rounded down.
///
/// # Panics
///
/// This function will panic if `self` is less than or equal to zero.
pub trait Ilog2 {
    /// The resulting type after applying the operation.
    type Output;

    /// Returns the base 2 logarithm of the number, rounded down.
    fn ilog2(self) -> Self::Output;
}

/// Returns the base 10 logarithm of the number, rounded down.
///
/// # Panics
///
/// This function will panic if `self` is less than or equal to zero.
pub trait Ilog10 {
    /// The resulting type after applying the operation.
    type Output;

    /// Returns the base 10 logarithm of the number, rounded down.
    fn ilog10(self) -> Self::Output;
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
    /// The resulting type after applying the operation.
    type Output;

    /// Returns the logarithm of the number with respect to an arbitrary base,
    /// rounded down.
    fn checked_ilog(self, base: Rhs) -> Option<Self::Output>;
}

/// Returns the base 2 logarithm of the number, rounded down.
///
/// Returns `None` if `self` is less than or equal to zero.
pub trait CheckedIlog2 {
    /// The resulting type after applying the operation.
    type Output;

    /// Returns the base 2 logarithm of the number, rounded down.
    fn checked_ilog2(self) -> Option<Self::Output>;
}

/// Returns the base 10 logarithm of the number, rounded down.
///
/// Returns `None` if `self` is less than or equal to zero.
pub trait CheckedIlog10 {
    /// The resulting type after applying the operation.
    type Output;

    /// Returns the base 10 logarithm of the number, rounded down.
    fn checked_ilog10(self) -> Option<Self::Output>;
}

macro_rules! impl_log_traits_for_floats {
    ($($ty:ty),*) => {
        $(
            impl Log<$ty> for $ty {
                type Output = $ty;

                #[inline]
                fn log(self, base: $ty) -> Self::Output {
                    <$ty>::log(self, base)
                }
            }

            impl Ln for $ty {
                type Output = $ty;

                #[inline]
                fn ln(self) -> Self::Output {
                    <$ty>::ln(self)
                }
            }

            impl Log2 for $ty {
                type Output = $ty;

                #[inline]
                fn log2(self) -> Self::Output {
                    <$ty>::log2(self)
                }
            }

            impl Log10 for $ty {
                type Output = $ty;

                #[inline]
                fn log10(self) -> Self::Output {
                    <$ty>::log10(self)
                }
            }

            impl Ln1p for $ty {
                type Output = $ty;

                #[inline]
                fn ln_1p(self) -> Self::Output {
                    <$ty>::ln_1p(self)
                }
            }

            impl CheckedLog<$ty> for $ty {
                type Output = $ty;

                #[inline]
                fn checked_log(self, base: $ty) -> Option<Self::Output> {
                    let result = <$ty>::log(self, base);

                    result.is_finite().then_some(result)
                }
            }

            impl CheckedLn for $ty {
                type Output = $ty;

                #[inline]
                fn checked_ln(self) -> Option<Self::Output> {
                    let result = <$ty>::ln(self);

                    result.is_finite().then_some(result)
                }
            }

            impl CheckedLog2 for $ty {
                type Output = $ty;

                #[inline]
                fn checked_log2(self) -> Option<Self::Output> {
                    let result = <$ty>::log2(self);

                    result.is_finite().then_some(result)
                }
            }

            impl CheckedLog10 for $ty {
                type Output = $ty;

                #[inline]
                fn checked_log10(self) -> Option<Self::Output> {
                    let result = <$ty>::log10(self);

                    result.is_finite().then_some(result)
                }
            }

            impl CheckedLn1p for $ty {
                type Output = $ty;

                #[inline]
                fn checked_ln_1p(self) -> Option<Self::Output> {
                    let result = <$ty>::ln_1p(self);

                    result.is_finite().then_some(result)
                }
            }
        )*
    };
}

impl_log_traits_for_floats!(f32, f64);

macro_rules! impl_ilog_traits_for_ints {
    ($($ty:ty),*) => {
        $(
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
        )*
    };
}

impl_ilog_traits_for_ints!(i8, i16, i32, i64, i128, isize);
impl_ilog_traits_for_ints!(u8, u16, u32, u64, u128, usize);
