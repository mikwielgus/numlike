// SPDX-FileCopyrightText: 2026 numlike contributors
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use core::num::{Saturating, Wrapping};
use core::ops::{
    BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Not, Shl, ShlAssign, Shr,
    ShrAssign,
};

/// Bundle of bitwise operations, including assign variants.
pub trait FullBitOps<Rhs = Self>: BitOps<Rhs> + BitAssignOps<Rhs> {}

/// Bundle of bitwise operations.
pub trait BitOps<Rhs = Self>:
    BitAnd<Rhs> + BitOr<Rhs> + BitXor<Rhs> + Not + Shl<Rhs> + Shr<Rhs>
{
}
impl<Rhs, T: BitAnd<Rhs> + BitOr<Rhs> + BitXor<Rhs> + Not + Shl<Rhs> + Shr<Rhs>> BitOps<Rhs> for T {}

/// Bundle of bitwise assignment operations.
pub trait BitAssignOps<Rhs = Self>:
    BitAndAssign<Rhs>
    + BitOrAssign<Rhs>
    + BitXorAssign<Rhs>
    + NotAssign
    + ShlAssign<Rhs>
    + ShrAssign<Rhs>
{
}
impl<
    Rhs,
    T: BitAndAssign<Rhs>
        + BitOrAssign<Rhs>
        + BitXorAssign<Rhs>
        + NotAssign
        + ShlAssign<Rhs>
        + ShrAssign<Rhs>,
> BitAssignOps<Rhs> for T
{
}

/// Inverts all bits of `self` in place.
pub trait NotAssign {
    /// Inverts all bits of `self` in place.
    fn not_assign(&mut self);
}

macro_rules! impl_not_assign_for_bitwisables {
    ($($ty:ty),*) => {
        $(
            impl NotAssign for $ty {
                #[inline]
                fn not_assign(&mut self) {
                    *self = Not::not(*self);
                }
            }
        )*
    };
}

impl_not_assign_for_bitwisables!(i8, i16, i32, i64, i128, isize);
impl_not_assign_for_bitwisables!(u8, u16, u32, u64, u128, usize);
impl_not_assign_for_bitwisables!(bool);
impl_not_assign_for_bitwisables!(
    Wrapping<i8>,
    Wrapping<i16>,
    Wrapping<i32>,
    Wrapping<i64>,
    Wrapping<i128>,
    Wrapping<isize>,
    Wrapping<u8>,
    Wrapping<u16>,
    Wrapping<u32>,
    Wrapping<u64>,
    Wrapping<u128>,
    Wrapping<usize>
);
impl_not_assign_for_bitwisables!(
    Saturating<i8>,
    Saturating<i16>,
    Saturating<i32>,
    Saturating<i64>,
    Saturating<i128>,
    Saturating<isize>,
    Saturating<u8>,
    Saturating<u16>,
    Saturating<u32>,
    Saturating<u64>,
    Saturating<u128>,
    Saturating<usize>
);

macro_rules! test_plain_bitshift_traits_int_nonnegative {
    ($ty:ty, $tests_mod:ident) => {
        #[cfg(test)]
        mod $tests_mod {
            use core::ops::{Shl, ShlAssign, Shr, ShrAssign};

            use crate::elem::*;

            #[test]
            fn test_shl() {
                let zero = <$ty as Zero>::ZERO;
                let one = <$ty as One>::ONE;
                let two = one + one;
                let four = two + two;

                assert_eq!(Shl::shl(zero, one), zero);
                assert_eq!(Shl::shl(one, one), two);
                assert_eq!(Shl::shl(two, one), four);
                assert_eq!(Shl::shl(one, two), four);
            }

            #[test]
            fn test_shr() {
                let zero = <$ty as Zero>::ZERO;
                let one = <$ty as One>::ONE;
                let two = one + one;
                let four = two + two;

                assert_eq!(Shr::shr(zero, one), zero);
                assert_eq!(Shr::shr(one, one), zero);
                assert_eq!(Shr::shr(two, one), one);
                assert_eq!(Shr::shr(four, one), two);
                assert_eq!(Shr::shr(four, two), one);
            }

            #[test]
            fn test_shl_assign() {
                let one = <$ty as One>::ONE;
                let two = one + one;
                let four = two + two;
                let mut value = one;

                ShlAssign::shl_assign(&mut value, one);
                assert_eq!(value, two);

                ShlAssign::shl_assign(&mut value, one);
                assert_eq!(value, four);
            }

            #[test]
            fn test_shr_assign() {
                let one = <$ty as One>::ONE;
                let two = one + one;
                let four = two + two;
                let mut value = four;

                ShrAssign::shr_assign(&mut value, one);
                assert_eq!(value, two);

                ShrAssign::shr_assign(&mut value, one);
                assert_eq!(value, one);
            }
        }
    };
}

macro_rules! test_plain_bitshift_traits_int_negative {
    ($ty:ty, $tests_mod:ident) => {
        #[cfg(test)]
        mod $tests_mod {
            use core::ops::{Shl, ShlAssign, Shr, ShrAssign};

            use crate::elem::*;

            #[test]
            fn test_shl() {
                let one = <$ty as One>::ONE;
                let two = one + one;
                let four = two + two;

                assert_eq!(Shl::shl(-one, one), -two);
                assert_eq!(Shl::shl(-two, one), -four);
            }

            #[test]
            fn test_shr() {
                let one = <$ty as One>::ONE;
                let two = one + one;
                let four = two + two;

                assert_eq!(Shr::shr(-two, one), -one);
                assert_eq!(Shr::shr(-four, one), -two);
                assert_eq!(Shr::shr(-four, two), -one);
            }

            #[test]
            fn test_shl_assign() {
                let one = <$ty as One>::ONE;
                let two = one + one;
                let four = two + two;
                let mut value = -one;

                ShlAssign::shl_assign(&mut value, one);
                assert_eq!(value, -two);

                ShlAssign::shl_assign(&mut value, one);
                assert_eq!(value, -four);
            }

            #[test]
            fn test_shr_assign() {
                let one = <$ty as One>::ONE;
                let two = one + one;
                let four = two + two;
                let mut value = -four;

                ShrAssign::shr_assign(&mut value, one);
                assert_eq!(value, -two);

                ShrAssign::shr_assign(&mut value, one);
                assert_eq!(value, -one);
            }
        }
    };
}

test_plain_bitshift_traits_int_nonnegative!(i8, i8_nonnegative_tests);
test_plain_bitshift_traits_int_negative!(i8, i8_negative_tests);
test_plain_bitshift_traits_int_nonnegative!(i16, i16_nonnegative_tests);
test_plain_bitshift_traits_int_negative!(i16, i16_negative_tests);
test_plain_bitshift_traits_int_nonnegative!(i32, i32_nonnegative_tests);
test_plain_bitshift_traits_int_negative!(i32, i32_negative_tests);
test_plain_bitshift_traits_int_nonnegative!(i64, i64_nonnegative_tests);
test_plain_bitshift_traits_int_negative!(i64, i64_negative_tests);
test_plain_bitshift_traits_int_nonnegative!(i128, i128_nonnegative_tests);
test_plain_bitshift_traits_int_negative!(i128, i128_negative_tests);
test_plain_bitshift_traits_int_nonnegative!(isize, isize_nonnegative_tests);
test_plain_bitshift_traits_int_negative!(isize, isize_negative_tests);

test_plain_bitshift_traits_int_nonnegative!(u8, u8_tests);
test_plain_bitshift_traits_int_nonnegative!(u16, u16_tests);
test_plain_bitshift_traits_int_nonnegative!(u32, u32_tests);
test_plain_bitshift_traits_int_nonnegative!(u64, u64_tests);
test_plain_bitshift_traits_int_nonnegative!(u128, u128_tests);
test_plain_bitshift_traits_int_nonnegative!(usize, usize_tests);
