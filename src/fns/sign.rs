// SPDX-FileCopyrightText: 2026 numlike contributors
//
// SPDX-License-Identifier: MIT OR Apache-2.0

pub trait Signum {
    type Output;

    fn signum(self) -> Self::Output;
}

pub trait Abs {
    type Output;

    fn abs(self) -> Self::Output;
}

pub trait Sign: Signum + Abs {}
impl<T: Signum + Abs> Sign for T {}

macro_rules! impl_sign_traits_for_signeds {
    ($($ty:ty),*) => {
        $(
            impl Signum for $ty {
                type Output = $ty;

                #[inline]
                fn signum(self) -> Self::Output {
                    <$ty>::signum(self)
                }
            }

            impl Abs for $ty {
                type Output = $ty;

                #[inline]
                fn abs(self) -> Self::Output {
                    <$ty>::abs(self)
                }
            }
        )*
    };
}

impl_sign_traits_for_signeds!(i8, i16, i32, i64, i128, isize);
// TODO: unsigned types, probably.
