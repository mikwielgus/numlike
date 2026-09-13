// SPDX-FileCopyrightText: 2026 numlike contributors
//
// SPDX-License-Identifier: MIT OR Apache-2.0

//! Equality and ordering with defined NaN behavior.

use core::cmp::Ordering;

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

        nanfix_eq_traits_nonnegative_tests!($ty, $nonnegative_tests_mod);
    };
    ($ty:ty, $nonnegative_tests_mod:ident, $negative_tests_mod:ident) => {
        impl_nanfix_eq_traits_for_ord!($ty, $nonnegative_tests_mod);

        nanfix_eq_traits_negative_tests!($ty, $negative_tests_mod);
    };
}

macro_rules! nanfix_eq_traits_nonnegative_tests {
    ($ty:ty, $tests_mod:ident) => {
        #[cfg(test)]
        mod $tests_mod {
            use crate::cmp::*;
            use crate::elem::*;

            #[test]
            fn test_nonnegative_nanfix_eq() {
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
            fn test_nonnegative_nanfix_ne() {
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

macro_rules! nanfix_eq_traits_negative_tests {
    ($ty:ty, $tests_mod:ident) => {
        #[cfg(test)]
        mod $tests_mod {
            use crate::cmp::*;
            use crate::elem::*;

            #[test]
            fn test_negative_nanfix_eq() {
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
            fn test_negative_nanfix_ne() {
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
}

impl_nanmin_nanmax_ord_traits_for_ord!(i8);
impl_nanmin_nanmax_ord_traits_for_ord!(i16);
impl_nanmin_nanmax_ord_traits_for_ord!(i32);
impl_nanmin_nanmax_ord_traits_for_ord!(i64);
impl_nanmin_nanmax_ord_traits_for_ord!(i128);
impl_nanmin_nanmax_ord_traits_for_ord!(isize);

impl_nanmin_nanmax_ord_traits_for_ord!(u8);
impl_nanmin_nanmax_ord_traits_for_ord!(u16);
impl_nanmin_nanmax_ord_traits_for_ord!(u32);
impl_nanmin_nanmax_ord_traits_for_ord!(u64);
impl_nanmin_nanmax_ord_traits_for_ord!(u128);
impl_nanmin_nanmax_ord_traits_for_ord!(usize);

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

        nanfix_eq_traits_nonnegative_tests!($ty, $nonnegative_tests_mod);
        nanfix_eq_traits_negative_tests!($ty, $negative_tests_mod);
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
    ($ty:ty) => {
        impl NanmaxPartialOrd<$ty> for $ty {
            #[inline]
            fn nanmax_partial_cmp(&self, other: &$ty) -> Option<Ordering> {
                Some(NanmaxOrd::nanmax_cmp(self, other))
            }

            #[inline]
            fn nanmax_ge(&self, other: &$ty) -> bool {
                self.is_nan() | PartialOrd::ge(self, other)
            }
        }

        impl NanmaxOrd<$ty> for $ty {
            #[inline]
            fn nanmax_cmp(&self, other: &$ty) -> Ordering {
                if NanmaxPartialOrd::nanmax_lt(self, other) {
                    Ordering::Less
                } else if NanmaxPartialOrd::nanmax_gt(other, self) {
                    Ordering::Greater
                } else {
                    Ordering::Equal
                }
            }
        }

        impl NanminPartialOrd<$ty> for $ty {
            #[inline]
            fn nanmin_partial_cmp(&self, other: &$ty) -> Option<Ordering> {
                Some(NanminOrd::nanmin_cmp(self, other))
            }

            #[inline]
            fn nanmin_ge(&self, other: &$ty) -> bool {
                self.is_nan() | PartialOrd::ge(self, other)
            }
        }

        impl NanminOrd<$ty> for $ty {
            #[inline]
            fn nanmin_cmp(&self, other: &$ty) -> Ordering {
                if NanminPartialOrd::nanmin_lt(self, other) {
                    Ordering::Less
                } else if NanminPartialOrd::nanmin_gt(other, self) {
                    Ordering::Greater
                } else {
                    Ordering::Equal
                }
            }
        }
    };
}

impl_nanmin_nanmax_ord_traits_for_float!(f32);
impl_nanmin_nanmax_ord_traits_for_float!(f64);
