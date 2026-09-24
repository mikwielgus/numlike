// SPDX-FileCopyrightText: 2026 numlike contributors
//
// SPDX-License-Identifier: MIT OR Apache-2.0

/// Multiply a number by a multiplier, round it to an integer, and convert it to
/// this integer type, or do nothing if the input type is integer.
///
/// This trait is analogous to standard library's [`From`], though unlike it
/// it's lossy. It's the inverse of [`QuantizeInto`].
pub trait QuantizeFrom<T> {
    /// Multiply a number by a multiplier, round it to an integer, and convert it to
    /// this integer type, or do nothing if input type is integer.
    fn quantize_from(value: T, multiplier: T) -> Self;
}

/// Multiply this number by a multiplier, round it to an integer, and convert it
/// to another type, or do nothing if the input type is integer.
///
/// This trait is analogous to standard library's [`Into`], though unlike it
/// it's lossy. It's the inverse of [`QuantizeFrom`].
///
/// Analogously to Rust standard library's [`Into`], it is recommended to not
/// implement this trait directly, as it already has a blanket implementation
/// for types that implement [`QuantizeFrom`].
pub trait QuantizeInto<T> {
    /// Multiply a number by a multiplier, round it to an integer, and convert
    /// it to another type, or do nothing if input type is integer.
    fn quantize_into(self, multiplier: Self) -> T;
}

impl<T, U> QuantizeInto<U> for T
where
    U: QuantizeFrom<T>,
{
    #[inline]
    fn quantize_into(self, multiplier: Self) -> U {
        U::quantize_from(self, multiplier)
    }
}

macro_rules! impl_passthrough_quantize_from {
    ($src:ty => $($dst:ty),+) => {
        $(
            impl QuantizeFrom<$src> for $dst {
                #[inline]
                fn quantize_from(value: $src, _multiplier: $src) -> Self {
                    value as $dst
                }
            }
        )+
    };
}

macro_rules! impl_quantize_from_for_ints {
    ($($src:ty),+) => {
        $(
            impl_passthrough_quantize_from!(
                $src =>
                i8, i16, i32, i64, i128, isize,
                u8, u16, u32, u64, u128, usize
            );
        )+
    };
}

impl_quantize_from_for_ints!(
    i8, i16, i32, i64, i128, isize, u8, u16, u32, u64, u128, usize
);

macro_rules! impl_round_quantize_from {
    ($round:path, $src:ty => $($dst:ty),+) => {
        $(
            impl QuantizeFrom<$src> for $dst {
                #[inline]
                fn quantize_from(value: $src, multiplier: $src) -> Self {
                    $round(value * multiplier) as $dst
                }
            }
        )+
    };
}

macro_rules! impl_round_quantize_from_for_machreal {
    ($src:ty, $round:path) => {
        impl_round_quantize_from!(
            $round, $src =>
            i8, i16, i32, i64, i128, isize,
            u8, u16, u32, u64, u128, usize
        );
    };
}

#[cfg(feature = "std")]
impl_round_quantize_from_for_machreal!(f32, f32::round);
#[cfg(feature = "std")]
impl_round_quantize_from_for_machreal!(f64, f64::round);
#[cfg(all(not(feature = "std"), feature = "libm"))]
impl_round_quantize_from_for_machreal!(f32, libm::roundf);
#[cfg(all(not(feature = "std"), feature = "libm"))]
impl_round_quantize_from_for_machreal!(f64, libm::round);

// TODO tests?
