// SPDX-FileCopyrightText: 2026 numlike contributors
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use crate::types::Object;

/// Panic-free bitwise shift-left; yields `self << mask(rhs)`, where `mask`
/// removes any high-order bits of `rhs` that would cause the shift to
/// exceed the bitwidth of the type.
///
/// Beware that, unlike most other `wrapping_*` methods on integers, this
/// does *not* give the same result as doing the shift in infinite precision
/// then truncating as needed.  The behaviour matches what shift instructions
/// do on many processors, and is what the `<<` operator does when overflow
/// checks are disabled, but numerically it's weird.
///
/// Note that this is *not* the same as a rotate-left; the RHS of a wrapping
/// shift-left is restricted to the range of the type, rather than the bits
/// shifted out of the LHS being returned to the other end.
pub trait WrappingShl {
    /// The resulting type after applying the operation.
    type Output;

    /// Panic-free bitwise shift-left; yields `self << mask(rhs)`, where `mask`
    /// removes any high-order bits of `rhs` that would cause the shift to exceed
    /// the bitwidth of the type.
    fn wrapping_shl(self, rhs: u32) -> Self::Output;
}

impl<B: WrappingShl<Output = B>> WrappingShl for Object<B> {
    type Output = Self;

    fn wrapping_shl(self, rhs: u32) -> Self::Output {
        Object(<B as WrappingShl>::wrapping_shl(self.0, rhs))
    }
}

impl<B: WrappingShl<Output = B>> Object<B> {
    /// Panic-free bitwise shift-left; yields `self << mask(rhs)`, where `mask`
    /// removes any high-order bits of `rhs` that would cause the shift to exceed
    /// the bitwidth of the type.
    pub fn wrapping_shl(self, rhs: u32) -> Self {
        Object(<B as WrappingShl>::wrapping_shl(self.0, rhs))
    }
}

/// Panic-free bitwise shift-right; yields `self >> mask(rhs)`, where `mask`
/// removes any high-order bits of `rhs` that would cause the shift to
/// exceed the bitwidth of the type.
///
/// Beware that, unlike most other `wrapping_*` methods on integers, this
/// does *not* give the same result as doing the shift in infinite precision
/// then truncating as needed.  The behaviour matches what shift instructions
/// do on many processors, and is what the `>>` operator does when overflow
/// checks are disabled, but numerically it's weird.
///
/// Note that this is *not* the same as a rotate-right; the RHS of a
/// wrapping shift-right is restricted to the range of the type, rather than
/// the bits shifted out of the LHS being returned to the other end.
pub trait WrappingShr {
    /// The resulting type after applying the operation.
    type Output;

    /// Panic-free bitwise shift-right; yields `self >> mask(rhs)`, where `mask`
    /// removes any high-order bits of `rhs` that would cause the shift to exceed
    /// the bitwidth of the type.
    fn wrapping_shr(self, rhs: u32) -> Self::Output;
}

impl<B: WrappingShr<Output = B>> WrappingShr for Object<B> {
    type Output = Self;

    fn wrapping_shr(self, rhs: u32) -> Self::Output {
        Object(<B as WrappingShr>::wrapping_shr(self.0, rhs))
    }
}

impl<B: WrappingShr<Output = B>> Object<B> {
    /// Panic-free bitwise shift-right; yields `self >> mask(rhs)`, where `mask`
    /// removes any high-order bits of `rhs` that would cause the shift to exceed
    /// the bitwidth of the type.
    pub fn wrapping_shr(self, rhs: u32) -> Self {
        Object(<B as WrappingShr>::wrapping_shr(self.0, rhs))
    }
}

macro_rules! impl_wrapping_shift_traits_for_ints {
    ($ty:ty $(, $rest:ident)*) => {
        impl WrappingShl for $ty {
            type Output = $ty;

            #[inline]
            fn wrapping_shl(self, rhs: u32) -> Self::Output {
                <$ty>::wrapping_shl(self, rhs)
            }
        }

        impl WrappingShr for $ty {
            type Output = $ty;

            #[inline]
            fn wrapping_shr(self, rhs: u32) -> Self::Output {
                <$ty>::wrapping_shr(self, rhs)
            }
        }
    };
}

macro_rules! impl_wrapping_shift_traits_via_inner {
    ($ty:ty $(, $rest:ident)*) => {
        impl WrappingShl for $ty {
            type Output = $ty;

            #[inline]
            fn wrapping_shl(self, rhs: u32) -> Self::Output {
                Self(self.0.wrapping_shl(rhs))
            }
        }

        impl WrappingShr for $ty {
            type Output = $ty;

            #[inline]
            fn wrapping_shr(self, rhs: u32) -> Self::Output {
                Self(self.0.wrapping_shr(rhs))
            }
        }
    };
}

for_signed_ints!(impl_wrapping_shift_traits_for_ints);
for_unsigned_ints!(impl_wrapping_shift_traits_for_ints);
for_wrapping_signed_ints!(impl_wrapping_shift_traits_via_inner);
for_wrapping_unsigned_ints!(impl_wrapping_shift_traits_via_inner);
for_saturating_signed_ints!(impl_wrapping_shift_traits_via_inner);
for_saturating_unsigned_ints!(impl_wrapping_shift_traits_via_inner);

macro_rules! test_wrapping_bitshift_traits_int_nonnegative {
    ($ty:ty, $tests_mod:ident) => {
        #[cfg(test)]
        mod $tests_mod {
            use crate::elem::*;
            use crate::ops::*;
            use crate::types::Object;

            #[test]
            fn test_shl() {
                let zero = <$ty as ConstZero>::ZERO;
                let one = <$ty as ConstOne>::ONE;
                let two = one + one;
                let four = two + two;

                assert_eq!(WrappingShl::wrapping_shl(zero, 1), zero);
                assert_eq!(WrappingShl::wrapping_shl(one, 1), two);
                assert_eq!(WrappingShl::wrapping_shl(two, 1), four);
                assert_eq!(WrappingShl::wrapping_shl(one, 2), four);
                assert_eq!(WrappingShl::wrapping_shl(one, <$ty>::BITS), one);
                assert_eq!(
                    Object::new(one).wrapping_shl(1),
                    Object::new(WrappingShl::wrapping_shl(one, 1))
                );
            }

            #[test]
            fn test_shr() {
                let zero = <$ty as ConstZero>::ZERO;
                let one = <$ty as ConstOne>::ONE;
                let two = one + one;
                let four = two + two;

                assert_eq!(WrappingShr::wrapping_shr(zero, 1), zero);
                assert_eq!(WrappingShr::wrapping_shr(one, 1), zero);
                assert_eq!(WrappingShr::wrapping_shr(two, 1), one);
                assert_eq!(WrappingShr::wrapping_shr(four, 1), two);
                assert_eq!(WrappingShr::wrapping_shr(four, 2), one);
                assert_eq!(WrappingShr::wrapping_shr(one, <$ty>::BITS), one);
                assert_eq!(
                    Object::new(four).wrapping_shr(2),
                    Object::new(WrappingShr::wrapping_shr(four, 2))
                );
            }
        }
    };
}

macro_rules! test_wrapping_bitshift_traits_int_negative {
    ($ty:ty, $tests_mod:ident) => {
        #[cfg(test)]
        mod $tests_mod {
            use crate::elem::*;
            use crate::ops::*;
            use crate::types::Object;

            #[test]
            fn test_shl() {
                let one = <$ty as ConstOne>::ONE;
                let two = one + one;
                let four = two + two;

                assert_eq!(WrappingShl::wrapping_shl(-one, 1), -two);
                assert_eq!(WrappingShl::wrapping_shl(-two, 1), -four);
                assert_eq!(WrappingShl::wrapping_shl(-one, <$ty>::BITS), -one);
                assert_eq!(
                    Object::new(-one).wrapping_shl(1),
                    Object::new(WrappingShl::wrapping_shl(-one, 1))
                );
            }

            #[test]
            fn test_shr() {
                let one = <$ty as ConstOne>::ONE;
                let two = one + one;
                let four = two + two;

                assert_eq!(WrappingShr::wrapping_shr(-two, 1), -one);
                assert_eq!(WrappingShr::wrapping_shr(-four, 1), -two);
                assert_eq!(WrappingShr::wrapping_shr(-four, 2), -one);
                assert_eq!(WrappingShr::wrapping_shr(-one, <$ty>::BITS), -one);
                assert_eq!(
                    Object::new(-four).wrapping_shr(2),
                    Object::new(WrappingShr::wrapping_shr(-four, 2))
                );
            }
        }
    };
}

for_signed_ints!(
    test_wrapping_bitshift_traits_int_nonnegative; test_wrapping_bitshift_traits_int_negative
);
for_unsigned_ints!(test_wrapping_bitshift_traits_int_nonnegative);
