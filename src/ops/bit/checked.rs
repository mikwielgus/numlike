// SPDX-FileCopyrightText: 2026 numlike contributors
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use crate::types::Object;

/// Bundle of checked bitwise shift operations.
pub trait CheckedBitOps: CheckedShl + CheckedShr {}
impl<T: CheckedShl + CheckedShr> CheckedBitOps for T {}

/// Checked shift left. Computes `self << rhs`, returning `None` if `rhs` is larger
/// than or equal to the number of bits in `self`.
pub trait CheckedShl {
    /// The resulting type after applying the operation.
    type Output;

    /// Checked shift left. Computes `self << rhs`, returning `None` if `rhs` is larger
    /// than or equal to the number of bits in `self`.
    fn checked_shl(self, rhs: u32) -> Option<Self::Output>;
}

impl<B: CheckedShl<Output = B>> CheckedShl for Object<B> {
    type Output = Self;

    fn checked_shl(self, rhs: u32) -> Option<Self::Output> {
        <B as CheckedShl>::checked_shl(self.0, rhs).map(Object)
    }
}

impl<B: CheckedShl<Output = B>> Object<B> {
    /// Checked shift left. Computes `self << rhs`, returning `None` if `rhs` is larger
    /// than or equal to the number of bits in `self`.
    pub fn checked_shl(self, rhs: u32) -> Option<Self> {
        <B as CheckedShl>::checked_shl(self.0, rhs).map(Object)
    }
}

/// Checked shift right. Computes `self >> rhs`, returning `None` if `rhs` is
/// larger than or equal to the number of bits in `self`.
pub trait CheckedShr {
    /// The resulting type after applying the operation.
    type Output;

    /// Checked shift right. Computes `self >> rhs`, returning `None` if `rhs` is
    /// larger than or equal to the number of bits in `self`.
    fn checked_shr(self, rhs: u32) -> Option<Self::Output>;
}

impl<B: CheckedShr<Output = B>> CheckedShr for Object<B> {
    type Output = Self;

    fn checked_shr(self, rhs: u32) -> Option<Self::Output> {
        <B as CheckedShr>::checked_shr(self.0, rhs).map(Object)
    }
}

impl<B: CheckedShr<Output = B>> Object<B> {
    /// Checked shift right. Computes `self >> rhs`, returning `None` if `rhs` is
    /// larger than or equal to the number of bits in `self`.
    pub fn checked_shr(self, rhs: u32) -> Option<Self> {
        <B as CheckedShr>::checked_shr(self.0, rhs).map(Object)
    }
}

macro_rules! impl_checked_shift_traits_for_ints {
    ($ty:ty $(, $rest:ident)*) => {
        impl CheckedShl for $ty {
            type Output = $ty;

            #[inline]
            fn checked_shl(self, rhs: u32) -> Option<Self::Output> {
                <$ty>::checked_shl(self, rhs)
            }
        }

        impl CheckedShr for $ty {
            type Output = $ty;

            #[inline]
            fn checked_shr(self, rhs: u32) -> Option<Self::Output> {
                <$ty>::checked_shr(self, rhs)
            }
        }
    };
}

macro_rules! impl_checked_shift_traits_via_inner {
    ($ty:ty $(, $rest:ident)*) => {
        impl CheckedShl for $ty {
            type Output = $ty;

            #[inline]
            fn checked_shl(self, rhs: u32) -> Option<Self::Output> {
                self.0.checked_shl(rhs).map(Self)
            }
        }

        impl CheckedShr for $ty {
            type Output = $ty;

            #[inline]
            fn checked_shr(self, rhs: u32) -> Option<Self::Output> {
                self.0.checked_shr(rhs).map(Self)
            }
        }
    };
}

for_signed_ints!(impl_checked_shift_traits_for_ints);
for_unsigned_ints!(impl_checked_shift_traits_for_ints);
for_wrapping_signed_ints!(impl_checked_shift_traits_via_inner);
for_wrapping_unsigned_ints!(impl_checked_shift_traits_via_inner);
for_saturating_signed_ints!(impl_checked_shift_traits_via_inner);
for_saturating_unsigned_ints!(impl_checked_shift_traits_via_inner);

macro_rules! test_checked_bitshift_traits_int_nonnegative {
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

                assert_eq!(CheckedShl::checked_shl(zero, 1), Some(zero));
                assert_eq!(CheckedShl::checked_shl(one, 1), Some(two));
                assert_eq!(CheckedShl::checked_shl(two, 1), Some(four));
                assert_eq!(CheckedShl::checked_shl(one, 2), Some(four));
                assert_eq!(CheckedShl::checked_shl(one, <$ty>::BITS), None);
                assert_eq!(
                    Object::new(one).checked_shl(1),
                    CheckedShl::checked_shl(one, 1).map(Object::new)
                );
            }

            #[test]
            fn test_shr() {
                let zero = <$ty as AssocConstZero>::ZERO;
                let one = <$ty as AssocConstOne>::ONE;
                let two = one + one;
                let four = two + two;

                assert_eq!(CheckedShr::checked_shr(zero, 1), Some(zero));
                assert_eq!(CheckedShr::checked_shr(one, 1), Some(zero));
                assert_eq!(CheckedShr::checked_shr(two, 1), Some(one));
                assert_eq!(CheckedShr::checked_shr(four, 1), Some(two));
                assert_eq!(CheckedShr::checked_shr(four, 2), Some(one));
                assert_eq!(CheckedShr::checked_shr(one, <$ty>::BITS), None);
                assert_eq!(
                    Object::new(four).checked_shr(2),
                    CheckedShr::checked_shr(four, 2).map(Object::new)
                );
            }
        }
    };
}

macro_rules! test_checked_bitshift_traits_int_negative {
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

                assert_eq!(CheckedShl::checked_shl(-one, 1), Some(-two));
                assert_eq!(CheckedShl::checked_shl(-two, 1), Some(-four));
                assert_eq!(CheckedShl::checked_shl(-one, <$ty>::BITS), None);
                assert_eq!(
                    Object::new(-one).checked_shl(1),
                    CheckedShl::checked_shl(-one, 1).map(Object::new)
                );
            }

            #[test]
            fn test_shr() {
                let one = <$ty as AssocConstOne>::ONE;
                let two = one + one;
                let four = two + two;

                assert_eq!(CheckedShr::checked_shr(-two, 1), Some(-one));
                assert_eq!(CheckedShr::checked_shr(-four, 1), Some(-two));
                assert_eq!(CheckedShr::checked_shr(-four, 2), Some(-one));
                assert_eq!(CheckedShr::checked_shr(-one, <$ty>::BITS), None);
                assert_eq!(
                    Object::new(-four).checked_shr(2),
                    CheckedShr::checked_shr(-four, 2).map(Object::new)
                );
            }
        }
    };
}

for_signed_ints!(
    test_checked_bitshift_traits_int_nonnegative; test_checked_bitshift_traits_int_negative
);
for_unsigned_ints!(test_checked_bitshift_traits_int_nonnegative);
