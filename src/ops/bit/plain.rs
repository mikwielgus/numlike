// SPDX-FileCopyrightText: 2026 numlike contributors
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use core::ops::{
    BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Not, Shl, ShlAssign, Shr,
    ShrAssign,
};

use crate::types::Object;

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

impl<B: NotAssign> NotAssign for Object<B> {
    fn not_assign(&mut self) {
        <B as NotAssign>::not_assign(&mut self.0)
    }
}

impl<B: NotAssign> Object<B> {
    /// Inverts all bits of `self` in place.
    pub fn not_assign(&mut self) {
        <B as NotAssign>::not_assign(&mut self.0)
    }
}

macro_rules! impl_not_assign_for_bitwisables {
    ($ty:ty $(, $rest:ident)*) => {
        impl NotAssign for $ty {
            #[inline]
            fn not_assign(&mut self) {
                *self = Not::not(*self);
            }
        }
    };
}

for_all_signed_ints!(impl_not_assign_for_bitwisables);
for_all_unsigned_ints!(impl_not_assign_for_bitwisables);
impl_not_assign_for_bitwisables!(bool);

macro_rules! test_plain_bitshift_traits_int_nonnegative {
    ($ty:ty, $tests_mod:ident) => {
        #[cfg(test)]
        mod $tests_mod {
            use core::ops::{Shl, ShlAssign, Shr, ShrAssign};

            use crate::elem::*;
            use crate::types::Object;

            #[test]
            fn test_shl() {
                let zero = <$ty as ConstZero>::ZERO;
                let one = <$ty as ConstOne>::ONE;
                let two = one + one;
                let four = two + two;

                assert_eq!(Shl::shl(zero, one), zero);
                assert_eq!(Shl::shl(one, one), two);
                assert_eq!(Shl::shl(two, one), four);
                assert_eq!(Shl::shl(one, two), four);
                assert_eq!(
                    Shl::shl(Object::new(one), Object::new(one)),
                    Object::new(Shl::shl(one, one))
                );
            }

            #[test]
            fn test_shr() {
                let zero = <$ty as ConstZero>::ZERO;
                let one = <$ty as ConstOne>::ONE;
                let two = one + one;
                let four = two + two;

                assert_eq!(Shr::shr(zero, one), zero);
                assert_eq!(Shr::shr(one, one), zero);
                assert_eq!(Shr::shr(two, one), one);
                assert_eq!(Shr::shr(four, one), two);
                assert_eq!(Shr::shr(four, two), one);
                assert_eq!(
                    Shr::shr(Object::new(four), Object::new(two)),
                    Object::new(Shr::shr(four, two))
                );
            }

            #[test]
            fn test_shl_assign() {
                let one = <$ty as ConstOne>::ONE;
                let two = one + one;
                let four = two + two;
                let mut value = one;

                ShlAssign::shl_assign(&mut value, one);
                assert_eq!(value, two);

                ShlAssign::shl_assign(&mut value, one);
                assert_eq!(value, four);

                let mut object_value = Object::new(one);

                ShlAssign::shl_assign(&mut object_value, Object::new(one));

                assert_eq!(object_value, Object::new(two));
            }

            #[test]
            fn test_shr_assign() {
                let one = <$ty as ConstOne>::ONE;
                let two = one + one;
                let four = two + two;
                let mut value = four;

                ShrAssign::shr_assign(&mut value, one);
                assert_eq!(value, two);

                ShrAssign::shr_assign(&mut value, one);
                assert_eq!(value, one);

                let mut object_value = Object::new(four);

                ShrAssign::shr_assign(&mut object_value, Object::new(one));

                assert_eq!(object_value, Object::new(two));
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
            use crate::types::Object;

            #[test]
            fn test_shl() {
                let one = <$ty as ConstOne>::ONE;
                let two = one + one;
                let four = two + two;

                assert_eq!(Shl::shl(-one, one), -two);
                assert_eq!(Shl::shl(-two, one), -four);
                assert_eq!(
                    Shl::shl(Object::new(-one), Object::new(one)),
                    Object::new(Shl::shl(-one, one))
                );
            }

            #[test]
            fn test_shr() {
                let one = <$ty as ConstOne>::ONE;
                let two = one + one;
                let four = two + two;

                assert_eq!(Shr::shr(-two, one), -one);
                assert_eq!(Shr::shr(-four, one), -two);
                assert_eq!(Shr::shr(-four, two), -one);
                assert_eq!(
                    Shr::shr(Object::new(-four), Object::new(two)),
                    Object::new(Shr::shr(-four, two))
                );
            }

            #[test]
            fn test_shl_assign() {
                let one = <$ty as ConstOne>::ONE;
                let two = one + one;
                let four = two + two;
                let mut value = -one;

                ShlAssign::shl_assign(&mut value, one);
                assert_eq!(value, -two);

                ShlAssign::shl_assign(&mut value, one);
                assert_eq!(value, -four);

                let mut object_value = Object::new(-one);

                ShlAssign::shl_assign(&mut object_value, Object::new(one));

                assert_eq!(object_value, Object::new(-two));
            }

            #[test]
            fn test_shr_assign() {
                let one = <$ty as ConstOne>::ONE;
                let two = one + one;
                let four = two + two;
                let mut value = -four;

                ShrAssign::shr_assign(&mut value, one);
                assert_eq!(value, -two);

                ShrAssign::shr_assign(&mut value, one);
                assert_eq!(value, -one);

                let mut object_value = Object::new(-four);

                ShrAssign::shr_assign(&mut object_value, Object::new(one));

                assert_eq!(object_value, Object::new(-two));
            }
        }
    };
}

for_signed_ints!(
    test_plain_bitshift_traits_int_nonnegative; test_plain_bitshift_traits_int_negative
);
for_unsigned_ints!(test_plain_bitshift_traits_int_nonnegative);
