// SPDX-FileCopyrightText: 2026 numlike contributors
//
// SPDX-License-Identifier: MIT OR Apache-2.0

macro_rules! for_signed_ints {
    ($m:ident) => {
        $m!(i8, i8_nonnegative_tests, i8_negative_tests);
        $m!(i16, i16_nonnegative_tests, i16_negative_tests);
        $m!(i32, i32_nonnegative_tests, i32_negative_tests);
        $m!(i64, i64_nonnegative_tests, i64_negative_tests);
        $m!(i128, i128_nonnegative_tests, i128_negative_tests);
        $m!(isize, isize_nonnegative_tests, isize_negative_tests);
    };
    ($nonnegative:ident; $negative:ident) => {
        $nonnegative!(i8, i8_nonnegative_tests);
        $negative!(i8, i8_negative_tests);
        $nonnegative!(i16, i16_nonnegative_tests);
        $negative!(i16, i16_negative_tests);
        $nonnegative!(i32, i32_nonnegative_tests);
        $negative!(i32, i32_negative_tests);
        $nonnegative!(i64, i64_nonnegative_tests);
        $negative!(i64, i64_negative_tests);
        $nonnegative!(i128, i128_nonnegative_tests);
        $negative!(i128, i128_negative_tests);
        $nonnegative!(isize, isize_nonnegative_tests);
        $negative!(isize, isize_negative_tests);
    };
    ($m:ident, $suffix:ident) => {
        pastey::paste! {
            $m!(i8, [<i8 _ $suffix _ nonnegative_tests>], [<i8 _ $suffix _ negative_tests>]);
            $m!(i16, [<i16 _ $suffix _ nonnegative_tests>], [<i16 _ $suffix _ negative_tests>]);
            $m!(i32, [<i32 _ $suffix _ nonnegative_tests>], [<i32 _ $suffix _ negative_tests>]);
            $m!(i64, [<i64 _ $suffix _ nonnegative_tests>], [<i64 _ $suffix _ negative_tests>]);
            $m!(i128, [<i128 _ $suffix _ nonnegative_tests>], [<i128 _ $suffix _ negative_tests>]);
            $m!(isize, [<isize _ $suffix _ nonnegative_tests>], [<isize _ $suffix _ negative_tests>]);
        }
    };
}

macro_rules! for_wrapping_signed_ints {
    ($m:ident) => {
        $m!(::core::num::Wrapping<i8>, wrapping_i8_nonnegative_tests, wrapping_i8_negative_tests);
        $m!(::core::num::Wrapping<i16>, wrapping_i16_nonnegative_tests, wrapping_i16_negative_tests);
        $m!(::core::num::Wrapping<i32>, wrapping_i32_nonnegative_tests, wrapping_i32_negative_tests);
        $m!(::core::num::Wrapping<i64>, wrapping_i64_nonnegative_tests, wrapping_i64_negative_tests);
        $m!(::core::num::Wrapping<i128>, wrapping_i128_nonnegative_tests, wrapping_i128_negative_tests);
        $m!(::core::num::Wrapping<isize>, wrapping_isize_nonnegative_tests, wrapping_isize_negative_tests);
    };
    ($m:ident, $suffix:ident) => {
        pastey::paste! {
            $m!(::core::num::Wrapping<i8>, [<wrapping _ i8 _ $suffix _ nonnegative_tests>], [<wrapping _ i8 _ $suffix _ negative_tests>]);
            $m!(::core::num::Wrapping<i16>, [<wrapping _ i16 _ $suffix _ nonnegative_tests>], [<wrapping _ i16 _ $suffix _ negative_tests>]);
            $m!(::core::num::Wrapping<i32>, [<wrapping _ i32 _ $suffix _ nonnegative_tests>], [<wrapping _ i32 _ $suffix _ negative_tests>]);
            $m!(::core::num::Wrapping<i64>, [<wrapping _ i64 _ $suffix _ nonnegative_tests>], [<wrapping _ i64 _ $suffix _ negative_tests>]);
            $m!(::core::num::Wrapping<i128>, [<wrapping _ i128 _ $suffix _ nonnegative_tests>], [<wrapping _ i128 _ $suffix _ negative_tests>]);
            $m!(::core::num::Wrapping<isize>, [<wrapping _ isize _ $suffix _ nonnegative_tests>], [<wrapping _ isize _ $suffix _ negative_tests>]);
        }
    };
    ($m:ident, $($prefix:tt)+) => {
        $m!($($prefix)+ ::core::num::Wrapping<i8>, wrapping_i8_nonnegative_tests, wrapping_i8_negative_tests);
        $m!($($prefix)+ ::core::num::Wrapping<i16>, wrapping_i16_nonnegative_tests, wrapping_i16_negative_tests);
        $m!($($prefix)+ ::core::num::Wrapping<i32>, wrapping_i32_nonnegative_tests, wrapping_i32_negative_tests);
        $m!($($prefix)+ ::core::num::Wrapping<i64>, wrapping_i64_nonnegative_tests, wrapping_i64_negative_tests);
        $m!($($prefix)+ ::core::num::Wrapping<i128>, wrapping_i128_nonnegative_tests, wrapping_i128_negative_tests);
        $m!($($prefix)+ ::core::num::Wrapping<isize>, wrapping_isize_nonnegative_tests, wrapping_isize_negative_tests);
    };
}

macro_rules! for_saturating_signed_ints {
    ($m:ident) => {
        $m!(::core::num::Saturating<i8>, saturating_i8_nonnegative_tests, saturating_i8_negative_tests);
        $m!(::core::num::Saturating<i16>, saturating_i16_nonnegative_tests, saturating_i16_negative_tests);
        $m!(::core::num::Saturating<i32>, saturating_i32_nonnegative_tests, saturating_i32_negative_tests);
        $m!(::core::num::Saturating<i64>, saturating_i64_nonnegative_tests, saturating_i64_negative_tests);
        $m!(::core::num::Saturating<i128>, saturating_i128_nonnegative_tests, saturating_i128_negative_tests);
        $m!(::core::num::Saturating<isize>, saturating_isize_nonnegative_tests, saturating_isize_negative_tests);
    };
    ($m:ident, $suffix:ident) => {
        pastey::paste! {
            $m!(::core::num::Saturating<i8>, [<saturating _ i8 _ $suffix _ nonnegative_tests>], [<saturating _ i8 _ $suffix _ negative_tests>]);
            $m!(::core::num::Saturating<i16>, [<saturating _ i16 _ $suffix _ nonnegative_tests>], [<saturating _ i16 _ $suffix _ negative_tests>]);
            $m!(::core::num::Saturating<i32>, [<saturating _ i32 _ $suffix _ nonnegative_tests>], [<saturating _ i32 _ $suffix _ negative_tests>]);
            $m!(::core::num::Saturating<i64>, [<saturating _ i64 _ $suffix _ nonnegative_tests>], [<saturating _ i64 _ $suffix _ negative_tests>]);
            $m!(::core::num::Saturating<i128>, [<saturating _ i128 _ $suffix _ nonnegative_tests>], [<saturating _ i128 _ $suffix _ negative_tests>]);
            $m!(::core::num::Saturating<isize>, [<saturating _ isize _ $suffix _ nonnegative_tests>], [<saturating _ isize _ $suffix _ negative_tests>]);
        }
    };
    ($m:ident, $($prefix:tt)+) => {
        $m!($($prefix)+ ::core::num::Saturating<i8>, saturating_i8_nonnegative_tests, saturating_i8_negative_tests);
        $m!($($prefix)+ ::core::num::Saturating<i16>, saturating_i16_nonnegative_tests, saturating_i16_negative_tests);
        $m!($($prefix)+ ::core::num::Saturating<i32>, saturating_i32_nonnegative_tests, saturating_i32_negative_tests);
        $m!($($prefix)+ ::core::num::Saturating<i64>, saturating_i64_nonnegative_tests, saturating_i64_negative_tests);
        $m!($($prefix)+ ::core::num::Saturating<i128>, saturating_i128_nonnegative_tests, saturating_i128_negative_tests);
        $m!($($prefix)+ ::core::num::Saturating<isize>, saturating_isize_nonnegative_tests, saturating_isize_negative_tests);
    };
}

macro_rules! for_all_signed_ints {
    ($m:ident) => {
        for_signed_ints!($m);
        for_wrapping_signed_ints!($m);
        for_saturating_signed_ints!($m);
    };
    ($m:ident, $suffix:ident) => {
        for_signed_ints!($m, $suffix);
        for_wrapping_signed_ints!($m, $suffix);
        for_saturating_signed_ints!($m, $suffix);
    };
    ($m:ident, $($prefix:tt)+) => {
        for_signed_ints!($m);
        for_wrapping_signed_ints!($m, $($prefix)+);
        for_saturating_signed_ints!($m, $($prefix)+);
    };
}

macro_rules! for_unsigned_ints {
    ($m:ident) => {
        $m!(u8, u8_tests);
        $m!(u16, u16_tests);
        $m!(u32, u32_tests);
        $m!(u64, u64_tests);
        $m!(u128, u128_tests);
        $m!(usize, usize_tests);
    };
    ($m:ident, $suffix:ident) => {
        pastey::paste! {
            $m!(u8, [<u8 _ $suffix _ tests>]);
            $m!(u16, [<u16 _ $suffix _ tests>]);
            $m!(u32, [<u32 _ $suffix _ tests>]);
            $m!(u64, [<u64 _ $suffix _ tests>]);
            $m!(u128, [<u128 _ $suffix _ tests>]);
            $m!(usize, [<usize _ $suffix _ tests>]);
        }
    };
}

macro_rules! for_wrapping_unsigned_ints {
    ($m:ident) => {
        $m!(::core::num::Wrapping<u8>, wrapping_u8_tests);
        $m!(::core::num::Wrapping<u16>, wrapping_u16_tests);
        $m!(::core::num::Wrapping<u32>, wrapping_u32_tests);
        $m!(::core::num::Wrapping<u64>, wrapping_u64_tests);
        $m!(::core::num::Wrapping<u128>, wrapping_u128_tests);
        $m!(::core::num::Wrapping<usize>, wrapping_usize_tests);
    };
    ($m:ident, $suffix:ident) => {
        pastey::paste! {
            $m!(::core::num::Wrapping<u8>, [<wrapping _ u8 _ $suffix _ tests>]);
            $m!(::core::num::Wrapping<u16>, [<wrapping _ u16 _ $suffix _ tests>]);
            $m!(::core::num::Wrapping<u32>, [<wrapping _ u32 _ $suffix _ tests>]);
            $m!(::core::num::Wrapping<u64>, [<wrapping _ u64 _ $suffix _ tests>]);
            $m!(::core::num::Wrapping<u128>, [<wrapping _ u128 _ $suffix _ tests>]);
            $m!(::core::num::Wrapping<usize>, [<wrapping _ usize _ $suffix _ tests>]);
        }
    };
    ($m:ident, $($prefix:tt)+) => {
        $m!($($prefix)+ ::core::num::Wrapping<u8>, wrapping_u8_tests);
        $m!($($prefix)+ ::core::num::Wrapping<u16>, wrapping_u16_tests);
        $m!($($prefix)+ ::core::num::Wrapping<u32>, wrapping_u32_tests);
        $m!($($prefix)+ ::core::num::Wrapping<u64>, wrapping_u64_tests);
        $m!($($prefix)+ ::core::num::Wrapping<u128>, wrapping_u128_tests);
        $m!($($prefix)+ ::core::num::Wrapping<usize>, wrapping_usize_tests);
    };
}

macro_rules! for_saturating_unsigned_ints {
    ($m:ident) => {
        $m!(::core::num::Saturating<u8>, saturating_u8_tests);
        $m!(::core::num::Saturating<u16>, saturating_u16_tests);
        $m!(::core::num::Saturating<u32>, saturating_u32_tests);
        $m!(::core::num::Saturating<u64>, saturating_u64_tests);
        $m!(::core::num::Saturating<u128>, saturating_u128_tests);
        $m!(::core::num::Saturating<usize>, saturating_usize_tests);
    };
    ($m:ident, $suffix:ident) => {
        pastey::paste! {
            $m!(::core::num::Saturating<u8>, [<saturating _ u8 _ $suffix _ tests>]);
            $m!(::core::num::Saturating<u16>, [<saturating _ u16 _ $suffix _ tests>]);
            $m!(::core::num::Saturating<u32>, [<saturating _ u32 _ $suffix _ tests>]);
            $m!(::core::num::Saturating<u64>, [<saturating _ u64 _ $suffix _ tests>]);
            $m!(::core::num::Saturating<u128>, [<saturating _ u128 _ $suffix _ tests>]);
            $m!(::core::num::Saturating<usize>, [<saturating _ usize _ $suffix _ tests>]);
        }
    };
    ($m:ident, $($prefix:tt)+) => {
        $m!($($prefix)+ ::core::num::Saturating<u8>, saturating_u8_tests);
        $m!($($prefix)+ ::core::num::Saturating<u16>, saturating_u16_tests);
        $m!($($prefix)+ ::core::num::Saturating<u32>, saturating_u32_tests);
        $m!($($prefix)+ ::core::num::Saturating<u64>, saturating_u64_tests);
        $m!($($prefix)+ ::core::num::Saturating<u128>, saturating_u128_tests);
        $m!($($prefix)+ ::core::num::Saturating<usize>, saturating_usize_tests);
    };
}

macro_rules! for_all_unsigned_ints {
    ($m:ident) => {
        for_unsigned_ints!($m);
        for_wrapping_unsigned_ints!($m);
        for_saturating_unsigned_ints!($m);
    };
    ($m:ident, $suffix:ident) => {
        for_unsigned_ints!($m, $suffix);
        for_wrapping_unsigned_ints!($m, $suffix);
        for_saturating_unsigned_ints!($m, $suffix);
    };
    ($m:ident, $($prefix:tt)+) => {
        for_unsigned_ints!($m);
        for_wrapping_unsigned_ints!($m, $($prefix)+);
        for_saturating_unsigned_ints!($m, $($prefix)+);
    };
}

macro_rules! for_misc {
    ($m:ident) => {
        $m!(char);
        $m!(bool);
        $m!(());
    };
}

#[cfg(feature = "maplike")]
macro_rules! impl_maplike_traits_for_scalar {
    ($t:ident, $type:ident) => {
        impl<$t> ::maplike::abc::Container for $type<$t> {
            type Value = Self;
        }

        impl<$t> ::maplike::abc::Keyed for $type<$t> {
            type Key = usize;
        }

        impl<$t> ::maplike::ctors::WithOne<$type<$t>> for $type<$t> {
            #[inline(always)]
            fn with_one(value: Self) -> Self {
                value
            }
        }

        impl<$t> ::maplike::ops::ContainsKey<usize> for $type<$t> {
            #[inline(always)]
            fn contains_key(&self, index: &usize) -> bool {
                *index == 0
            }
        }

        impl<$t> ::maplike::ops::Get<usize> for $type<$t> {
            #[inline(always)]
            fn get(&self, index: &usize) -> Option<&Self> {
                if *index == 0 { Some(self) } else { None }
            }
        }

        impl<$t> ::maplike::ops::Set<usize> for $type<$t> {
            type Output = Option<Self>;

            #[inline(always)]
            fn set(&mut self, index: usize, value: Self) -> Option<Self> {
                assert_eq!(index, 0);
                Some(core::mem::replace(self, value))
            }
        }

        impl<$t> ::maplike::ops::SetOrExpandTo<usize> for $type<$t> {
            type Output = Option<Self>;

            #[inline(always)]
            fn set_or_expand_to(&mut self, index: usize, value: Self) -> Option<Self> {
                ::maplike::ops::Set::set(self, index, value)
            }
        }

        impl<$t> ::maplike::ops::Modify<usize> for $type<$t> {
            #[inline(always)]
            fn modify<F>(&mut self, index: &usize, f: F)
            where
                F: FnOnce(&mut Self),
            {
                assert_eq!(*index, 0);
                f(self)
            }
        }

        impl<$t> ::maplike::ops::Put<$type<$t>> for $type<$t> {
            #[inline(always)]
            fn put(&mut self, value: Self) -> Option<Self> {
                Some(core::mem::replace(self, value))
            }
        }

        impl<$t> ::maplike::ops::Len for $type<$t> {
            #[inline(always)]
            fn len(&self) -> usize {
                1
            }
        }

        impl<'a, $t: 'a> ::maplike::iter::Values<'a> for $type<$t> {
            type Values = core::iter::Once<&'a Self>;

            #[inline(always)]
            fn values(&'a self) -> Self::Values {
                core::iter::once(self)
            }
        }

        impl<$t> ::maplike::iter::IntoValues for $type<$t> {
            type IntoValues = core::iter::Once<Self>;

            #[inline(always)]
            fn into_values(self) -> Self::IntoValues {
                core::iter::once(self)
            }
        }

        impl<'a, $t: 'a> ::maplike::iter::Iter<'a, usize> for $type<$t> {
            type Iter = core::iter::Once<(usize, &'a Self)>;

            #[inline(always)]
            fn iter(&'a self) -> Self::Iter {
                core::iter::once((0, self))
            }
        }

        impl<$t> ::maplike::iter::IntoIter<usize> for $type<$t> {
            type IntoIter = core::iter::Enumerate<core::iter::Once<Self>>;

            #[inline(always)]
            fn into_iter(self) -> Self::IntoIter {
                core::iter::once(self).enumerate()
            }
        }
    };
}
