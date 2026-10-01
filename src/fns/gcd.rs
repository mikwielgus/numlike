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
}

macro_rules! impl_gcd_for_ints {
    ($($t:ty),+) => {
        $(
            impl_gcd_for_int!(
                $t =>
                i8, i16, i32, i64, i128, isize,
                u8, u16, u32, u64, u128, usize
            );
        )+
    };
}

impl_gcd_for_ints!(
    i8, i16, i32, i64, i128, isize, u8, u16, u32, u64, u128, usize
);
