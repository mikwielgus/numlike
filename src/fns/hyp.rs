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

macro_rules! impl_trig_traits_for_floats {
    ($($ty:ty),*) => {
        $(
            impl Sinh for $ty {
                type Output = $ty;

                #[inline]
                fn sinh(self) -> Self::Output {
                    <$ty>::sinh(self)
                }
            }

            impl Cosh for $ty {
                type Output = $ty;

                #[inline]
                fn cosh(self) -> Self::Output {
                    <$ty>::cosh(self)
                }
            }

            impl Tanh for $ty {
                type Output = $ty;

                #[inline]
                fn tanh(self) -> Self::Output {
                    <$ty>::tanh(self)
                }
            }

            impl CheckedSinh for $ty {
                type Output = $ty;

                #[inline]
                fn checked_sinh(self) -> Option<Self::Output> {
                    let result = <$ty>::sinh(self);

                    result.is_finite().then_some(result)
                }
            }

            impl CheckedCosh for $ty {
                type Output = $ty;

                #[inline]
                fn checked_cosh(self) -> Option<Self::Output> {
                    let result = <$ty>::cosh(self);

                    result.is_finite().then_some(result)
                }
            }

            impl Asinh for $ty {
                type Output = $ty;

                #[inline]
                fn asinh(self) -> Self::Output {
                    <$ty>::asinh(self)
                }
            }

            impl Acosh for $ty {
                type Output = $ty;

                #[inline]
                fn acosh(self) -> Self::Output {
                    <$ty>::acosh(self)
                }
            }

            impl Atanh for $ty {
                type Output = $ty;

                #[inline]
                fn atanh(self) -> Self::Output {
                    <$ty>::atanh(self)
                }
            }

            /*impl CheckedAsinh for $ty {
                type Output = $ty;

                #[inline]
                fn checked_asinh(self) -> Option<Self::Output> {
                    let result = <$ty>::asinh(self);

                    result.is_finite().then_some(result)
                }
            }*/

            impl CheckedAcosh for $ty {
                type Output = $ty;

                #[inline]
                fn checked_acosh(self) -> Option<Self::Output> {
                    let result = <$ty>::acosh(self);

                    result.is_finite().then_some(result)
                }
            }

            impl CheckedAtanh for $ty {
                type Output = $ty;

                #[inline]
                fn checked_atanh(self) -> Option<Self::Output> {
                    let result = <$ty>::atanh(self);

                    result.is_finite().then_some(result)
                }
            }
        )*
    }
}

impl_trig_traits_for_floats!(f32, f64);
