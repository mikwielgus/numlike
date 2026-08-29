// SPDX-FileCopyrightText: 2026 numlike contributors
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use core::cmp::Ordering;

pub trait NanfixPartialEq<Rhs: ?Sized = Self> {
    fn eq(&self, other: &Rhs) -> bool;

    #[inline]
    fn ne(&self, other: &Rhs) -> bool {
        !self.eq(other)
    }
}

pub trait NanfixEq<Rhs: ?Sized = Self>: NanfixPartialEq {}

macro_rules! def_cmp_traits {
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
}

def_cmp_traits!(NanminPartialOrd, NanminOrd);
def_cmp_traits!(NanmaxPartialOrd, NanmaxOrd);

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
