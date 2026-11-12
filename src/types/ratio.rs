// SPDX-FileCopyrightText: 2026 numlike contributors
//
// SPDX-License-Identifier: MIT OR Apache-2.0

//! A ratio between two numbers.

use core::{
    cmp::Ordering,
    ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign},
};

use crate::{
    cmp::{NanfixEq, NanfixPartialEq, NanmaxOrd, NanmaxPartialOrd, NanminOrd, NanminPartialOrd},
    elem::{
        AssocConstMinusInfinity, AssocConstNan, AssocConstOne, AssocConstPlusInfinity,
        AssocConstZero, FnMinusInfinity, FnNan, FnOne, FnPlusInfinity, FnZero, IsMinusInfinity,
        IsNan, IsOne, IsPlusInfinity, IsZero,
    },
    fns::{Abs, CheckedAbs, Gcd, IsNegative, IsPositive, Lcm, Sgn, SignFns},
    limits::{
        MaxExactInteger, MaxExtended, MaxFinite, MaxNegative, MaximizerDenom, MinExactInteger,
        MinExtended, MinFinite, MinPositive, MinimizerDenom,
    },
    ops::{CheckedAdd, CheckedDiv, CheckedMul, CheckedNeg, CheckedSub, NegAssign},
};

/// A ratio between two numbers.
#[derive(Clone, Copy, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
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
    T: Clone
        + Div<Output = T>
        + Gcd<Output = T>
        + Neg<Output = T>
        + AssocConstOne<Value = T>
        + PartialEq
        + SignFns
        + AssocConstZero<Value = T>,
> Ratio<T>
{
    /// Creates a new ratio. The numerator and denominator will be reduced.
    #[inline]
    pub fn new(numer: T, denom: T) -> Self {
        Self::reduce(numer, denom)
    }

    #[inline]
    fn reduce(numer: T, denom: T) -> Self {
        if numer == <T as AssocConstZero>::ZERO {
            return Ratio {
                numer,
                denom: <T as AssocConstOne>::ONE,
            };
        }

        if numer == denom {
            return Ratio {
                numer: <T as AssocConstOne>::ONE,
                denom: <T as AssocConstOne>::ONE,
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
    T: CheckedNeg<Output = T>
        + Clone
        + Div<Output = T>
        + Gcd<Output = T>
        + AssocConstOne<Value = T>
        + PartialEq
        + SignFns
        + AssocConstZero<Value = T>,
> Ratio<T>
{
    #[inline]
    fn checked_reduce(numer: T, denom: T) -> Option<Self> {
        if numer == <T as AssocConstZero>::ZERO {
            return Some(Ratio {
                numer,
                denom: <T as AssocConstOne>::ONE,
            });
        }

        if numer == denom {
            return Some(Ratio {
                numer: <T as AssocConstOne>::ONE,
                denom: <T as AssocConstOne>::ONE,
            });
        }

        let gcd = numer.clone().gcd(denom.clone());

        if denom.clone().is_positive() {
            Some(Ratio {
                numer: numer / gcd.clone(),
                denom: denom / gcd,
            })
        } else {
            Some(Ratio {
                numer: numer.checked_neg()? / gcd.clone(),
                denom: denom.checked_neg()? / gcd,
            })
        }
    }
}

impl<
    T: CheckedAbs<Output = T>
        + CheckedMul<Output = T>
        + Clone
        + Div<Output = T>
        + Gcd<Output = T>
        + PartialEq
        + AssocConstZero<Value = T>,
> Ratio<T>
{
    #[inline]
    fn checked_lcm(lhs: T, rhs: T) -> Option<T> {
        if lhs == <T as AssocConstZero>::ZERO && rhs == <T as AssocConstZero>::ZERO {
            return Some(<T as AssocConstZero>::ZERO);
        }

        let gcd = lhs.clone().gcd(rhs.clone());

        lhs.checked_mul(rhs / gcd)?.checked_abs()
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
        + AssocConstOne<Value = T>
        + PartialEq
        + SignFns
        + AssocConstZero<Value = T>,
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
        + Lcm<Output = T>
        + Mul<Output = T>
        + Neg<Output = T>
        + AssocConstOne<Value = T>
        + PartialEq
        + SignFns
        + AssocConstZero<Value = T>,
> AddAssign<Ratio<T>> for Ratio<T>
{
    #[inline]
    fn add_assign(&mut self, rhs: Ratio<T>) {
        *self = self.clone() + rhs;
    }
}

impl<
    T: CheckedAbs<Output = T>
        + CheckedAdd<Output = T>
        + CheckedMul<Output = T>
        + CheckedNeg<Output = T>
        + Clone
        + Div<Output = T>
        + Gcd<Output = T>
        + AssocConstOne<Value = T>
        + PartialEq
        + SignFns
        + AssocConstZero<Value = T>,
> CheckedAdd<Ratio<T>> for Ratio<T>
{
    type Output = Ratio<T>;

    #[inline]
    fn checked_add(self, rhs: Ratio<T>) -> Option<Ratio<T>> {
        if self.denom == rhs.denom {
            return Self::checked_reduce(self.numer.checked_add(rhs.numer)?, self.denom);
        }

        let lcm = Self::checked_lcm(self.denom.clone(), rhs.denom.clone())?;

        Self::checked_reduce(
            self.numer
                .checked_mul(lcm.clone() / self.denom)?
                .checked_add(rhs.numer.checked_mul(lcm.clone() / rhs.denom)?)?,
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
        + AssocConstOne<Value = T>
        + PartialEq
        + SignFns
        + AssocConstZero<Value = T>,
> Add<T> for Ratio<T>
{
    type Output = Ratio<T>;

    #[inline]
    fn add(self, rhs: T) -> Ratio<T> {
        Self::reduce(self.numer + rhs * self.denom.clone(), self.denom)
    }
}

impl<
    T: Add<Output = T>
        + Clone
        + Div<Output = T>
        + Gcd<Output = T>
        + Mul<Output = T>
        + Neg<Output = T>
        + AssocConstOne<Value = T>
        + PartialEq
        + SignFns
        + AssocConstZero<Value = T>,
> AddAssign<T> for Ratio<T>
{
    #[inline]
    fn add_assign(&mut self, rhs: T) {
        *self = self.clone() + rhs;
    }
}

impl<
    T: CheckedAdd<Output = T>
        + CheckedMul<Output = T>
        + CheckedNeg<Output = T>
        + Clone
        + Div<Output = T>
        + Gcd<Output = T>
        + AssocConstOne<Value = T>
        + PartialEq
        + SignFns
        + AssocConstZero<Value = T>,
> CheckedAdd<T> for Ratio<T>
{
    type Output = Ratio<T>;

    #[inline]
    fn checked_add(self, rhs: T) -> Option<Ratio<T>> {
        Self::checked_reduce(
            self.numer
                .checked_add(rhs.checked_mul(self.denom.clone())?)?,
            self.denom,
        )
    }
}

impl<
    T: Clone
        + Div<Output = T>
        + Gcd<Output = T>
        + Lcm<Output = T>
        + Mul<Output = T>
        + Neg<Output = T>
        + AssocConstOne<Value = T>
        + PartialEq
        + SignFns
        + Sub<Output = T>
        + AssocConstZero<Value = T>,
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
        + Lcm<Output = T>
        + Mul<Output = T>
        + Neg<Output = T>
        + AssocConstOne<Value = T>
        + PartialEq
        + SignFns
        + Sub<Output = T>
        + AssocConstZero<Value = T>,
> SubAssign<Ratio<T>> for Ratio<T>
{
    #[inline]
    fn sub_assign(&mut self, rhs: Ratio<T>) {
        *self = self.clone() - rhs;
    }
}

impl<
    T: CheckedAbs<Output = T>
        + CheckedMul<Output = T>
        + CheckedNeg<Output = T>
        + CheckedSub<Output = T>
        + Clone
        + Div<Output = T>
        + Gcd<Output = T>
        + AssocConstOne<Value = T>
        + PartialEq
        + SignFns
        + AssocConstZero<Value = T>,
> CheckedSub<Ratio<T>> for Ratio<T>
{
    type Output = Ratio<T>;

    #[inline]
    fn checked_sub(self, rhs: Ratio<T>) -> Option<Ratio<T>> {
        if self.denom == rhs.denom {
            return Self::checked_reduce(self.numer.checked_sub(rhs.numer)?, self.denom);
        }

        let lcm = Self::checked_lcm(self.denom.clone(), rhs.denom.clone())?;

        Self::checked_reduce(
            self.numer
                .checked_mul(lcm.clone() / self.denom)?
                .checked_sub(rhs.numer.checked_mul(lcm.clone() / rhs.denom)?)?,
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
        + AssocConstOne<Value = T>
        + PartialEq
        + SignFns
        + Sub<Output = T>
        + AssocConstZero<Value = T>,
> Sub<T> for Ratio<T>
{
    type Output = Ratio<T>;

    #[inline]
    fn sub(self, rhs: T) -> Ratio<T> {
        Self::reduce(self.numer - rhs * self.denom.clone(), self.denom)
    }
}

impl<
    T: Clone
        + Div<Output = T>
        + Gcd<Output = T>
        + Mul<Output = T>
        + Neg<Output = T>
        + AssocConstOne<Value = T>
        + PartialEq
        + SignFns
        + Sub<Output = T>
        + AssocConstZero<Value = T>,
> SubAssign<T> for Ratio<T>
{
    #[inline]
    fn sub_assign(&mut self, rhs: T) {
        *self = self.clone() - rhs;
    }
}

impl<
    T: CheckedMul<Output = T>
        + CheckedNeg<Output = T>
        + CheckedSub<Output = T>
        + Clone
        + Div<Output = T>
        + Gcd<Output = T>
        + AssocConstOne<Value = T>
        + PartialEq
        + SignFns
        + AssocConstZero<Value = T>,
> CheckedSub<T> for Ratio<T>
{
    type Output = Ratio<T>;

    #[inline]
    fn checked_sub(self, rhs: T) -> Option<Ratio<T>> {
        Self::checked_reduce(
            self.numer
                .checked_sub(rhs.checked_mul(self.denom.clone())?)?,
            self.denom,
        )
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

impl<T: Clone + Div<Output = T> + Gcd<Output = T> + Mul<Output = T>> MulAssign<Ratio<T>>
    for Ratio<T>
{
    #[inline]
    fn mul_assign(&mut self, rhs: Ratio<T>) {
        *self = self.clone() * rhs;
    }
}

impl<T: CheckedMul<Output = T> + Clone + Div<Output = T> + Gcd<Output = T>> CheckedMul<Ratio<T>>
    for Ratio<T>
{
    type Output = Ratio<T>;

    #[inline]
    fn checked_mul(self, rhs: Ratio<T>) -> Option<Ratio<T>> {
        let gcd_ad = self.numer.clone().gcd(rhs.denom.clone());
        let gcd_bc = self.denom.clone().gcd(rhs.numer.clone());

        Some(Ratio {
            numer: (self.numer / gcd_ad.clone()).checked_mul(rhs.numer / gcd_bc.clone())?,
            denom: (self.denom / gcd_bc).checked_mul(rhs.denom / gcd_ad)?,
        })
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

impl<T: Clone + Gcd<Output = T> + Mul<Output = T> + Div<Output = T>> MulAssign<T> for Ratio<T> {
    #[inline]
    fn mul_assign(&mut self, rhs: T) {
        *self = self.clone() * rhs;
    }
}

impl<T: CheckedMul<Output = T> + Clone + Div<Output = T> + Gcd<Output = T>> CheckedMul<T>
    for Ratio<T>
{
    type Output = Ratio<T>;

    #[inline]
    fn checked_mul(self, rhs: T) -> Option<Ratio<T>> {
        let gcd = self.denom.clone().gcd(rhs.clone());

        Some(Ratio {
            numer: self.numer.checked_mul(rhs / gcd.clone())?,
            denom: self.denom / gcd,
        })
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

impl<T: Clone + Div<Output = T> + Gcd<Output = T> + Mul<Output = T> + Neg<Output = T> + SignFns>
    DivAssign<Ratio<T>> for Ratio<T>
{
    #[inline]
    fn div_assign(&mut self, rhs: Ratio<T>) {
        *self = self.clone() / rhs;
    }
}

impl<
    T: CheckedMul<Output = T>
        + CheckedNeg<Output = T>
        + Clone
        + Div<Output = T>
        + Gcd<Output = T>
        + PartialEq
        + SignFns
        + AssocConstZero<Value = T>,
> CheckedDiv<Ratio<T>> for Ratio<T>
{
    type Output = Ratio<T>;

    #[inline]
    fn checked_div(self, rhs: Ratio<T>) -> Option<Ratio<T>> {
        if rhs.numer == <T as AssocConstZero>::ZERO {
            return None;
        }

        let gcd_ac = self.numer.clone().gcd(rhs.numer.clone());
        let gcd_bd = self.denom.clone().gcd(rhs.denom.clone());

        let numer = (self.numer / gcd_ac.clone()).checked_mul(rhs.denom / gcd_bd.clone())?;
        let denom = (self.denom / gcd_bd).checked_mul(rhs.numer / gcd_ac)?;

        if denom.clone().is_positive() {
            Some(Ratio { numer, denom })
        } else {
            Some(Ratio {
                numer: numer.checked_neg()?,
                denom: denom.checked_neg()?,
            })
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

impl<T: Clone + Gcd<Output = T> + Mul<Output = T> + Div<Output = T>> DivAssign<T> for Ratio<T> {
    #[inline]
    fn div_assign(&mut self, rhs: T) {
        *self = self.clone() / rhs;
    }
}

impl<
    T: CheckedMul<Output = T>
        + Clone
        + Div<Output = T>
        + Gcd<Output = T>
        + PartialEq
        + AssocConstZero<Value = T>,
> CheckedDiv<T> for Ratio<T>
{
    type Output = Ratio<T>;

    #[inline]
    fn checked_div(self, rhs: T) -> Option<Ratio<T>> {
        if rhs == <T as AssocConstZero>::ZERO {
            return None;
        }

        let gcd = self.numer.clone().gcd(rhs.clone());

        Some(Ratio {
            numer: self.numer / gcd.clone(),
            denom: self.denom.checked_mul(rhs / gcd)?,
        })
    }
}

impl<T: Neg<Output = T>> Neg for Ratio<T> {
    type Output = Ratio<T>;

    #[inline]
    fn neg(self) -> Ratio<T> {
        Ratio {
            numer: -self.numer,
            denom: self.denom,
        }
    }
}

impl<T: Clone + Neg<Output = T>> NegAssign for Ratio<T> {
    #[inline]
    fn neg_assign(&mut self) {
        *self = -self.clone();
    }
}

impl<T: CheckedNeg<Output = T>> CheckedNeg for Ratio<T> {
    type Output = Ratio<T>;

    #[inline]
    fn checked_neg(self) -> Option<Ratio<T>> {
        Some(Ratio {
            numer: self.numer.checked_neg()?,
            denom: self.denom,
        })
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

impl<T: Clone + IsNan + Mul<Output = T> + PartialEq> NanfixPartialEq for Ratio<T> {
    #[inline]
    fn nanfix_eq(&self, other: &Self) -> bool {
        (self.clone().is_nan() && other.clone().is_nan()) || PartialEq::eq(self, other)
    }
}

impl<T: Clone + IsNan + Mul<Output = T> + PartialEq> NanfixEq for Ratio<T> {}

impl<T: Clone + IsNan + Mul<Output = T> + PartialEq + PartialOrd> NanminPartialOrd for Ratio<T> {
    #[inline]
    fn nanmin_partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(NanminOrd::nanmin_cmp(self, other))
    }
}

impl<T: Clone + IsNan + Mul<Output = T> + PartialEq + PartialOrd> NanminOrd for Ratio<T> {
    #[inline]
    fn nanmin_cmp(&self, other: &Self) -> Ordering {
        match (self.clone().is_nan(), other.clone().is_nan()) {
            (true, true) => Ordering::Equal,
            (true, false) => Ordering::Less,
            (false, true) => Ordering::Greater,
            (false, false) => PartialOrd::partial_cmp(self, other).unwrap(),
        }
    }
}

impl<T: Clone + IsNan + Mul<Output = T> + PartialEq + PartialOrd> NanmaxPartialOrd for Ratio<T> {
    #[inline]
    fn nanmax_partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(NanmaxOrd::nanmax_cmp(self, other))
    }
}

impl<T: Clone + IsNan + Mul<Output = T> + PartialEq + PartialOrd> NanmaxOrd for Ratio<T> {
    #[inline]
    fn nanmax_cmp(&self, other: &Self) -> Ordering {
        match (self.clone().is_nan(), other.clone().is_nan()) {
            (true, true) => Ordering::Equal,
            (true, false) => Ordering::Greater,
            (false, true) => Ordering::Less,
            (false, false) => PartialOrd::partial_cmp(self, other).unwrap(),
        }
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

impl<T: MinExactInteger + AssocConstOne<Value = T>> MinExactInteger for Ratio<T> {
    const MIN_EXACT_INTEGER: Self = Ratio {
        numer: MinExactInteger::MIN_EXACT_INTEGER,
        denom: <T as AssocConstOne>::ONE,
    };
}

impl<T: MaxExactInteger + AssocConstOne<Value = T>> MaxExactInteger for Ratio<T> {
    const MAX_EXACT_INTEGER: Self = Ratio {
        numer: MaxExactInteger::MAX_EXACT_INTEGER,
        denom: <T as AssocConstOne>::ONE,
    };
}

impl<T: MinPositive + MaxFinite + AssocConstOne<Value = T>> MinPositive for Ratio<T> {
    const MIN_POSITIVE: Self = Ratio {
        numer: MinPositive::MIN_POSITIVE,
        denom: MaxFinite::MAX_FINITE,
    };
}

impl<T: MaxNegative + MaxFinite + AssocConstOne<Value = T>> MaxNegative for Ratio<T> {
    const MAX_NEGATIVE: Self = Ratio {
        numer: MaxNegative::MAX_NEGATIVE,
        denom: MaxFinite::MAX_FINITE,
    };
}

impl<T: FnZero + FnOne> FnZero for Ratio<T> {
    #[inline]
    fn zero() -> Self {
        Ratio {
            numer: FnZero::zero(),
            denom: FnOne::one(),
        }
    }
}

impl<T: AssocConstZero<Value = T> + AssocConstOne<Value = T>> AssocConstZero for Ratio<T> {
    type Value = Self;

    const ZERO: Self::Value = Ratio {
        numer: <T as AssocConstZero>::ZERO,
        denom: <T as AssocConstOne>::ONE,
    };
}

impl<T: FnOne> FnOne for Ratio<T> {
    #[inline]
    fn one() -> Self {
        Ratio {
            numer: FnOne::one(),
            denom: FnOne::one(),
        }
    }
}

impl<T: AssocConstOne<Value = T>> AssocConstOne for Ratio<T> {
    type Value = Self;

    const ONE: Self::Value = Ratio {
        numer: <T as AssocConstOne>::ONE,
        denom: <T as AssocConstOne>::ONE,
    };
}

impl<T: AssocConstMinusInfinity<Value = T> + AssocConstOne<Value = T>> AssocConstMinusInfinity
    for Ratio<T>
{
    type Value = Self;

    const MINUS_INFINITY: Self::Value = Ratio {
        numer: <T as AssocConstMinusInfinity>::MINUS_INFINITY,
        denom: <T as AssocConstOne>::ONE,
    };
}

impl<T: FnMinusInfinity + FnOne> FnMinusInfinity for Ratio<T> {
    #[inline]
    fn minus_infinity() -> Self {
        Ratio {
            numer: <T as FnMinusInfinity>::minus_infinity(),
            denom: FnOne::one(),
        }
    }
}

impl<T: AssocConstPlusInfinity<Value = T> + AssocConstOne<Value = T>> AssocConstPlusInfinity
    for Ratio<T>
{
    type Value = Self;

    const PLUS_INFINITY: Self::Value = Ratio {
        numer: <T as AssocConstPlusInfinity>::PLUS_INFINITY,
        denom: <T as AssocConstOne>::ONE,
    };
}

impl<T: FnPlusInfinity + FnOne> FnPlusInfinity for Ratio<T> {
    #[inline]
    fn plus_infinity() -> Self {
        Ratio {
            numer: <T as FnPlusInfinity>::plus_infinity(),
            denom: FnOne::one(),
        }
    }
}

impl<T: FnNan + FnOne> FnNan for Ratio<T> {
    #[inline]
    fn nan() -> Self {
        Ratio {
            numer: FnNan::nan(),
            denom: FnOne::one(),
        }
    }
}

impl<T: AssocConstNan<Value = T> + AssocConstOne<Value = T>> AssocConstNan for Ratio<T> {
    type Value = Self;

    const NAN: Self::Value = Ratio {
        numer: <T as AssocConstNan>::NAN,
        denom: <T as AssocConstOne>::ONE,
    };
}

impl<T: IsNan> IsNan for Ratio<T> {
    #[inline]
    fn is_nan(self) -> bool {
        // NaN is contagious by any arithmetic operations, including division,
        // so we believe it will be the most natural to assume ratio is NaN not
        // only when its numerator is NaN, but also when denominator is.
        self.numer.is_nan() || self.denom.is_nan()
    }
}

impl<T: IsZero> IsZero for Ratio<T> {
    #[inline]
    fn is_zero(self) -> bool {
        self.numer.is_zero()
    }
}

impl<T: IsMinusInfinity> IsMinusInfinity for Ratio<T> {
    #[inline]
    fn is_minus_infinity(self) -> bool {
        self.numer.is_minus_infinity()
    }
}

impl<T: IsPlusInfinity> IsPlusInfinity for Ratio<T> {
    #[inline]
    fn is_plus_infinity(self) -> bool {
        self.numer.is_plus_infinity()
    }
}

impl<T: IsOne> IsOne for Ratio<T> {
    #[inline]
    fn is_one(self) -> bool {
        self.numer.is_one() && self.denom.is_one()
    }
}

impl<T: IsPositive> IsPositive for Ratio<T> {
    #[inline]
    fn is_positive(self) -> bool {
        self.numer.is_positive()
    }
}

impl<T: IsNegative> IsNegative for Ratio<T> {
    #[inline]
    fn is_negative(self) -> bool {
        self.numer.is_negative()
    }
}

impl<T: AssocConstOne<Value = T> + Sgn<Output = T>> Sgn for Ratio<T> {
    type Output = Ratio<T>;

    #[inline]
    fn sgn(self) -> Self::Output {
        Ratio {
            numer: self.numer.sgn(),
            denom: <T as AssocConstOne>::ONE,
        }
    }
}

impl<T: Abs<Output = T>> Abs for Ratio<T> {
    type Output = Ratio<T>;

    #[inline]
    fn abs(self) -> Self::Output {
        Ratio {
            numer: self.numer.abs(),
            denom: self.denom,
        }
    }
}

impl<T: CheckedAbs<Output = T>> CheckedAbs for Ratio<T> {
    type Output = Ratio<T>;

    #[inline]
    fn checked_abs(self) -> Option<Self::Output> {
        Some(Ratio {
            numer: self.numer.checked_abs()?,
            denom: self.denom,
        })
    }
}

// TODO: the remaining traits.
