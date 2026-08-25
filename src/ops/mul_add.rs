// SPDX-FileCopyrightText: 2026 numlike contributors
//
// SPDX-License-Identifier: MIT OR Apache-2.0

pub trait MulAdd<A = Self, B = Self> {
    type Output;

    fn mul_add(self, a: A, b: B) -> Self::Output;
}

pub trait CheckedMulAdd<A = Self, B = Self> {
    type Output;

    fn checked_mul_add(self, a: A, b: B) -> Option<Self::Output>;
}

macro_rules! impl_mul_add_for_ints {
    ($($ty:ty),*) => {
        $(
            impl MulAdd<$ty, $ty> for $ty {
                type Output = $ty;

                #[inline]
                fn mul_add(self, a: $ty, b: $ty) -> Self::Output {
                    (self * a) + b
                }
            }

            impl CheckedMulAdd<$ty, $ty> for $ty {
                type Output = $ty;

                #[inline]
                fn checked_mul_add(self, a: $ty, b: $ty) -> Option<Self::Output> {
                    <$ty>::checked_mul(self, a).and_then(|product| <$ty>::checked_add(product, b))
                }
            }
        )*
    };
}

macro_rules! impl_mul_add_for_floats {
    ($($ty:ty),*) => {
        $(
            impl MulAdd<$ty, $ty> for $ty {
                type Output = $ty;

                #[inline]
                fn mul_add(self, a: $ty, b: $ty) -> Self::Output {
                    <$ty>::mul_add(self, a, b)
                }
            }

            impl CheckedMulAdd<$ty, $ty> for $ty {
                type Output = $ty;

                #[inline]
                fn checked_mul_add(self, a: $ty, b: $ty) -> Option<Self::Output> {
                    let result = <$ty>::mul_add(self, a, b);

                    result.is_finite().then_some(result)
                }
            }
        )*
    };
}

impl_mul_add_for_ints!(i8, i16, i32, i64, i128, isize);
impl_mul_add_for_ints!(u8, u16, u32, u64, u128, usize);
impl_mul_add_for_floats!(f32, f64);
