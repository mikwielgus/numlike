// SPDX-FileCopyrightText: 2026 numlike contributors
//
// SPDX-License-Identifier: MIT OR Apache-2.0

/// Bundle of sign-related functions.
pub trait SignFns: Signum + Abs {}
impl<T: Signum + Abs> SignFns for T {}

/// Returns a number representing sign of `self`.
///
///  - `0` if the number is zero
///  - `1` if the number is positive
///  - `-1` if the number is negative
pub trait Signum {
    /// The resulting type after applying the operation.
    type Output;

    /// Returns a number representing sign of `self`.
    fn signum(self) -> Self::Output;
}

/// Computes the absolute value of `self`.
///
/// # Overflow behavior
///
/// The absolute value of the minimum value of a signed integer type cannot
/// be represented as that same type, and attempting to calculate it will
/// cause an overflow. This means that code in debug mode will trigger a
/// panic on this case and optimized code will return the minimum value
/// without a panic.
pub trait Abs {
    /// The resulting type after applying the operation.
    type Output;

    /// Computes the absolute value of `self`.
    fn abs(self) -> Self::Output;
}

/// Checked absolute value. Computes `self.abs()`, returning `None` if
/// `self` is the minimum value of a signed integer type.
pub trait CheckedAbs {
    /// The resulting type after applying the operation.
    type Output;

    /// Checked absolute value. Computes `self.abs()`, returning `None` if
    /// `self` is the minimum value of a signed integer type.
    fn checked_abs(self) -> Option<Self::Output>;
}

macro_rules! impl_sign_traits_for_signeds {
    ($($ty:ty),*) => {
        $(
            impl Signum for $ty {
                type Output = $ty;

                #[inline]
                fn signum(self) -> Self::Output {
                    <$ty>::signum(self)
                }
            }

            impl Abs for $ty {
                type Output = $ty;

                #[inline]
                fn abs(self) -> Self::Output {
                    <$ty>::abs(self)
                }
            }

            impl CheckedAbs for $ty {
                type Output = $ty;

                #[inline]
                fn checked_abs(self) -> Option<Self::Output> {
                    <$ty>::checked_abs(self)
                }
            }
        )*
    };
}

impl_sign_traits_for_signeds!(i8, i16, i32, i64, i128, isize);
// TODO: unsigned types, probably.
