// SPDX-FileCopyrightText: 2026 numlike contributors
//
// SPDX-License-Identifier: MIT OR Apache-2.0

//! Equality and ordering with defined NaN behavior.

use crate::fns::{IsNegative, IsPositive};
use crate::types::Object;
use core::cmp::Ordering;

/// Compare the value to zero. This is just `sgn` function, but returns
/// `Ordering`.
pub trait Cmp0 {
    /// Compare the value to zero. This is just `sgn` function, but returns
    /// `Ordering`.
    fn cmp0(&self) -> Ordering;
}

impl<B: Cmp0> Cmp0 for Object<B> {
    fn cmp0(&self) -> Ordering {
        <B as Cmp0>::cmp0(&self.0)
    }
}

impl<B: Cmp0> Object<B> {
    /// Compare the value to zero. This is just `sgn` function, but returns
    /// `Ordering`.
    pub fn cmp0(&self) -> Ordering {
        <B as Cmp0>::cmp0(&self.0)
    }
}

macro_rules! impl_cmp0_trait_for_signed {
    ($ty:ty) => {
        impl Cmp0 for $ty {
            #[inline]
            fn cmp0(&self) -> Ordering {
                if <$ty as IsPositive>::is_positive(*self) {
                    Ordering::Greater
                } else if <$ty as IsNegative>::is_negative(*self) {
                    Ordering::Less
                } else {
                    Ordering::Equal
                }
            }
        }
    };
    ($ty:ty, $nonnegative_tests_mod:ident) => {
        impl_cmp0_trait_for_signed!($ty);

        test_cmp0_trait_nonnegative!($ty, $nonnegative_tests_mod);
    };
    ($ty:ty, $nonnegative_tests_mod:ident, $negative_tests_mod:ident) => {
        impl_cmp0_trait_for_signed!($ty, $nonnegative_tests_mod);

        test_cmp0_trait_negative!($ty, $negative_tests_mod);
    };
}

macro_rules! test_cmp0_trait_nonnegative {
    ($ty:ty, $tests_mod:ident) => {
        #[cfg(test)]
        mod $tests_mod {
            use crate::cmp::*;
            use crate::elem::*;
            use crate::types::Object;

            #[test]
            fn test_cmp0() {
                let zero = <$ty as ConstZero>::ZERO;
                let one = <$ty as ConstOne>::ONE;
                let two = one + one;

                assert_eq!(Cmp0::cmp0(&zero), Ordering::Equal);
                assert_eq!(Cmp0::cmp0(&one), Ordering::Greater);
                assert_eq!(Cmp0::cmp0(&two), Ordering::Greater);

                assert_eq!(Object::new(two).cmp0(), Cmp0::cmp0(&two));
            }
        }
    };
}

macro_rules! test_cmp0_trait_negative {
    ($ty:ty, $tests_mod:ident) => {
        #[cfg(test)]
        mod $tests_mod {
            use crate::cmp::*;
            use crate::elem::*;
            use crate::types::Object;

            #[test]
            fn test_cmp0() {
                let one = <$ty as ConstOne>::ONE;
                let two = one + one;

                assert_eq!(Cmp0::cmp0(&-one), Ordering::Less);
                assert_eq!(Cmp0::cmp0(&-two), Ordering::Less);

                assert_eq!(Object::new(-two).cmp0(), Cmp0::cmp0(&-two));
            }
        }
    };
}

/// Trait for partial equality, identical to [`PartialEq`], except that `NaN` is
/// treated as equal to itself, in contradiction to the IEEE 754 floating point
/// number standard.
///
/// NaN-fix equality is virtually always total, so you probably want to also
/// implement [`NanfixEq`].
pub trait NanfixPartialEq<Rhs: ?Sized = Self> {
    /// Test whether self and the other value are equal, treating NaN as equal
    /// to itself.
    fn nanfix_eq(&self, other: &Rhs) -> bool;

    /// Test whether self and the other value not equal, treating NaN as equal
    /// to itself.
    #[inline]
    fn nanfix_ne(&self, other: &Rhs) -> bool {
        !self.nanfix_eq(other)
    }
}

impl<B: NanfixPartialEq> NanfixPartialEq for Object<B> {
    fn nanfix_eq(&self, other: &Self) -> bool {
        <B as NanfixPartialEq>::nanfix_eq(&self.0, &other.0)
    }
}

impl<B: NanfixPartialEq> Object<B> {
    /// Test whether self and the other value are equal, treating NaN as equal
    /// to itself.
    pub fn nanfix_eq(&self, other: &Self) -> bool {
        <B as NanfixPartialEq>::nanfix_eq(&self.0, &other.0)
    }

    /// Test whether self and the other value not equal, treating NaN as equal
    /// to itself.
    pub fn nanfix_ne(&self, other: &Self) -> bool {
        <B as NanfixPartialEq>::nanfix_ne(&self.0, &other.0)
    }
}

/// Trait to be implemented if [`NanfixPartialEq`] is a total equality
/// (virtually always it is).
///
/// The distinction between [`NanfixPartialEq`] and [`NanfixEq`] primarily
/// exists to mirror Rust standard library's [`PartialEq`] and [`Eq`].
pub trait NanfixEq<Rhs: ?Sized = Self>: NanfixPartialEq {}

impl<B: NanfixEq> NanfixEq for Object<B> {}

/// Trait for partial order where NaNs are fixed to be the smallest element in
/// the set, smaller even than the negative infinity.
///
/// This is virtually always also a total order, so most likely you want to
/// implement [`NanminOrd`] too.
pub trait NanminPartialOrd<Rhs: ?Sized = Self>: NanfixPartialEq {
    /// This method returns an (NaN-min) ordering between self and other values
    /// if one exists.
    fn nanmin_partial_cmp(&self, other: &Rhs) -> Option<Ordering>;

    /// Checks if `self` is (NaN-min) less than `other`.
    #[inline]
    fn nanmin_lt(&self, other: &Rhs) -> bool {
        self.nanmin_partial_cmp(other).is_some_and(Ordering::is_lt)
    }

    /// Checks if `self` is (NaN-min) less or equal to `other`.
    #[inline]
    fn nanmin_le(&self, other: &Rhs) -> bool {
        self.nanmin_partial_cmp(other).is_some_and(Ordering::is_le)
    }

    /// Checks if `self` is (NaN-min) greater than `other`.
    #[inline]
    fn nanmin_gt(&self, other: &Rhs) -> bool {
        self.nanmin_partial_cmp(other).is_some_and(Ordering::is_gt)
    }

    /// Checks if `self` is (NaN-min) greater or equal to `other`.
    #[inline]
    fn nanmin_ge(&self, other: &Rhs) -> bool {
        self.nanmin_partial_cmp(other).is_some_and(Ordering::is_ge)
    }
}

impl<B: NanminPartialOrd> NanminPartialOrd for Object<B> {
    fn nanmin_partial_cmp(&self, other: &Self) -> Option<Ordering> {
        <B as NanminPartialOrd>::nanmin_partial_cmp(&self.0, &other.0)
    }
}

impl<B: NanminPartialOrd> Object<B> {
    /// This method returns an (NaN-min) ordering between self and other values
    /// if one exists.
    pub fn nanmin_partial_cmp(&self, other: &Self) -> Option<Ordering> {
        <B as NanminPartialOrd>::nanmin_partial_cmp(&self.0, &other.0)
    }

    /// Checks if `self` is (NaN-min) less than `other`.
    pub fn nanmin_lt(&self, other: &Self) -> bool {
        <B as NanminPartialOrd>::nanmin_lt(&self.0, &other.0)
    }

    /// Checks if `self` is (NaN-min) less or equal to `other`.
    pub fn nanmin_le(&self, other: &Self) -> bool {
        <B as NanminPartialOrd>::nanmin_le(&self.0, &other.0)
    }

    /// Checks if `self` is (NaN-min) greater than `other`.
    pub fn nanmin_gt(&self, other: &Self) -> bool {
        <B as NanminPartialOrd>::nanmin_gt(&self.0, &other.0)
    }

    /// Checks if `self` is (NaN-min) greater or equal to `other`.
    pub fn nanmin_ge(&self, other: &Self) -> bool {
        <B as NanminPartialOrd>::nanmin_ge(&self.0, &other.0)
    }
}

/// Trait for total order where NaNs are fixed to be the smallest element in the
/// set, smaller even than the negative infinity.
///
/// NaN-min order is virtually always total. The distinction between
/// [`NanminPartialOrd`] and [`NanminOrd`] primarily exists to mirror Rust
/// standard library's [`PartialOrd`] and [`Ord`].
pub trait NanminOrd<Rhs: ?Sized = Self>: NanfixEq + NanminPartialOrd {
    /// This method returns an (NaN-min) `Ordering` between `self` and `other`.
    ///
    /// By convention, `self.nanmin_cmp(&other)` returns the ordering matching
    /// the expression `self <operator> other` if true.
    fn nanmin_cmp(&self, other: &Rhs) -> Ordering;
}

impl<B: NanminOrd> NanminOrd for Object<B> {
    fn nanmin_cmp(&self, other: &Self) -> Ordering {
        <B as NanminOrd>::nanmin_cmp(&self.0, &other.0)
    }
}

impl<B: NanminOrd> Object<B> {
    /// This method returns an (NaN-min) `Ordering` between `self` and `other`.
    ///
    /// By convention, `self.nanmin_cmp(&other)` returns the ordering matching
    /// the expression `self <operator> other` if true.
    pub fn nanmin_cmp(&self, other: &Self) -> Ordering {
        <B as NanminOrd>::nanmin_cmp(&self.0, &other.0)
    }
}

/// Trait for partial order where NaNs are fixed to be the greatest element in
/// the set, greater even than the positive infinity.
///
/// This is virtually always also a total order, so most likely you want to
/// implement [`NanmaxOrd`] too.
pub trait NanmaxPartialOrd<Rhs: ?Sized = Self>: NanfixPartialEq {
    /// This method returns an (NaN-max) ordering between self and other values
    /// if one exists.
    fn nanmax_partial_cmp(&self, other: &Rhs) -> Option<Ordering>;

    /// Checks if `self` is (NaN-max) less than `other`.
    #[inline]
    fn nanmax_lt(&self, other: &Rhs) -> bool {
        self.nanmax_partial_cmp(other).is_some_and(Ordering::is_lt)
    }

    /// Checks if `self` is (NaN-max) less or equal to `other`.
    #[inline]
    fn nanmax_le(&self, other: &Rhs) -> bool {
        self.nanmax_partial_cmp(other).is_some_and(Ordering::is_le)
    }

    /// Checks if `self` is (NaN-max) greater than `other`.
    #[inline]
    fn nanmax_gt(&self, other: &Rhs) -> bool {
        self.nanmax_partial_cmp(other).is_some_and(Ordering::is_gt)
    }

    /// Checks if `self` is (NaN-max) greater or equal to `other`.
    #[inline]
    fn nanmax_ge(&self, other: &Rhs) -> bool {
        self.nanmax_partial_cmp(other).is_some_and(Ordering::is_ge)
    }
}

impl<B: NanmaxPartialOrd> NanmaxPartialOrd for Object<B> {
    fn nanmax_partial_cmp(&self, other: &Self) -> Option<Ordering> {
        <B as NanmaxPartialOrd>::nanmax_partial_cmp(&self.0, &other.0)
    }
}

impl<B: NanmaxPartialOrd> Object<B> {
    /// This method returns an (NaN-max) ordering between self and other values
    /// if one exists.
    pub fn nanmax_partial_cmp(&self, other: &Self) -> Option<Ordering> {
        <B as NanmaxPartialOrd>::nanmax_partial_cmp(&self.0, &other.0)
    }

    /// Checks if `self` is (NaN-max) less than `other`.
    pub fn nanmax_lt(&self, other: &Self) -> bool {
        <B as NanmaxPartialOrd>::nanmax_lt(&self.0, &other.0)
    }

    /// Checks if `self` is (NaN-max) less or equal to `other`.
    pub fn nanmax_le(&self, other: &Self) -> bool {
        <B as NanmaxPartialOrd>::nanmax_le(&self.0, &other.0)
    }

    /// Checks if `self` is (NaN-max) greater than `other`.
    pub fn nanmax_gt(&self, other: &Self) -> bool {
        <B as NanmaxPartialOrd>::nanmax_gt(&self.0, &other.0)
    }

    /// Checks if `self` is (NaN-max) greater or equal to `other`.
    pub fn nanmax_ge(&self, other: &Self) -> bool {
        <B as NanmaxPartialOrd>::nanmax_ge(&self.0, &other.0)
    }
}

/// Trait for total order where NaNs are fixed to be the greatest element in the
/// set, greater even than the positive infinity.
///
/// NaN-max order is virtually always total. The distinction between
/// [`NanmaxPartialOrd`] and [`NanmaxOrd`] primarily exists to mirror Rust
/// standard library's [`PartialOrd`] and [`Ord`].
pub trait NanmaxOrd<Rhs: ?Sized = Self>: NanfixEq + NanmaxPartialOrd {
    /// This method returns an (NaN-max) `Ordering` between `self` and `other`.
    ///
    /// By convention, `self.nanmax_cmp(&other)` returns the ordering matching
    /// the expression `self <operator> other` if true.
    fn nanmax_cmp(&self, other: &Rhs) -> Ordering;
}

impl<B: NanmaxOrd> NanmaxOrd for Object<B> {
    fn nanmax_cmp(&self, other: &Self) -> Ordering {
        <B as NanmaxOrd>::nanmax_cmp(&self.0, &other.0)
    }
}

impl<B: NanmaxOrd> Object<B> {
    /// This method returns an (NaN-max) `Ordering` between `self` and `other`.
    ///
    /// By convention, `self.nanmax_cmp(&other)` returns the ordering matching
    /// the expression `self <operator> other` if true.
    pub fn nanmax_cmp(&self, other: &Self) -> Ordering {
        <B as NanmaxOrd>::nanmax_cmp(&self.0, &other.0)
    }
}

macro_rules! impl_nanfix_eq_traits_for_ord {
    ($ty:ty) => {
        impl NanfixPartialEq<$ty> for $ty {
            #[inline]
            fn nanfix_eq(&self, other: &$ty) -> bool {
                PartialEq::eq(self, other)
            }
        }

        impl NanfixEq<$ty> for $ty {}
    };
    ($ty:ty, $nonnegative_tests_mod:ident) => {
        impl_nanfix_eq_traits_for_ord!($ty);

        test_nanfix_eq_traits_nonnegative!($ty, $nonnegative_tests_mod);
    };
    ($ty:ty, $nonnegative_tests_mod:ident, $negative_tests_mod:ident) => {
        impl_nanfix_eq_traits_for_ord!($ty, $nonnegative_tests_mod);

        test_nanfix_eq_traits_negative!($ty, $negative_tests_mod);
    };
}

macro_rules! test_nanfix_eq_traits_nonnegative {
    ($ty:ty, $tests_mod:ident) => {
        #[cfg(test)]
        mod $tests_mod {
            use crate::cmp::*;
            use crate::elem::*;
            use crate::types::Object;

            #[test]
            fn test_nanfix_eq() {
                let zero = <$ty as ConstZero>::ZERO;
                let one = <$ty as ConstOne>::ONE;
                let two = one + one;
                let four = two + two;

                assert!(NanfixPartialEq::nanfix_eq(&zero, &zero));
                assert!(NanfixPartialEq::nanfix_eq(&one, &one));
                assert!(NanfixPartialEq::nanfix_eq(&two, &two));
                assert!(NanfixPartialEq::nanfix_eq(&four, &four));

                assert!(!NanfixPartialEq::nanfix_eq(&zero, &one));
                assert!(!NanfixPartialEq::nanfix_eq(&one, &two));
                assert!(!NanfixPartialEq::nanfix_eq(&two, &four));

                assert_eq!(
                    Object::new(two).nanfix_eq(&Object::new(two)),
                    NanfixPartialEq::nanfix_eq(&two, &two)
                );
            }

            #[test]
            fn test_nanfix_ne() {
                let zero = <$ty as ConstZero>::ZERO;
                let one = <$ty as ConstOne>::ONE;
                let two = one + one;
                let four = two + two;

                assert!(!NanfixPartialEq::nanfix_ne(&zero, &zero));
                assert!(!NanfixPartialEq::nanfix_ne(&one, &one));
                assert!(!NanfixPartialEq::nanfix_ne(&two, &two));
                assert!(!NanfixPartialEq::nanfix_ne(&four, &four));

                assert!(NanfixPartialEq::nanfix_ne(&zero, &one));
                assert!(NanfixPartialEq::nanfix_ne(&one, &two));
                assert!(NanfixPartialEq::nanfix_ne(&two, &four));

                assert_eq!(
                    Object::new(two).nanfix_ne(&Object::new(four)),
                    NanfixPartialEq::nanfix_ne(&two, &four)
                );
            }
        }
    };
}

macro_rules! test_nanfix_eq_traits_negative {
    ($ty:ty, $tests_mod:ident) => {
        #[cfg(test)]
        mod $tests_mod {
            use crate::cmp::*;
            use crate::elem::*;
            use crate::types::Object;

            #[test]
            fn test_nanfix_eq() {
                let one = <$ty as ConstOne>::ONE;
                let two = one + one;
                let four = two + two;

                assert!(NanfixPartialEq::nanfix_eq(&(-one), &(-one)));
                assert!(NanfixPartialEq::nanfix_eq(&(-two), &(-two)));
                assert!(NanfixPartialEq::nanfix_eq(&(-four), &(-four)));

                assert!(!NanfixPartialEq::nanfix_eq(&(-one), &(-two)));
                assert!(!NanfixPartialEq::nanfix_eq(&(-two), &(-four)));
                assert!(!NanfixPartialEq::nanfix_eq(&(-one), &one));

                assert_eq!(
                    Object::new(-two).nanfix_eq(&Object::new(-two)),
                    NanfixPartialEq::nanfix_eq(&(-two), &(-two))
                );
            }

            #[test]
            fn test_nanfix_ne() {
                let one = <$ty as ConstOne>::ONE;
                let two = one + one;
                let four = two + two;

                assert!(!NanfixPartialEq::nanfix_ne(&(-one), &(-one)));
                assert!(!NanfixPartialEq::nanfix_ne(&(-two), &(-two)));
                assert!(!NanfixPartialEq::nanfix_ne(&(-four), &(-four)));

                assert!(NanfixPartialEq::nanfix_ne(&(-one), &(-two)));
                assert!(NanfixPartialEq::nanfix_ne(&(-two), &(-four)));
                assert!(NanfixPartialEq::nanfix_ne(&(-one), &one));

                assert_eq!(
                    Object::new(-two).nanfix_ne(&Object::new(-four)),
                    NanfixPartialEq::nanfix_ne(&(-two), &(-four))
                );
            }
        }
    };
}

macro_rules! impl_nanmin_nanmax_ord_traits_for_ord {
    ($ty:ty) => {
        impl NanminPartialOrd<$ty> for $ty {
            #[inline]
            fn nanmin_partial_cmp(&self, other: &$ty) -> Option<Ordering> {
                PartialOrd::partial_cmp(self, other)
            }
        }

        impl NanminOrd<$ty> for $ty {
            #[inline]
            fn nanmin_cmp(&self, other: &$ty) -> Ordering {
                Ord::cmp(self, other)
            }
        }

        impl NanmaxPartialOrd<$ty> for $ty {
            #[inline]
            fn nanmax_partial_cmp(&self, other: &$ty) -> Option<Ordering> {
                PartialOrd::partial_cmp(self, other)
            }
        }

        impl NanmaxOrd<$ty> for $ty {
            #[inline]
            fn nanmax_cmp(&self, other: &$ty) -> Ordering {
                Ord::cmp(self, other)
            }
        }
    };
    ($ty:ty, $nonnegative_tests_mod:ident) => {
        impl_nanmin_nanmax_ord_traits_for_ord!($ty);

        test_nanmin_nanmax_ord_traits_nonnegative!($ty, $nonnegative_tests_mod);
    };
    ($ty:ty, $nonnegative_tests_mod:ident, $negative_tests_mod:ident) => {
        impl_nanmin_nanmax_ord_traits_for_ord!($ty, $nonnegative_tests_mod);

        test_nanmin_nanmax_ord_traits_negative!($ty, $negative_tests_mod);
    };
}
macro_rules! impl_nanfix_eq_traits_for_float {
    ($ty:ty, $nonnegative_tests_mod:ident, $negative_tests_mod:ident, $nan_tests_mod:ident) => {
        impl NanfixPartialEq<$ty> for $ty {
            #[inline]
            fn nanfix_eq(&self, other: &$ty) -> bool {
                (self.is_nan() && other.is_nan()) || PartialEq::eq(self, other)
            }
        }

        impl NanfixEq<$ty> for $ty {}

        test_nanfix_eq_traits_nonnegative!($ty, $nonnegative_tests_mod);
        test_nanfix_eq_traits_negative!($ty, $negative_tests_mod);
        test_nanfix_eq_traits_nan!($ty, $nan_tests_mod);
    };
}

macro_rules! impl_nanmin_nanmax_ord_traits_for_float {
    ($ty:ty, $nonnegative_tests_mod:ident, $negative_tests_mod:ident, $nan_tests_mod:ident) => {
        impl NanmaxPartialOrd<$ty> for $ty {
            #[inline]
            fn nanmax_partial_cmp(&self, other: &$ty) -> Option<Ordering> {
                Some(NanmaxOrd::nanmax_cmp(self, other))
            }
        }

        impl NanmaxOrd<$ty> for $ty {
            #[inline]
            fn nanmax_cmp(&self, other: &$ty) -> Ordering {
                match (self.is_nan(), other.is_nan()) {
                    (true, true) => Ordering::Equal,
                    (true, false) => Ordering::Greater,
                    (false, true) => Ordering::Less,
                    (false, false) => PartialOrd::partial_cmp(self, other).unwrap(),
                }
            }
        }

        impl NanminPartialOrd<$ty> for $ty {
            #[inline]
            fn nanmin_partial_cmp(&self, other: &$ty) -> Option<Ordering> {
                Some(NanminOrd::nanmin_cmp(self, other))
            }
        }

        impl NanminOrd<$ty> for $ty {
            #[inline]
            fn nanmin_cmp(&self, other: &$ty) -> Ordering {
                match (self.is_nan(), other.is_nan()) {
                    (true, true) => Ordering::Equal,
                    (true, false) => Ordering::Less,
                    (false, true) => Ordering::Greater,
                    (false, false) => PartialOrd::partial_cmp(self, other).unwrap(),
                }
            }
        }

        test_nanmin_nanmax_ord_traits_nonnegative!($ty, $nonnegative_tests_mod);
        test_nanmin_nanmax_ord_traits_negative!($ty, $negative_tests_mod);
        test_nan_for_nanmin_nanmax_ord_traits!($ty, $nan_tests_mod);
    };
}

macro_rules! test_nanfix_eq_traits_nan {
    ($ty:ty, $tests_mod:ident) => {
        #[cfg(test)]
        mod $tests_mod {
            use crate::cmp::*;
            use crate::elem::*;
            use crate::types::Object;

            #[test]
            fn test_nan_nanfix_eq() {
                let zero = <$ty as ConstZero>::ZERO;
                let nan = <$ty>::NAN;

                assert!(NanfixPartialEq::nanfix_eq(&nan, &nan));

                assert!(!NanfixPartialEq::nanfix_eq(&nan, &zero));
                assert!(!NanfixPartialEq::nanfix_eq(&zero, &nan));

                assert_eq!(
                    Object::new(nan).nanfix_eq(&Object::new(nan)),
                    NanfixPartialEq::nanfix_eq(&nan, &nan)
                );
            }

            #[test]
            fn test_nan_nanfix_ne() {
                let zero = <$ty as ConstZero>::ZERO;
                let nan = <$ty>::NAN;

                assert!(!NanfixPartialEq::nanfix_ne(&nan, &nan));

                assert!(NanfixPartialEq::nanfix_ne(&nan, &zero));
                assert!(NanfixPartialEq::nanfix_ne(&zero, &nan));

                assert_eq!(
                    Object::new(nan).nanfix_ne(&Object::new(zero)),
                    NanfixPartialEq::nanfix_ne(&nan, &zero)
                );
            }
        }
    };
}

macro_rules! test_nanmin_nanmax_ord_traits_nonnegative {
    ($ty:ty, $tests_mod:ident) => {
        #[cfg(test)]
        mod $tests_mod {
            use core::cmp::Ordering;

            use crate::cmp::*;
            use crate::elem::*;
            use crate::types::Object;

            #[test]
            fn test_nanmin_cmp() {
                let zero = <$ty as ConstZero>::ZERO;
                let one = <$ty as ConstOne>::ONE;
                let two = one + one;
                let four = two + two;

                assert_eq!(NanminOrd::nanmin_cmp(&zero, &zero), Ordering::Equal);
                assert_eq!(NanminOrd::nanmin_cmp(&one, &one), Ordering::Equal);
                assert_eq!(NanminOrd::nanmin_cmp(&two, &two), Ordering::Equal);
                assert_eq!(NanminOrd::nanmin_cmp(&four, &four), Ordering::Equal);

                assert_eq!(NanminOrd::nanmin_cmp(&zero, &one), Ordering::Less);
                assert_eq!(NanminOrd::nanmin_cmp(&one, &two), Ordering::Less);
                assert_eq!(NanminOrd::nanmin_cmp(&two, &four), Ordering::Less);

                assert_eq!(NanminOrd::nanmin_cmp(&one, &zero), Ordering::Greater);
                assert_eq!(NanminOrd::nanmin_cmp(&two, &one), Ordering::Greater);
                assert_eq!(NanminOrd::nanmin_cmp(&four, &two), Ordering::Greater);

                assert_eq!(
                    Object::new(zero).nanmin_cmp(&Object::new(one)),
                    NanminOrd::nanmin_cmp(&zero, &one)
                );
            }

            #[test]
            fn test_nanmax_cmp() {
                let zero = <$ty as ConstZero>::ZERO;
                let one = <$ty as ConstOne>::ONE;
                let two = one + one;
                let four = two + two;

                assert_eq!(NanmaxOrd::nanmax_cmp(&zero, &zero), Ordering::Equal);
                assert_eq!(NanmaxOrd::nanmax_cmp(&one, &one), Ordering::Equal);
                assert_eq!(NanmaxOrd::nanmax_cmp(&two, &two), Ordering::Equal);
                assert_eq!(NanmaxOrd::nanmax_cmp(&four, &four), Ordering::Equal);

                assert_eq!(NanmaxOrd::nanmax_cmp(&zero, &one), Ordering::Less);
                assert_eq!(NanmaxOrd::nanmax_cmp(&one, &two), Ordering::Less);
                assert_eq!(NanmaxOrd::nanmax_cmp(&two, &four), Ordering::Less);

                assert_eq!(NanmaxOrd::nanmax_cmp(&one, &zero), Ordering::Greater);
                assert_eq!(NanmaxOrd::nanmax_cmp(&two, &one), Ordering::Greater);
                assert_eq!(NanmaxOrd::nanmax_cmp(&four, &two), Ordering::Greater);

                assert_eq!(
                    Object::new(zero).nanmax_cmp(&Object::new(one)),
                    NanmaxOrd::nanmax_cmp(&zero, &one)
                );
            }

            #[test]
            fn test_nanmin_lt_le_gt_ge() {
                let zero = <$ty as ConstZero>::ZERO;
                let one = <$ty as ConstOne>::ONE;
                let two = one + one;

                assert!(NanminPartialOrd::nanmin_lt(&zero, &one));
                assert!(NanminPartialOrd::nanmin_le(&zero, &one));
                assert!(NanminPartialOrd::nanmin_le(&one, &one));
                assert!(!NanminPartialOrd::nanmin_lt(&one, &zero));
                assert!(!NanminPartialOrd::nanmin_le(&two, &one));

                assert!(NanminPartialOrd::nanmin_gt(&one, &zero));
                assert!(NanminPartialOrd::nanmin_ge(&one, &zero));
                assert!(NanminPartialOrd::nanmin_ge(&one, &one));
                assert!(!NanminPartialOrd::nanmin_gt(&zero, &one));
                assert!(!NanminPartialOrd::nanmin_ge(&one, &two));

                assert_eq!(
                    Object::new(zero).nanmin_lt(&Object::new(one)),
                    NanminPartialOrd::nanmin_lt(&zero, &one)
                );
            }

            #[test]
            fn test_nanmax_lt_le_gt_ge() {
                let zero = <$ty as ConstZero>::ZERO;
                let one = <$ty as ConstOne>::ONE;
                let two = one + one;

                assert!(NanmaxPartialOrd::nanmax_lt(&zero, &one));
                assert!(NanmaxPartialOrd::nanmax_le(&zero, &one));
                assert!(NanmaxPartialOrd::nanmax_le(&one, &one));
                assert!(!NanmaxPartialOrd::nanmax_lt(&one, &zero));
                assert!(!NanmaxPartialOrd::nanmax_le(&two, &one));

                assert!(NanmaxPartialOrd::nanmax_gt(&one, &zero));
                assert!(NanmaxPartialOrd::nanmax_ge(&one, &zero));
                assert!(NanmaxPartialOrd::nanmax_ge(&one, &one));
                assert!(!NanmaxPartialOrd::nanmax_gt(&zero, &one));
                assert!(!NanmaxPartialOrd::nanmax_ge(&one, &two));

                assert_eq!(
                    Object::new(zero).nanmax_lt(&Object::new(one)),
                    NanmaxPartialOrd::nanmax_lt(&zero, &one)
                );
            }

            #[test]
            fn test_nanmin_partial_cmp() {
                let zero = <$ty as ConstZero>::ZERO;
                let one = <$ty as ConstOne>::ONE;

                assert_eq!(
                    NanminPartialOrd::nanmin_partial_cmp(&zero, &zero),
                    Some(Ordering::Equal)
                );
                assert_eq!(
                    NanminPartialOrd::nanmin_partial_cmp(&zero, &one),
                    Some(Ordering::Less)
                );
                assert_eq!(
                    NanminPartialOrd::nanmin_partial_cmp(&one, &zero),
                    Some(Ordering::Greater)
                );

                assert_eq!(
                    Object::new(zero).nanmin_partial_cmp(&Object::new(one)),
                    NanminPartialOrd::nanmin_partial_cmp(&zero, &one)
                );
            }

            #[test]
            fn test_nanmax_partial_cmp() {
                let zero = <$ty as ConstZero>::ZERO;
                let one = <$ty as ConstOne>::ONE;

                assert_eq!(
                    NanmaxPartialOrd::nanmax_partial_cmp(&zero, &zero),
                    Some(Ordering::Equal)
                );
                assert_eq!(
                    NanmaxPartialOrd::nanmax_partial_cmp(&zero, &one),
                    Some(Ordering::Less)
                );
                assert_eq!(
                    NanmaxPartialOrd::nanmax_partial_cmp(&one, &zero),
                    Some(Ordering::Greater)
                );

                assert_eq!(
                    Object::new(zero).nanmax_partial_cmp(&Object::new(one)),
                    NanmaxPartialOrd::nanmax_partial_cmp(&zero, &one)
                );
            }
        }
    };
}

macro_rules! test_nanmin_nanmax_ord_traits_negative {
    ($ty:ty, $tests_mod:ident) => {
        #[cfg(test)]
        mod $tests_mod {
            use core::cmp::Ordering;

            use crate::cmp::*;
            use crate::elem::*;
            use crate::types::Object;

            #[test]
            fn test_nanmin_cmp() {
                let one = <$ty as ConstOne>::ONE;
                let two = one + one;
                let four = two + two;

                assert_eq!(NanminOrd::nanmin_cmp(&(-one), &(-one)), Ordering::Equal);
                assert_eq!(NanminOrd::nanmin_cmp(&(-two), &(-two)), Ordering::Equal);
                assert_eq!(NanminOrd::nanmin_cmp(&(-four), &(-four)), Ordering::Equal);

                assert_eq!(NanminOrd::nanmin_cmp(&(-four), &(-two)), Ordering::Less);
                assert_eq!(NanminOrd::nanmin_cmp(&(-two), &(-one)), Ordering::Less);
                assert_eq!(NanminOrd::nanmin_cmp(&(-one), &one), Ordering::Less);

                assert_eq!(NanminOrd::nanmin_cmp(&(-two), &(-four)), Ordering::Greater);
                assert_eq!(NanminOrd::nanmin_cmp(&(-one), &(-two)), Ordering::Greater);
                assert_eq!(NanminOrd::nanmin_cmp(&one, &(-one)), Ordering::Greater);

                assert_eq!(
                    Object::new(-two).nanmin_cmp(&Object::new(-one)),
                    NanminOrd::nanmin_cmp(&(-two), &(-one))
                );
            }

            #[test]
            fn test_nanmax_cmp() {
                let one = <$ty as ConstOne>::ONE;
                let two = one + one;
                let four = two + two;

                assert_eq!(NanmaxOrd::nanmax_cmp(&(-one), &(-one)), Ordering::Equal);
                assert_eq!(NanmaxOrd::nanmax_cmp(&(-two), &(-two)), Ordering::Equal);
                assert_eq!(NanmaxOrd::nanmax_cmp(&(-four), &(-four)), Ordering::Equal);

                assert_eq!(NanmaxOrd::nanmax_cmp(&(-four), &(-two)), Ordering::Less);
                assert_eq!(NanmaxOrd::nanmax_cmp(&(-two), &(-one)), Ordering::Less);
                assert_eq!(NanmaxOrd::nanmax_cmp(&(-one), &one), Ordering::Less);

                assert_eq!(NanmaxOrd::nanmax_cmp(&(-two), &(-four)), Ordering::Greater);
                assert_eq!(NanmaxOrd::nanmax_cmp(&(-one), &(-two)), Ordering::Greater);
                assert_eq!(NanmaxOrd::nanmax_cmp(&one, &(-one)), Ordering::Greater);

                assert_eq!(
                    Object::new(-two).nanmax_cmp(&Object::new(-one)),
                    NanmaxOrd::nanmax_cmp(&(-two), &(-one))
                );
            }

            #[test]
            fn test_nanmin_lt_le_gt_ge() {
                let one = <$ty as ConstOne>::ONE;
                let two = one + one;

                assert!(NanminPartialOrd::nanmin_lt(&(-two), &(-one)));
                assert!(NanminPartialOrd::nanmin_le(&(-two), &(-one)));
                assert!(NanminPartialOrd::nanmin_le(&(-one), &(-one)));
                assert!(!NanminPartialOrd::nanmin_lt(&(-one), &(-two)));
                assert!(!NanminPartialOrd::nanmin_le(&one, &(-one)));

                assert!(NanminPartialOrd::nanmin_gt(&(-one), &(-two)));
                assert!(NanminPartialOrd::nanmin_ge(&(-one), &(-two)));
                assert!(NanminPartialOrd::nanmin_ge(&(-one), &(-one)));
                assert!(!NanminPartialOrd::nanmin_gt(&(-two), &(-one)));
                assert!(!NanminPartialOrd::nanmin_ge(&(-one), &one));

                assert_eq!(
                    Object::new(-two).nanmin_lt(&Object::new(-one)),
                    NanminPartialOrd::nanmin_lt(&(-two), &(-one))
                );
            }

            #[test]
            fn test_nanmax_lt_le_gt_ge() {
                let one = <$ty as ConstOne>::ONE;
                let two = one + one;

                assert!(NanmaxPartialOrd::nanmax_lt(&(-two), &(-one)));
                assert!(NanmaxPartialOrd::nanmax_le(&(-two), &(-one)));
                assert!(NanmaxPartialOrd::nanmax_le(&(-one), &(-one)));
                assert!(!NanmaxPartialOrd::nanmax_lt(&(-one), &(-two)));
                assert!(!NanmaxPartialOrd::nanmax_le(&one, &(-one)));

                assert!(NanmaxPartialOrd::nanmax_gt(&(-one), &(-two)));
                assert!(NanmaxPartialOrd::nanmax_ge(&(-one), &(-two)));
                assert!(NanmaxPartialOrd::nanmax_ge(&(-one), &(-one)));
                assert!(!NanmaxPartialOrd::nanmax_gt(&(-two), &(-one)));
                assert!(!NanmaxPartialOrd::nanmax_ge(&(-one), &one));

                assert_eq!(
                    Object::new(-two).nanmax_lt(&Object::new(-one)),
                    NanmaxPartialOrd::nanmax_lt(&(-two), &(-one))
                );
            }
        }
    };
}

macro_rules! test_nan_for_nanmin_nanmax_ord_traits {
    ($ty:ty, $tests_mod:ident) => {
        #[cfg(test)]
        mod $tests_mod {
            use core::cmp::Ordering;

            use crate::cmp::*;
            use crate::elem::*;
            use crate::types::Object;

            #[test]
            fn test_nan_nanmin_cmp() {
                let zero = <$ty as ConstZero>::ZERO;
                let nan = <$ty>::NAN;
                let neg_infinity = <$ty>::NEG_INFINITY;
                let infinity = <$ty>::INFINITY;

                assert_eq!(NanminOrd::nanmin_cmp(&nan, &nan), Ordering::Equal);
                assert_eq!(NanminOrd::nanmin_cmp(&nan, &zero), Ordering::Less);
                assert_eq!(NanminOrd::nanmin_cmp(&zero, &nan), Ordering::Greater);
                assert_eq!(NanminOrd::nanmin_cmp(&nan, &neg_infinity), Ordering::Less);
                assert_eq!(NanminOrd::nanmin_cmp(&nan, &infinity), Ordering::Less);
                assert_eq!(
                    NanminOrd::nanmin_cmp(&neg_infinity, &nan),
                    Ordering::Greater
                );
                assert_eq!(NanminOrd::nanmin_cmp(&infinity, &nan), Ordering::Greater);

                assert_eq!(
                    Object::new(nan).nanmin_cmp(&Object::new(zero)),
                    NanminOrd::nanmin_cmp(&nan, &zero)
                );
            }

            #[test]
            fn test_nan_nanmax_cmp() {
                let zero = <$ty as ConstZero>::ZERO;
                let nan = <$ty>::NAN;
                let neg_infinity = <$ty>::NEG_INFINITY;
                let infinity = <$ty>::INFINITY;

                assert_eq!(NanmaxOrd::nanmax_cmp(&nan, &nan), Ordering::Equal);
                assert_eq!(NanmaxOrd::nanmax_cmp(&nan, &zero), Ordering::Greater);
                assert_eq!(NanmaxOrd::nanmax_cmp(&zero, &nan), Ordering::Less);
                assert_eq!(
                    NanmaxOrd::nanmax_cmp(&nan, &neg_infinity),
                    Ordering::Greater
                );
                assert_eq!(NanmaxOrd::nanmax_cmp(&nan, &infinity), Ordering::Greater);
                assert_eq!(NanmaxOrd::nanmax_cmp(&neg_infinity, &nan), Ordering::Less);
                assert_eq!(NanmaxOrd::nanmax_cmp(&infinity, &nan), Ordering::Less);

                assert_eq!(
                    Object::new(nan).nanmax_cmp(&Object::new(zero)),
                    NanmaxOrd::nanmax_cmp(&nan, &zero)
                );
            }

            #[test]
            fn test_nan_nanmin_lt_le_gt_ge() {
                let zero = <$ty as ConstZero>::ZERO;
                let nan = <$ty>::NAN;
                let neg_infinity = <$ty>::NEG_INFINITY;

                assert!(NanminPartialOrd::nanmin_lt(&nan, &zero));
                assert!(NanminPartialOrd::nanmin_lt(&nan, &neg_infinity));
                assert!(NanminPartialOrd::nanmin_le(&nan, &nan));
                assert!(!NanminPartialOrd::nanmin_lt(&zero, &nan));

                assert!(NanminPartialOrd::nanmin_gt(&zero, &nan));
                assert!(NanminPartialOrd::nanmin_ge(&nan, &nan));
                assert!(!NanminPartialOrd::nanmin_ge(&nan, &zero));
                assert!(!NanminPartialOrd::nanmin_gt(&nan, &zero));

                assert_eq!(
                    Object::new(nan).nanmin_lt(&Object::new(zero)),
                    NanminPartialOrd::nanmin_lt(&nan, &zero)
                );
            }

            #[test]
            fn test_nan_nanmax_lt_le_gt_ge() {
                let zero = <$ty as ConstZero>::ZERO;
                let nan = <$ty>::NAN;
                let infinity = <$ty>::INFINITY;

                assert!(NanmaxPartialOrd::nanmax_lt(&zero, &nan));
                assert!(NanmaxPartialOrd::nanmax_lt(&infinity, &nan));
                assert!(NanmaxPartialOrd::nanmax_le(&nan, &nan));
                assert!(!NanmaxPartialOrd::nanmax_lt(&nan, &zero));

                assert!(NanmaxPartialOrd::nanmax_gt(&nan, &zero));
                assert!(NanmaxPartialOrd::nanmax_ge(&nan, &nan));
                assert!(NanmaxPartialOrd::nanmax_ge(&nan, &infinity));
                assert!(!NanmaxPartialOrd::nanmax_ge(&zero, &nan));
                assert!(!NanmaxPartialOrd::nanmax_gt(&zero, &nan));

                assert_eq!(
                    Object::new(zero).nanmax_lt(&Object::new(nan)),
                    NanmaxPartialOrd::nanmax_lt(&zero, &nan)
                );
            }
        }
    };
}

for_all_signed_ints!(impl_cmp0_trait_for_signed, cmp0);

for_all_signed_ints!(impl_nanfix_eq_traits_for_ord);
for_all_unsigned_ints!(impl_nanfix_eq_traits_for_ord);
for_misc!(impl_nanfix_eq_traits_for_ord);

for_all_signed_ints!(impl_nanmin_nanmax_ord_traits_for_ord, ord);
for_all_unsigned_ints!(impl_nanmin_nanmax_ord_traits_for_ord, ord);
for_misc!(impl_nanmin_nanmax_ord_traits_for_ord);

impl_nanfix_eq_traits_for_float!(
    f32,
    f32_nonnegative_tests,
    f32_negative_tests,
    f32_nan_tests
);
impl_nanfix_eq_traits_for_float!(
    f64,
    f64_nonnegative_tests,
    f64_negative_tests,
    f64_nan_tests
);

impl_nanmin_nanmax_ord_traits_for_float!(
    f32,
    f32_ord_nonnegative_tests,
    f32_ord_negative_tests,
    f32_ord_nan_tests
);
impl_nanmin_nanmax_ord_traits_for_float!(
    f64,
    f64_ord_nonnegative_tests,
    f64_ord_negative_tests,
    f64_ord_nan_tests
);
