// SPDX-FileCopyrightText: 2026 numlike contributors
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use crate::fns::Abs;

/// Calculate the greatest common divisor (GCD) and least common multiple (LCM)
/// together.
///
/// Could be more efficient than calling [`Gcd::gcd`] and [`Lcm::lcm`]
/// individually with identical inputs.
pub trait GcdLcm<Rhs = Self> {
    /// The resulting type after evaluating the function.
    type Output;

    /// Calculate the greatest common divisor (GCD) and least common multiple
    /// (LCM) together.
    ///
    /// Could be more efficient than calling [`Gcd::gcd`] and [`Lcm::lcm`]
    /// individually with identical inputs.
    fn gcd_lcm(self, rhs: Rhs) -> (Self::Output, Self::Output);
}

/// Calculate the greatest common divisor (GCD).
pub trait Gcd<Rhs = Self> {
    /// The resulting type after evaluating the function.
    type Output;

    /// Calculate the greatest common divisor (GCD).
    fn gcd(self, rhs: Rhs) -> Self::Output;
}

/// Calculate the least common multiple (LCM).
pub trait Lcm<Rhs = Self> {
    /// The resulting type after evaluating the function.
    type Output;

    /// Calculate the least common multiple (LCM).
    fn lcm(self, rhs: Rhs) -> Self::Output;
}

macro_rules! impl_gcd_lcm_for_int {
    ($t:ty) => {
        impl GcdLcm for $t {
            type Output = $t;

            #[inline]
            fn gcd_lcm(self, rhs: $t) -> ($t, $t) {
                let gcd = Gcd::gcd(self, rhs);

                if self == 0 && rhs == 0 {
                    return (gcd, 0);
                }

                (gcd, Abs::abs(self * (rhs / gcd)))
            }
        }

        impl Gcd for $t {
            type Output = $t;

            #[inline]
            fn gcd(mut self, mut rhs: $t) -> $t {
                // Stein's algorithm.

                // `gcd(x,0)` and `gcd(0,x)` are usually defined to evaluate to
                // `|x|`, we do so too.
                if self == 0 || rhs == 0 {
                    return Abs::abs(self | rhs);
                }

                // First find common factors of 2, since these are cheap to
                // handle in base-2.
                let shift = (self | rhs).trailing_zeros();

                // Stein's algorithm operates on positive numbers, so we take
                // absolute value.
                self = Abs::abs(self);
                rhs = Abs::abs(rhs);

                // Bitwise shift, a cheap operation, corresponds to division
                // by 2. So we use it to avoid wasting compute on a loop for
                // factors of 2.
                self >>= self.trailing_zeros();
                rhs >>= rhs.trailing_zeros();

                while self != rhs {
                    if self > rhs {
                        self -= rhs;
                        self >>= self.trailing_zeros();
                    } else {
                        rhs -= self;
                        rhs >>= rhs.trailing_zeros();
                    }
                }

                // Multiply the skipped factors of 2 back into the result.
                self << shift
            }
        }

        impl Lcm for $t {
            type Output = $t;

            #[inline]
            fn lcm(self, rhs: $t) -> $t {
                if self == 0 && rhs == 0 {
                    return 0;
                }

                // `lcm(a, b) = |a * (b / gcd(a, b))|`. We divide before
                // multiplying to make overflow less likely.
                Abs::abs(self * (rhs / Gcd::gcd(self, rhs)))
            }
        }
    };
    ($t:ty, $nonnegative_tests_mod:ident) => {
        impl_gcd_lcm_for_int!($t);

        test_gcd_lcm_traits_int_nonnegative!($t, $nonnegative_tests_mod);
    };
    ($t:ty, $nonnegative_tests_mod:ident, $negative_tests_mod:ident) => {
        impl_gcd_lcm_for_int!($t);

        test_gcd_lcm_traits_int_nonnegative!($t, $nonnegative_tests_mod);
        test_gcd_lcm_traits_int_negative!($t, $negative_tests_mod);
    };
}

macro_rules! test_gcd_lcm_traits_int_nonnegative {
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
                let seven = six + one;
                let eight = four + four;
                let nine = six + three;
                let twelve = six + six;
                let eighteen = nine + nine;

                assert_eq!(Gcd::gcd(zero, zero), zero);
                assert_eq!(Gcd::gcd(zero, eight), eight);
                assert_eq!(Gcd::gcd(one, eighteen), one);
                assert_eq!(Gcd::gcd(two, four), two);
                assert_eq!(Gcd::gcd(four, six), two);
                assert_eq!(Gcd::gcd(six, four), two);
                assert_eq!(Gcd::gcd(seven, three), one);
                assert_eq!(Gcd::gcd(eight, zero), eight);
                assert_eq!(Gcd::gcd(eight, twelve), four);
                assert_eq!(Gcd::gcd(twelve, eight), four);
                assert_eq!(Gcd::gcd(twelve, twelve), twelve);
                assert_eq!(Gcd::gcd(twelve, eighteen), six);
                assert_eq!(Gcd::gcd(eighteen, one), one);
                assert_eq!(Gcd::gcd(eighteen, twelve), six);
            }

            #[test]
            fn test_lcm() {
                let zero = <$ty as Zero>::ZERO;
                let one = <$ty as One>::ONE;
                let two = one + one;
                let three = two + one;
                let four = two + two;
                let six = three + three;
                let seven = six + one;
                let eight = four + four;
                let nine = six + three;
                let twelve = six + six;
                let eighteen = nine + nine;
                let twenty_one = seven * three;
                let twenty_four = twelve + twelve;
                let thirty_six = eighteen + eighteen;

                assert_eq!(Lcm::lcm(zero, zero), zero);
                assert_eq!(Lcm::lcm(zero, eight), zero);
                assert_eq!(Lcm::lcm(one, eighteen), eighteen);
                assert_eq!(Lcm::lcm(two, four), four);
                assert_eq!(Lcm::lcm(four, six), twelve);
                assert_eq!(Lcm::lcm(six, four), twelve);
                assert_eq!(Lcm::lcm(seven, three), twenty_one);
                assert_eq!(Lcm::lcm(eight, zero), zero);
                assert_eq!(Lcm::lcm(eight, twelve), twenty_four);
                assert_eq!(Lcm::lcm(twelve, eight), twenty_four);
                assert_eq!(Lcm::lcm(twelve, twelve), twelve);
                assert_eq!(Lcm::lcm(twelve, eighteen), thirty_six);
                assert_eq!(Lcm::lcm(eighteen, one), eighteen);
                assert_eq!(Lcm::lcm(eighteen, twelve), thirty_six);
            }

            #[test]
            fn test_gcd_lcm() {
                let zero = <$ty as Zero>::ZERO;
                let one = <$ty as One>::ONE;
                let two = one + one;
                let three = two + one;
                let four = two + two;
                let six = three + three;
                let seven = six + one;
                let eight = four + four;
                let nine = six + three;
                let twelve = six + six;
                let eighteen = nine + nine;
                let twenty_one = seven * three;
                let twenty_four = twelve + twelve;
                let thirty_six = eighteen + eighteen;

                assert_eq!(GcdLcm::gcd_lcm(zero, zero), (zero, zero));
                assert_eq!(GcdLcm::gcd_lcm(zero, eight), (eight, zero));
                assert_eq!(GcdLcm::gcd_lcm(one, eighteen), (one, eighteen));
                assert_eq!(GcdLcm::gcd_lcm(two, four), (two, four));
                assert_eq!(GcdLcm::gcd_lcm(four, six), (two, twelve));
                assert_eq!(GcdLcm::gcd_lcm(six, four), (two, twelve));
                assert_eq!(GcdLcm::gcd_lcm(seven, three), (one, twenty_one));
                assert_eq!(GcdLcm::gcd_lcm(eight, zero), (eight, zero));
                assert_eq!(GcdLcm::gcd_lcm(eight, twelve), (four, twenty_four));
                assert_eq!(GcdLcm::gcd_lcm(twelve, eight), (four, twenty_four));
                assert_eq!(GcdLcm::gcd_lcm(twelve, twelve), (twelve, twelve));
                assert_eq!(GcdLcm::gcd_lcm(twelve, eighteen), (six, thirty_six));
                assert_eq!(GcdLcm::gcd_lcm(eighteen, one), (one, eighteen));
                assert_eq!(GcdLcm::gcd_lcm(eighteen, twelve), (six, thirty_six));
            }
        }
    };
}

macro_rules! test_gcd_lcm_traits_int_negative {
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

                assert_eq!(Gcd::gcd(-twelve, -eighteen), six);
                assert_eq!(Gcd::gcd(-twelve, eighteen), six);
                assert_eq!(Gcd::gcd(-eight, zero), eight);
                assert_eq!(Gcd::gcd(-eight, twelve), four);
                assert_eq!(Gcd::gcd(-four, -six), two);
                assert_eq!(Gcd::gcd(-four, six), two);
                assert_eq!(Gcd::gcd(zero, -eight), eight);
                assert_eq!(Gcd::gcd(four, -six), two);
                assert_eq!(Gcd::gcd(eight, -twelve), four);
                assert_eq!(Gcd::gcd(twelve, -eighteen), six);
            }

            #[test]
            fn test_lcm() {
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
                let twenty_four = twelve + twelve;
                let thirty_six = eighteen + eighteen;

                assert_eq!(Lcm::lcm(-twelve, -eighteen), thirty_six);
                assert_eq!(Lcm::lcm(-twelve, eighteen), thirty_six);
                assert_eq!(Lcm::lcm(-eight, zero), zero);
                assert_eq!(Lcm::lcm(-eight, twelve), twenty_four);
                assert_eq!(Lcm::lcm(-four, -six), twelve);
                assert_eq!(Lcm::lcm(-four, six), twelve);
                assert_eq!(Lcm::lcm(zero, -eight), zero);
                assert_eq!(Lcm::lcm(four, -six), twelve);
                assert_eq!(Lcm::lcm(eight, -twelve), twenty_four);
                assert_eq!(Lcm::lcm(twelve, -eighteen), thirty_six);
            }

            #[test]
            fn test_gcd_lcm() {
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
                let twenty_four = twelve + twelve;
                let thirty_six = eighteen + eighteen;

                assert_eq!(GcdLcm::gcd_lcm(-twelve, -eighteen), (six, thirty_six));
                assert_eq!(GcdLcm::gcd_lcm(-twelve, eighteen), (six, thirty_six));
                assert_eq!(GcdLcm::gcd_lcm(-eight, zero), (eight, zero));
                assert_eq!(GcdLcm::gcd_lcm(-eight, twelve), (four, twenty_four));
                assert_eq!(GcdLcm::gcd_lcm(-four, -six), (two, twelve));
                assert_eq!(GcdLcm::gcd_lcm(-four, six), (two, twelve));
                assert_eq!(GcdLcm::gcd_lcm(zero, -eight), (eight, zero));
                assert_eq!(GcdLcm::gcd_lcm(four, -six), (two, twelve));
                assert_eq!(GcdLcm::gcd_lcm(eight, -twelve), (four, twenty_four));
                assert_eq!(GcdLcm::gcd_lcm(twelve, -eighteen), (six, thirty_six));
            }
        }
    };
}

impl_gcd_lcm_for_int!(u8, u8_tests);
impl_gcd_lcm_for_int!(u16, u16_tests);
impl_gcd_lcm_for_int!(u32, u32_tests);
impl_gcd_lcm_for_int!(u64, u64_tests);
impl_gcd_lcm_for_int!(u128, u128_tests);
impl_gcd_lcm_for_int!(usize, usize_tests);

impl_gcd_lcm_for_int!(i8, i8_nonnegative_tests, i8_negative_tests);
impl_gcd_lcm_for_int!(i16, i16_nonnegative_tests, i16_negative_tests);
impl_gcd_lcm_for_int!(i32, i32_nonnegative_tests, i32_negative_tests);
impl_gcd_lcm_for_int!(i64, i64_nonnegative_tests, i64_negative_tests);
impl_gcd_lcm_for_int!(i128, i128_nonnegative_tests, i128_negative_tests);
impl_gcd_lcm_for_int!(isize, isize_nonnegative_tests, isize_negative_tests);
