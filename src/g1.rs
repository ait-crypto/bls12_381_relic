//! Implementation of the first source group `G1`

use core::{
    iter::Sum,
    mem::MaybeUninit,
    ops::{Add, AddAssign, Mul, MulAssign, Neg, Sub, SubAssign},
};

#[cfg(feature = "alloc")]
extern crate alloc;
#[cfg(feature = "alloc")]
use alloc::vec::Vec;

use generic_array::{
    GenericArray,
    typenum::{U49, U97, Unsigned},
};
#[cfg(feature = "alloc")]
use librelic_sys::wrapper_g1_simmul;
use librelic_sys::{
    RLC_OK, wrapper_g1_add, wrapper_g1_add_assign, wrapper_g1_double, wrapper_g1_generator,
    wrapper_g1_hash_to_curve, wrapper_g1_init, wrapper_g1_is_equal, wrapper_g1_is_neutral,
    wrapper_g1_is_valid, wrapper_g1_mul, wrapper_g1_mul_assign, wrapper_g1_neg, wrapper_g1_neutral,
    wrapper_g1_norm, wrapper_g1_read_bin, wrapper_g1_sub, wrapper_g1_sub_assign, wrapper_g1_t,
    wrapper_g1_write_bin,
};
use pairing::group::{
    Curve, Group, GroupEncoding, UncompressedEncoding,
    prime::{PrimeCurve, PrimeGroup},
};
use rand_core::RngCore;
use subtle::{Choice, CtOption};

use crate::{Affine, Error, RANDOM_DOMAIN_SEPERATOR, Scalar, affine};

type CompressedSize = U49;
type UncompressedSize = U97;

const COMPRESSED_BYTES_SIZE: usize = CompressedSize::USIZE;
const UNCOMPRESSED_BYTES_SIZE: usize = UncompressedSize::USIZE;

#[inline]
fn new_wrapper() -> wrapper_g1_t {
    let mut g1 = MaybeUninit::uninit();
    unsafe {
        wrapper_g1_init(g1.as_mut_ptr());
        g1.assume_init()
    }
}

/// Representation of a G1 element
#[derive(Clone, Copy, Debug)]
#[repr(transparent)]
pub struct G1Projective(pub(crate) wrapper_g1_t);

impl G1Projective {
    /// Hash to a point on the curve.
    // TODO: make compatible with bls12-381 crate
    pub fn hash_to_curve(msg: impl AsRef<[u8]>, dst: &[u8]) -> Self {
        let mut g1 = new_wrapper();
        let msg = msg.as_ref();
        unsafe {
            wrapper_g1_hash_to_curve(&mut g1, msg.as_ptr(), msg.len(), dst.as_ptr(), dst.len());
        }
        g1.into()
    }
}

impl Default for G1Projective {
    fn default() -> Self {
        let mut value = new_wrapper();
        unsafe {
            wrapper_g1_neutral(&mut value);
        }
        Self(value)
    }
}

impl AsRef<G1Projective> for G1Projective {
    fn as_ref(&self) -> &G1Projective {
        self
    }
}

impl From<wrapper_g1_t> for G1Projective {
    #[inline]
    fn from(value: wrapper_g1_t) -> Self {
        Self(value)
    }
}

impl From<&wrapper_g1_t> for G1Projective {
    #[inline]
    fn from(value: &wrapper_g1_t) -> Self {
        Self(*value)
    }
}

impl TryFrom<[u8; UNCOMPRESSED_BYTES_SIZE]> for G1Projective {
    type Error = Error;

    #[inline]
    fn try_from(value: [u8; UNCOMPRESSED_BYTES_SIZE]) -> Result<Self, Self::Error> {
        Self::try_from(&value)
    }
}

impl TryFrom<&[u8; UNCOMPRESSED_BYTES_SIZE]> for G1Projective {
    type Error = Error;

    fn try_from(value: &[u8; UNCOMPRESSED_BYTES_SIZE]) -> Result<Self, Self::Error> {
        let mut g1 = new_wrapper();
        let ret = unsafe { wrapper_g1_read_bin(&mut g1, value.as_ptr(), value.len()) };
        if ret == RLC_OK {
            if unsafe { wrapper_g1_is_valid(&g1) } {
                Ok(Self(g1))
            } else {
                Err(Error::InvalidBytesRepresentation)
            }
        } else {
            Err(Error::RelicError(ret))
        }
    }
}

impl TryFrom<[u8; COMPRESSED_BYTES_SIZE]> for G1Projective {
    type Error = Error;

    #[inline]
    fn try_from(value: [u8; COMPRESSED_BYTES_SIZE]) -> Result<Self, Self::Error> {
        Self::try_from(&value)
    }
}

impl TryFrom<&[u8; COMPRESSED_BYTES_SIZE]> for G1Projective {
    type Error = Error;

    fn try_from(value: &[u8; COMPRESSED_BYTES_SIZE]) -> Result<Self, Self::Error> {
        let mut g1 = new_wrapper();
        let ret = unsafe { wrapper_g1_read_bin(&mut g1, value.as_ptr(), value.len()) };
        if ret == RLC_OK {
            if unsafe { wrapper_g1_is_valid(&g1) } {
                Ok(Self(g1))
            } else {
                Err(Error::InvalidBytesRepresentation)
            }
        } else {
            Err(Error::RelicError(ret))
        }
    }
}

impl From<G1Projective> for wrapper_g1_t {
    #[inline]
    fn from(value: G1Projective) -> Self {
        value.0
    }
}

impl From<&G1Projective> for wrapper_g1_t {
    #[inline]
    fn from(value: &G1Projective) -> Self {
        value.0
    }
}

impl From<G1Projective> for [u8; UNCOMPRESSED_BYTES_SIZE] {
    fn from(value: G1Projective) -> Self {
        Self::from(&value)
    }
}

impl From<&G1Projective> for [u8; UNCOMPRESSED_BYTES_SIZE] {
    fn from(value: &G1Projective) -> Self {
        let mut ret = [0u8; UNCOMPRESSED_BYTES_SIZE];
        unsafe {
            wrapper_g1_write_bin(ret.as_mut_ptr(), ret.len(), &value.0, false);
        }
        ret
    }
}

impl From<G1Projective> for [u8; COMPRESSED_BYTES_SIZE] {
    fn from(value: G1Projective) -> Self {
        Self::from(&value)
    }
}

impl From<&G1Projective> for [u8; COMPRESSED_BYTES_SIZE] {
    fn from(value: &G1Projective) -> Self {
        let mut ret = [0u8; COMPRESSED_BYTES_SIZE];
        unsafe {
            wrapper_g1_write_bin(ret.as_mut_ptr(), ret.len(), &value.0, true);
        }
        ret
    }
}

impl TryFrom<&[u8]> for G1Projective {
    type Error = Error;

    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        let mut g1 = new_wrapper();
        let ret = unsafe { wrapper_g1_read_bin(&mut g1, value.as_ptr(), value.len()) };
        if ret == RLC_OK {
            if unsafe { wrapper_g1_is_valid(&g1) } {
                Ok(Self(g1))
            } else {
                Err(Error::InvalidBytesRepresentation)
            }
        } else {
            Err(Error::RelicError(ret))
        }
    }
}

impl<G> Add<G> for G1Projective
where
    G: AsRef<Self>,
{
    type Output = Self;

    #[inline]
    fn add(mut self, rhs: G) -> Self::Output {
        let rhs = rhs.as_ref();
        unsafe {
            wrapper_g1_add_assign(&mut self.0, &rhs.0);
        }
        self
    }
}

impl<G> Add<G> for &G1Projective
where
    G: AsRef<G1Projective>,
{
    type Output = G1Projective;

    #[inline]
    fn add(self, rhs: G) -> Self::Output {
        let mut ret = new_wrapper();
        let rhs = rhs.as_ref();
        unsafe {
            wrapper_g1_add(&mut ret, &self.0, &rhs.0);
        }
        ret.into()
    }
}

impl<G> AddAssign<G> for G1Projective
where
    G: AsRef<G1Projective>,
{
    #[inline]
    fn add_assign(&mut self, rhs: G) {
        let rhs = rhs.as_ref();
        unsafe {
            wrapper_g1_add_assign(&mut self.0, &rhs.0);
        }
    }
}

impl Neg for G1Projective {
    type Output = G1Projective;

    #[inline]
    fn neg(mut self) -> Self::Output {
        unsafe {
            wrapper_g1_neg(&mut self.0);
        }
        self
    }
}

impl Neg for &G1Projective {
    type Output = G1Projective;

    #[inline]
    fn neg(self) -> Self::Output {
        let mut ret = self.into();
        unsafe {
            wrapper_g1_neg(&mut ret);
        }
        G1Projective(ret)
    }
}

impl<G> Sub<G> for G1Projective
where
    G: AsRef<Self>,
{
    type Output = Self;

    #[inline]
    fn sub(mut self, rhs: G) -> Self::Output {
        let rhs = rhs.as_ref();
        unsafe {
            wrapper_g1_sub_assign(&mut self.0, &rhs.into());
        }
        self
    }
}

impl<G> Sub<G> for &G1Projective
where
    G: AsRef<G1Projective>,
{
    type Output = G1Projective;

    #[inline]
    fn sub(self, rhs: G) -> Self::Output {
        let mut ret = new_wrapper();
        let rhs = rhs.as_ref();
        unsafe {
            wrapper_g1_sub(&mut ret, &self.0, &rhs.0);
        }
        ret.into()
    }
}

impl<G> SubAssign<G> for G1Projective
where
    G: AsRef<Self>,
{
    #[inline]
    fn sub_assign(&mut self, rhs: G) {
        let rhs = rhs.as_ref();
        unsafe {
            wrapper_g1_sub_assign(&mut self.0, &rhs.0);
        }
    }
}

impl<G> Sum<G> for G1Projective
where
    G: AsRef<Self>,
{
    fn sum<I: Iterator<Item = G>>(iter: I) -> Self {
        let mut start = new_wrapper();
        unsafe {
            wrapper_g1_neutral(&mut start);
        }
        Self(iter.fold(start, |mut sum, v| {
            let g1 = v.as_ref();
            unsafe {
                wrapper_g1_add_assign(&mut sum, &g1.0);
            }
            sum
        }))
    }
}

impl<S> Mul<S> for G1Projective
where
    S: AsRef<Scalar>,
{
    type Output = G1Projective;

    #[inline]
    fn mul(mut self, rhs: S) -> Self::Output {
        let rhs = rhs.as_ref();
        unsafe {
            wrapper_g1_mul_assign(&mut self.0, &rhs.0);
        }
        self
    }
}

impl<S> Mul<S> for &G1Projective
where
    S: AsRef<Scalar>,
{
    type Output = G1Projective;

    fn mul(self, rhs: S) -> Self::Output {
        let mut g1 = new_wrapper();
        let rhs = rhs.as_ref();
        unsafe {
            wrapper_g1_mul(&mut g1, &self.0, &rhs.0);
        }
        G1Projective(g1)
    }
}

impl Mul<G1Projective> for Scalar {
    type Output = G1Projective;

    #[inline]
    fn mul(self, rhs: G1Projective) -> Self::Output {
        rhs * self
    }
}

impl Mul<&G1Projective> for Scalar {
    type Output = G1Projective;

    #[inline]
    fn mul(self, rhs: &G1Projective) -> Self::Output {
        rhs * self
    }
}

impl Mul<G1Projective> for &Scalar {
    type Output = G1Projective;

    #[inline]
    fn mul(self, rhs: G1Projective) -> Self::Output {
        rhs * self
    }
}

impl Mul<&G1Projective> for &Scalar {
    type Output = G1Projective;

    #[inline]
    fn mul(self, rhs: &G1Projective) -> Self::Output {
        rhs * self
    }
}

impl<S> MulAssign<S> for G1Projective
where
    S: AsRef<Scalar>,
{
    fn mul_assign(&mut self, rhs: S) {
        let rhs = rhs.as_ref();
        unsafe {
            wrapper_g1_mul_assign(&mut self.0, &rhs.0);
        }
    }
}

impl PartialEq for G1Projective {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        unsafe { wrapper_g1_is_equal(&self.0, &other.0) }
    }
}

impl Eq for G1Projective {}

impl GroupEncoding for G1Projective {
    type Repr = GenericArray<u8, CompressedSize>;

    fn from_bytes(bytes: &Self::Repr) -> CtOption<Self> {
        let mut wrapper = new_wrapper();
        let is_valid = unsafe { wrapper_g1_read_bin(&mut wrapper, bytes.as_ptr(), bytes.len()) }
            == RLC_OK
            && unsafe { wrapper_g1_is_valid(&wrapper) };
        CtOption::new(Self(wrapper), (is_valid as u8).into())
    }

    fn from_bytes_unchecked(bytes: &Self::Repr) -> CtOption<Self> {
        let mut wrapper = new_wrapper();
        let is_valid =
            unsafe { wrapper_g1_read_bin(&mut wrapper, bytes.as_ptr(), bytes.len()) } == RLC_OK;
        CtOption::new(Self(wrapper), (is_valid as u8).into())
    }

    fn to_bytes(&self) -> Self::Repr {
        GenericArray::from_array(<[u8; COMPRESSED_BYTES_SIZE]>::from(self))
    }
}

impl UncompressedEncoding for G1Projective {
    type Uncompressed = GenericArray<u8, UncompressedSize>;

    fn from_uncompressed(bytes: &Self::Uncompressed) -> CtOption<Self> {
        let mut wrapper = new_wrapper();
        let is_valid = unsafe { wrapper_g1_read_bin(&mut wrapper, bytes.as_ptr(), bytes.len()) }
            == RLC_OK
            && unsafe { wrapper_g1_is_valid(&wrapper) };
        CtOption::new(Self(wrapper), (is_valid as u8).into())
    }

    fn from_uncompressed_unchecked(bytes: &Self::Uncompressed) -> CtOption<Self> {
        let mut wrapper = new_wrapper();
        let is_valid =
            unsafe { wrapper_g1_read_bin(&mut wrapper, bytes.as_ptr(), bytes.len()) } == RLC_OK;
        CtOption::new(Self(wrapper), (is_valid as u8).into())
    }

    fn to_uncompressed(&self) -> Self::Uncompressed {
        GenericArray::from_array(<[u8; UNCOMPRESSED_BYTES_SIZE]>::from(self))
    }
}

impl Group for G1Projective {
    type Scalar = Scalar;

    fn random(mut rng: impl RngCore) -> Self {
        let mut buf = [0u8; 64];
        rng.fill_bytes(&mut buf);
        Self::hash_to_curve(buf, RANDOM_DOMAIN_SEPERATOR)
    }

    #[inline]
    fn identity() -> Self {
        Self::default()
    }

    #[inline]
    fn generator() -> Self {
        let mut value = new_wrapper();
        unsafe {
            wrapper_g1_generator(&mut value);
        }
        Self(value)
    }

    #[inline]
    fn is_identity(&self) -> Choice {
        Choice::from(unsafe { wrapper_g1_is_neutral(&self.0) } as u8)
    }

    #[inline]
    fn double(&self) -> Self {
        let mut ret = new_wrapper();
        unsafe {
            wrapper_g1_double(&mut ret, &self.0);
        }
        Self(ret)
    }
}

impl PrimeGroup for G1Projective {}

impl<G, S> Sum<(G, S)> for G1Projective
where
    G: AsRef<G1Projective>,
    S: AsRef<Scalar>,
{
    #[cfg(feature = "alloc")]
    fn sum<I: Iterator<Item = (G, S)>>(iter: I) -> Self {
        let size = iter.size_hint().0;

        let mut g1s = Vec::with_capacity(size);
        let mut scalars = Vec::with_capacity(size);
        iter.for_each(|(g1, scalar)| {
            g1s.push(g1.as_ref().into());
            scalars.push(scalar.as_ref().into());
        });

        let mut g1 = new_wrapper();
        unsafe {
            wrapper_g1_simmul(&mut g1, g1s.as_ptr(), scalars.as_ptr(), g1s.len());
        }
        g1.into()
    }

    #[cfg(not(feature = "alloc"))]
    fn sum<I: Iterator<Item = (G, S)>>(iter: I) -> Self {
        iter.fold(Self::identity(), |a, (g, s)| a + g.as_ref() * s.as_ref())
    }
}

impl<'a, G, S> Sum<&'a (G, S)> for G1Projective
where
    G: AsRef<G1Projective>,
    S: AsRef<Scalar>,
{
    #[cfg(feature = "alloc")]
    fn sum<I: Iterator<Item = &'a (G, S)>>(iter: I) -> Self {
        let size = iter.size_hint().0;

        let mut g1s = Vec::with_capacity(size);
        let mut scalars = Vec::with_capacity(size);
        iter.for_each(|(g1, scalar)| {
            g1s.push(g1.as_ref().into());
            scalars.push(scalar.as_ref().into());
        });

        let mut g1 = new_wrapper();
        unsafe {
            wrapper_g1_simmul(&mut g1, g1s.as_ptr(), scalars.as_ptr(), g1s.len());
        }
        g1.into()
    }

    #[cfg(not(feature = "alloc"))]
    fn sum<I: Iterator<Item = &'a (G, S)>>(iter: I) -> Self {
        iter.fold(Self::identity(), |a, (g, s)| a + g.as_ref() * s.as_ref())
    }
}

/// The affine representation of G1.
pub type G1Affine = Affine<G1Projective>;

impl affine::private::Sealed for G1Projective {}

impl Curve for G1Projective {
    type AffineRepr = Affine<Self>;

    fn to_affine(&self) -> Self::AffineRepr {
        let mut g1 = new_wrapper();
        unsafe {
            wrapper_g1_norm(&mut g1, &self.0);
        }
        Affine(Self(g1))
    }
}

impl PrimeCurve for G1Projective {
    type Affine = Affine<Self>;
}

impl From<Affine<G1Projective>> for G1Projective {
    #[inline]
    fn from(value: Affine<G1Projective>) -> Self {
        value.0
    }
}

impl From<&Affine<G1Projective>> for G1Projective {
    #[inline]
    fn from(value: &Affine<G1Projective>) -> Self {
        value.0
    }
}

impl From<wrapper_g1_t> for Affine<G1Projective> {
    fn from(mut value: wrapper_g1_t) -> Self {
        unsafe {
            wrapper_g1_norm(&mut value, &value);
        }
        Self(G1Projective(value))
    }
}

impl From<G1Projective> for Affine<G1Projective> {
    fn from(value: G1Projective) -> Self {
        Self::from(value.0)
    }
}

impl From<&G1Projective> for Affine<G1Projective> {
    fn from(value: &G1Projective) -> Self {
        value.to_affine()
    }
}

impl From<Affine<G1Projective>> for wrapper_g1_t {
    fn from(value: Affine<G1Projective>) -> Self {
        value.0.into()
    }
}

impl From<&Affine<G1Projective>> for wrapper_g1_t {
    fn from(value: &Affine<G1Projective>) -> Self {
        value.0.into()
    }
}

impl GroupEncoding for Affine<G1Projective> {
    type Repr = <G1Projective as GroupEncoding>::Repr;

    fn from_bytes(bytes: &Self::Repr) -> CtOption<Self> {
        let mut wrapper = new_wrapper();
        let is_valid = unsafe { wrapper_g1_read_bin(&mut wrapper, bytes.as_ptr(), bytes.len()) }
            == RLC_OK
            && unsafe { wrapper_g1_is_valid(&wrapper) };
        CtOption::new(Self::from(wrapper), (is_valid as u8).into())
    }

    fn from_bytes_unchecked(bytes: &Self::Repr) -> CtOption<Self> {
        let mut wrapper = new_wrapper();
        let is_valid =
            unsafe { wrapper_g1_read_bin(&mut wrapper, bytes.as_ptr(), bytes.len()) } == RLC_OK;
        CtOption::new(Self::from(wrapper), (is_valid as u8).into())
    }

    #[inline]
    fn to_bytes(&self) -> Self::Repr {
        self.0.to_bytes()
    }
}

impl UncompressedEncoding for Affine<G1Projective> {
    type Uncompressed = GenericArray<u8, UncompressedSize>;

    fn from_uncompressed(bytes: &Self::Uncompressed) -> CtOption<Self> {
        let mut wrapper = new_wrapper();
        let is_valid = unsafe { wrapper_g1_read_bin(&mut wrapper, bytes.as_ptr(), bytes.len()) }
            == RLC_OK
            && unsafe { wrapper_g1_is_valid(&wrapper) };
        CtOption::new(Self::from(wrapper), (is_valid as u8).into())
    }

    fn from_uncompressed_unchecked(bytes: &Self::Uncompressed) -> CtOption<Self> {
        let mut wrapper = new_wrapper();
        let is_valid =
            unsafe { wrapper_g1_read_bin(&mut wrapper, bytes.as_ptr(), bytes.len()) } == RLC_OK;
        CtOption::new(Self::from(wrapper), (is_valid as u8).into())
    }

    fn to_uncompressed(&self) -> Self::Uncompressed {
        self.0.to_uncompressed()
    }
}

#[cfg(feature = "zeroize")]
impl zeroize::Zeroize for G1Projective {
    fn zeroize(&mut self) {
        unsafe {
            wrapper_g1_neutral(&mut self.0);
        }
    }
}

#[cfg(feature = "serde")]
impl serde::Serialize for G1Projective {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        crate::serde_helpers::serialize(self, serializer)
    }
}

#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for G1Projective {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        crate::serde_helpers::deserialize(deserializer)
    }
}

#[cfg(test)]
mod test {
    use pairing::group::ff::Field;

    use super::*;

    #[test]
    fn generator() {
        let generator = G1Projective::generator();
        let identity = G1Projective::identity();
        assert_ne!(generator, identity);
    }

    #[test]
    fn add() {
        let mut rng = rand::thread_rng();
        let v1 = G1Projective::random(&mut rng);
        let v2 = G1Projective::random(&mut rng);
        let check = v1 + v2;
        assert_eq!(check, v2 + v1);

        let rv1 = &v1;
        let rv2 = &v2;
        assert_eq!(check, v1 + rv2);
        assert_eq!(check, rv1 + v2);

        let a1 = v1.to_affine();
        let a2 = v2.to_affine();
        assert_eq!(check, a1 + a2);
        assert_eq!(check, a1 + v2);
        assert_eq!(check, v1 + a2);
        assert_eq!(check, a1 + rv2);
        assert_eq!(check, rv1 + a2);

        let ra1 = &a1;
        let ra2 = &a2;
        assert_eq!(check, ra1 + ra2);
        assert_eq!(check, ra1 + v2);
        assert_eq!(check, v1 + ra2);
        assert_eq!(check, ra1 + rv2);
        assert_eq!(check, rv1 + ra2);
    }

    #[test]
    fn sub() {
        let mut rng = rand::thread_rng();
        let v1 = G1Projective::random(&mut rng);
        let v2 = G1Projective::random(&mut rng);
        assert_eq!(v1 - v1, G1Projective::identity());
        let check = v1 - v2;

        let rv1 = &v1;
        let rv2 = &v2;
        assert_eq!(check, v1 - rv2);
        assert_eq!(check, rv1 - v2);

        let a1 = v1.to_affine();
        let a2 = v2.to_affine();
        assert_eq!(check, a1 - a2);
        assert_eq!(check, a1 - v2);
        assert_eq!(check, v1 - a2);
        assert_eq!(check, a1 - rv2);
        assert_eq!(check, rv1 - a2);

        let ra1 = &a1;
        let ra2 = &a2;
        assert_eq!(check, ra1 - ra2);
        assert_eq!(check, ra1 - v2);
        assert_eq!(check, v1 - ra2);
        assert_eq!(check, ra1 - rv2);
        assert_eq!(check, rv1 - ra2);
    }

    #[test]
    fn mul() {
        let mut rng = rand::thread_rng();
        let v = G1Projective::random(&mut rng);
        let s = Scalar::random(&mut rng);
        let check = v * s;

        let rv = &v;
        let rs = &s;
        assert_eq!(check, rv * s);
        assert_eq!(check, rv * rs);
        assert_eq!(check, v * rs);
        assert_eq!(check, s * rv);
        assert_eq!(check, rs * rv);
        assert_eq!(check, rs * v);

        let a = G1Affine::from(v);
        let ra = &a;
        assert_eq!(check, a * s);
        assert_eq!(check, ra * s);
        assert_eq!(check, ra * rs);
        assert_eq!(check, a * rs);
        // assert_eq!(check, s * a);
        // assert_eq!(check, rs * a);
        // assert_eq!(check, rs * ra);
        // assert_eq!(check, s * ra);

        let mut mv = v;
        mv *= s;
        assert_eq!(check, mv);
    }

    #[test]
    fn simmul() {
        let mut rng = rand::thread_rng();
        let v1 = G1Projective::random(&mut rng);
        let v2 = G1Projective::random(&mut rng);
        let s1 = Scalar::random(&mut rng);
        let s2 = Scalar::random(&mut rng);
        let check = v1 * s1 + v2 * s2;

        assert_eq!(G1Projective::sum([(v1, s1), (v2, s2)].iter()), check);
        assert_eq!(
            G1Projective::sum([(&v1, &s1), (&v2, &s2)].into_iter()),
            check
        );
        assert_eq!(G1Projective::sum([(v1, s1), (v2, s2)].into_iter()), check);
    }

    #[test]
    fn hash() {
        let h1 = G1Projective::hash_to_curve(b"1", b"dst");
        let h2 = G1Projective::hash_to_curve(b"2", b"dst");

        assert_ne!(h1, h2);
    }

    #[test]
    fn bytes() {
        let mut rng = rand::thread_rng();
        let v1 = G1Projective::random(&mut rng);

        let v2 = G1Projective::from_bytes_unchecked(&v1.to_bytes()).unwrap();
        assert_eq!(v1, v2);
        let v2 = G1Projective::from_bytes(&v1.to_bytes()).unwrap();
        assert_eq!(v1, v2);

        let v2 = G1Projective::from_uncompressed_unchecked(&v1.to_uncompressed()).unwrap();
        assert_eq!(v1, v2);
        let v2 = G1Projective::from_uncompressed(&v1.to_uncompressed()).unwrap();
        assert_eq!(v1, v2);

        let a1 = v1.to_affine();
        let a2 = G1Affine::from_bytes(&a1.to_bytes()).unwrap();
        assert_eq!(a1, a2);
        let v2 = G1Projective::from_bytes(&a1.to_bytes()).unwrap();
        assert_eq!(v1, v2);
    }

    #[cfg(feature = "serde")]
    #[test]
    fn serde_serialization() {
        let mut rng = rand::thread_rng();
        let config = bincode::config::standard();

        let v1 = G1Projective::random(&mut rng);

        let bytes = bincode::serde::encode_to_vec(v1, config).unwrap();
        let (v2, _) = bincode::serde::decode_from_slice(&bytes, config).unwrap();
        assert_eq!(v1, v2);

        let a1 = v1.to_affine();
        let (a2, _) = bincode::serde::decode_from_slice(&bytes, config).unwrap();
        assert_eq!(a1, a2);

        let abytes = bincode::serde::encode_to_vec(a1, config).unwrap();
        assert_eq!(bytes, abytes);
    }
}
