// SPDX-FileCopyrightText: 2026 numlike contributors
//
// SPDX-License-Identifier: MIT OR Apache-2.0

/// Convert a value from one type to another, possibly with lossy approximation.
///
/// This trait is analogous to standard library's [`From`], but allows
/// the conversion to be approximate and thus lossy. It is the inverse of
/// [`CastInto`].
///
/// Internally, `as` operator is used to convert between Rust primitives. For
/// non-exact conversion where a non-primitive type are involved, rounding
/// behavior is left to the implementors, but aiming for consistency with `as`
/// is highly encouraged.
pub trait CastFrom<T> {
    /// Convert to this type from the input type, possibly with lossy
    /// approximation.
    fn cast_from(value: T) -> Self;
}

/// Convert a value from one type to another, possibly with lossy approximation.
///
/// This trait is analogous to standard library's [`From`], but allows
/// the conversion to be approximate and thus lossy. It is the inverse of
/// [`CastFrom`].
///
/// Internally, the `as` operator is used to convert between Rust primitives.
/// For non-exact conversion where a non-primitive type are involved, rounding
/// behavior is left to the implementors, but aiming for consistency with `as`
/// is highly encouraged.
///
/// Analogously to Rust standard library's [`Into`], it is recommended to not
/// implement this trait directly, as it already has a blanket implementation
/// for types that implement [`CastFrom`].
pub trait CastInto<T> {
    /// Convert this type into the (usually inferred) input type, possibly with
    /// lossy approximation.
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
