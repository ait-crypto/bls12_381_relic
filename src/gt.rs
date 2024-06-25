//! Implementation of the target group `Gt`

use core::{
    iter::Sum,
    mem::MaybeUninit,
    ops::{Add, AddAssign, Mul, MulAssign, Neg, Sub, SubAssign},
};

use generic_array::{
    typenum::{Unsigned, U384, U576},
    GenericArray,
};
use librelic_sys::{
    wrapper_gt_add, wrapper_gt_add_assign, wrapper_gt_double, wrapper_gt_generator,
    wrapper_gt_init, wrapper_gt_is_equal, wrapper_gt_is_neutral, wrapper_gt_is_valid,
    wrapper_gt_mul, wrapper_gt_mul_assign, wrapper_gt_neg, wrapper_gt_neutral, wrapper_gt_read_bin,
    wrapper_gt_sub, wrapper_gt_sub_assign, wrapper_gt_t, wrapper_gt_write_bin, RLC_OK,
};
use pairing::group::{prime::PrimeGroup, Group, GroupEncoding, UncompressedEncoding};
use subtle::{Choice, CtOption};

use crate::{pair, Error, G1Projective, G2Projective, Scalar};
use rand_core::RngCore;

type CompressedSize = U384;
type UncompressedSize = U576;

const COMPRESSED_BYTES_SIZE: usize = CompressedSize::USIZE;
const UNCOMPRESSED_BYTES_SIZE: usize = UncompressedSize::USIZE;

#[inline]
pub(crate) fn new_wrapper() -> wrapper_gt_t {
    let mut gt = MaybeUninit::uninit();
    unsafe {
        wrapper_gt_init(gt.as_mut_ptr());
        gt.assume_init()
    }
}

/// Representation of an group element in the target group
#[derive(Clone, Copy, Debug)]
#[repr(transparent)]
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

impl TryFrom<[u8; UNCOMPRESSED_BYTES_SIZE]> for Gt {
    type Error = Error;

    #[inline]
    fn try_from(value: [u8; UNCOMPRESSED_BYTES_SIZE]) -> Result<Self, Self::Error> {
        Self::try_from(&value)
    }
}

impl TryFrom<&[u8; UNCOMPRESSED_BYTES_SIZE]> for Gt {
    type Error = Error;

    fn try_from(value: &[u8; UNCOMPRESSED_BYTES_SIZE]) -> Result<Self, Self::Error> {
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

impl TryFrom<[u8; COMPRESSED_BYTES_SIZE]> for Gt {
    type Error = Error;

    #[inline]
    fn try_from(value: [u8; COMPRESSED_BYTES_SIZE]) -> Result<Self, Self::Error> {
        Self::try_from(&value)
    }
}

impl TryFrom<&[u8; COMPRESSED_BYTES_SIZE]> for Gt {
    type Error = Error;

    fn try_from(value: &[u8; COMPRESSED_BYTES_SIZE]) -> Result<Self, Self::Error> {
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

impl From<Gt> for [u8; UNCOMPRESSED_BYTES_SIZE] {
    fn from(value: Gt) -> Self {
        Self::from(&value)
    }
}

impl From<&Gt> for [u8; UNCOMPRESSED_BYTES_SIZE] {
    fn from(value: &Gt) -> Self {
        let mut ret = [0u8; UNCOMPRESSED_BYTES_SIZE];
        unsafe {
            wrapper_gt_write_bin(ret.as_mut_ptr(), ret.len(), &value.0, false);
        }
        ret
    }
}

impl From<Gt> for [u8; COMPRESSED_BYTES_SIZE] {
    fn from(value: Gt) -> Self {
        Self::from(&value)
    }
}

impl From<&Gt> for [u8; COMPRESSED_BYTES_SIZE] {
    fn from(value: &Gt) -> Self {
        let mut ret = [0u8; COMPRESSED_BYTES_SIZE];
        unsafe {
            wrapper_gt_write_bin(ret.as_mut_ptr(), ret.len(), &value.0, true);
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

impl<G> Add<G> for Gt
where
    G: AsRef<Self>,
{
    type Output = Self;

    #[inline]
    fn add(mut self, rhs: G) -> Self::Output {
        let rhs = rhs.as_ref();
        unsafe {
            wrapper_gt_add_assign(&mut self.0, &rhs.0);
        }
        self
    }
}

impl<G> Add<G> for &Gt
where
    G: AsRef<Gt>,
{
    type Output = Gt;

    #[inline]
    fn add(self, rhs: G) -> Self::Output {
        let rhs = rhs.as_ref();
        let mut ret = new_wrapper();
        unsafe {
            wrapper_gt_add(&mut ret, &self.0, &rhs.0);
        }
        ret.into()
    }
}

impl<G> AddAssign<G> for Gt
where
    G: AsRef<Gt>,
{
    #[inline]
    fn add_assign(&mut self, rhs: G) {
        let rhs = rhs.as_ref();
        unsafe {
            wrapper_gt_add_assign(&mut self.0, &rhs.0);
        }
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

impl<G> Sub<G> for Gt
where
    G: AsRef<Self>,
{
    type Output = Self;

    #[inline]
    fn sub(mut self, rhs: G) -> Self::Output {
        let rhs = rhs.as_ref();
        unsafe {
            wrapper_gt_sub_assign(&mut self.0, &rhs.0);
        }
        self
    }
}

impl<G> Sub<G> for &Gt
where
    G: AsRef<Gt>,
{
    type Output = Gt;

    #[inline]
    fn sub(self, rhs: G) -> Self::Output {
        let rhs = rhs.as_ref();
        let mut ret = new_wrapper();
        unsafe {
            wrapper_gt_sub(&mut ret, &self.0, &rhs.0);
        }
        ret.into()
    }
}

impl<G> SubAssign<G> for Gt
where
    G: AsRef<Self>,
{
    #[inline]
    fn sub_assign(&mut self, rhs: G) {
        let rhs = rhs.as_ref();
        unsafe {
            wrapper_gt_sub_assign(&mut self.0, &rhs.0);
        }
    }
}

impl<G> Sum<G> for Gt
where
    G: AsRef<Self>,
{
    fn sum<I: Iterator<Item = G>>(iter: I) -> Self {
        let mut start = new_wrapper();
        unsafe {
            wrapper_gt_neutral(&mut start);
        }
        Self(iter.fold(start, |mut sum, v| {
            let v = v.as_ref();
            unsafe {
                wrapper_gt_add_assign(&mut sum, &v.0);
            }
            sum
        }))
    }
}

impl<S> Mul<S> for Gt
where
    S: AsRef<Scalar>,
{
    type Output = Self;

    #[inline]
    fn mul(mut self, rhs: S) -> Self::Output {
        let rhs = rhs.as_ref();
        unsafe {
            wrapper_gt_mul_assign(&mut self.0, &rhs.0);
        }
        self
    }
}

impl<S> Mul<S> for &Gt
where
    S: AsRef<Scalar>,
{
    type Output = Gt;

    #[inline]
    fn mul(self, rhs: S) -> Self::Output {
        let rhs = rhs.as_ref();
        let mut gt = new_wrapper();
        unsafe {
            wrapper_gt_mul(&mut gt, &self.0, &rhs.0);
        }
        Gt(gt)
    }
}

impl Mul<Gt> for Scalar {
    type Output = Gt;

    #[inline]
    fn mul(self, rhs: Gt) -> Self::Output {
        rhs * self
    }
}

impl Mul<&Gt> for Scalar {
    type Output = Gt;

    #[inline]
    fn mul(self, rhs: &Gt) -> Self::Output {
        rhs * self
    }
}

impl Mul<Gt> for &Scalar {
    type Output = Gt;

    #[inline]
    fn mul(self, rhs: Gt) -> Self::Output {
        rhs * self
    }
}

impl Mul<&Gt> for &Scalar {
    type Output = Gt;

    #[inline]
    fn mul(self, rhs: &Gt) -> Self::Output {
        rhs * self
    }
}

impl<S> MulAssign<S> for Gt
where
    S: AsRef<Scalar>,
{
    #[inline]
    fn mul_assign(&mut self, rhs: S) {
        let rhs = rhs.as_ref();
        unsafe {
            wrapper_gt_mul_assign(&mut self.0, &rhs.0);
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

impl GroupEncoding for Gt {
    type Repr = GenericArray<u8, CompressedSize>;

    fn from_bytes(bytes: &Self::Repr) -> CtOption<Self> {
        let mut wrapper = new_wrapper();
        let is_valid = unsafe { wrapper_gt_read_bin(&mut wrapper, bytes.as_ptr(), bytes.len()) }
            == RLC_OK
            && unsafe { wrapper_gt_is_valid(&wrapper) };
        CtOption::new(Self(wrapper), (is_valid as u8).into())
    }

    fn from_bytes_unchecked(bytes: &Self::Repr) -> CtOption<Self> {
        let mut wrapper = new_wrapper();
        let is_valid =
            unsafe { wrapper_gt_read_bin(&mut wrapper, bytes.as_ptr(), bytes.len()) } == RLC_OK;
        CtOption::new(Self(wrapper), (is_valid as u8).into())
    }

    #[inline]
    fn to_bytes(&self) -> Self::Repr {
        GenericArray::from_array(<[u8; COMPRESSED_BYTES_SIZE]>::from(self))
    }
}

impl UncompressedEncoding for Gt {
    type Uncompressed = GenericArray<u8, UncompressedSize>;

    fn from_uncompressed(bytes: &Self::Uncompressed) -> CtOption<Self> {
        let mut wrapper = new_wrapper();
        let is_valid = unsafe { wrapper_gt_read_bin(&mut wrapper, bytes.as_ptr(), bytes.len()) }
            == RLC_OK
            && unsafe { wrapper_gt_is_valid(&wrapper) };
        CtOption::new(Self(wrapper), (is_valid as u8).into())
    }

    fn from_uncompressed_unchecked(bytes: &Self::Uncompressed) -> CtOption<Self> {
        let mut wrapper = new_wrapper();
        let is_valid =
            unsafe { wrapper_gt_read_bin(&mut wrapper, bytes.as_ptr(), bytes.len()) } == RLC_OK;
        CtOption::new(Self(wrapper), (is_valid as u8).into())
    }

    fn to_uncompressed(&self) -> Self::Uncompressed {
        GenericArray::from_array(<[u8; UNCOMPRESSED_BYTES_SIZE]>::from(self))
    }
}

impl Group for Gt {
    type Scalar = Scalar;

    fn random(mut rng: impl RngCore) -> Self {
        pair(
            G1Projective::random(&mut rng),
            G2Projective::random(&mut rng),
        )
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
    use pairing::group::ff::Field;

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
        let check = v1 + v2;
        assert_eq!(check, v2 + v1);

        let rv1 = &v1;
        let rv2 = &v2;
        assert_eq!(check, v1 + rv2);
        assert_eq!(check, rv1 + v2);
    }

    #[test]
    fn sub() {
        let mut rng = rand::thread_rng();
        let v1 = Gt::random(&mut rng);
        let v2 = Gt::random(&mut rng);
        assert_eq!(v1 - v1, Gt::identity());
        let check = v1 - v2;

        let rv1 = &v1;
        let rv2 = &v2;
        assert_eq!(check, v1 - rv2);
        assert_eq!(check, rv1 - v2);
    }

    #[test]
    fn mul() {
        let mut rng = rand::thread_rng();
        let v = Gt::random(&mut rng);
        let s = Scalar::random(&mut rng);
        let check = v * s;

        let rv = &v;
        let rs = &s;
        assert_eq!(check, rv * s);
        assert_eq!(check, rv * rs);
        assert_eq!(check, v * rs);

        let mut mv = v;
        mv *= s;
        assert_eq!(check, mv);
    }

    #[test]
    fn bytes() {
        let mut rng = rand::thread_rng();
        let v1 = Gt::random(&mut rng);

        let v2 = Gt::from_bytes_unchecked(&v1.to_bytes()).unwrap();
        assert_eq!(v1, v2);
        let v2 = Gt::from_bytes(&v1.to_bytes()).unwrap();
        assert_eq!(v1, v2);
    }
}
