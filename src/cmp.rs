// SPDX-FileCopyrightText: 2026 numlike contributors
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use std::cmp::Ordering;

pub trait NanPartialEq<Rhs: ?Sized = Self> {
    fn eq(&self, other: &Rhs) -> bool;

    fn ne(&self, other: &Rhs) -> bool {
        !self.eq(other)
    }
}

pub trait NanEq<Rhs: ?Sized = Self>: NanPartialEq {}

macro_rules! def_cmp_traits {
    ($partial_ord:ident, $ord:ident, $partial_eq:ident, $eq:ident) => {
        pub trait $partial_ord<Rhs: ?Sized = Self>: $partial_eq {
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

        pub trait $ord<Rhs: ?Sized = Self>: $eq + $partial_ord {
            fn cmp(&self, other: &Rhs) -> Ordering;
        }
    };
}

def_cmp_traits!(NanPartialOrd, NanOrd, NanPartialEq, NanEq);
//def_cmp_traits!(NanMaxPartialOrd, NanMaxOrd, NanPartialEq, NanEq);
//def_cmp_traits!(NanMinPartialOrd, NanMinOrd, NanPartialEq, NanEq);

macro_rules! impl_cmp_traits_for_ords {
    ($($ty:ty),*) => {
        $(
            impl NanPartialEq<$ty> for $ty {
                #[inline]
                fn eq(&self, other: &$ty) -> bool {
                    PartialEq::eq(self, other)
                }
            }

            impl NanEq<$ty> for $ty {}

            impl NanPartialOrd<$ty> for $ty {
                #[inline]
                fn partial_cmp(&self, other: &$ty) -> Option<Ordering> {
                    PartialOrd::partial_cmp(self, other)
                }
            }

            impl NanOrd<$ty> for $ty {
                #[inline]
                fn cmp(&self, other: &$ty) -> Ordering {
                    Ord::cmp(self, other)
                }
            }
        )*
    };
}

impl_cmp_traits_for_ords!(i8, i16, i32, i64, i128, isize);
impl_cmp_traits_for_ords!(u8, u16, u32, u64, u128, usize);
impl_cmp_traits_for_ords!(char, bool, ());

macro_rules! impl_cmp_traits_for_floats {
    ($($ty:ty),*) => {
        $(
            impl NanPartialEq<$ty> for $ty {
                #[inline]
                fn eq(&self, other: &$ty) -> bool {
                    (self.is_nan() && other.is_nan()) || PartialEq::eq(self, other)
                }
            }

            impl NanEq<$ty> for $ty {}

            impl NanPartialOrd<$ty> for $ty {
                #[inline]
                fn partial_cmp(&self, other: &$ty) -> Option<Ordering> {
                    Some(NanOrd::cmp(self, other))
                }

                #[inline]
                fn ge(&self, other: &$ty) -> bool {
                    self.is_nan() | PartialOrd::ge(self, other)
                }
            }

            impl NanOrd<$ty> for $ty {
                #[inline]
                fn cmp(&self, other: &$ty) -> Ordering {
                    if NanPartialOrd::lt(self, other) {
                        Ordering::Less
                    } else if NanPartialOrd::gt(other, self) {
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
