// SPDX-FileCopyrightText: 2026 numlike contributors
//
// SPDX-License-Identifier: MIT OR Apache-2.0

/// Converts a value from one type to another.
///
/// Internally, `as` operator is used to convert between Rust primitives. For
/// non-exact conversion where a non-primitive type are involved, rounding
/// behavior is left to the implementors, but consistency with `as` is
/// encouraged.
pub trait CastFrom<T> {
    fn cast_from(value: T) -> Self;
}

/// Converts a value from one type to another.
///
/// Internally, `as` operator is used to convert between Rust primitives. For
/// non-exact conversion where a non-primitive type are involved, rounding
/// behavior is left to the implementors, but consistency with `as` is
/// encouraged.
///
/// Just as with Rust's standard library's `Into`, It is recommended to not
/// implement this trait directly, as it already has a blanket implementation
/// for types that implement `CastFrom`.
pub trait CastInto<T> {
    fn cast_into(self) -> T;
}

impl<T, U> CastInto<U> for T
where
    U: CastFrom<T>,
{
    #[inline]
    fn cast_into(self) -> U {
        U::cast_from(self)
    }
}

macro_rules! impl_cast_from {
    ($src:ty => $($dst:ty),+) => {
        $(
            impl CastFrom<$src> for $dst {
                #[inline]
                fn cast_from(value: $src) -> Self {
                    value as $dst
                }
            }
        )+
    };
}

macro_rules! impl_cast_from_for_primitives {
    ($($src:ty),+) => {
        $(
            impl_cast_from!(
                $src =>
                u8, u16, u32, u64, u128, usize,
                i8, i16, i32, i64, i128, isize,
                f32, f64
            );
        )+
    };
}

impl_cast_from_for_primitives!(
    u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize, f32, f64
);
