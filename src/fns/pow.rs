// SPDX-FileCopyrightText: 2026 numlike contributors
//
// SPDX-License-Identifier: MIT OR Apache-2.0

/// Raises `self` to the power of `rhs`.
///
/// For integers, this uses exponentiation by squaring.
///
/// For floats, this may use an integer or floating-point power depending on
/// the exponent type. Using an integer power is generally faster than using
/// a floating-point power. It might have a different sequence of rounding
/// operations than a floating-point power, so the results are not
/// guaranteed to agree.
///
/// Note that the floating-point power is special in that it can return
/// non-NaN results for NaN inputs. For example, `f32::NAN.pow(0.0)` returns
/// `1.0`. However, if an input is a *signaling* NaN, then the result is
/// non-deterministically either a NaN or the result that the corresponding
/// quiet NaN would produce.
pub trait Pow<Rhs = Self> {
    /// The resulting type after applying the operation.
    type Output;

    /// Raises `self` to the power of `rhs`.
    fn pow(self, rhs: Rhs) -> Self::Output;
}

/// Checked exponentiation. Computes `self.pow(rhs)`, returning `None` if
/// overflow occurred or the result is not finite.
pub trait CheckedPow<Rhs = Self> {
    /// The resulting type after applying the operation.
    type Output;

    /// Checked exponentiation. Computes `self.pow(rhs)`, returning `None` if
    /// overflow occurred or the result is not finite.
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

#[cfg(any(feature = "std", feature = "libm"))]
macro_rules! impl_pow_for_float {
    ($t:ty, $rhs:ty, $desired_rhs:ty, $method:path) => {
        impl Pow<$rhs> for $t {
            type Output = $t;

            #[inline]
            fn pow(self, rhs: $rhs) -> $t {
                //<$t>::powi(
                //self,
                //rhs.try_into().expect("exponent out of range for `powi`"),
                //)
                $method(self, rhs as $desired_rhs)
            }
        }

        impl CheckedPow<$rhs> for $t {
            type Output = $t;

            #[inline]
            fn checked_pow(self, rhs: $rhs) -> Option<$t> {
                //let result = rhs.try_into().ok().map(|exp| <$t>::powi(self, exp))?;
                let result = $method(self, rhs as $desired_rhs);

                result.is_finite().then_some(result)
            }
        }
    };
}

#[cfg(any(feature = "std", feature = "libm"))]
macro_rules! impl_pow_for_float_for_all_rhs {
    ($t:ty, $int_pow:path, $int_rhs:ty, $float_pow:path) => {
        impl_pow_for_float!($t, i8, $int_rhs, $int_pow);
        impl_pow_for_float!($t, u8, $int_rhs, $int_pow);
        impl_pow_for_float!($t, i16, $int_rhs, $int_pow);
        impl_pow_for_float!($t, u16, $int_rhs, $int_pow);
        impl_pow_for_float!($t, i32, $int_rhs, $int_pow);
        impl_pow_for_float!($t, u32, $t, $float_pow);
        //impl_pow_for_float!($t, u32);
        //impl_pow_for_float!($t, i64);
        //impl_pow_for_float!($t, u64);
        //impl_pow_for_float!($t, i128);
        //impl_pow_for_float!($t, u128);
        //impl_pow_for_float!($t, isize);
        //impl_pow_for_float!($t, usize);
        impl_pow_for_float!($t, $t, $t, $float_pow);
    };
}

#[cfg(feature = "std")]
impl_pow_for_float_for_all_rhs!(f32, f32::powi, i32, f32::powf);
#[cfg(feature = "std")]
impl_pow_for_float_for_all_rhs!(f64, f64::powi, i32, f64::powf);
#[cfg(all(not(feature = "std"), feature = "libm"))]
impl_pow_for_float_for_all_rhs!(f32, libm::powf, f32, libm::powf);
#[cfg(all(not(feature = "std"), feature = "libm"))]
impl_pow_for_float_for_all_rhs!(f64, libm::pow, f64, libm::pow);
