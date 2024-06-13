//! Implementation of the target group `Gt`

use core::{
    fmt,
    iter::Sum,
    mem::MaybeUninit,
    ops::{Add, AddAssign, Mul, MulAssign, Neg, Sub, SubAssign},
};

use generic_array::{
    typenum::{Unsigned, U576},
    GenericArray,
};
use librelic_sys::{
    wrapper_gt_add, wrapper_gt_add_assign, wrapper_gt_double, wrapper_gt_generator,
    wrapper_gt_init, wrapper_gt_is_equal, wrapper_gt_is_neutral, wrapper_gt_is_valid,
    wrapper_gt_mul, wrapper_gt_mul_assign, wrapper_gt_neg, wrapper_gt_neutral, wrapper_gt_rand,
    wrapper_gt_read_bin, wrapper_gt_t, wrapper_gt_write_bin, RLC_OK,
};
use pairing::group::{prime::PrimeGroup, Group, GroupEncoding};
use subtle::{Choice, CtOption};

use crate::{Error, Scalar};
use rand_core::RngCore;

const BYTES_SIZE: usize = U576::USIZE;

#[inline]
pub(crate) fn new_wrapper() -> wrapper_gt_t {
    let mut gt = MaybeUninit::uninit();
    unsafe {
        wrapper_gt_init(gt.as_mut_ptr());
        gt.assume_init()
    }
}

/// Representation of an group element in the target group
#[derive(Clone, Copy)]
#[allow(clippy::large_enum_variant)]
pub struct Gt(pub(crate) wrapper_gt_t);

impl AsRef<Gt> for Gt {
    fn as_ref(&self) -> &Gt {
        self
    }
}

impl Default for Gt {
    fn default() -> Self {
        let mut value = new_wrapper();
        unsafe {
            wrapper_gt_neutral(&mut value);
        }
        Self(value)
    }
}

impl From<wrapper_gt_t> for Gt {
    #[inline]
    fn from(value: wrapper_gt_t) -> Self {
        Self(value)
    }
}

impl From<&wrapper_gt_t> for Gt {
    #[inline]
    fn from(value: &wrapper_gt_t) -> Self {
        Self(*value)
    }
}

impl TryFrom<[u8; BYTES_SIZE]> for Gt {
    type Error = Error;

    #[inline]
    fn try_from(value: [u8; BYTES_SIZE]) -> Result<Self, Self::Error> {
        Self::try_from(&value)
    }
}

impl TryFrom<&[u8; BYTES_SIZE]> for Gt {
    type Error = Error;

    fn try_from(value: &[u8; BYTES_SIZE]) -> Result<Self, Self::Error> {
        let mut gt = new_wrapper();
        let ret = unsafe { wrapper_gt_read_bin(&mut gt, value.as_ptr(), value.len()) };
        if ret == RLC_OK {
            if unsafe { wrapper_gt_is_valid(&gt) } {
                Ok(Self(gt))
            } else {
                Err(Error::InvalidBytesRepresentation)
            }
        } else {
            Err(Error::RelicError(ret))
        }
    }
}

impl From<Gt> for wrapper_gt_t {
    #[inline]
    fn from(value: Gt) -> Self {
        value.0
    }
}

impl From<&Gt> for wrapper_gt_t {
    #[inline]
    fn from(value: &Gt) -> Self {
        value.0
    }
}

impl From<Gt> for [u8; BYTES_SIZE] {
    fn from(value: Gt) -> Self {
        let mut ret = [0u8; BYTES_SIZE];
        unsafe {
            wrapper_gt_write_bin(ret.as_mut_ptr(), ret.len(), &value.0);
        }
        ret
    }
}

impl From<&Gt> for [u8; BYTES_SIZE] {
    fn from(value: &Gt) -> Self {
        let mut ret = [0u8; BYTES_SIZE];
        unsafe {
            wrapper_gt_write_bin(ret.as_mut_ptr(), ret.len(), &value.0);
        }
        ret
    }
}

impl TryFrom<&[u8]> for Gt {
    type Error = Error;

    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        let mut gt = new_wrapper();
        let ret = unsafe { wrapper_gt_read_bin(&mut gt, value.as_ptr(), value.len()) };
        if ret == RLC_OK {
            if unsafe { wrapper_gt_is_valid(&gt) } {
                Ok(Self(gt))
            } else {
                Err(Error::InvalidBytesRepresentation)
            }
        } else {
            Err(Error::RelicError(ret))
        }
    }
}

impl Add for Gt {
    type Output = Gt;

    #[inline]
    fn add(mut self, rhs: Self) -> Self::Output {
        unsafe {
            wrapper_gt_add_assign(&mut self.0, &rhs.into());
        }
        self
    }
}

impl Add<&Gt> for Gt {
    type Output = Gt;

    #[inline]
    fn add(mut self, rhs: &Self) -> Self::Output {
        unsafe { wrapper_gt_add_assign(&mut self.0, &rhs.0) };
        self
    }
}

impl Add for &Gt {
    type Output = Gt;

    #[inline]
    fn add(self, rhs: Self) -> Self::Output {
        let mut ret = new_wrapper();
        unsafe {
            wrapper_gt_add(&mut ret, &self.0, &rhs.0);
        }
        ret.into()
    }
}

impl Add<Gt> for &Gt {
    type Output = Gt;

    #[inline]
    fn add(self, rhs: Gt) -> Self::Output {
        rhs + self
    }
}

impl AddAssign for Gt {
    #[inline]
    fn add_assign(&mut self, rhs: Self) {
        unsafe { wrapper_gt_add_assign(&mut self.0, &rhs.0) };
    }
}

impl AddAssign<&Gt> for Gt {
    #[inline]
    fn add_assign(&mut self, rhs: &Self) {
        unsafe { wrapper_gt_add_assign(&mut self.0, &rhs.0) };
    }
}

impl Neg for Gt {
    type Output = Gt;

    #[inline]
    fn neg(mut self) -> Self::Output {
        unsafe {
            wrapper_gt_neg(&mut self.0);
        }
        self
    }
}

impl Neg for &Gt {
    type Output = Gt;

    #[inline]
    fn neg(self) -> Self::Output {
        let mut ret = self.into();
        unsafe {
            wrapper_gt_neg(&mut ret);
        }
        Gt(ret)
    }
}

impl Sub for Gt {
    type Output = Gt;

    #[inline]
    fn sub(self, rhs: Self) -> Self::Output {
        self + -rhs
    }
}

impl Sub<&Gt> for Gt {
    type Output = Gt;

    #[inline]
    fn sub(self, rhs: &Self) -> Self::Output {
        self + -rhs
    }
}

impl Sub for &Gt {
    type Output = Gt;

    #[inline]
    fn sub(self, rhs: Self) -> Self::Output {
        self + -rhs
    }
}

impl Sub<Gt> for &Gt {
    type Output = Gt;

    #[inline]
    fn sub(self, rhs: Gt) -> Self::Output {
        self + -rhs
    }
}

impl SubAssign for Gt {
    #[inline]
    fn sub_assign(&mut self, rhs: Self) {
        *self += -rhs;
    }
}

impl SubAssign<&Gt> for Gt {
    #[inline]
    fn sub_assign(&mut self, rhs: &Self) {
        *self += -rhs;
    }
}

impl Sum for Gt {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        let mut start = new_wrapper();
        unsafe {
            wrapper_gt_neutral(&mut start);
        }
        Self(iter.fold(start, |mut sum, v| {
            unsafe {
                wrapper_gt_add_assign(&mut sum, &v.0);
            }
            sum
        }))
    }
}

impl<'a> Sum<&'a Gt> for Gt {
    fn sum<I: Iterator<Item = &'a Self>>(iter: I) -> Self {
        let mut start = new_wrapper();
        unsafe {
            wrapper_gt_neutral(&mut start);
        }
        Self(iter.fold(start, |mut sum, v| {
            unsafe {
                wrapper_gt_add_assign(&mut sum, &v.0);
            }
            sum
        }))
    }
}

// TODO: Scalar * G!

impl Mul<Scalar> for Gt {
    type Output = Gt;

    #[inline]
    fn mul(self, rhs: Scalar) -> Self::Output {
        self * &rhs
    }
}

impl Mul<&Scalar> for Gt {
    type Output = Gt;

    fn mul(mut self, rhs: &Scalar) -> Self::Output {
        match rhs {
            Scalar::Relic(bn) => unsafe {
                wrapper_gt_mul_assign(&mut self.0, bn);
            },
            _ => {
                let bn = rhs.into();
                unsafe {
                    wrapper_gt_mul_assign(&mut self.0, &bn);
                }
            }
        }
        self
    }
}

impl Mul<Scalar> for &Gt {
    type Output = Gt;

    fn mul(self, rhs: Scalar) -> Self::Output {
        let mut gt = new_wrapper();
        match rhs {
            Scalar::Relic(bn) => unsafe {
                wrapper_gt_mul(&mut gt, &self.0, &bn);
            },
            _ => {
                let bn = rhs.into();
                unsafe {
                    wrapper_gt_mul(&mut gt, &self.0, &bn);
                }
            }
        }
        Gt(gt)
    }
}

impl Mul<&Scalar> for &Gt {
    type Output = Gt;

    fn mul(self, rhs: &Scalar) -> Self::Output {
        let mut gt = new_wrapper();
        match rhs {
            Scalar::Relic(bn) => unsafe {
                wrapper_gt_mul(&mut gt, &self.0, bn);
            },
            _ => {
                let bn = rhs.into();
                unsafe {
                    wrapper_gt_mul(&mut gt, &self.0, &bn);
                }
            }
        }
        Gt(gt)
    }
}

impl MulAssign<Scalar> for Gt {
    #[inline]
    fn mul_assign(&mut self, rhs: Scalar) {
        *self *= &rhs;
    }
}

impl MulAssign<&Scalar> for Gt {
    fn mul_assign(&mut self, rhs: &Scalar) {
        match rhs {
            Scalar::Relic(bn) => unsafe {
                wrapper_gt_mul_assign(&mut self.0, bn);
            },
            _ => {
                let bn = rhs.into();
                unsafe {
                    wrapper_gt_mul_assign(&mut self.0, &bn);
                }
            }
        }
    }
}

impl PartialEq for Gt {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        unsafe { wrapper_gt_is_equal(&self.0, &other.0) }
    }
}

impl Eq for Gt {}

impl fmt::Debug for Gt {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let bytes: [u8; BYTES_SIZE] = self.into();
        f.debug_tuple("Relic").field(&bytes).finish()
    }
}

impl GroupEncoding for Gt {
    // FIXME: use [u8; 576]
    type Repr = GenericArray<u8, U576>;

    fn from_bytes(bytes: &Self::Repr) -> CtOption<Self> {
        let mut wrapper = new_wrapper();
        if unsafe { wrapper_gt_read_bin(&mut wrapper, bytes.as_ptr(), bytes.len()) } == RLC_OK {
            CtOption::new(
                Self(wrapper),
                Choice::from(unsafe { wrapper_gt_is_valid(&wrapper) } as u8),
            )
        } else {
            CtOption::new(Self(wrapper), 0.into())
        }
    }

    fn from_bytes_unchecked(bytes: &Self::Repr) -> CtOption<Self> {
        let mut wrapper = new_wrapper();
        if unsafe { wrapper_gt_read_bin(&mut wrapper, bytes.as_ptr(), bytes.len()) } == RLC_OK {
            CtOption::new(Self(wrapper), 1.into())
        } else {
            CtOption::new(Self(wrapper), 0.into())
        }
    }

    #[inline]
    fn to_bytes(&self) -> Self::Repr {
        GenericArray::from_array(self.into())
    }
}

impl Group for Gt {
    type Scalar = Scalar;

    fn random(_rng: impl RngCore) -> Self {
        let mut gt = new_wrapper();
        unsafe {
            wrapper_gt_rand(&mut gt);
        }
        Self(gt)
    }

    #[inline]
    fn identity() -> Self {
        Self::default()
    }

    #[inline]
    fn generator() -> Self {
        let mut value = new_wrapper();
        unsafe {
            wrapper_gt_generator(&mut value);
        }
        Self(value)
    }

    #[inline]
    fn is_identity(&self) -> Choice {
        Choice::from(unsafe { wrapper_gt_is_neutral(&self.0) } as u8)
    }

    #[inline]
    fn double(&self) -> Self {
        let mut ret = new_wrapper();
        unsafe {
            wrapper_gt_double(&mut ret, &self.0);
        }
        Self(ret)
    }
}

impl PrimeGroup for Gt {}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn generator() {
        let generator = Gt::generator();
        let identity = Gt::identity();
        assert_ne!(generator, identity);
    }

    #[test]
    fn add() {
        let mut rng = rand::thread_rng();
        let v1 = Gt::random(&mut rng);
        let v2 = Gt::random(&mut rng);
        assert_eq!(v1 + v2, v2 + v1);
    }
}
