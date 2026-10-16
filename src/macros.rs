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

macro_rules! for_signed_int_seq {
    ($m:ident) => {
        $m!(i8, i16, i32, i64, i128, isize);
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

macro_rules! for_unsigned_int_seq {
    ($m:ident) => {
        $m!(u8, u16, u32, u64, u128, usize);
    };
}

macro_rules! for_wrapping_signed_ints {
    ($m:ident) => {
        $m!(Wrapping<i8>, wrapping_i8_nonnegative_tests, wrapping_i8_negative_tests);
        $m!(Wrapping<i16>, wrapping_i16_nonnegative_tests, wrapping_i16_negative_tests);
        $m!(Wrapping<i32>, wrapping_i32_nonnegative_tests, wrapping_i32_negative_tests);
        $m!(Wrapping<i64>, wrapping_i64_nonnegative_tests, wrapping_i64_negative_tests);
        $m!(Wrapping<i128>, wrapping_i128_nonnegative_tests, wrapping_i128_negative_tests);
        $m!(Wrapping<isize>, wrapping_isize_nonnegative_tests, wrapping_isize_negative_tests);
    };
    ($m:ident, $suffix:ident) => {
        pastey::paste! {
            $m!(Wrapping<i8>, [<wrapping _ i8 _ $suffix _ nonnegative_tests>], [<wrapping _ i8 _ $suffix _ negative_tests>]);
            $m!(Wrapping<i16>, [<wrapping _ i16 _ $suffix _ nonnegative_tests>], [<wrapping _ i16 _ $suffix _ negative_tests>]);
            $m!(Wrapping<i32>, [<wrapping _ i32 _ $suffix _ nonnegative_tests>], [<wrapping _ i32 _ $suffix _ negative_tests>]);
            $m!(Wrapping<i64>, [<wrapping _ i64 _ $suffix _ nonnegative_tests>], [<wrapping _ i64 _ $suffix _ negative_tests>]);
            $m!(Wrapping<i128>, [<wrapping _ i128 _ $suffix _ nonnegative_tests>], [<wrapping _ i128 _ $suffix _ negative_tests>]);
            $m!(Wrapping<isize>, [<wrapping _ isize _ $suffix _ nonnegative_tests>], [<wrapping _ isize _ $suffix _ negative_tests>]);
        }
    };
}

macro_rules! for_wrapping_signed_int_seq {
    ($m:ident) => {
        $m!(
            Wrapping<i8>,
            Wrapping<i16>,
            Wrapping<i32>,
            Wrapping<i64>,
            Wrapping<i128>,
            Wrapping<isize>
        );
    };
    ($m:ident, $($prefix:tt)+) => {
        $m!(
            $($prefix)+
            Wrapping<i8>,
            Wrapping<i16>,
            Wrapping<i32>,
            Wrapping<i64>,
            Wrapping<i128>,
            Wrapping<isize>
        );
    };
}

macro_rules! for_saturating_signed_ints {
    ($m:ident) => {
        $m!(Saturating<i8>, saturating_i8_nonnegative_tests, saturating_i8_negative_tests);
        $m!(Saturating<i16>, saturating_i16_nonnegative_tests, saturating_i16_negative_tests);
        $m!(Saturating<i32>, saturating_i32_nonnegative_tests, saturating_i32_negative_tests);
        $m!(Saturating<i64>, saturating_i64_nonnegative_tests, saturating_i64_negative_tests);
        $m!(Saturating<i128>, saturating_i128_nonnegative_tests, saturating_i128_negative_tests);
        $m!(Saturating<isize>, saturating_isize_nonnegative_tests, saturating_isize_negative_tests);
    };
    ($m:ident, $suffix:ident) => {
        pastey::paste! {
            $m!(Saturating<i8>, [<saturating _ i8 _ $suffix _ nonnegative_tests>], [<saturating _ i8 _ $suffix _ negative_tests>]);
            $m!(Saturating<i16>, [<saturating _ i16 _ $suffix _ nonnegative_tests>], [<saturating _ i16 _ $suffix _ negative_tests>]);
            $m!(Saturating<i32>, [<saturating _ i32 _ $suffix _ nonnegative_tests>], [<saturating _ i32 _ $suffix _ negative_tests>]);
            $m!(Saturating<i64>, [<saturating _ i64 _ $suffix _ nonnegative_tests>], [<saturating _ i64 _ $suffix _ negative_tests>]);
            $m!(Saturating<i128>, [<saturating _ i128 _ $suffix _ nonnegative_tests>], [<saturating _ i128 _ $suffix _ negative_tests>]);
            $m!(Saturating<isize>, [<saturating _ isize _ $suffix _ nonnegative_tests>], [<saturating _ isize _ $suffix _ negative_tests>]);
        }
    };
}

macro_rules! for_saturating_signed_int_seq {
    ($m:ident) => {
        $m!(
            Saturating<i8>,
            Saturating<i16>,
            Saturating<i32>,
            Saturating<i64>,
            Saturating<i128>,
            Saturating<isize>
        );
    };
    ($m:ident, $($prefix:tt)+) => {
        $m!(
            $($prefix)+
            Saturating<i8>,
            Saturating<i16>,
            Saturating<i32>,
            Saturating<i64>,
            Saturating<i128>,
            Saturating<isize>
        );
    };
}

macro_rules! for_wrapping_unsigned_ints {
    ($m:ident) => {
        $m!(Wrapping<u8>, wrapping_u8_tests);
        $m!(Wrapping<u16>, wrapping_u16_tests);
        $m!(Wrapping<u32>, wrapping_u32_tests);
        $m!(Wrapping<u64>, wrapping_u64_tests);
        $m!(Wrapping<u128>, wrapping_u128_tests);
        $m!(Wrapping<usize>, wrapping_usize_tests);
    };
    ($m:ident, $suffix:ident) => {
        pastey::paste! {
            $m!(Wrapping<u8>, [<wrapping _ u8 _ $suffix _ tests>]);
            $m!(Wrapping<u16>, [<wrapping _ u16 _ $suffix _ tests>]);
            $m!(Wrapping<u32>, [<wrapping _ u32 _ $suffix _ tests>]);
            $m!(Wrapping<u64>, [<wrapping _ u64 _ $suffix _ tests>]);
            $m!(Wrapping<u128>, [<wrapping _ u128 _ $suffix _ tests>]);
            $m!(Wrapping<usize>, [<wrapping _ usize _ $suffix _ tests>]);
        }
    };
}

macro_rules! for_wrapping_unsigned_int_seq {
    ($m:ident) => {
        $m!(
            Wrapping<u8>,
            Wrapping<u16>,
            Wrapping<u32>,
            Wrapping<u64>,
            Wrapping<u128>,
            Wrapping<usize>
        );
    };
    ($m:ident, $($prefix:tt)+) => {
        $m!(
            $($prefix)+
            Wrapping<u8>,
            Wrapping<u16>,
            Wrapping<u32>,
            Wrapping<u64>,
            Wrapping<u128>,
            Wrapping<usize>
        );
    };
}

macro_rules! for_saturating_unsigned_ints {
    ($m:ident) => {
        $m!(Saturating<u8>, saturating_u8_tests);
        $m!(Saturating<u16>, saturating_u16_tests);
        $m!(Saturating<u32>, saturating_u32_tests);
        $m!(Saturating<u64>, saturating_u64_tests);
        $m!(Saturating<u128>, saturating_u128_tests);
        $m!(Saturating<usize>, saturating_usize_tests);
    };
    ($m:ident, $suffix:ident) => {
        pastey::paste! {
            $m!(Saturating<u8>, [<saturating _ u8 _ $suffix _ tests>]);
            $m!(Saturating<u16>, [<saturating _ u16 _ $suffix _ tests>]);
            $m!(Saturating<u32>, [<saturating _ u32 _ $suffix _ tests>]);
            $m!(Saturating<u64>, [<saturating _ u64 _ $suffix _ tests>]);
            $m!(Saturating<u128>, [<saturating _ u128 _ $suffix _ tests>]);
            $m!(Saturating<usize>, [<saturating _ usize _ $suffix _ tests>]);
        }
    };
}

macro_rules! for_saturating_unsigned_int_seq {
    ($m:ident) => {
        $m!(
            Saturating<u8>,
            Saturating<u16>,
            Saturating<u32>,
            Saturating<u64>,
            Saturating<u128>,
            Saturating<usize>
        );
    };
    ($m:ident, $($prefix:tt)+) => {
        $m!(
            $($prefix)+
            Saturating<u8>,
            Saturating<u16>,
            Saturating<u32>,
            Saturating<u64>,
            Saturating<u128>,
            Saturating<usize>
        );
    };
}
