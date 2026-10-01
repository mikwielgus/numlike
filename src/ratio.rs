// SPDX-FileCopyrightText: 2026 numlike contributors
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use crate::elem::{One, Zero};

pub struct Ratio<T> {
    numer: T,
    denom: T,
}

impl<T> Ratio<T> {
    pub fn numer(&self) -> &T {
        &self.numer
    }

    pub fn denom(&self) -> &T {
        &self.denom
    }
}

impl<T: PartialEq + Zero + One> Ratio<T> {
    pub fn new(numer: T, denom: T) -> Self {
        let mut this = Self { numer, denom };
        this.reduce();

        this
    }

    fn reduce(&mut self) {
        if self.numer == Zero::ZERO {
            self.denom = One::ONE;
            return;
        }

        if self.numer == self.denom {
            self.numer = One::ONE;
            self.denom = One::ONE;
            return;
        }

        let gcd = self.numer.gcd(&self.denom);
    }
}
