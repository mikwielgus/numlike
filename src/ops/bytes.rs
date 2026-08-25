// SPDX-FileCopyrightText: 2026 numlike contributors
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use std::mem::size_of;

pub trait ToBeBytes {
    type Bytes;

    fn to_be_bytes(self) -> Self::Bytes;
}

pub trait ToLeBytes {
    type Bytes;

    fn to_le_bytes(self) -> Self::Bytes;
}

pub trait ToNeBytes {
    type Bytes;

    fn to_ne_bytes(self) -> Self::Bytes;
}

pub trait FromBeBytes {
    type Bytes;

    fn from_be_bytes(bytes: Self::Bytes) -> Self;
}

pub trait FromLeBytes {
    type Bytes;

    fn from_le_bytes(bytes: Self::Bytes) -> Self;
}

pub trait FromNeBytes {
    type Bytes;

    fn from_ne_bytes(bytes: Self::Bytes) -> Self;
}

macro_rules! impl_bytes_traits {
    ($($ty:ty),*) => {
        $(
            impl ToBeBytes for $ty {
                type Bytes = [u8; size_of::<$ty>()];

                #[inline]
                fn to_be_bytes(self) -> Self::Bytes {
                    <$ty>::to_be_bytes(self)
                }
            }

            impl ToLeBytes for $ty {
                type Bytes = [u8; size_of::<$ty>()];

                #[inline]
                fn to_le_bytes(self) -> Self::Bytes {
                    <$ty>::to_le_bytes(self)
                }
            }

            impl ToNeBytes for $ty {
                type Bytes = [u8; size_of::<$ty>()];

                #[inline]
                fn to_ne_bytes(self) -> Self::Bytes {
                    <$ty>::to_ne_bytes(self)
                }
            }

            impl FromBeBytes for $ty {
                type Bytes = [u8; size_of::<$ty>()];

                #[inline]
                fn from_be_bytes(bytes: Self::Bytes) -> Self {
                    <$ty>::from_be_bytes(bytes)
                }
            }

            impl FromLeBytes for $ty {
                type Bytes = [u8; size_of::<$ty>()];

                #[inline]
                fn from_le_bytes(bytes: Self::Bytes) -> Self {
                    <$ty>::from_le_bytes(bytes)
                }
            }

            impl FromNeBytes for $ty {
                type Bytes = [u8; size_of::<$ty>()];

                #[inline]
                fn from_ne_bytes(bytes: Self::Bytes) -> Self {
                    <$ty>::from_ne_bytes(bytes)
                }
            }
        )*
    };
}

impl_bytes_traits!(i8, i16, i32, i64, i128, isize);
impl_bytes_traits!(u8, u16, u32, u64, u128, usize);
impl_bytes_traits!(f32, f64);
