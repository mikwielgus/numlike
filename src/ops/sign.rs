// SPDX-FileCopyrightText: 2026 numlike contributors
//
// SPDX-License-Identifier: MIT OR Apache-2.0

pub trait Signum {
    fn signum(self) -> Self;
}

pub trait Abs {
    fn abs(self) -> Self;
}

pub trait Sign: Signum + Abs {}
impl<T: Signum + Abs> Sign for T {}

macro_rules! impl_sign_traits_for_ords {
    ($($ty:ty),*) => {
        $(
            impl Signum for $ty {
                #[inline]
                fn signum(self) -> Self {
                    <$ty>::signum(self)
                }
            }

            impl Abs for $ty {
                #[inline]
                fn abs(self) -> Self {
                    <$ty>::abs(self)
                }
            }
        )*
    };
}

impl_sign_traits_for_ords!(i8, i16, i32, i64, i128, isize);
