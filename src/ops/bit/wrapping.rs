// SPDX-FileCopyrightText: 2026 numlike contributors
//
// SPDX-License-Identifier: MIT OR Apache-2.0

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

macro_rules! impl_wrapping_shift_traits_for_ints {
    ($($ty:ty),*) => {
        $(
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
        )*
    };
}

impl_wrapping_shift_traits_for_ints!(i8, i16, i32, i64, i128, isize);
impl_wrapping_shift_traits_for_ints!(u8, u16, u32, u64, u128, usize);

macro_rules! test_wrapping_bitshift_traits_int_nonnegative {
    ($ty:ty, $tests_mod:ident) => {
        #[cfg(test)]
        mod $tests_mod {
            use crate::elem::*;
            use crate::ops::*;

            #[test]
            fn test_shl() {
                let zero = <$ty as Zero>::ZERO;
                let one = <$ty as One>::ONE;
                let two = one + one;
                let four = two + two;

                assert_eq!(WrappingShl::wrapping_shl(zero, 1), zero);
                assert_eq!(WrappingShl::wrapping_shl(one, 1), two);
                assert_eq!(WrappingShl::wrapping_shl(two, 1), four);
                assert_eq!(WrappingShl::wrapping_shl(one, 2), four);
                assert_eq!(WrappingShl::wrapping_shl(one, <$ty>::BITS), one);
            }

            #[test]
            fn test_shr() {
                let zero = <$ty as Zero>::ZERO;
                let one = <$ty as One>::ONE;
                let two = one + one;
                let four = two + two;

                assert_eq!(WrappingShr::wrapping_shr(zero, 1), zero);
                assert_eq!(WrappingShr::wrapping_shr(one, 1), zero);
                assert_eq!(WrappingShr::wrapping_shr(two, 1), one);
                assert_eq!(WrappingShr::wrapping_shr(four, 1), two);
                assert_eq!(WrappingShr::wrapping_shr(four, 2), one);
                assert_eq!(WrappingShr::wrapping_shr(one, <$ty>::BITS), one);
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

            #[test]
            fn test_shl() {
                let one = <$ty as One>::ONE;
                let two = one + one;
                let four = two + two;

                assert_eq!(WrappingShl::wrapping_shl(-one, 1), -two);
                assert_eq!(WrappingShl::wrapping_shl(-two, 1), -four);
                assert_eq!(WrappingShl::wrapping_shl(-one, <$ty>::BITS), -one);
            }

            #[test]
            fn test_shr() {
                let one = <$ty as One>::ONE;
                let two = one + one;
                let four = two + two;

                assert_eq!(WrappingShr::wrapping_shr(-two, 1), -one);
                assert_eq!(WrappingShr::wrapping_shr(-four, 1), -two);
                assert_eq!(WrappingShr::wrapping_shr(-four, 2), -one);
                assert_eq!(WrappingShr::wrapping_shr(-one, <$ty>::BITS), -one);
            }
        }
    };
}

test_wrapping_bitshift_traits_int_nonnegative!(i8, i8_nonnegative_tests);
test_wrapping_bitshift_traits_int_negative!(i8, i8_negative_tests);
test_wrapping_bitshift_traits_int_nonnegative!(i16, i16_nonnegative_tests);
test_wrapping_bitshift_traits_int_negative!(i16, i16_negative_tests);
test_wrapping_bitshift_traits_int_nonnegative!(i32, i32_nonnegative_tests);
test_wrapping_bitshift_traits_int_negative!(i32, i32_negative_tests);
test_wrapping_bitshift_traits_int_nonnegative!(i64, i64_nonnegative_tests);
test_wrapping_bitshift_traits_int_negative!(i64, i64_negative_tests);
test_wrapping_bitshift_traits_int_nonnegative!(i128, i128_nonnegative_tests);
test_wrapping_bitshift_traits_int_negative!(i128, i128_negative_tests);
test_wrapping_bitshift_traits_int_nonnegative!(isize, isize_nonnegative_tests);
test_wrapping_bitshift_traits_int_negative!(isize, isize_negative_tests);

test_wrapping_bitshift_traits_int_nonnegative!(u8, u8_tests);
test_wrapping_bitshift_traits_int_nonnegative!(u16, u16_tests);
test_wrapping_bitshift_traits_int_nonnegative!(u32, u32_tests);
test_wrapping_bitshift_traits_int_nonnegative!(u64, u64_tests);
test_wrapping_bitshift_traits_int_nonnegative!(u128, u128_tests);
test_wrapping_bitshift_traits_int_nonnegative!(usize, usize_tests);
