// SPDX-FileCopyrightText: 2026 numlike contributors
//
// SPDX-License-Identifier: MIT OR Apache-2.0

//! Equality and ordering with defined NaN behavior.

use core::cmp::Ordering;
use core::num::{Saturating, Wrapping};

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

/// Trait to be implemented if [`NanfixPartialEq`] is a total equality
/// (virtually always it is).
///
/// The distinction between [`NanfixPartialEq`] and [`NanfixEq`] primarily
/// exists to mirror Rust standard library's [`PartialEq`] and [`Eq`].
pub trait NanfixEq<Rhs: ?Sized = Self>: NanfixPartialEq {}

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

            #[test]
            fn test_nanfix_eq() {
                let zero = <$ty as Zero>::ZERO;
                let one = <$ty as One>::ONE;
                let two = one + one;
                let four = two + two;

                assert!(NanfixPartialEq::nanfix_eq(&zero, &zero));
                assert!(NanfixPartialEq::nanfix_eq(&one, &one));
                assert!(NanfixPartialEq::nanfix_eq(&two, &two));
                assert!(NanfixPartialEq::nanfix_eq(&four, &four));

                assert!(!NanfixPartialEq::nanfix_eq(&zero, &one));
                assert!(!NanfixPartialEq::nanfix_eq(&one, &two));
                assert!(!NanfixPartialEq::nanfix_eq(&two, &four));
            }

            #[test]
            fn test_nanfix_ne() {
                let zero = <$ty as Zero>::ZERO;
                let one = <$ty as One>::ONE;
                let two = one + one;
                let four = two + two;

                assert!(!NanfixPartialEq::nanfix_ne(&zero, &zero));
                assert!(!NanfixPartialEq::nanfix_ne(&one, &one));
                assert!(!NanfixPartialEq::nanfix_ne(&two, &two));
                assert!(!NanfixPartialEq::nanfix_ne(&four, &four));

                assert!(NanfixPartialEq::nanfix_ne(&zero, &one));
                assert!(NanfixPartialEq::nanfix_ne(&one, &two));
                assert!(NanfixPartialEq::nanfix_ne(&two, &four));
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

            #[test]
            fn test_nanfix_eq() {
                let one = <$ty as One>::ONE;
                let two = one + one;
                let four = two + two;

                assert!(NanfixPartialEq::nanfix_eq(&(-one), &(-one)));
                assert!(NanfixPartialEq::nanfix_eq(&(-two), &(-two)));
                assert!(NanfixPartialEq::nanfix_eq(&(-four), &(-four)));

                assert!(!NanfixPartialEq::nanfix_eq(&(-one), &(-two)));
                assert!(!NanfixPartialEq::nanfix_eq(&(-two), &(-four)));
                assert!(!NanfixPartialEq::nanfix_eq(&(-one), &one));
            }

            #[test]
            fn test_nanfix_ne() {
                let one = <$ty as One>::ONE;
                let two = one + one;
                let four = two + two;

                assert!(!NanfixPartialEq::nanfix_ne(&(-one), &(-one)));
                assert!(!NanfixPartialEq::nanfix_ne(&(-two), &(-two)));
                assert!(!NanfixPartialEq::nanfix_ne(&(-four), &(-four)));

                assert!(NanfixPartialEq::nanfix_ne(&(-one), &(-two)));
                assert!(NanfixPartialEq::nanfix_ne(&(-two), &(-four)));
                assert!(NanfixPartialEq::nanfix_ne(&(-one), &one));
            }
        }
    };
}

impl_nanfix_eq_traits_for_ord!(i8, i8_nonnegative_tests, i8_negative_tests);
impl_nanfix_eq_traits_for_ord!(i16, i16_nonnegative_tests, i16_negative_tests);
impl_nanfix_eq_traits_for_ord!(i32, i32_nonnegative_tests, i32_negative_tests);
impl_nanfix_eq_traits_for_ord!(i64, i64_nonnegative_tests, i64_negative_tests);
impl_nanfix_eq_traits_for_ord!(i128, i128_nonnegative_tests, i128_negative_tests);
impl_nanfix_eq_traits_for_ord!(isize, isize_nonnegative_tests, isize_negative_tests);

impl_nanfix_eq_traits_for_ord!(u8, u8_tests);
impl_nanfix_eq_traits_for_ord!(u16, u16_tests);
impl_nanfix_eq_traits_for_ord!(u32, u32_tests);
impl_nanfix_eq_traits_for_ord!(u64, u64_tests);
impl_nanfix_eq_traits_for_ord!(u128, u128_tests);
impl_nanfix_eq_traits_for_ord!(usize, usize_tests);

impl_nanfix_eq_traits_for_ord!(
    Wrapping<i8>,
    wrapping_i8_nonnegative_tests,
    wrapping_i8_negative_tests
);
impl_nanfix_eq_traits_for_ord!(
    Wrapping<i16>,
    wrapping_i16_nonnegative_tests,
    wrapping_i16_negative_tests
);
impl_nanfix_eq_traits_for_ord!(
    Wrapping<i32>,
    wrapping_i32_nonnegative_tests,
    wrapping_i32_negative_tests
);
impl_nanfix_eq_traits_for_ord!(
    Wrapping<i64>,
    wrapping_i64_nonnegative_tests,
    wrapping_i64_negative_tests
);
impl_nanfix_eq_traits_for_ord!(
    Wrapping<i128>,
    wrapping_i128_nonnegative_tests,
    wrapping_i128_negative_tests
);
impl_nanfix_eq_traits_for_ord!(
    Wrapping<isize>,
    wrapping_isize_nonnegative_tests,
    wrapping_isize_negative_tests
);

impl_nanfix_eq_traits_for_ord!(Wrapping<u8>, wrapping_u8_tests);
impl_nanfix_eq_traits_for_ord!(Wrapping<u16>, wrapping_u16_tests);
impl_nanfix_eq_traits_for_ord!(Wrapping<u32>, wrapping_u32_tests);
impl_nanfix_eq_traits_for_ord!(Wrapping<u64>, wrapping_u64_tests);
impl_nanfix_eq_traits_for_ord!(Wrapping<u128>, wrapping_u128_tests);
impl_nanfix_eq_traits_for_ord!(Wrapping<usize>, wrapping_usize_tests);

impl_nanfix_eq_traits_for_ord!(
    Saturating<i8>,
    saturating_i8_nonnegative_tests,
    saturating_i8_negative_tests
);
impl_nanfix_eq_traits_for_ord!(
    Saturating<i16>,
    saturating_i16_nonnegative_tests,
    saturating_i16_negative_tests
);
impl_nanfix_eq_traits_for_ord!(
    Saturating<i32>,
    saturating_i32_nonnegative_tests,
    saturating_i32_negative_tests
);
impl_nanfix_eq_traits_for_ord!(
    Saturating<i64>,
    saturating_i64_nonnegative_tests,
    saturating_i64_negative_tests
);
impl_nanfix_eq_traits_for_ord!(
    Saturating<i128>,
    saturating_i128_nonnegative_tests,
    saturating_i128_negative_tests
);
impl_nanfix_eq_traits_for_ord!(
    Saturating<isize>,
    saturating_isize_nonnegative_tests,
    saturating_isize_negative_tests
);

impl_nanfix_eq_traits_for_ord!(Saturating<u8>, saturating_u8_tests);
impl_nanfix_eq_traits_for_ord!(Saturating<u16>, saturating_u16_tests);
impl_nanfix_eq_traits_for_ord!(Saturating<u32>, saturating_u32_tests);
impl_nanfix_eq_traits_for_ord!(Saturating<u64>, saturating_u64_tests);
impl_nanfix_eq_traits_for_ord!(Saturating<u128>, saturating_u128_tests);
impl_nanfix_eq_traits_for_ord!(Saturating<usize>, saturating_usize_tests);

impl_nanfix_eq_traits_for_ord!(char);
impl_nanfix_eq_traits_for_ord!(bool);
impl_nanfix_eq_traits_for_ord!(());

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

macro_rules! test_nanmin_nanmax_ord_traits_nonnegative {
    ($ty:ty, $tests_mod:ident) => {
        #[cfg(test)]
        mod $tests_mod {
            use core::cmp::Ordering;

            use crate::cmp::*;
            use crate::elem::*;

            #[test]
            fn test_nanmin_cmp() {
                let zero = <$ty as Zero>::ZERO;
                let one = <$ty as One>::ONE;
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
            }

            #[test]
            fn test_nanmax_cmp() {
                let zero = <$ty as Zero>::ZERO;
                let one = <$ty as One>::ONE;
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
            }

            #[test]
            fn test_nanmin_lt_le_gt_ge() {
                let zero = <$ty as Zero>::ZERO;
                let one = <$ty as One>::ONE;
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
            }

            #[test]
            fn test_nanmax_lt_le_gt_ge() {
                let zero = <$ty as Zero>::ZERO;
                let one = <$ty as One>::ONE;
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
            }

            #[test]
            fn test_nanmin_partial_cmp() {
                let zero = <$ty as Zero>::ZERO;
                let one = <$ty as One>::ONE;

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
            }

            #[test]
            fn test_nanmax_partial_cmp() {
                let zero = <$ty as Zero>::ZERO;
                let one = <$ty as One>::ONE;

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

            #[test]
            fn test_nanmin_cmp() {
                let one = <$ty as One>::ONE;
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
            }

            #[test]
            fn test_nanmax_cmp() {
                let one = <$ty as One>::ONE;
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
            }

            #[test]
            fn test_nanmin_lt_le_gt_ge() {
                let one = <$ty as One>::ONE;
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
            }

            #[test]
            fn test_nanmax_lt_le_gt_ge() {
                let one = <$ty as One>::ONE;
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
            }
        }
    };
}

impl_nanmin_nanmax_ord_traits_for_ord!(i8, i8_ord_nonnegative_tests, i8_ord_negative_tests);
impl_nanmin_nanmax_ord_traits_for_ord!(i16, i16_ord_nonnegative_tests, i16_ord_negative_tests);
impl_nanmin_nanmax_ord_traits_for_ord!(i32, i32_ord_nonnegative_tests, i32_ord_negative_tests);
impl_nanmin_nanmax_ord_traits_for_ord!(i64, i64_ord_nonnegative_tests, i64_ord_negative_tests);
impl_nanmin_nanmax_ord_traits_for_ord!(i128, i128_ord_nonnegative_tests, i128_ord_negative_tests);
impl_nanmin_nanmax_ord_traits_for_ord!(
    isize,
    isize_ord_nonnegative_tests,
    isize_ord_negative_tests
);

impl_nanmin_nanmax_ord_traits_for_ord!(u8, u8_ord_tests);
impl_nanmin_nanmax_ord_traits_for_ord!(u16, u16_ord_tests);
impl_nanmin_nanmax_ord_traits_for_ord!(u32, u32_ord_tests);
impl_nanmin_nanmax_ord_traits_for_ord!(u64, u64_ord_tests);
impl_nanmin_nanmax_ord_traits_for_ord!(u128, u128_ord_tests);
impl_nanmin_nanmax_ord_traits_for_ord!(usize, usize_ord_tests);

impl_nanmin_nanmax_ord_traits_for_ord!(
    Wrapping<i8>,
    wrapping_i8_ord_nonnegative_tests,
    wrapping_i8_ord_negative_tests
);
impl_nanmin_nanmax_ord_traits_for_ord!(
    Wrapping<i16>,
    wrapping_i16_ord_nonnegative_tests,
    wrapping_i16_ord_negative_tests
);
impl_nanmin_nanmax_ord_traits_for_ord!(
    Wrapping<i32>,
    wrapping_i32_ord_nonnegative_tests,
    wrapping_i32_ord_negative_tests
);
impl_nanmin_nanmax_ord_traits_for_ord!(
    Wrapping<i64>,
    wrapping_i64_ord_nonnegative_tests,
    wrapping_i64_ord_negative_tests
);
impl_nanmin_nanmax_ord_traits_for_ord!(
    Wrapping<i128>,
    wrapping_i128_ord_nonnegative_tests,
    wrapping_i128_ord_negative_tests
);
impl_nanmin_nanmax_ord_traits_for_ord!(
    Wrapping<isize>,
    wrapping_isize_ord_nonnegative_tests,
    wrapping_isize_ord_negative_tests
);

impl_nanmin_nanmax_ord_traits_for_ord!(Wrapping<u8>, wrapping_u8_ord_tests);
impl_nanmin_nanmax_ord_traits_for_ord!(Wrapping<u16>, wrapping_u16_ord_tests);
impl_nanmin_nanmax_ord_traits_for_ord!(Wrapping<u32>, wrapping_u32_ord_tests);
impl_nanmin_nanmax_ord_traits_for_ord!(Wrapping<u64>, wrapping_u64_ord_tests);
impl_nanmin_nanmax_ord_traits_for_ord!(Wrapping<u128>, wrapping_u128_ord_tests);
impl_nanmin_nanmax_ord_traits_for_ord!(Wrapping<usize>, wrapping_usize_ord_tests);

impl_nanmin_nanmax_ord_traits_for_ord!(
    Saturating<i8>,
    saturating_i8_ord_nonnegative_tests,
    saturating_i8_ord_negative_tests
);
impl_nanmin_nanmax_ord_traits_for_ord!(
    Saturating<i16>,
    saturating_i16_ord_nonnegative_tests,
    saturating_i16_ord_negative_tests
);
impl_nanmin_nanmax_ord_traits_for_ord!(
    Saturating<i32>,
    saturating_i32_ord_nonnegative_tests,
    saturating_i32_ord_negative_tests
);
impl_nanmin_nanmax_ord_traits_for_ord!(
    Saturating<i64>,
    saturating_i64_ord_nonnegative_tests,
    saturating_i64_ord_negative_tests
);
impl_nanmin_nanmax_ord_traits_for_ord!(
    Saturating<i128>,
    saturating_i128_ord_nonnegative_tests,
    saturating_i128_ord_negative_tests
);
impl_nanmin_nanmax_ord_traits_for_ord!(
    Saturating<isize>,
    saturating_isize_ord_nonnegative_tests,
    saturating_isize_ord_negative_tests
);

impl_nanmin_nanmax_ord_traits_for_ord!(Saturating<u8>, saturating_u8_ord_tests);
impl_nanmin_nanmax_ord_traits_for_ord!(Saturating<u16>, saturating_u16_ord_tests);
impl_nanmin_nanmax_ord_traits_for_ord!(Saturating<u32>, saturating_u32_ord_tests);
impl_nanmin_nanmax_ord_traits_for_ord!(Saturating<u64>, saturating_u64_ord_tests);
impl_nanmin_nanmax_ord_traits_for_ord!(Saturating<u128>, saturating_u128_ord_tests);
impl_nanmin_nanmax_ord_traits_for_ord!(Saturating<usize>, saturating_usize_ord_tests);

impl_nanmin_nanmax_ord_traits_for_ord!(char);
impl_nanmin_nanmax_ord_traits_for_ord!(bool);
impl_nanmin_nanmax_ord_traits_for_ord!(());

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
        nanfix_eq_traits_nan_tests!($ty, $nan_tests_mod);
    };
}

macro_rules! nanfix_eq_traits_nan_tests {
    ($ty:ty, $tests_mod:ident) => {
        #[cfg(test)]
        mod $tests_mod {
            use crate::cmp::*;
            use crate::elem::*;

            #[test]
            fn test_nan_nanfix_eq() {
                let zero = <$ty as Zero>::ZERO;
                let nan = <$ty>::NAN;

                assert!(NanfixPartialEq::nanfix_eq(&nan, &nan));

                assert!(!NanfixPartialEq::nanfix_eq(&nan, &zero));
                assert!(!NanfixPartialEq::nanfix_eq(&zero, &nan));
            }

            #[test]
            fn test_nan_nanfix_ne() {
                let zero = <$ty as Zero>::ZERO;
                let nan = <$ty>::NAN;

                assert!(!NanfixPartialEq::nanfix_ne(&nan, &nan));

                assert!(NanfixPartialEq::nanfix_ne(&nan, &zero));
                assert!(NanfixPartialEq::nanfix_ne(&zero, &nan));
            }
        }
    };
}

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

macro_rules! test_nan_for_nanmin_nanmax_ord_traits {
    ($ty:ty, $tests_mod:ident) => {
        #[cfg(test)]
        mod $tests_mod {
            use core::cmp::Ordering;

            use crate::cmp::*;
            use crate::elem::*;

            #[test]
            fn test_nan_nanmin_cmp() {
                let zero = <$ty as Zero>::ZERO;
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
            }

            #[test]
            fn test_nan_nanmax_cmp() {
                let zero = <$ty as Zero>::ZERO;
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
            }

            #[test]
            fn test_nan_nanmin_lt_le_gt_ge() {
                let zero = <$ty as Zero>::ZERO;
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
            }

            #[test]
            fn test_nan_nanmax_lt_le_gt_ge() {
                let zero = <$ty as Zero>::ZERO;
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
            }
        }
    };
}

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
