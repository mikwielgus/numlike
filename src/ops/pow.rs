// SPDX-FileCopyrightText: 2026 numlike contributors
//
// SPDX-License-Identifier: MIT OR Apache-2.0

pub trait Pow<Rhs> {
    type Output;

    fn pow(self, rhs: Rhs) -> Self::Output;
}

pub trait CheckedPow<Rhs> {
    type Output;

    fn checked_pow(self, rhs: Rhs) -> Option<Self::Output>;
}

macro_rules! impl_pow_for_int {
    ($t:ty, $rhs:ty) => {
        impl Pow<$rhs> for $t {
            type Output = $t;

            #[inline]
            fn pow(self, rhs: $rhs) -> $t {
                //<$t>::pow(
                //self,
                //rhs.try_into()
                //.expect("exponent out of range for integer `pow`"),
                //)
                <$t>::pow(self, u32::from(rhs))
            }
        }

        impl CheckedPow<$rhs> for $t {
            type Output = $t;

            #[inline]
            fn checked_pow(self, rhs: $rhs) -> Option<$t> {
                //rhs.try_into()
                //.ok()
                //.and_then(|exp| <$t>::checked_pow(self, exp))
                <$t>::checked_pow(self, u32::from(rhs))
            }
        }
    };
}

macro_rules! impl_pow_for_int_for_all_rhs {
    ($t:ty) => {
        impl_pow_for_int!($t, u8);
        impl_pow_for_int!($t, u16);
        impl_pow_for_int!($t, u32);
        //impl_pow_for_int!($t, u64);
        //impl_pow_for_int!($t, u128);
        //impl_pow_for_int!($t, usize);
        //impl_pow_for_int!($t, i8);
        //impl_pow_for_int!($t, i16);
        //impl_pow_for_int!($t, i32);
        //impl_pow_for_int!($t, i64);
        //impl_pow_for_int!($t, i128);
        //impl_pow_for_int!($t, isize);
    };
}

impl_pow_for_int_for_all_rhs!(u8);
impl_pow_for_int_for_all_rhs!(u16);
impl_pow_for_int_for_all_rhs!(u32);
impl_pow_for_int_for_all_rhs!(u64);
impl_pow_for_int_for_all_rhs!(u128);
impl_pow_for_int_for_all_rhs!(usize);
impl_pow_for_int_for_all_rhs!(i8);
impl_pow_for_int_for_all_rhs!(i16);
impl_pow_for_int_for_all_rhs!(i32);
impl_pow_for_int_for_all_rhs!(i64);
impl_pow_for_int_for_all_rhs!(i128);
impl_pow_for_int_for_all_rhs!(isize);

#[cfg(feature = "std")]
macro_rules! impl_std_pow_for_float {
    ($t:ty, $rhs:ty, $desired_rhs:ty, $method:expr) => {
        impl Pow<$rhs> for $t {
            type Output = $t;

            #[inline]
            fn pow(self, rhs: $rhs) -> $t {
                //<$t>::powi(
                //self,
                //rhs.try_into().expect("exponent out of range for `powi`"),
                //)
                ($method)(self, rhs as $desired_rhs)
            }
        }

        impl CheckedPow<$rhs> for $t {
            type Output = $t;

            #[inline]
            fn checked_pow(self, rhs: $rhs) -> Option<$t> {
                //let result = rhs.try_into().ok().map(|exp| <$t>::powi(self, exp))?;
                let result = ($method)(self, rhs as $desired_rhs);

                result.is_finite().then_some(result)
            }
        }
    };
}

#[cfg(feature = "std")]
macro_rules! impl_std_pow_for_float_for_all_rhs {
    ($t:ty) => {
        impl_std_pow_for_float!($t, i8, i32, <$t>::powi);
        impl_std_pow_for_float!($t, u8, i32, <$t>::powi);
        impl_std_pow_for_float!($t, i16, i32, <$t>::powi);
        impl_std_pow_for_float!($t, u16, i32, <$t>::powi);
        impl_std_pow_for_float!($t, i32, i32, <$t>::powi);
        impl_std_pow_for_float!($t, u32, $t, <$t>::powf);
        //impl_std_pow_for_float!($t, u32);
        //impl_std_pow_for_float!($t, i64);
        //impl_std_pow_for_float!($t, u64);
        //impl_std_pow_for_float!($t, i128);
        //impl_std_pow_for_float!($t, u128);
        //impl_std_pow_for_float!($t, isize);
        //impl_std_pow_for_float!($t, usize);
        impl_std_pow_for_float!($t, $t, $t, <$t>::powf);
    };
}

#[cfg(feature = "std")]
impl_std_pow_for_float_for_all_rhs!(f32);
#[cfg(feature = "std")]
impl_std_pow_for_float_for_all_rhs!(f64);

#[cfg(all(not(feature = "std"), feature = "libm"))]
macro_rules! impl_libm_pow_for_float {
    ($t:ty, $rhs:ty, $method:path, $desired_rhs:ty) => {
        impl Pow<$rhs> for $t {
            type Output = $t;

            #[inline]
            fn pow(self, rhs: $rhs) -> $t {
                $method(self, rhs as $desired_rhs)
            }
        }

        impl CheckedPow<$rhs> for $t {
            type Output = $t;

            #[inline]
            fn checked_pow(self, rhs: $rhs) -> Option<$t> {
                let result = $method(self, rhs as $desired_rhs);

                result.is_finite().then_some(result)
            }
        }
    };
}

#[cfg(all(not(feature = "std"), feature = "libm"))]
macro_rules! impl_libm_pow_for_float_for_all_rhs {
    ($t:ty, $method:path, $desired_rhs:ty) => {
        impl_libm_pow_for_float!($t, i8, $method, $desired_rhs);
        impl_libm_pow_for_float!($t, u8, $method, $desired_rhs);
        impl_libm_pow_for_float!($t, i16, $method, $desired_rhs);
        impl_libm_pow_for_float!($t, u16, $method, $desired_rhs);
        impl_libm_pow_for_float!($t, i32, $method, $desired_rhs);
        impl_libm_pow_for_float!($t, u32, $method, $desired_rhs);
        //impl_std_pow_for_float!($t, u32);
        //impl_std_pow_for_float!($t, i64);
        //impl_std_pow_for_float!($t, u64);
        //impl_std_pow_for_float!($t, i128);
        //impl_std_pow_for_float!($t, u128);
        //impl_std_pow_for_float!($t, isize);
        //impl_std_pow_for_float!($t, usize);
        impl_libm_pow_for_float!($t, $t, $method, $desired_rhs);
    };
}

#[cfg(all(not(feature = "std"), feature = "libm"))]
impl_libm_pow_for_float_for_all_rhs!(f32, libm::powf, f32);
#[cfg(all(not(feature = "std"), feature = "libm"))]
impl_libm_pow_for_float_for_all_rhs!(f64, libm::pow, f64);
