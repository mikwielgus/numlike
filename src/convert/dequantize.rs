// SPDX-FileCopyrightText: 2026 numlike contributors
//
// SPDX-License-Identifier: MIT OR Apache-2.0

/// Convert an integer to a machine real (i.e. float) type and divide it by
/// a scale.
///
/// This trait is analogous to standard library's [`From`], though unlike it
/// it's lossy. It's the converse of [`DequantizeInto`].
///
/// This operation, called dequantization, is the reverse of quantization. Use
/// this trait to reverse [`QuantizeFrom`]/[`QuantizeInto`].
pub trait DequantizeFrom<T> {
    /// Convert an integer to a machine real (i.e. float) type and divide it by
    /// a scale.
    fn dequantize_from(value: T, scale: Self) -> Self;
}

/// Convert an integer to a machine real (i.e. float) type and divide it by
/// a scale.
///
/// This trait is analogous to standard library's [`Into`], though unlike it
/// it's lossy. It's the converse of [`DequantizeFrom`].
///
/// This operation, called dequantization, is the reverse of quantization. Use
/// this trait to reverse [`QuantizeFrom`]/[`QuantizeInto`].
pub trait DequantizeInto<T> {
    /// Convert an integer to a machine real (i.e. float) type and divide it by
    /// a scale.
    fn dequantize_into(self, scale: T) -> T;
}

impl<T, U> DequantizeInto<U> for T
where
    U: DequantizeFrom<T>,
{
    #[inline]
    fn dequantize_into(self, scale: U) -> U {
        U::dequantize_from(self, scale)
    }
}

macro_rules! impl_passthrough_dequantize_from {
    ($src:ty => $($dst:ty),+) => {
        $(
            impl DequantizeFrom<$src> for $dst {
                #[inline]
                fn dequantize_from(value: $src, _scale: Self) -> Self {
                    value as $dst
                }
            }
        )+
    };
}

macro_rules! impl_dequantize_from_for_ints {
    ($($src:ty),+) => {
        $(
            impl_passthrough_dequantize_from!(
                $src =>
                i8, i16, i32, i64, i128, isize,
                u8, u16, u32, u64, u128, usize
            );
        )+
    }
}

impl_dequantize_from_for_ints!(
    i8, i16, i32, i64, i128, isize, u8, u16, u32, u64, u128, usize
);

#[cfg(any(feature = "std", feature = "libm"))]
macro_rules! impl_machreal_dequantize_from {
    ($dst:ty => $($src:ty),+) => {
        $(
            impl DequantizeFrom<$src> for $dst {
                #[inline]
                fn dequantize_from(value: $src, scale: Self) -> Self {
                    value as $dst / scale
                }
            }
        )+
    };
}

#[cfg(any(feature = "std", feature = "libm"))]
macro_rules! impl_machreal_dequantize_from_for_ints {
    ($dst:ty) => {
        impl_machreal_dequantize_from!(
            $dst =>
            i8, i16, i32, i64, i128, isize,
            u8, u16, u32, u64, u128, usize
        );
    };
}

#[cfg(any(feature = "std", feature = "libm"))]
impl_machreal_dequantize_from_for_ints!(f32);
#[cfg(any(feature = "std", feature = "libm"))]
impl_machreal_dequantize_from_for_ints!(f64);

// TODO tests?
