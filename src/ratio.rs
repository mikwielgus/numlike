// SPDX-FileCopyrightText: 2026 numlike contributors
//
// SPDX-License-Identifier: MIT OR Apache-2.0

//! A ratio between two numbers.

use core::{
    cmp::Ordering,
    ops::{Add, Div, Mul, Neg, Sub},
};

use crate::{
    elem::{MinusInfinity, Nan, One, PlusInfinity, Zero},
    fns::{Gcd, Lcm, SignFns},
    limits::{
        MaxExactInteger, MaxExtended, MaxFinite, MaxNegative, MaximizerDenom, MinExactInteger,
        MinExtended, MinFinite, MinPositive, MinimizerDenom,
    },
};

/// A ratio between two numbers.
#[derive(Clone, Copy, Debug)]
pub struct Ratio<T> {
    numer: T,
    denom: T,
}

impl<T: Clone> Ratio<T> {
    /// Returns the ratio's numerator by value.
    pub fn numer(&self) -> T {
        self.numer.clone()
    }

    /// Returns the ratio's denominator by value.
    pub fn denom(&self) -> T {
        self.denom.clone()
    }
}

impl<T> Ratio<T> {
    /// Returns the ratio's numerator by immutable reference.
    pub fn numer_ref(&self) -> &T {
        &self.numer
    }

    /// Returns the ratio's denominator by immutable reference.
    pub fn denom_ref(&self) -> &T {
        &self.denom
    }
}

impl<
    T: Clone + Div<Output = T> + Gcd<Output = T> + Neg<Output = T> + One + PartialEq + SignFns + Zero,
> Ratio<T>
{
    /// Creates a new ratio.
    #[inline]
    pub fn new(numer: T, denom: T) -> Self {
        Self::reduce(numer, denom)
    }

    #[inline]
    fn reduce(numer: T, denom: T) -> Self {
        if numer == Zero::ZERO {
            return Ratio {
                numer,
                denom: One::ONE,
            };
        }

        if numer == denom {
            return Ratio {
                numer: One::ONE,
                denom: One::ONE,
            };
        }

        let gcd = numer.clone().gcd(denom.clone());

        if denom.clone().is_positive() {
            Ratio {
                numer: numer / gcd.clone(),
                denom: denom / gcd,
            }
        } else {
            Ratio {
                numer: -numer / gcd.clone(),
                denom: -denom / gcd,
            }
        }
    }
}

impl<
    T: Add<Output = T>
        + Clone
        + Div<Output = T>
        + Gcd<Output = T>
        + Lcm<Output = T>
        + Mul<Output = T>
        + Neg<Output = T>
        + One
        + PartialEq
        + SignFns
        + Zero,
> Add<Ratio<T>> for Ratio<T>
{
    type Output = Ratio<T>;

    #[inline]
    fn add(self, rhs: Ratio<T>) -> Ratio<T> {
        // If denominators are the same, it suffices to just add numerators and
        // then reduce, obviously.
        if self.denom == rhs.denom {
            return Self::reduce(self.numer + rhs.numer, self.denom);
        }

        let lcm = self.denom.clone().lcm(rhs.denom.clone());

        Self::reduce(
            self.numer * (lcm.clone() / self.denom) + rhs.numer * (lcm.clone() / rhs.denom),
            lcm,
        )
    }
}

impl<
    T: Add<Output = T>
        + Clone
        + Div<Output = T>
        + Gcd<Output = T>
        + Mul<Output = T>
        + Neg<Output = T>
        + One
        + PartialEq
        + SignFns
        + Zero,
> Add<T> for Ratio<T>
{
    type Output = Ratio<T>;

    #[inline]
    fn add(self, rhs: T) -> Ratio<T> {
        Self::reduce(self.numer + rhs * self.denom.clone(), self.denom)
    }
}

impl<
    T: Clone
        + Div<Output = T>
        + Gcd<Output = T>
        + Lcm<Output = T>
        + Mul<Output = T>
        + Neg<Output = T>
        + One
        + PartialEq
        + SignFns
        + Sub<Output = T>
        + Zero,
> Sub<Ratio<T>> for Ratio<T>
{
    type Output = Ratio<T>;

    #[inline]
    fn sub(self, rhs: Ratio<T>) -> Ratio<T> {
        // If denominators are the same, it suffices to just subtract one
        // numerator from the other and then reduce, obviously.
        if self.denom == rhs.denom {
            return Self::reduce(self.numer - rhs.numer, self.denom);
        }

        let lcm = self.denom.clone().lcm(rhs.denom.clone());

        Self::reduce(
            self.numer * (lcm.clone() / self.denom) - rhs.numer * (lcm.clone() / rhs.denom),
            lcm,
        )
    }
}

impl<
    T: Clone
        + Div<Output = T>
        + Gcd<Output = T>
        + Mul<Output = T>
        + Neg<Output = T>
        + One
        + PartialEq
        + SignFns
        + Sub<Output = T>
        + Zero,
> Sub<T> for Ratio<T>
{
    type Output = Ratio<T>;

    #[inline]
    fn sub(self, rhs: T) -> Ratio<T> {
        Self::reduce(self.numer - rhs * self.denom.clone(), self.denom)
    }
}

impl<T: Clone + Div<Output = T> + Gcd<Output = T> + Mul<Output = T>> Mul<Ratio<T>> for Ratio<T> {
    type Output = Ratio<T>;

    #[inline]
    fn mul(self, rhs: Ratio<T>) -> Ratio<T> {
        let gcd_ad = self.numer.clone().gcd(rhs.clone().denom);
        let gcd_bc = self.denom.clone().gcd(rhs.clone().numer);

        Ratio {
            numer: (self.numer.clone() / gcd_ad.clone()) * (rhs.numer.clone() / gcd_bc.clone()),
            denom: (self.denom / gcd_bc) * (rhs.denom / gcd_ad),
        }
    }
}

impl<T: Clone + Gcd<Output = T> + Mul<Output = T> + Div<Output = T>> Mul<T> for Ratio<T> {
    type Output = Ratio<T>;

    #[inline]
    fn mul(self, rhs: T) -> Ratio<T> {
        let gcd = self.denom.clone().gcd(rhs.clone());

        Ratio {
            numer: self.numer * (rhs / gcd.clone()),
            denom: self.denom / gcd,
        }
    }
}

impl<T: Clone + Div<Output = T> + Gcd<Output = T> + Mul<Output = T> + Neg<Output = T> + SignFns>
    Div<Ratio<T>> for Ratio<T>
{
    type Output = Ratio<T>;

    #[inline]
    fn div(self, rhs: Ratio<T>) -> Ratio<T> {
        let gcd_ac = self.numer.clone().gcd(rhs.numer.clone());
        let gcd_bd = self.denom.clone().gcd(rhs.denom.clone());

        let numer = (self.numer.clone() / gcd_ac.clone()) * (rhs.denom.clone() / gcd_bd.clone());
        let denom = (self.denom / gcd_bd) * (rhs.numer / gcd_ac);

        if denom.clone().is_positive() {
            Ratio { numer, denom }
        } else {
            Ratio {
                numer: -numer,
                denom: -denom,
            }
        }
    }
}

impl<T: Clone + Gcd<Output = T> + Mul<Output = T> + Div<Output = T>> Div<T> for Ratio<T> {
    type Output = Ratio<T>;

    #[inline]
    fn div(self, rhs: T) -> Ratio<T> {
        let gcd = self.numer.clone().gcd(rhs.clone());

        Ratio {
            numer: self.numer / gcd.clone(),
            denom: self.denom * (rhs / gcd),
        }
    }
}

impl<T: Clone + Mul<Output = T> + PartialEq> PartialEq for Ratio<T> {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.numer.clone() * other.denom.clone() == self.denom.clone() * other.numer.clone()
    }
}

impl<T: Clone + Mul<Output = T> + Eq> Eq for Ratio<T> {}

impl<T: Clone + Mul<Output = T> + PartialOrd> PartialOrd for Ratio<T> {
    #[inline]
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        (self.numer.clone() * other.denom.clone())
            .partial_cmp(&(self.denom.clone() * other.numer.clone()))
    }
}

impl<T: Clone + Mul<Output = T> + Ord> Ord for Ratio<T> {
    #[inline]
    fn cmp(&self, other: &Self) -> Ordering {
        // If denominators are equal, just compare the numerators.
        if self.denom == other.denom {
            return self.numer.cmp(&other.numer);
        }

        // If numerators are equal, just compare the denominators in reverse.
        if self.numer == other.numer {
            return other.denom.cmp(&self.denom);
        }

        // TODO: For larger integers, we probably want to use a less efficient
        // algorithm that won't overflow.
        (self.numer.clone() * other.denom.clone()).cmp(&(self.denom.clone() * other.numer.clone()))
    }
}

impl<T: MinFinite + MinimizerDenom> MinFinite for Ratio<T> {
    const MIN_FINITE: Self = Ratio {
        numer: MinFinite::MIN_FINITE,
        denom: MinimizerDenom::MINIMIZER_DENOM,
    };
}

impl<T: MaxFinite + MaximizerDenom> MaxFinite for Ratio<T> {
    const MAX_FINITE: Self = Ratio {
        numer: MaxFinite::MAX_FINITE,
        denom: MaximizerDenom::MAXIMIZER_DENOM,
    };
}

impl<T: MinExtended + MinimizerDenom> MinExtended for Ratio<T> {
    const MIN_EXTENDED: Self = Ratio {
        numer: MinExtended::MIN_EXTENDED,
        denom: MinimizerDenom::MINIMIZER_DENOM,
    };
}

impl<T: MaxExtended + MaximizerDenom> MaxExtended for Ratio<T> {
    const MAX_EXTENDED: Self = Ratio {
        numer: MaxExtended::MAX_EXTENDED,
        denom: MaximizerDenom::MAXIMIZER_DENOM,
    };
}

impl<T: MinExactInteger + One> MinExactInteger for Ratio<T> {
    const MIN_EXACT_INTEGER: Self = Ratio {
        numer: MinExactInteger::MIN_EXACT_INTEGER,
        denom: One::ONE,
    };
}

impl<T: MaxExactInteger + One> MaxExactInteger for Ratio<T> {
    const MAX_EXACT_INTEGER: Self = Ratio {
        numer: MaxExactInteger::MAX_EXACT_INTEGER,
        denom: One::ONE,
    };
}

impl<T: MinPositive + MaxFinite + One> MinPositive for Ratio<T> {
    const MIN_POSITIVE: Self = Ratio {
        numer: MinPositive::MIN_POSITIVE,
        denom: MaxFinite::MAX_FINITE,
    };
}

impl<T: MaxNegative + MaxFinite + One> MaxNegative for Ratio<T> {
    const MAX_NEGATIVE: Self = Ratio {
        numer: MaxNegative::MAX_NEGATIVE,
        denom: MaxFinite::MAX_FINITE,
    };
}

impl<T: Zero + One> Zero for Ratio<T> {
    const ZERO: Self = Ratio {
        numer: Zero::ZERO,
        denom: One::ONE,
    };
}

impl<T: One> One for Ratio<T> {
    const ONE: Self = Ratio {
        numer: One::ONE,
        denom: One::ONE,
    };
}

impl<T: One + MinusInfinity> MinusInfinity for Ratio<T> {
    const MINUS_INFINITY: Self = Ratio {
        numer: MinusInfinity::MINUS_INFINITY,
        denom: One::ONE,
    };
}

impl<T: One + PlusInfinity> PlusInfinity for Ratio<T> {
    const PLUS_INFINITY: Self = Ratio {
        numer: PlusInfinity::PLUS_INFINITY,
        denom: One::ONE,
    };
}

impl<T: Nan + One> Nan for Ratio<T> {
    const NAN: Self = Ratio {
        numer: Nan::NAN,
        denom: One::ONE,
    };
}
