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
