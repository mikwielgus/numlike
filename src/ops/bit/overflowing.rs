// SPDX-FileCopyrightText: 2026 numlike contributors
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use crate::types::Object;

/// Bundle of overflowing bitwise shift operations.
pub trait OverflowingBitOps: OverflowingShl + OverflowingShr {}
impl<T: OverflowingShl + OverflowingShr> OverflowingBitOps for T {}

/// Shifts self left by `rhs` bits.
///
/// Returns a tuple of the shifted version of self along with a boolean indicating whether the shift
/// value was larger than or equal to the number of bits. If the shift value is too large, then value is
/// masked (N-1) where N is the number of bits, and this value is then used to perform the shift.
pub trait OverflowingShl {
    /// The resulting type after applying the operation.
    type Output;

    /// Shifts self left by `rhs` bits.
    fn overflowing_shl(self, rhs: u32) -> (Self::Output, bool);
}

impl<B: OverflowingShl<Output = B>> OverflowingShl for Object<B> {
    type Output = Self;

    fn overflowing_shl(self, rhs: u32) -> (Self::Output, bool) {
        let (value, overflow) = <B as OverflowingShl>::overflowing_shl(self.0, rhs);

        (Object(value), overflow)
    }
}

impl<B: OverflowingShl<Output = B>> Object<B> {
    /// Shifts self left by `rhs` bits.
    pub fn overflowing_shl(self, rhs: u32) -> (Self, bool) {
        let (value, overflow) = <B as OverflowingShl>::overflowing_shl(self.0, rhs);

        (Object(value), overflow)
    }
}

/// Shifts self right by `rhs` bits.
///
/// Returns a tuple of the shifted version of self along with a boolean indicating whether the shift
/// value was larger than or equal to the number of bits. If the shift value is too large, then value is
/// masked (N-1) where N is the number of bits, and this value is then used to perform the shift.
pub trait OverflowingShr {
    /// The resulting type after applying the operation.
    type Output;

    /// Shifts self right by `rhs` bits.
    fn overflowing_shr(self, rhs: u32) -> (Self::Output, bool);
}

impl<B: OverflowingShr<Output = B>> OverflowingShr for Object<B> {
    type Output = Self;

    fn overflowing_shr(self, rhs: u32) -> (Self::Output, bool) {
        let (value, overflow) = <B as OverflowingShr>::overflowing_shr(self.0, rhs);

        (Object(value), overflow)
    }
}

impl<B: OverflowingShr<Output = B>> Object<B> {
    /// Shifts self right by `rhs` bits.
    pub fn overflowing_shr(self, rhs: u32) -> (Self, bool) {
        let (value, overflow) = <B as OverflowingShr>::overflowing_shr(self.0, rhs);

        (Object(value), overflow)
    }
}

macro_rules! impl_overflowing_shift_traits_for_ints {
    ($ty:ty $(, $rest:ident)*) => {
        impl OverflowingShl for $ty {
            type Output = $ty;

            #[inline]
            fn overflowing_shl(self, rhs: u32) -> (Self::Output, bool) {
                <$ty>::overflowing_shl(self, rhs)
            }
        }

        impl OverflowingShr for $ty {
            type Output = $ty;

            #[inline]
            fn overflowing_shr(self, rhs: u32) -> (Self::Output, bool) {
                <$ty>::overflowing_shr(self, rhs)
            }
        }
    };
}

macro_rules! impl_overflowing_shift_traits_via_inner {
    ($ty:ty $(, $rest:ident)*) => {
        impl OverflowingShl for $ty {
            type Output = $ty;

            #[inline]
            fn overflowing_shl(self, rhs: u32) -> (Self::Output, bool) {
                let (value, overflowed) = self.0.overflowing_shl(rhs);

                (Self(value), overflowed)
            }
        }

        impl OverflowingShr for $ty {
            type Output = $ty;

            #[inline]
            fn overflowing_shr(self, rhs: u32) -> (Self::Output, bool) {
                let (value, overflowed) = self.0.overflowing_shr(rhs);

                (Self(value), overflowed)
            }
        }
    };
}

for_signed_ints!(impl_overflowing_shift_traits_for_ints);
for_unsigned_ints!(impl_overflowing_shift_traits_for_ints);
for_wrapping_signed_ints!(impl_overflowing_shift_traits_via_inner);
for_wrapping_unsigned_ints!(impl_overflowing_shift_traits_via_inner);
for_saturating_signed_ints!(impl_overflowing_shift_traits_via_inner);
for_saturating_unsigned_ints!(impl_overflowing_shift_traits_via_inner);

macro_rules! test_overflowing_bitshift_traits_int_nonnegative {
    ($ty:ty, $tests_mod:ident) => {
        #[cfg(test)]
        mod $tests_mod {
            use crate::elem::*;
            use crate::ops::*;
            use crate::types::Object;

            #[test]
            fn test_shl() {
                let zero = <$ty as AssocConstZero>::ZERO;
                let one = <$ty as AssocConstOne>::ONE;
                let two = one + one;
                let four = two + two;

                assert_eq!(OverflowingShl::overflowing_shl(zero, 1), (zero, false));
                assert_eq!(OverflowingShl::overflowing_shl(one, 1), (two, false));
                assert_eq!(OverflowingShl::overflowing_shl(two, 1), (four, false));
                assert_eq!(OverflowingShl::overflowing_shl(one, 2), (four, false));
                assert_eq!(
                    OverflowingShl::overflowing_shl(one, <$ty>::BITS),
                    (one, true)
                );
                assert_eq!(Object::new(one).overflowing_shl(1), {
                    let (value, overflow) = OverflowingShl::overflowing_shl(one, 1);

                    (Object::new(value), overflow)
                });
            }

            #[test]
            fn test_shr() {
                let zero = <$ty as AssocConstZero>::ZERO;
                let one = <$ty as AssocConstOne>::ONE;
                let two = one + one;
                let four = two + two;

                assert_eq!(OverflowingShr::overflowing_shr(zero, 1), (zero, false));
                assert_eq!(OverflowingShr::overflowing_shr(one, 1), (zero, false));
                assert_eq!(OverflowingShr::overflowing_shr(two, 1), (one, false));
                assert_eq!(OverflowingShr::overflowing_shr(four, 1), (two, false));
                assert_eq!(OverflowingShr::overflowing_shr(four, 2), (one, false));
                assert_eq!(
                    OverflowingShr::overflowing_shr(one, <$ty>::BITS),
                    (one, true)
                );
                assert_eq!(Object::new(four).overflowing_shr(2), {
                    let (value, overflow) = OverflowingShr::overflowing_shr(four, 2);

                    (Object::new(value), overflow)
                });
            }
        }
    };
}

macro_rules! test_overflowing_bitshift_traits_int_negative {
    ($ty:ty, $tests_mod:ident) => {
        #[cfg(test)]
        mod $tests_mod {
            use crate::elem::*;
            use crate::ops::*;
            use crate::types::Object;

            #[test]
            fn test_shl() {
                let one = <$ty as AssocConstOne>::ONE;
                let two = one + one;
                let four = two + two;

                assert_eq!(OverflowingShl::overflowing_shl(-one, 1), (-two, false));
                assert_eq!(OverflowingShl::overflowing_shl(-two, 1), (-four, false));
                assert_eq!(
                    OverflowingShl::overflowing_shl(-one, <$ty>::BITS),
                    (-one, true)
                );
                assert_eq!(Object::new(-one).overflowing_shl(1), {
                    let (value, overflow) = OverflowingShl::overflowing_shl(-one, 1);

                    (Object::new(value), overflow)
                });
            }

            #[test]
            fn test_shr() {
                let one = <$ty as AssocConstOne>::ONE;
                let two = one + one;
                let four = two + two;

                assert_eq!(OverflowingShr::overflowing_shr(-two, 1), (-one, false));
                assert_eq!(OverflowingShr::overflowing_shr(-four, 1), (-two, false));
                assert_eq!(OverflowingShr::overflowing_shr(-four, 2), (-one, false));
                assert_eq!(
                    OverflowingShr::overflowing_shr(-one, <$ty>::BITS),
                    (-one, true)
                );
                assert_eq!(Object::new(-four).overflowing_shr(2), {
                    let (value, overflow) = OverflowingShr::overflowing_shr(-four, 2);

                    (Object::new(value), overflow)
                });
            }
        }
    };
}

for_signed_ints!(
    test_overflowing_bitshift_traits_int_nonnegative; test_overflowing_bitshift_traits_int_negative
);
for_unsigned_ints!(test_overflowing_bitshift_traits_int_nonnegative);
