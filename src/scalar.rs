//! Scalar field implementation
//!
//! This module provides the implementation of the scalar field.

use core::{
    iter::{Product, Sum},
    ops::{Add, AddAssign, Mul, MulAssign, Neg, Sub, SubAssign},
};

use librelic_sys::{
    bn_st, wrapper_bn_add, wrapper_bn_add_assign, wrapper_bn_double, wrapper_bn_inv,
    wrapper_bn_is_odd, wrapper_bn_is_zero, wrapper_bn_mul, wrapper_bn_mul_assign, wrapper_bn_neg,
    wrapper_bn_read_bin, wrapper_bn_sub, wrapper_bn_sub_assign, wrapper_bn_t, wrapper_bn_write_bin,
    RLC_OK, RLC_POS,
};
use pairing::group::ff::{Field, PrimeField};
use subtle::{Choice, ConditionallySelectable, ConstantTimeEq, CtOption};

use crate::Error;
use rand_core::RngCore;

/// Reimplementation of `bn_make` to have a `const` version
#[inline]
const fn new_wrapper_with_v(v: u64) -> wrapper_bn_t {
    [bn_st {
        alloc: 34,
        used: 1,
        sign: RLC_POS,
        dp: [
            v, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0,
        ],
    }]
}

/// Reimplementation of `bn_make` to have a `const` version
#[inline]
const fn new_wrapper() -> wrapper_bn_t {
    [bn_st {
        alloc: 34,
        used: 1,
        sign: RLC_POS,
        dp: [0; 34],
    }]
}

/// Scalar in the prime field induced by the order of the elliptic curve groups
#[derive(Clone, Copy, Debug)]
#[repr(transparent)]
pub struct Scalar(pub(crate) wrapper_bn_t);

impl Scalar {
    const fn from_u64(v: u64) -> Self {
        Self(new_wrapper_with_v(v))
    }

    const fn from_u8(v: u8) -> Self {
        Self(new_wrapper_with_v(v as u64))
    }

    /// Obtain a representation of 1
    pub const fn one() -> Self {
        Self::ONE
    }

    /// Obtain a representation of 0
    pub const fn zero() -> Self {
        Self::ZERO
    }

    /// Encode scalar as bytes
    pub fn to_bytes(&self) -> [u8; 32] {
        let mut ret = [0u8; 32];
        unsafe {
            wrapper_bn_write_bin(ret.as_mut_ptr(), ret.len(), &self.0);
        }
        ret
    }

    /// Decode scalar from bytes (internal)
    const fn from_bytes_internal(
        bytes0: [u8; 8],
        bytes1: [u8; 8],
        bytes2: [u8; 8],
        bytes3: [u8; 8],
    ) -> Self {
        let mut bn = new_wrapper_with_v(u64::from_be_bytes(bytes3));
        bn[0].dp[3] = u64::from_be_bytes(bytes0);
        bn[0].dp[2] = u64::from_be_bytes(bytes1);
        bn[0].dp[1] = u64::from_be_bytes(bytes2);
        bn[0].used = 4;
        Self(bn)
    }

    /// Decode scalar from bytes
    pub fn from_bytes(bytes: &[u8; 32]) -> CtOption<Self> {
        CtOption::new(Self::from(bytes), 1.into())
    }

    /// Decode scalar from bytes and reduce modulo the order
    pub fn from_bytes_wide(bytes: &[u8; 64]) -> Self {
        let mut bn = new_wrapper();
        unsafe { wrapper_bn_read_bin(&mut bn, bytes.as_ptr(), bytes.len(), true) };
        bn.into()
    }
}

impl AsRef<Scalar> for Scalar {
    fn as_ref(&self) -> &Scalar {
        self
    }
}

impl Default for Scalar {
    fn default() -> Self {
        Scalar::ZERO
    }
}

impl From<wrapper_bn_t> for Scalar {
    #[inline(always)]
    fn from(value: wrapper_bn_t) -> Self {
        Self(value)
    }
}

impl From<&wrapper_bn_t> for Scalar {
    #[inline(always)]
    fn from(value: &wrapper_bn_t) -> Self {
        Self(*value)
    }
}

impl From<[u8; 32]> for Scalar {
    #[inline(always)]
    fn from(value: [u8; 32]) -> Self {
        Self::from(&value)
    }
}

impl From<&[u8; 32]> for Scalar {
    #[inline(always)]
    fn from(value: &[u8; 32]) -> Self {
        let mut bn = new_wrapper();
        unsafe { wrapper_bn_read_bin(&mut bn, value.as_ptr(), value.len(), false) };
        bn.into()
    }
}

impl From<Scalar> for wrapper_bn_t {
    fn from(value: Scalar) -> Self {
        value.0
    }
}

impl From<&Scalar> for wrapper_bn_t {
    fn from(value: &Scalar) -> Self {
        value.0
    }
}

impl From<Scalar> for [u8; 32] {
    fn from(value: Scalar) -> Self {
        Self::from(&value)
    }
}

impl From<&Scalar> for [u8; 32] {
    fn from(value: &Scalar) -> Self {
        value.to_bytes()
    }
}

impl From<u64> for Scalar {
    #[inline(always)]
    fn from(value: u64) -> Self {
        Self::from_u64(value)
    }
}

impl TryFrom<&[u8]> for Scalar {
    type Error = Error;

    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        let mut bn = new_wrapper();
        let ret = unsafe { wrapper_bn_read_bin(&mut bn, value.as_ptr(), value.len(), false) };
        if ret == RLC_OK {
            Ok(Self(bn))
        } else {
            Err(Error::RelicError(ret))
        }
    }
}

impl Add for Scalar {
    type Output = Scalar;

    fn add(mut self, rhs: Self) -> Self::Output {
        unsafe {
            wrapper_bn_add_assign(&mut self.0, &rhs.0);
        }
        self
    }
}

impl Add<&Scalar> for Scalar {
    type Output = Scalar;

    fn add(mut self, rhs: &Self) -> Self::Output {
        unsafe {
            wrapper_bn_add_assign(&mut self.0, &rhs.0);
        }
        self
    }
}

impl Add for &Scalar {
    type Output = Scalar;

    fn add(self, rhs: Self) -> Self::Output {
        let mut ret = new_wrapper();
        unsafe {
            wrapper_bn_add(&mut ret, &self.0, &rhs.0);
        }
        Scalar(ret)
    }
}

impl Add<Scalar> for &Scalar {
    type Output = Scalar;

    fn add(self, rhs: Scalar) -> Self::Output {
        rhs + self
    }
}

impl AddAssign for Scalar {
    fn add_assign(&mut self, rhs: Self) {
        unsafe {
            wrapper_bn_add_assign(&mut self.0, &rhs.0);
        }
    }
}

impl AddAssign<&Scalar> for Scalar {
    fn add_assign(&mut self, rhs: &Self) {
        unsafe {
            wrapper_bn_add_assign(&mut self.0, &rhs.0);
        }
    }
}

impl Neg for Scalar {
    type Output = Scalar;

    fn neg(mut self) -> Self::Output {
        unsafe {
            wrapper_bn_neg(&mut self.0);
        }
        self
    }
}

impl Sub for Scalar {
    type Output = Scalar;

    fn sub(mut self, rhs: Self) -> Self::Output {
        unsafe {
            wrapper_bn_sub_assign(&mut self.0, &rhs.0);
        }
        self
    }
}

impl Sub<&Scalar> for Scalar {
    type Output = Scalar;

    fn sub(mut self, rhs: &Self) -> Self::Output {
        unsafe {
            wrapper_bn_sub_assign(&mut self.0, &rhs.0);
        }
        self
    }
}

impl Sub for &Scalar {
    type Output = Scalar;

    fn sub(self, rhs: Self) -> Self::Output {
        let mut ret = new_wrapper();
        unsafe {
            wrapper_bn_sub(&mut ret, &self.0, &rhs.0);
        }
        Scalar(ret)
    }
}

impl Sub<Scalar> for &Scalar {
    type Output = Scalar;

    fn sub(self, rhs: Scalar) -> Self::Output {
        let mut ret = new_wrapper();
        unsafe {
            wrapper_bn_sub(&mut ret, &self.0, &rhs.0);
        }
        Scalar(ret)
    }
}

impl SubAssign for Scalar {
    fn sub_assign(&mut self, rhs: Self) {
        unsafe {
            wrapper_bn_sub_assign(&mut self.0, &rhs.0);
        }
    }
}

impl SubAssign<&Scalar> for Scalar {
    fn sub_assign(&mut self, rhs: &Self) {
        unsafe {
            wrapper_bn_sub_assign(&mut self.0, &rhs.0);
        }
    }
}

impl Mul for Scalar {
    type Output = Scalar;

    fn mul(mut self, rhs: Self) -> Self::Output {
        unsafe {
            wrapper_bn_mul_assign(&mut self.0, &rhs.0);
        }
        self
    }
}

impl Mul<&Scalar> for Scalar {
    type Output = Scalar;

    fn mul(mut self, rhs: &Self) -> Self::Output {
        unsafe {
            wrapper_bn_mul_assign(&mut self.0, &rhs.0);
        }
        self
    }
}

impl Mul for &Scalar {
    type Output = Scalar;

    fn mul(self, rhs: Self) -> Self::Output {
        let mut ret = new_wrapper();
        unsafe {
            wrapper_bn_mul(&mut ret, &self.0, &rhs.0);
        }
        Scalar(ret)
    }
}

impl Mul<Scalar> for &Scalar {
    type Output = Scalar;

    fn mul(self, rhs: Scalar) -> Self::Output {
        rhs * self
    }
}

impl MulAssign for Scalar {
    fn mul_assign(&mut self, rhs: Self) {
        unsafe {
            wrapper_bn_mul_assign(&mut self.0, &rhs.0);
        }
    }
}

impl MulAssign<&Scalar> for Scalar {
    fn mul_assign(&mut self, rhs: &Self) {
        unsafe {
            wrapper_bn_mul_assign(&mut self.0, &rhs.0);
        }
    }
}

impl Sum for Scalar {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        Self(iter.fold(new_wrapper(), |mut sum, v| {
            unsafe {
                wrapper_bn_add_assign(&mut sum, &v.0);
            }
            sum
        }))
    }
}

impl<'a> Sum<&'a Scalar> for Scalar {
    fn sum<I: Iterator<Item = &'a Self>>(iter: I) -> Self {
        Self(iter.fold(new_wrapper(), |mut sum, v| {
            unsafe {
                wrapper_bn_add_assign(&mut sum, &v.0);
            }
            sum
        }))
    }
}

impl Product for Scalar {
    fn product<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.fold(Self::ONE, |mut prod, v| {
            unsafe {
                wrapper_bn_mul_assign(&mut prod.0, &v.0);
            }
            prod
        })
    }
}

impl<'a> Product<&'a Scalar> for Scalar {
    fn product<I: Iterator<Item = &'a Self>>(iter: I) -> Self {
        iter.fold(Self::ONE, |mut prod, v| {
            unsafe {
                wrapper_bn_mul_assign(&mut prod.0, &v.0);
            }
            prod
        })
    }
}

impl ConstantTimeEq for Scalar {
    fn ct_eq(&self, other: &Self) -> Choice {
        let lhs: [u8; 32] = self.into();
        let rhs: [u8; 32] = other.into();
        lhs.ct_eq(&rhs)
    }
}

impl ConditionallySelectable for Scalar {
    fn conditional_select(a: &Self, b: &Self, choice: Choice) -> Self {
        let lhs: [u8; 32] = a.into();
        let rhs: [u8; 32] = b.into();
        Self::from(<[u8; 32]>::conditional_select(&lhs, &rhs, choice))
    }
}

impl PartialEq for Scalar {
    fn eq(&self, other: &Self) -> bool {
        let lhs: [u8; 32] = self.into();
        let rhs: [u8; 32] = other.into();
        lhs.eq(&rhs)
    }
}

impl Eq for Scalar {}

impl Field for Scalar {
    const ZERO: Self = Self::from_u8(0);

    const ONE: Self = Self::from_u8(1);

    fn random(mut rng: impl RngCore) -> Self {
        // oversample by 64 bits
        let mut bytes = [0u8; 40];
        rng.fill_bytes(&mut bytes);
        let mut bn = new_wrapper();
        unsafe {
            wrapper_bn_read_bin(&mut bn, bytes.as_ptr(), bytes.len(), true);
        }
        Scalar::from(bn)
    }

    fn square(&self) -> Self {
        let mut value = self.into();
        unsafe {
            wrapper_bn_mul_assign(&mut value, &value);
        }
        Self(value)
    }

    fn double(&self) -> Self {
        let mut ret = new_wrapper();
        unsafe {
            wrapper_bn_double(&mut ret, &self.0);
        }
        Self(ret)
    }

    fn invert(&self) -> CtOption<Self> {
        let mut value = new_wrapper();
        let ret = unsafe { wrapper_bn_inv(&mut value, &self.0) };
        CtOption::new(Self(value), ((ret == RLC_OK) as u8).into())
    }

    fn sqrt_ratio(_num: &Self, _div: &Self) -> (Choice, Self) {
        // TODO: implement
        unimplemented!("The wrapper has no use for this function.")
    }

    fn is_zero_vartime(&self) -> bool {
        unsafe { wrapper_bn_is_zero(&self.0) }
    }
}

impl PrimeField for Scalar {
    type Repr = [u8; 32];

    fn from_repr(repr: Self::Repr) -> CtOption<Self> {
        CtOption::new(Self::from(repr), 1.into())
    }

    fn to_repr(&self) -> Self::Repr {
        self.into()
    }

    fn is_odd(&self) -> Choice {
        Choice::from(unsafe { wrapper_bn_is_odd(&self.0) } as u8)
    }

    const MODULUS: &'static str =
        "0x73eda753299d7d483339d80809a1d80553bda402fffe5bfeffffffff00000001";

    const NUM_BITS: u32 = 255;

    const CAPACITY: u32 = 254;

    const TWO_INV: Self = Self::from_bytes_internal(
        [0x39, 0xf6, 0xd3, 0xa9, 0x94, 0xce, 0xbe, 0xa4],
        [0x19, 0x9c, 0xec, 0x04, 0x04, 0xd0, 0xec, 0x02],
        [0xa9, 0xde, 0xd2, 0x01, 0x7f, 0xff, 0x2d, 0xff],
        [0x7f, 0xff, 0xff, 0xff, 0x80, 0x00, 0x00, 0x01],
    );

    const MULTIPLICATIVE_GENERATOR: Self = Self::from_u8(7);

    const S: u32 = 32;

    const ROOT_OF_UNITY: Self = Self::from_bytes_internal(
        [0x16, 0xa2, 0xa1, 0x9e, 0xdf, 0xe8, 0x1f, 0x20],
        [0xd0, 0x9b, 0x68, 0x19, 0x22, 0xc8, 0x13, 0xb4],
        [0xb6, 0x36, 0x83, 0x50, 0x8c, 0x22, 0x80, 0xb9],
        [0x38, 0x29, 0x97, 0x1f, 0x43, 0x9f, 0x0d, 0x2b],
    );

    const ROOT_OF_UNITY_INV: Self = Self::from_bytes_internal(
        [0x05, 0x38, 0xa6, 0xf6, 0x6e, 0x19, 0xc6, 0x53],
        [0xed, 0x4f, 0x2f, 0x74, 0xa3, 0x5d, 0x01, 0x68],
        [0x6f, 0x67, 0xd4, 0xa2, 0xb5, 0x66, 0xf8, 0x33],
        [0x0f, 0xb4, 0xd6, 0xe1, 0x3c, 0xf1, 0x9a, 0x78],
    );

    const DELTA: Self = Self::from_bytes_internal(
        [0x08, 0x63, 0x4d, 0x0a, 0xa0, 0x21, 0xaa, 0xf8],
        [0x43, 0xca, 0xb3, 0x54, 0xfa, 0xbb, 0x00, 0x62],
        [0xf6, 0x50, 0x24, 0x37, 0xc6, 0xa0, 0x9c, 0x00],
        [0x6c, 0x08, 0x34, 0x79, 0x59, 0x01, 0x89, 0xd7],
    );
}

#[cfg(feature = "zeroize")]
impl zeroize::Zeroize for Scalar {
    fn zeroize(&mut self) {
        unsafe {
            wrapper_bn_zero(&mut self.0);
        }
    }
}

#[cfg(test)]
mod test {
    use librelic_sys::{wrapper_bn_one, wrapper_bn_zero};
    use pairing::group::ff::{Field, PrimeField};

    use crate::scalar::new_wrapper;

    use super::Scalar;

    #[test]
    fn from_u64() {
        assert_eq!(Scalar::from_u64(128), Scalar::from_u8(128));
    }

    #[test]
    fn zero() {
        let mut zero_relic = new_wrapper();
        unsafe {
            wrapper_bn_zero(&mut zero_relic);
        }
        let zero_relic = Scalar::from(zero_relic);

        let zero = Scalar::ZERO;
        assert_eq!(zero_relic, zero);
        assert!(zero.is_zero_vartime());
        assert_eq!(zero.is_zero().unwrap_u8(), 1);

        let scalar = Scalar::default();
        assert_eq!(scalar.invert().is_none().unwrap_u8(), 1);
        assert_eq!(scalar + scalar, Scalar::ZERO);
    }

    #[test]
    fn one() {
        let mut one_relic = new_wrapper();
        unsafe {
            wrapper_bn_one(&mut one_relic);
        }
        let one_relic = Scalar::from(one_relic);

        let one = Scalar::ONE;
        assert_eq!(one_relic, one);
        assert_eq!(one, Scalar::from_u64(1));

        assert_eq!(one.invert().is_some().unwrap_u8(), 1);
        assert_eq!(one * one, one);
    }

    #[test]
    fn two() {
        let two = Scalar::ONE.double();
        assert_eq!(two, Scalar::from_u64(2));
        assert_eq!(two.is_even().unwrap_u8(), 1);
        assert_eq!(two.is_odd().unwrap_u8(), 0);

        let two_inverse = two.invert().unwrap();
        assert_eq!(two_inverse, Scalar::TWO_INV);
        assert_eq!(two_inverse * two, Scalar::ONE);
    }

    #[test]
    fn root_of_unity() {
        assert_eq!(
            Scalar::ROOT_OF_UNITY * Scalar::ROOT_OF_UNITY_INV,
            Scalar::ONE
        );
    }
}
