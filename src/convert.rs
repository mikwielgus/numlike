// SPDX-FileCopyrightText: 2026 numlike contributors
//
// SPDX-License-Identifier: MIT OR Apache-2.0

//! Lossy conversions between numeric types.

/// Convert a value from one type to another, possibly with lossy approximation.
///
/// This trait is analogous to standard library's [`From`], but allows
/// the conversion to be approximate and thus lossy. It is the inverse of
/// [`CastInto`].
///
/// Internally, `as` operator is used to convert between Rust primitives. For
/// non-exact conversion where a non-primitive type is involved, the decision
/// how rounding should be done is left to the implementors, but aiming for
/// consistency with `as` is highly encouraged.
pub trait CastFrom<T> {
    /// Convert to this type from the input type, possibly with lossy
    /// approximation.
    fn cast_from(value: T) -> Self;
}

/// Convert a value from one type to another, possibly with lossy approximation.
///
/// This trait is analogous to standard library's [`From`], but allows
/// the conversion to be approximate and thus lossy. It is the inverse of
/// [`CastFrom`].
///
/// Internally, `as` operator is used to convert between Rust primitives. For
/// non-exact conversion where a non-primitive type is involved, the decision
/// how rounding should be done is left to the implementors, but aiming for
/// consistency with `as` is highly encouraged.
///
/// Analogously to Rust standard library's [`Into`], it is recommended to not
/// implement this trait directly, as it already has a blanket implementation
/// for types that implement [`CastFrom`].
pub trait CastInto<T> {
    /// Convert this type into the (usually inferred) input type, possibly with
    /// lossy approximation.
    fn cast_into(self) -> T;
}

impl<T, U> CastInto<U> for T
where
    U: CastFrom<T>,
{
    #[inline]
    fn cast_into(self) -> U {
        U::cast_from(self)
    }
}

macro_rules! impl_cast_from {
    ($src:ty => $($dst:ty),+) => {
        $(
            impl CastFrom<$src> for $dst {
                #[inline]
                fn cast_from(value: $src) -> Self {
                    value as $dst
                }
            }
        )+
    };
}

macro_rules! impl_cast_from_for_primitives {
    ($($src:ty),+) => {
        $(
            impl_cast_from!(
                $src =>
                u8, u16, u32, u64, u128, usize,
                i8, i16, i32, i64, i128, isize,
                f32, f64
            );
        )+
    };
}

impl_cast_from_for_primitives!(
    u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize, f32, f64
);

macro_rules! convert_traits_nonnegative_tests {
    ($ty:ty, $tests_mod:ident) => {
        #[cfg(test)]
        mod $tests_mod {
            use crate::convert::*;
            use crate::elem::*;

            #[test]
            fn test_nonnegative_cast_into_self() {
                let zero = <$ty as Zero>::ZERO;
                let one = <$ty as One>::ONE;
                let two = one + one;
                let four = two + two;

                assert_eq!(CastInto::<$ty>::cast_into(zero), zero);
                assert_eq!(CastInto::<$ty>::cast_into(one), one);
                assert_eq!(CastInto::<$ty>::cast_into(two), two);
                assert_eq!(CastInto::<$ty>::cast_into(four), four);
            }

            #[test]
            fn test_nonnegative_cast_from_self() {
                let zero = <$ty as Zero>::ZERO;
                let one = <$ty as One>::ONE;
                let two = one + one;
                let four = two + two;

                assert_eq!(<$ty as CastFrom<$ty>>::cast_from(zero), zero);
                assert_eq!(<$ty as CastFrom<$ty>>::cast_from(one), one);
                assert_eq!(<$ty as CastFrom<$ty>>::cast_from(two), two);
                assert_eq!(<$ty as CastFrom<$ty>>::cast_from(four), four);
            }

            #[test]
            fn test_nonnegative_cast_into_other() {
                let zero = <$ty as Zero>::ZERO;
                let one = <$ty as One>::ONE;
                let two = one + one;
                let four = two + two;

                assert_eq!(CastInto::<u8>::cast_into(zero), 0);
                assert_eq!(CastInto::<u8>::cast_into(one), 1);
                assert_eq!(CastInto::<u8>::cast_into(two), 2);
                assert_eq!(CastInto::<u8>::cast_into(four), 4);

                assert_eq!(CastInto::<i32>::cast_into(zero), 0);
                assert_eq!(CastInto::<i32>::cast_into(one), 1);
                assert_eq!(CastInto::<i32>::cast_into(two), 2);
                assert_eq!(CastInto::<i32>::cast_into(four), 4);

                assert_eq!(CastInto::<f64>::cast_into(zero), 0.0);
                assert_eq!(CastInto::<f64>::cast_into(one), 1.0);
                assert_eq!(CastInto::<f64>::cast_into(two), 2.0);
                assert_eq!(CastInto::<f64>::cast_into(four), 4.0);
            }

            #[test]
            fn test_nonnegative_cast_from_other() {
                let zero = <$ty as Zero>::ZERO;
                let one = <$ty as One>::ONE;
                let two = one + one;
                let four = two + two;

                assert_eq!(<u8 as CastFrom<$ty>>::cast_from(zero), 0);
                assert_eq!(<u8 as CastFrom<$ty>>::cast_from(one), 1);
                assert_eq!(<u8 as CastFrom<$ty>>::cast_from(two), 2);
                assert_eq!(<u8 as CastFrom<$ty>>::cast_from(four), 4);

                assert_eq!(<i32 as CastFrom<$ty>>::cast_from(zero), 0);
                assert_eq!(<i32 as CastFrom<$ty>>::cast_from(one), 1);
                assert_eq!(<i32 as CastFrom<$ty>>::cast_from(two), 2);
                assert_eq!(<i32 as CastFrom<$ty>>::cast_from(four), 4);

                assert_eq!(<f64 as CastFrom<$ty>>::cast_from(zero), 0.0);
                assert_eq!(<f64 as CastFrom<$ty>>::cast_from(one), 1.0);
                assert_eq!(<f64 as CastFrom<$ty>>::cast_from(two), 2.0);
                assert_eq!(<f64 as CastFrom<$ty>>::cast_from(four), 4.0);
            }
        }
    };
}

macro_rules! convert_traits_negative_tests {
    ($ty:ty, $tests_mod:ident) => {
        #[cfg(test)]
        mod $tests_mod {
            use crate::convert::*;
            use crate::elem::*;

            #[test]
            fn test_negative_cast_into_self() {
                let one = <$ty as One>::ONE;
                let two = one + one;
                let four = two + two;

                assert_eq!(CastInto::<$ty>::cast_into(-one), -one);
                assert_eq!(CastInto::<$ty>::cast_into(-two), -two);
                assert_eq!(CastInto::<$ty>::cast_into(-four), -four);
            }

            #[test]
            fn test_negative_cast_from_self() {
                let one = <$ty as One>::ONE;
                let two = one + one;
                let four = two + two;

                assert_eq!(<$ty as CastFrom<$ty>>::cast_from(-one), -one);
                assert_eq!(<$ty as CastFrom<$ty>>::cast_from(-two), -two);
                assert_eq!(<$ty as CastFrom<$ty>>::cast_from(-four), -four);
            }

            #[test]
            fn test_negative_cast_into_other() {
                let one = <$ty as One>::ONE;
                let two = one + one;
                let four = two + two;

                assert_eq!(CastInto::<i32>::cast_into(-one), -1);
                assert_eq!(CastInto::<i32>::cast_into(-two), -2);
                assert_eq!(CastInto::<i32>::cast_into(-four), -4);

                assert_eq!(CastInto::<f64>::cast_into(-one), -1.0);
                assert_eq!(CastInto::<f64>::cast_into(-two), -2.0);
                assert_eq!(CastInto::<f64>::cast_into(-four), -4.0);

                assert_eq!(CastInto::<u8>::cast_into(-one), (-one) as u8);
                assert_eq!(CastInto::<u8>::cast_into(-two), (-two) as u8);
                assert_eq!(CastInto::<u8>::cast_into(-four), (-four) as u8);
            }

            #[test]
            fn test_negative_cast_from_other() {
                let one = <$ty as One>::ONE;
                let two = one + one;
                let four = two + two;

                assert_eq!(<i32 as CastFrom<$ty>>::cast_from(-one), -1);
                assert_eq!(<i32 as CastFrom<$ty>>::cast_from(-two), -2);
                assert_eq!(<i32 as CastFrom<$ty>>::cast_from(-four), -4);

                assert_eq!(<f64 as CastFrom<$ty>>::cast_from(-one), -1.0);
                assert_eq!(<f64 as CastFrom<$ty>>::cast_from(-two), -2.0);
                assert_eq!(<f64 as CastFrom<$ty>>::cast_from(-four), -4.0);

                assert_eq!(<u8 as CastFrom<$ty>>::cast_from(-one), (-one) as u8);
                assert_eq!(<u8 as CastFrom<$ty>>::cast_from(-two), (-two) as u8);
                assert_eq!(<u8 as CastFrom<$ty>>::cast_from(-four), (-four) as u8);
            }
        }
    };
}

macro_rules! test_convert_traits {
    ($ty:ty, $nonnegative_tests_mod:ident) => {
        convert_traits_nonnegative_tests!($ty, $nonnegative_tests_mod);
    };
    ($ty:ty, $nonnegative_tests_mod:ident, $negative_tests_mod:ident) => {
        convert_traits_nonnegative_tests!($ty, $nonnegative_tests_mod);
        convert_traits_negative_tests!($ty, $negative_tests_mod);
    };
}

test_convert_traits!(i8, i8_nonnegative_tests, i8_negative_tests);
test_convert_traits!(i16, i16_nonnegative_tests, i16_negative_tests);
test_convert_traits!(i32, i32_nonnegative_tests, i32_negative_tests);
test_convert_traits!(i64, i64_nonnegative_tests, i64_negative_tests);
test_convert_traits!(i128, i128_nonnegative_tests, i128_negative_tests);
test_convert_traits!(isize, isize_nonnegative_tests, isize_negative_tests);

test_convert_traits!(u8, u8_tests);
test_convert_traits!(u16, u16_tests);
test_convert_traits!(u32, u32_tests);
test_convert_traits!(u64, u64_tests);
test_convert_traits!(u128, u128_tests);
test_convert_traits!(usize, usize_tests);

test_convert_traits!(f32, f32_nonnegative_tests, f32_negative_tests);
test_convert_traits!(f64, f64_nonnegative_tests, f64_negative_tests);
