// SPDX-FileCopyrightText: 2026 numlike contributors
//
// SPDX-License-Identifier: MIT OR Apache-2.0

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
    fn eq(&self, other: &Rhs) -> bool;

    /// Test whether self and the other value not equal, treating NaN as equal
    /// to itself.
    #[inline]
    fn ne(&self, other: &Rhs) -> bool {
        !self.eq(other)
    }
}

/// Trait to be implemented if [`NanfixPartialEq`] is a total equality
/// (virtually always it is).
///
/// The distinction between [`NanfixPartialEq`] and [`NanfixEq`] primarily
/// exists to mirror Rust standard library's [`PartialEq`] and [`Eq`].
pub trait NanfixEq<Rhs: ?Sized = Self>: NanfixPartialEq {}

/*macro_rules! def_cmp_traits {
    ($partial_ord:ident, $ord:ident) => {
        pub trait $partial_ord<Rhs: ?Sized = Self>: NanfixPartialEq {
            fn partial_cmp(&self, other: &Rhs) -> Option<Ordering>;

            #[inline]
            fn lt(&self, other: &Rhs) -> bool {
                self.partial_cmp(other).is_some_and(Ordering::is_lt)
            }

            #[inline]
            fn le(&self, other: &Rhs) -> bool {
                self.partial_cmp(other).is_some_and(Ordering::is_le)
            }

            #[inline]
            fn gt(&self, other: &Rhs) -> bool {
                self.partial_cmp(other).is_some_and(Ordering::is_gt)
            }

            #[inline]
            fn ge(&self, other: &Rhs) -> bool {
                self.partial_cmp(other).is_some_and(Ordering::is_ge)
            }
        }

        pub trait $ord<Rhs: ?Sized = Self>: NanfixEq + $partial_ord {
            fn cmp(&self, other: &Rhs) -> Ordering;
        }
    };
}*/

/*def_cmp_traits!(NanminPartialOrd, NanminOrd);
def_cmp_traits!(NanmaxPartialOrd, NanmaxOrd);*/

/// Trait for partial order where NaNs are fixed to be the smallest element in
/// the set, smaller even than the negative infinity.
///
/// This is virtually always also a total order, so most likely you want to
/// implement [`NanminOrd`] too.
pub trait NanminPartialOrd<Rhs: ?Sized = Self>: NanfixPartialEq {
    /// This method returns an (NaN-min) ordering between self and other values
    /// if one exists.
    fn partial_cmp(&self, other: &Rhs) -> Option<Ordering>;

    /// Checks if `self` is (NaN-min) less than `other`.
    #[inline]
    fn lt(&self, other: &Rhs) -> bool {
        self.partial_cmp(other).is_some_and(Ordering::is_lt)
    }

    /// Checks if `self` is (NaN-min) less or equal to `other`.
    #[inline]
    fn le(&self, other: &Rhs) -> bool {
        self.partial_cmp(other).is_some_and(Ordering::is_le)
    }

    /// Checks if `self` is (NaN-min) greater than `other`.
    #[inline]
    fn gt(&self, other: &Rhs) -> bool {
        self.partial_cmp(other).is_some_and(Ordering::is_gt)
    }

    /// Checks if `self` is (NaN-min) greater or equal to `other`.
    #[inline]
    fn ge(&self, other: &Rhs) -> bool {
        self.partial_cmp(other).is_some_and(Ordering::is_ge)
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
    /// By convention, `self.cmp(&other)` returns the ordering matching the
    /// expression `self <operator> other` if true.
    fn cmp(&self, other: &Rhs) -> Ordering;
}

/// Trait for partial order where NaNs are fixed to be the greatest element in
/// the set, greater even than the positive infinity.
///
/// This is virtually always also a total order, so most likely you want to
/// implement [`NanmaxOrd`] too.
pub trait NanmaxPartialOrd<Rhs: ?Sized = Self>: NanfixPartialEq {
    /// This method returns an (NaN-max) ordering between self and other values
    /// if one exists.
    fn partial_cmp(&self, other: &Rhs) -> Option<Ordering>;

    /// Checks if `self` is (NaN-max) less than `other`.
    #[inline]
    fn lt(&self, other: &Rhs) -> bool {
        self.partial_cmp(other).is_some_and(Ordering::is_lt)
    }

    /// Checks if `self` is (NaN-max) less or equal to `other`.
    #[inline]
    fn le(&self, other: &Rhs) -> bool {
        self.partial_cmp(other).is_some_and(Ordering::is_le)
    }

    /// Checks if `self` is (NaN-max) greater than `other`.
    #[inline]
    fn gt(&self, other: &Rhs) -> bool {
        self.partial_cmp(other).is_some_and(Ordering::is_gt)
    }

    /// Checks if `self` is (NaN-max) greater or equal to `other`.
    #[inline]
    fn ge(&self, other: &Rhs) -> bool {
        self.partial_cmp(other).is_some_and(Ordering::is_ge)
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

    /// By convention, `self.cmp(&other)` returns the ordering matching the
    /// expression `self <operator> other` if true.
    fn cmp(&self, other: &Rhs) -> Ordering;
}

macro_rules! impl_nanfix_eq_traits_for_ords {
    ($($ty:ty),*) => {
        $(
            impl NanfixPartialEq<$ty> for $ty {
                #[inline]
                fn eq(&self, other: &$ty) -> bool {
                    PartialEq::eq(self, other)
                }
            }

            impl NanfixEq<$ty> for $ty {}
        )*
    };
}

macro_rules! impl_nanmin_nanmax_ord_traits_for_ords {
    ($($ty:ty),*) => {
        $(
            impl NanminPartialOrd<$ty> for $ty {
                #[inline]
                fn partial_cmp(&self, other: &$ty) -> Option<Ordering> {
                    PartialOrd::partial_cmp(self, other)
                }
            }

            impl NanminOrd<$ty> for $ty {
                #[inline]
                fn cmp(&self, other: &$ty) -> Ordering {
                    Ord::cmp(self, other)
                }
            }

            impl NanmaxPartialOrd<$ty> for $ty {
                #[inline]
                fn partial_cmp(&self, other: &$ty) -> Option<Ordering> {
                    PartialOrd::partial_cmp(self, other)
                }
            }

            impl NanmaxOrd<$ty> for $ty {
                #[inline]
                fn cmp(&self, other: &$ty) -> Ordering {
                    Ord::cmp(self, other)
                }
            }
        )*
    };
}

impl_nanfix_eq_traits_for_ords!(i8, i16, i32, i64, i128, isize);
impl_nanfix_eq_traits_for_ords!(u8, u16, u32, u64, u128, usize);
impl_nanfix_eq_traits_for_ords!(char, bool, ());

impl_nanmin_nanmax_ord_traits_for_ords!(i8, i16, i32, i64, i128, isize);
impl_nanmin_nanmax_ord_traits_for_ords!(u8, u16, u32, u64, u128, usize);
impl_nanmin_nanmax_ord_traits_for_ords!(char, bool, ());

macro_rules! impl_cmp_traits_for_floats {
    ($($ty:ty),*) => {
        $(
            impl NanfixPartialEq<$ty> for $ty {
                #[inline]
                fn eq(&self, other: &$ty) -> bool {
                    (self.is_nan() && other.is_nan()) || PartialEq::eq(self, other)
                }
            }

            impl NanfixEq<$ty> for $ty {}

            impl NanmaxPartialOrd<$ty> for $ty {
                #[inline]
                fn partial_cmp(&self, other: &$ty) -> Option<Ordering> {
                    Some(NanmaxOrd::cmp(self, other))
                }

                #[inline]
                fn ge(&self, other: &$ty) -> bool {
                    self.is_nan() | PartialOrd::ge(self, other)
                }
            }

            impl NanmaxOrd<$ty> for $ty {
                #[inline]
                fn cmp(&self, other: &$ty) -> Ordering {
                    if NanmaxPartialOrd::lt(self, other) {
                        Ordering::Less
                    } else if NanmaxPartialOrd::gt(other, self) {
                        Ordering::Greater
                    } else {
                        Ordering::Equal
                    }
                }
            }

            impl NanminPartialOrd<$ty> for $ty {
                #[inline]
                fn partial_cmp(&self, other: &$ty) -> Option<Ordering> {
                    Some(NanminOrd::cmp(self, other))
                }

                #[inline]
                fn ge(&self, other: &$ty) -> bool {
                    self.is_nan() | PartialOrd::ge(self, other)
                }
            }

            impl NanminOrd<$ty> for $ty {
                #[inline]
                fn cmp(&self, other: &$ty) -> Ordering {
                    if NanminPartialOrd::lt(self, other) {
                        Ordering::Less
                    } else if NanminPartialOrd::gt(other, self) {
                        Ordering::Greater
                    } else {
                        Ordering::Equal
                    }
                }
            }
        )*
    };
}

impl_cmp_traits_for_floats!(f32, f64);
