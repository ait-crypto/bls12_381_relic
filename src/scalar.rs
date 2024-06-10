//! Scalar field implementation
//!
//! This module provides the implementation of the scalar field.

use core::{
    fmt,
    iter::{Product, Sum},
    mem::MaybeUninit,
    ops::{Add, AddAssign, Mul, MulAssign, Neg, Sub, SubAssign},
};

use librelic_sys::{
    wrapper_bn_add, wrapper_bn_add_assign, wrapper_bn_double, wrapper_bn_init, wrapper_bn_inv,
    wrapper_bn_is_odd, wrapper_bn_is_zero, wrapper_bn_mul, wrapper_bn_mul_assign, wrapper_bn_neg,
    wrapper_bn_one, wrapper_bn_read_bin, wrapper_bn_sub_assign, wrapper_bn_t, wrapper_bn_write_bin,
    wrapper_bn_zero, RLC_OK,
};
use pairing::group::ff::{Field, PrimeField};
use subtle::{Choice, ConditionallySelectable, ConstantTimeEq, CtOption};

use crate::Error;
use rand_core::RngCore;

fn new_wrapper() -> wrapper_bn_t {
    let mut bn = MaybeUninit::uninit();
    unsafe {
        wrapper_bn_init(bn.as_mut_ptr());
        bn.assume_init()
    }
}

/// Scalar in the prime field induced by the order of the elliptic curve groups
///
/// The interface is intended to be compatible with `bls12_381::Scalar`. The
/// variants are an implementation detail and may change without notice.
#[derive(Clone, Copy)]
#[allow(clippy::large_enum_variant)]
pub enum Scalar {
    /// Byte representation for const construction
    Bytes([u8; 32]),
    /// Relic-based representation
    Relic(wrapper_bn_t),
}

impl Scalar {
    const fn from_u64(v: u64) -> Self {
        let bytes = v.to_le_bytes();
        Scalar::Bytes([
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, bytes[7],
            bytes[6], bytes[5], bytes[4], bytes[3], bytes[2], bytes[1], bytes[0],
        ])
    }

    const fn from_u8(v: u8) -> Self {
        Scalar::Bytes([
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, v,
        ])
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
        From::from(self)
    }

    /// Decode scalar from bytes
    pub fn from_bytes(bytes: &[u8; 32]) -> CtOption<Self> {
        CtOption::new(Self::from(bytes), 1.into())
    }

    /// Decode scalar from bytes and reduce modulo the order
    pub fn from_bytes_wide(bytes: &[u8; 64]) -> Self {
        let mut bn = new_wrapper();
        unsafe { wrapper_bn_read_bin(&mut bn, bytes.as_ptr(), bytes.len(), true) };
        Self::Relic(bn)
    }
}

impl Default for Scalar {
    fn default() -> Self {
        Scalar::ZERO
    }
}

impl From<wrapper_bn_t> for Scalar {
    fn from(value: wrapper_bn_t) -> Self {
        Scalar::Relic(value)
    }
}

impl From<&wrapper_bn_t> for Scalar {
    fn from(value: &wrapper_bn_t) -> Self {
        Scalar::Relic(*value)
    }
}

impl From<[u8; 32]> for Scalar {
    fn from(value: [u8; 32]) -> Self {
        Scalar::Bytes(value)
    }
}

impl From<&[u8; 32]> for Scalar {
    fn from(value: &[u8; 32]) -> Self {
        Scalar::Bytes(*value)
    }
}

impl From<Scalar> for wrapper_bn_t {
    fn from(value: Scalar) -> Self {
        match value {
            Scalar::Relic(value) => value,
            Scalar::Bytes(ref bytes) => {
                let mut bn = new_wrapper();
                unsafe {
                    wrapper_bn_read_bin(&mut bn, bytes.as_ptr(), bytes.len(), false);
                }
                bn
            }
        }
    }
}

impl From<&Scalar> for wrapper_bn_t {
    fn from(value: &Scalar) -> Self {
        match value {
            Scalar::Relic(value) => *value,
            Scalar::Bytes(ref bytes) => {
                let mut bn = new_wrapper();
                unsafe {
                    wrapper_bn_read_bin(&mut bn, bytes.as_ptr(), bytes.len(), false);
                }
                bn
            }
        }
    }
}

impl From<Scalar> for [u8; 32] {
    fn from(value: Scalar) -> Self {
        match value {
            Scalar::Relic(ref value) => {
                let mut ret = [0u8; 32];
                unsafe {
                    wrapper_bn_write_bin(ret.as_mut_ptr(), ret.len(), value);
                }
                ret
            }
            Scalar::Bytes(bytes) => bytes,
        }
    }
}

impl From<&Scalar> for [u8; 32] {
    fn from(value: &Scalar) -> Self {
        match value {
            Scalar::Relic(value) => {
                let mut ret = [0u8; 32];
                unsafe {
                    wrapper_bn_write_bin(ret.as_mut_ptr(), ret.len(), value);
                }
                ret
            }
            Scalar::Bytes(bytes) => *bytes,
        }
    }
}

impl From<u64> for Scalar {
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
            Ok(Scalar::Relic(bn))
        } else {
            Err(Error::RelicError(ret))
        }
    }
}

impl Add for Scalar {
    type Output = Scalar;

    fn add(self, rhs: Self) -> Self::Output {
        let mut lhs = self.into();
        unsafe {
            wrapper_bn_add_assign(&mut lhs, &rhs.into());
        }
        Scalar::Relic(lhs)
    }
}

impl Add<&Scalar> for Scalar {
    type Output = Scalar;

    fn add(self, rhs: &Self) -> Self::Output {
        let mut lhs = self.into();
        match rhs {
            Scalar::Relic(rhs) => unsafe {
                wrapper_bn_add_assign(&mut lhs, rhs);
            },
            _ => {
                let rhs = rhs.into();
                unsafe {
                    wrapper_bn_add_assign(&mut lhs, &rhs);
                }
            }
        }
        Scalar::Relic(lhs)
    }
}

impl Add for &Scalar {
    type Output = Scalar;

    fn add(self, rhs: Self) -> Self::Output {
        let ret = match (self, rhs) {
            (Scalar::Relic(lhs), Scalar::Relic(rhs)) => {
                let mut ret = new_wrapper();
                unsafe {
                    wrapper_bn_add(&mut ret, lhs, rhs);
                }
                ret
            }
            (Scalar::Relic(ref lhs), _) => {
                let mut rhs = rhs.into();
                unsafe {
                    wrapper_bn_add_assign(&mut rhs, lhs);
                }
                rhs
            }
            (_, Scalar::Relic(rhs)) => {
                let mut lhs = self.into();
                unsafe {
                    wrapper_bn_add_assign(&mut lhs, rhs);
                }
                lhs
            }
            _ => {
                let mut lhs = self.into();
                let rhs = rhs.into();
                unsafe {
                    wrapper_bn_add_assign(&mut lhs, &rhs);
                }
                lhs
            }
        };
        Scalar::Relic(ret)
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
        let rhs = rhs.into();
        if let Scalar::Relic(ref mut lhs) = *self {
            unsafe {
                wrapper_bn_add_assign(lhs, &rhs);
            }
        } else {
            *self = Scalar::Relic({
                let mut lhs = (*self).into();
                unsafe {
                    wrapper_bn_add_assign(&mut lhs, &rhs);
                }
                lhs
            });
        }
    }
}

impl AddAssign<&Scalar> for Scalar {
    fn add_assign(&mut self, rhs: &Self) {
        if let Scalar::Relic(ref mut lhs) = *self {
            match rhs {
                Scalar::Relic(rhs) => unsafe {
                    wrapper_bn_add_assign(lhs, rhs);
                },
                _ => {
                    let rhs = rhs.into();
                    unsafe {
                        wrapper_bn_add_assign(lhs, &rhs);
                    }
                }
            };
        } else {
            *self = Scalar::Relic(match rhs {
                Scalar::Relic(rhs) => {
                    let mut lhs = (*self).into();
                    unsafe {
                        wrapper_bn_add_assign(&mut lhs, rhs);
                    }
                    lhs
                }
                _ => {
                    let mut lhs = (*self).into();
                    let rhs = rhs.into();
                    unsafe {
                        wrapper_bn_add_assign(&mut lhs, &rhs);
                    }
                    lhs
                }
            });
        }
    }
}

impl Neg for Scalar {
    type Output = Scalar;

    fn neg(self) -> Self::Output {
        let mut bn = self.into();
        unsafe {
            wrapper_bn_neg(&mut bn);
        }
        Scalar::Relic(bn)
    }
}

impl Sub for Scalar {
    type Output = Scalar;

    fn sub(self, rhs: Self) -> Self::Output {
        let mut lhs = self.into();
        unsafe {
            wrapper_bn_sub_assign(&mut lhs, &rhs.into());
        }
        Scalar::Relic(lhs)
    }
}

impl Sub<&Scalar> for Scalar {
    type Output = Scalar;

    fn sub(self, rhs: &Self) -> Self::Output {
        let mut lhs = self.into();
        match rhs {
            Scalar::Relic(rhs) => unsafe {
                wrapper_bn_sub_assign(&mut lhs, rhs);
            },
            _ => {
                let rhs = rhs.into();
                unsafe {
                    wrapper_bn_sub_assign(&mut lhs, &rhs);
                }
            }
        }
        Scalar::Relic(lhs)
    }
}

impl Sub for &Scalar {
    type Output = Scalar;

    fn sub(self, rhs: Self) -> Self::Output {
        let mut lhs = self.into();
        match rhs {
            Scalar::Relic(rhs) => unsafe {
                wrapper_bn_sub_assign(&mut lhs, rhs);
            },
            _ => {
                let rhs = rhs.into();
                unsafe {
                    wrapper_bn_sub_assign(&mut lhs, &rhs);
                }
            }
        }
        Scalar::Relic(lhs)
    }
}

impl Sub<Scalar> for &Scalar {
    type Output = Scalar;

    fn sub(self, rhs: Scalar) -> Self::Output {
        let mut lhs = self.into();
        let rhs = rhs.into();
        unsafe {
            wrapper_bn_sub_assign(&mut lhs, &rhs);
        }
        Scalar::Relic(lhs)
    }
}

impl SubAssign for Scalar {
    fn sub_assign(&mut self, rhs: Self) {
        let rhs = rhs.into();
        if let Scalar::Relic(ref mut lhs) = *self {
            unsafe {
                wrapper_bn_sub_assign(lhs, &rhs);
            }
        } else {
            *self = Scalar::Relic({
                let mut lhs = (*self).into();
                unsafe {
                    wrapper_bn_sub_assign(&mut lhs, &rhs);
                }
                lhs
            });
        }
    }
}

impl SubAssign<&Scalar> for Scalar {
    fn sub_assign(&mut self, rhs: &Self) {
        if let Scalar::Relic(ref mut lhs) = *self {
            match rhs {
                Scalar::Relic(rhs) => unsafe {
                    wrapper_bn_sub_assign(lhs, rhs);
                },
                _ => {
                    let rhs = rhs.into();
                    unsafe {
                        wrapper_bn_sub_assign(lhs, &rhs);
                    }
                }
            };
        } else {
            *self = Scalar::Relic(match rhs {
                Scalar::Relic(rhs) => {
                    let mut lhs = (*self).into();
                    unsafe {
                        wrapper_bn_sub_assign(&mut lhs, rhs);
                    }
                    lhs
                }
                _ => {
                    let mut lhs = (*self).into();
                    let rhs = rhs.into();
                    unsafe {
                        wrapper_bn_sub_assign(&mut lhs, &rhs);
                    }
                    lhs
                }
            });
        }
    }
}

impl Mul for Scalar {
    type Output = Scalar;

    fn mul(self, rhs: Self) -> Self::Output {
        let mut lhs = self.into();
        unsafe {
            wrapper_bn_mul_assign(&mut lhs, &rhs.into());
        }
        Scalar::Relic(lhs)
    }
}

impl Mul<&Scalar> for Scalar {
    type Output = Scalar;

    fn mul(self, rhs: &Self) -> Self::Output {
        let mut lhs = self.into();
        match rhs {
            Scalar::Relic(rhs) => unsafe {
                wrapper_bn_mul_assign(&mut lhs, rhs);
            },
            _ => {
                let rhs = rhs.into();
                unsafe {
                    wrapper_bn_mul_assign(&mut lhs, &rhs);
                }
            }
        }
        Scalar::Relic(lhs)
    }
}

impl Mul for &Scalar {
    type Output = Scalar;

    fn mul(self, rhs: Self) -> Self::Output {
        let ret = match (self, rhs) {
            (Scalar::Relic(lhs), Scalar::Relic(rhs)) => {
                let mut ret = new_wrapper();
                unsafe {
                    wrapper_bn_mul(&mut ret, lhs, rhs);
                }
                ret
            }
            (Scalar::Relic(ref lhs), _) => {
                let mut rhs = rhs.into();
                unsafe {
                    wrapper_bn_mul_assign(&mut rhs, lhs);
                }
                rhs
            }
            (_, Scalar::Relic(rhs)) => {
                let mut lhs = self.into();
                unsafe {
                    wrapper_bn_mul_assign(&mut lhs, rhs);
                }
                lhs
            }
            _ => {
                let mut lhs = self.into();
                let rhs = rhs.into();
                unsafe {
                    wrapper_bn_mul_assign(&mut lhs, &rhs);
                }
                lhs
            }
        };
        Scalar::Relic(ret)
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
        if let Scalar::Relic(ref mut lhs) = *self {
            match rhs {
                Scalar::Relic(ref rhs) => unsafe {
                    wrapper_bn_mul_assign(lhs, rhs);
                },
                _ => {
                    let rhs = rhs.into();
                    unsafe {
                        wrapper_bn_mul_assign(lhs, &rhs);
                    }
                }
            };
        } else {
            *self = Scalar::Relic(match rhs {
                Scalar::Relic(ref rhs) => {
                    let mut lhs = (*self).into();
                    unsafe {
                        wrapper_bn_mul_assign(&mut lhs, rhs);
                    }
                    lhs
                }
                _ => {
                    let mut lhs = (*self).into();
                    let rhs = rhs.into();
                    unsafe {
                        wrapper_bn_mul_assign(&mut lhs, &rhs);
                    }
                    lhs
                }
            });
        }
    }
}

impl MulAssign<&Scalar> for Scalar {
    fn mul_assign(&mut self, rhs: &Self) {
        if let Scalar::Relic(ref mut lhs) = *self {
            match rhs {
                Scalar::Relic(rhs) => unsafe {
                    wrapper_bn_mul_assign(lhs, rhs);
                },
                _ => {
                    let rhs = rhs.into();
                    unsafe {
                        wrapper_bn_mul_assign(lhs, &rhs);
                    }
                }
            };
        } else {
            *self = Scalar::Relic(match rhs {
                Scalar::Relic(rhs) => {
                    let mut lhs = (*self).into();
                    unsafe {
                        wrapper_bn_mul_assign(&mut lhs, rhs);
                    }
                    lhs
                }
                _ => {
                    let mut lhs = (*self).into();
                    let rhs = rhs.into();
                    unsafe {
                        wrapper_bn_mul_assign(&mut lhs, &rhs);
                    }
                    lhs
                }
            });
        }
    }
}

impl Sum for Scalar {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        let mut start = new_wrapper();
        unsafe {
            wrapper_bn_zero(&mut start);
        }
        Scalar::Relic(iter.fold(start, |mut sum, v| {
            let rhs = v.into();
            unsafe {
                wrapper_bn_add_assign(&mut sum, &rhs);
            }
            sum
        }))
    }
}

impl<'a> Sum<&'a Scalar> for Scalar {
    fn sum<I: Iterator<Item = &'a Self>>(iter: I) -> Self {
        let mut start = new_wrapper();
        unsafe {
            wrapper_bn_zero(&mut start);
        }
        Scalar::Relic(iter.fold(start, |mut sum, v| {
            match v {
                Scalar::Relic(rhs) => unsafe {
                    wrapper_bn_add_assign(&mut sum, rhs);
                },
                _ => {
                    let rhs = v.into();
                    unsafe {
                        wrapper_bn_add_assign(&mut sum, &rhs);
                    }
                }
            }
            sum
        }))
    }
}

impl Product for Scalar {
    fn product<I: Iterator<Item = Self>>(iter: I) -> Self {
        let mut start = new_wrapper();
        unsafe {
            wrapper_bn_one(&mut start);
        }
        Scalar::Relic(iter.fold(start, |mut prod, v| {
            let rhs = v.into();
            unsafe {
                wrapper_bn_mul_assign(&mut prod, &rhs);
            }
            prod
        }))
    }
}

impl<'a> Product<&'a Scalar> for Scalar {
    fn product<I: Iterator<Item = &'a Self>>(iter: I) -> Self {
        let mut start = new_wrapper();
        unsafe {
            wrapper_bn_one(&mut start);
        }
        Scalar::Relic(iter.fold(start, |mut prod, v| {
            match v {
                Scalar::Relic(rhs) => unsafe {
                    wrapper_bn_mul_assign(&mut prod, rhs);
                },
                _ => {
                    let rhs = v.into();
                    unsafe {
                        wrapper_bn_add_assign(&mut prod, &rhs);
                    }
                }
            }
            prod
        }))
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
        Scalar::Bytes(<[u8; 32]>::conditional_select(&lhs, &rhs, choice))
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

impl fmt::Debug for Scalar {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Bytes(bytes) => f.debug_tuple("Bytes").field(bytes).finish(),
            Self::Relic(_) => {
                let bytes: [u8; 32] = (*self).into();
                f.debug_tuple("Relic").field(&bytes).finish()
            }
        }
    }
}

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
        Scalar::Relic(value)
    }

    fn double(&self) -> Self {
        let value = match self {
            Scalar::Relic(bn) => {
                let mut ret = new_wrapper();
                unsafe {
                    wrapper_bn_double(&mut ret, bn);
                }
                ret
            }
            _ => {
                let mut value = self.into();
                unsafe {
                    wrapper_bn_double(&mut value, &value);
                }
                value
            }
        };

        Scalar::Relic(value)
    }

    fn invert(&self) -> CtOption<Self> {
        let mut value = self.into();
        let ret = unsafe { wrapper_bn_inv(&mut value) };
        CtOption::new(Scalar::Relic(value), ((ret == RLC_OK) as u8).into())
    }

    fn sqrt_ratio(_num: &Self, _div: &Self) -> (Choice, Self) {
        // TODO: implement
        unimplemented!()
    }

    fn is_zero_vartime(&self) -> bool {
        match self {
            Scalar::Bytes(bytes) => [0u8; 32] == *bytes,
            Scalar::Relic(bn) => unsafe { wrapper_bn_is_zero(bn) },
        }
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
        match self {
            Self::Bytes(bytes) => (bytes[31] & 1).into(),
            Self::Relic(bn) => Choice::from(unsafe { wrapper_bn_is_odd(bn) } as u8),
        }
    }

    const MODULUS: &'static str =
        "0x73eda753299d7d483339d80809a1d80553bda402fffe5bfeffffffff00000001";

    const NUM_BITS: u32 = 255;

    const CAPACITY: u32 = 254;

    const TWO_INV: Self = Self::Bytes([
        57, 246, 211, 169, 148, 206, 190, 164, 25, 156, 236, 4, 4, 208, 236, 2, 169, 222, 210, 1,
        127, 255, 45, 255, 127, 255, 255, 255, 128, 0, 0, 1,
    ]);

    const MULTIPLICATIVE_GENERATOR: Self = Self::from_u8(7);

    const S: u32 = 32;

    const ROOT_OF_UNITY: Self = Self::Bytes([
        0x16, 0xa2, 0xa1, 0x9e, 0xdf, 0xe8, 0x1f, 0x20, 0xd0, 0x9b, 0x68, 0x19, 0x22, 0xc8, 0x13,
        0xb4, 0xb6, 0x36, 0x83, 0x50, 0x8c, 0x22, 0x80, 0xb9, 0x38, 0x29, 0x97, 0x1f, 0x43, 0x9f,
        0x0d, 0x2b,
    ]);

    const ROOT_OF_UNITY_INV: Self = Self::Bytes([
        0x05, 0x38, 0xa6, 0xf6, 0x6e, 0x19, 0xc6, 0x53, 0xed, 0x4f, 0x2f, 0x74, 0xa3, 0x5d, 0x01,
        0x68, 0x6f, 0x67, 0xd4, 0xa2, 0xb5, 0x66, 0xf8, 0x33, 0x0f, 0xb4, 0xd6, 0xe1, 0x3c, 0xf1,
        0x9a, 0x78,
    ]);

    const DELTA: Self = Self::Bytes([
        0x08, 0x63, 0x4d, 0x0a, 0xa0, 0x21, 0xaa, 0xf8, 0x43, 0xca, 0xb3, 0x54, 0xfa, 0xbb, 0x00,
        0x62, 0xf6, 0x50, 0x24, 0x37, 0xc6, 0xa0, 0x9c, 0x00, 0x6c, 0x08, 0x34, 0x79, 0x59, 0x01,
        0x89, 0xd7,
    ]);
}

#[cfg(feature = "zeroize")]
impl zeroize::Zeroize for Scalar {
    fn zeroize(&mut self) {
        match self {
            Scalar::Bytes(ref mut bytes) => bytes.zeroize(),
            Scalar::Relic(ref mut bn) => unsafe { wrapper_bn_zero(bn) },
        }
    }
}

#[cfg(test)]
mod test {
    use librelic_sys::wrapper_bn_one;
    use pairing::group::ff::{Field, PrimeField};

    use crate::scalar::new_wrapper;

    use super::Scalar;

    #[test]
    fn from_u64() {
        assert_eq!(Scalar::from_u64(128), Scalar::from_u8(128));
    }

    #[test]
    fn zero() {
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
        assert_eq!(one_relic, one,);

        assert_eq!(one.invert().is_some().unwrap_u8(), 1);
        assert_eq!(one * one, one);
    }

    #[test]
    fn two() {
        let two = Scalar::ONE.double();
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
