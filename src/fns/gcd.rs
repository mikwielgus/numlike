// SPDX-FileCopyrightText: 2026 numlike contributors
//
// SPDX-License-Identifier: MIT OR Apache-2.0

/// Calculate the greatest common divisor.
pub trait Gcd<Rhs = Self> {
    /// The resulting type after evaluating the function.
    type Output;

    /// Calculate the greatest common divisor.
    fn gcd(self, rhs: Rhs) -> Self::Output;
}

macro_rules! impl_gcd_for_int {
    ($t:ty => $($rhs:ty),+) => {
        $(
            impl Gcd<$rhs> for $t {
                type Output = $t;

                #[inline]
                fn gcd(self, rhs: $rhs) -> $t {
                    use crate::fns::Abs;

                    // Stein's algorithm.

                    let mut m = self;
                    let mut n = rhs as $t;

                    // `gcd(x,0)` and `gcd(0,x)` are usually defined to evaluate to
                    // `|x|`, we do so too.
                    if m == 0 || n == 0 {
                        return Abs::abs(m | n);
                    }

                    // First find common factors of 2, since these are cheap to
                    // handle in base-2.
                    let shift = (m | n).trailing_zeros();

                    // Stein's algorithm operates on positive numbers, so we take
                    // absolute value.
                    m = Abs::abs(m);
                    n = Abs::abs(n);

                    // Bitwise shift corresponds to division by 2. So we use it to
                    // avoid wasting compute on a loop for factors of 2.
                    m >>= m.trailing_zeros();
                    n >>= n.trailing_zeros();

                    while m != n {
                        if m > n {
                            m -= n;
                            m >>= m.trailing_zeros();
                        } else {
                            n -= m;
                            n >>= n.trailing_zeros();
                        }
                    }

                    // Multiply the skipped factors of 2 back into the result.
                    m << shift
                }
            }
        )+
    };
    ($t:ty) => {
        impl_gcd_for_int!(
            $t =>
            i8, i16, i32, i64, i128, isize,
            u8, u16, u32, u64, u128, usize
        );
    };
    ($t:ty, $nonnegative_tests_mod:ident) => {
        impl_gcd_for_int!($t);

        test_gcd_traits_int_nonnegative!($t, $nonnegative_tests_mod);
    };
    ($t:ty, $nonnegative_tests_mod:ident, $negative_tests_mod:ident) => {
        impl_gcd_for_int!($t, $nonnegative_tests_mod);

        test_gcd_traits_int_negative!($t, $negative_tests_mod);
    };
}

macro_rules! test_gcd_traits_int_nonnegative {
    ($ty:ty, $tests_mod:ident) => {
        #[cfg(test)]
        mod $tests_mod {
            use crate::elem::*;
            use crate::fns::*;

            #[test]
            fn test_gcd() {
                let zero = <$ty as Zero>::ZERO;
                let one = <$ty as One>::ONE;
                let two = one + one;
                let three = two + one;
                let four = two + two;
                let six = three + three;
                let eight = four + four;
                let nine = six + three;
                let twelve = six + six;
                let eighteen = nine + nine;

                assert_eq!(Gcd::gcd(zero, zero), zero);
                assert_eq!(Gcd::gcd(zero, eight), eight);
                assert_eq!(Gcd::gcd(eight, zero), eight);
                assert_eq!(Gcd::gcd(one, eighteen), one);
                assert_eq!(Gcd::gcd(eighteen, one), one);
                assert_eq!(Gcd::gcd(twelve, twelve), twelve);
                assert_eq!(Gcd::gcd(twelve, eighteen), six);
                assert_eq!(Gcd::gcd(eighteen, twelve), six);
                assert_eq!(Gcd::gcd(eight, twelve), four);
                assert_eq!(Gcd::gcd(twelve, eight), four);

                assert_eq!(Gcd::gcd(twelve, 18u8), six);
                assert_eq!(Gcd::gcd(twelve, 18u16), six);
                assert_eq!(Gcd::gcd(twelve, 18), six);
            }
        }
    };
}

macro_rules! test_gcd_traits_int_negative {
    ($ty:ty, $tests_mod:ident) => {
        #[cfg(test)]
        mod $tests_mod {
            use crate::elem::*;
            use crate::fns::*;

            #[test]
            fn test_gcd() {
                let zero = <$ty as Zero>::ZERO;
                let one = <$ty as One>::ONE;
                let two = one + one;
                let three = two + one;
                let four = two + two;
                let six = three + three;
                let eight = four + four;
                let nine = six + three;
                let twelve = six + six;
                let eighteen = nine + nine;

                assert_eq!(Gcd::gcd(-eight, zero), eight);
                assert_eq!(Gcd::gcd(zero, -eight), eight);
                assert_eq!(Gcd::gcd(-twelve, eighteen), six);
                assert_eq!(Gcd::gcd(twelve, -eighteen), six);
                assert_eq!(Gcd::gcd(-twelve, -eighteen), six);
                assert_eq!(Gcd::gcd(-eight, twelve), four);
                assert_eq!(Gcd::gcd(eight, -twelve), four);

                assert_eq!(Gcd::gcd(-twelve, 18u8), six);
                assert_eq!(Gcd::gcd(-twelve, 18), six);
            }
        }
    };
}

impl_gcd_for_int!(u8, u8_tests);
impl_gcd_for_int!(u16, u16_tests);
impl_gcd_for_int!(u32, u32_tests);
impl_gcd_for_int!(u64, u64_tests);
impl_gcd_for_int!(u128, u128_tests);
impl_gcd_for_int!(usize, usize_tests);

impl_gcd_for_int!(i8, i8_nonnegative_tests, i8_negative_tests);
impl_gcd_for_int!(i16, i16_nonnegative_tests, i16_negative_tests);
impl_gcd_for_int!(i32, i32_nonnegative_tests, i32_negative_tests);
impl_gcd_for_int!(i64, i64_nonnegative_tests, i64_negative_tests);
impl_gcd_for_int!(i128, i128_nonnegative_tests, i128_negative_tests);
impl_gcd_for_int!(isize, isize_nonnegative_tests, isize_negative_tests);
