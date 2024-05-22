use core::{
    fmt,
    iter::Sum,
    mem::MaybeUninit,
    ops::{Add, AddAssign, Mul, MulAssign, Neg, Sub, SubAssign},
};

use generic_array::{
    typenum::{Unsigned, U193},
    GenericArray,
};
use librelic_sys::{
    wrapper_g2_add, wrapper_g2_add_assign, wrapper_g2_double, wrapper_g2_generator,
    wrapper_g2_init, wrapper_g2_is_equal, wrapper_g2_is_neutral, wrapper_g2_is_valid,
    wrapper_g2_mul, wrapper_g2_mul_assign, wrapper_g2_neg, wrapper_g2_neutral, wrapper_g2_norm,
    wrapper_g2_rand, wrapper_g2_read_bin, wrapper_g2_sub, wrapper_g2_sub_assign, wrapper_g2_t,
    wrapper_g2_write_bin, RLC_OK,
};
use pairing::group::{
    prime::{PrimeCurve, PrimeGroup},
    Curve, Group, GroupEncoding,
};
use subtle::{Choice, CtOption};

use crate::{Affine, Error, Scalar};
use rand_core::RngCore;

const BYTES_SIZE: usize = U193::USIZE;

fn new_wrapper() -> wrapper_g2_t {
    let mut g2 = MaybeUninit::uninit();
    unsafe {
        wrapper_g2_init(g2.as_mut_ptr());
        g2.assume_init()
    }
}

#[derive(Clone, Copy)]
#[allow(clippy::large_enum_variant)]
pub struct G2Projective(pub(crate) wrapper_g2_t);

impl Default for G2Projective {
    fn default() -> Self {
        let mut value = new_wrapper();
        unsafe {
            wrapper_g2_neutral(&mut value);
        }
        Self(value)
    }
}

impl From<wrapper_g2_t> for G2Projective {
    fn from(value: wrapper_g2_t) -> Self {
        Self(value)
    }
}

impl From<&wrapper_g2_t> for G2Projective {
    fn from(value: &wrapper_g2_t) -> Self {
        Self(*value)
    }
}

impl TryFrom<[u8; BYTES_SIZE]> for G2Projective {
    type Error = Error;

    fn try_from(value: [u8; BYTES_SIZE]) -> Result<Self, Self::Error> {
        Self::try_from(&value)
    }
}

impl TryFrom<&[u8; BYTES_SIZE]> for G2Projective {
    type Error = Error;

    fn try_from(value: &[u8; BYTES_SIZE]) -> Result<Self, Self::Error> {
        let mut g2 = new_wrapper();
        let ret = unsafe { wrapper_g2_read_bin(&mut g2, value.as_ptr(), value.len()) };
        if ret == RLC_OK {
            if unsafe { wrapper_g2_is_valid(&g2) } {
                Ok(Self(g2))
            } else {
                Err(Error::InvalidBytesRepresentation)
            }
        } else {
            Err(Error::RelicError(ret))
        }
    }
}

impl From<G2Projective> for wrapper_g2_t {
    fn from(value: G2Projective) -> Self {
        value.0
    }
}

impl From<&G2Projective> for wrapper_g2_t {
    fn from(value: &G2Projective) -> Self {
        value.0
    }
}

impl From<G2Projective> for [u8; BYTES_SIZE] {
    fn from(value: G2Projective) -> Self {
        let mut ret = [0u8; BYTES_SIZE];
        unsafe {
            wrapper_g2_write_bin(ret.as_mut_ptr(), ret.len(), &value.0);
        }
        ret
    }
}

impl From<&G2Projective> for [u8; BYTES_SIZE] {
    fn from(value: &G2Projective) -> Self {
        let mut ret = [0u8; BYTES_SIZE];
        unsafe {
            wrapper_g2_write_bin(ret.as_mut_ptr(), ret.len(), &value.0);
        }
        ret
    }
}

impl TryFrom<&[u8]> for G2Projective {
    type Error = Error;

    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        let mut g2 = new_wrapper();
        let ret = unsafe { wrapper_g2_read_bin(&mut g2, value.as_ptr(), value.len()) };
        if ret == RLC_OK {
            if unsafe { wrapper_g2_is_valid(&g2) } {
                Ok(Self(g2))
            } else {
                Err(Error::InvalidBytesRepresentation)
            }
        } else {
            Err(Error::RelicError(ret))
        }
    }
}

impl Add for G2Projective {
    type Output = G2Projective;

    fn add(mut self, rhs: Self) -> Self::Output {
        unsafe {
            wrapper_g2_add_assign(&mut self.0, &rhs.into());
        }
        self
    }
}

impl Add<&G2Projective> for G2Projective {
    type Output = G2Projective;

    fn add(mut self, rhs: &Self) -> Self::Output {
        unsafe { wrapper_g2_add_assign(&mut self.0, &rhs.0) };
        self
    }
}

impl Add for &G2Projective {
    type Output = G2Projective;

    fn add(self, rhs: Self) -> Self::Output {
        let mut ret = new_wrapper();
        unsafe {
            wrapper_g2_add(&mut ret, &self.0, &rhs.0);
        }
        ret.into()
    }
}

impl Add<G2Projective> for &G2Projective {
    type Output = G2Projective;

    fn add(self, rhs: G2Projective) -> Self::Output {
        rhs + self
    }
}

impl AddAssign for G2Projective {
    fn add_assign(&mut self, rhs: Self) {
        unsafe { wrapper_g2_add_assign(&mut self.0, &rhs.0) };
    }
}

impl AddAssign<&G2Projective> for G2Projective {
    fn add_assign(&mut self, rhs: &Self) {
        unsafe { wrapper_g2_add_assign(&mut self.0, &rhs.0) };
    }
}

impl Neg for G2Projective {
    type Output = G2Projective;

    fn neg(mut self) -> Self::Output {
        unsafe {
            wrapper_g2_neg(&mut self.0);
        }
        self
    }
}

impl Neg for &G2Projective {
    type Output = G2Projective;

    fn neg(self) -> Self::Output {
        let mut ret = self.into();
        unsafe {
            wrapper_g2_neg(&mut ret);
        }
        G2Projective(ret)
    }
}

impl Sub for G2Projective {
    type Output = G2Projective;

    fn sub(mut self, rhs: Self) -> Self::Output {
        unsafe {
            wrapper_g2_sub_assign(&mut self.0, &rhs.into());
        }
        self
    }
}

impl Sub<&G2Projective> for G2Projective {
    type Output = G2Projective;

    fn sub(mut self, rhs: &Self) -> Self::Output {
        unsafe { wrapper_g2_sub_assign(&mut self.0, &rhs.0) };
        self
    }
}

impl Sub for &G2Projective {
    type Output = G2Projective;

    fn sub(self, rhs: Self) -> Self::Output {
        let mut ret = new_wrapper();
        unsafe {
            wrapper_g2_sub(&mut ret, &self.0, &rhs.0);
        }
        ret.into()
    }
}

impl Sub<G2Projective> for &G2Projective {
    type Output = G2Projective;

    fn sub(self, rhs: G2Projective) -> Self::Output {
        let mut ret = new_wrapper();
        unsafe {
            wrapper_g2_sub(&mut ret, &self.0, &rhs.0);
        }
        ret.into()
    }
}

impl SubAssign for G2Projective {
    fn sub_assign(&mut self, rhs: Self) {
        unsafe { wrapper_g2_sub_assign(&mut self.0, &rhs.0) };
    }
}

impl SubAssign<&G2Projective> for G2Projective {
    fn sub_assign(&mut self, rhs: &Self) {
        unsafe { wrapper_g2_sub_assign(&mut self.0, &rhs.0) };
    }
}

impl Sum for G2Projective {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        let mut start = new_wrapper();
        unsafe {
            wrapper_g2_neutral(&mut start);
        }
        Self(iter.fold(start, |mut sum, v| {
            unsafe {
                wrapper_g2_add_assign(&mut sum, &v.0);
            }
            sum
        }))
    }
}

impl<'a> Sum<&'a G2Projective> for G2Projective {
    fn sum<I: Iterator<Item = &'a Self>>(iter: I) -> Self {
        let mut start = new_wrapper();
        unsafe {
            wrapper_g2_neutral(&mut start);
        }
        Self(iter.fold(start, |mut sum, v| {
            unsafe {
                wrapper_g2_add_assign(&mut sum, &v.0);
            }
            sum
        }))
    }
}

// TODO: Scalar * G!

impl Mul<Scalar> for G2Projective {
    type Output = G2Projective;

    fn mul(self, rhs: Scalar) -> Self::Output {
        self * &rhs
    }
}

impl Mul<&Scalar> for G2Projective {
    type Output = G2Projective;

    fn mul(mut self, rhs: &Scalar) -> Self::Output {
        match rhs {
            Scalar::Relic(bn) => unsafe {
                wrapper_g2_mul_assign(&mut self.0, bn);
            },
            _ => {
                let bn = rhs.into();
                unsafe {
                    wrapper_g2_mul_assign(&mut self.0, &bn);
                }
            }
        }
        self
    }
}

impl Mul<Scalar> for &G2Projective {
    type Output = G2Projective;

    fn mul(self, rhs: Scalar) -> Self::Output {
        let mut g2 = new_wrapper();
        match rhs {
            Scalar::Relic(bn) => unsafe {
                wrapper_g2_mul(&mut g2, &self.0, &bn);
            },
            _ => {
                let bn = rhs.into();
                unsafe {
                    wrapper_g2_mul(&mut g2, &self.0, &bn);
                }
            }
        }
        G2Projective(g2)
    }
}

impl Mul<&Scalar> for &G2Projective {
    type Output = G2Projective;

    fn mul(self, rhs: &Scalar) -> Self::Output {
        let mut g2 = new_wrapper();
        match rhs {
            Scalar::Relic(bn) => unsafe {
                wrapper_g2_mul(&mut g2, &self.0, bn);
            },
            _ => {
                let bn = rhs.into();
                unsafe {
                    wrapper_g2_mul(&mut g2, &self.0, &bn);
                }
            }
        }
        G2Projective(g2)
    }
}

impl MulAssign<Scalar> for G2Projective {
    fn mul_assign(&mut self, rhs: Scalar) {
        *self *= &rhs;
    }
}

impl MulAssign<&Scalar> for G2Projective {
    fn mul_assign(&mut self, rhs: &Scalar) {
        match rhs {
            Scalar::Relic(bn) => unsafe {
                wrapper_g2_mul_assign(&mut self.0, bn);
            },
            _ => {
                let bn = rhs.into();
                unsafe {
                    wrapper_g2_mul_assign(&mut self.0, &bn);
                }
            }
        }
    }
}

impl PartialEq for G2Projective {
    fn eq(&self, other: &Self) -> bool {
        unsafe { wrapper_g2_is_equal(&self.0, &other.0) }
    }
}

impl Eq for G2Projective {}

impl fmt::Debug for G2Projective {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let bytes: [u8; BYTES_SIZE] = self.into();
        f.debug_tuple("Relic").field(&bytes).finish()
    }
}

impl GroupEncoding for G2Projective {
    type Repr = GenericArray<u8, U193>;

    fn from_bytes(bytes: &Self::Repr) -> CtOption<Self> {
        let mut wrapper = new_wrapper();
        if unsafe { wrapper_g2_read_bin(&mut wrapper, bytes.as_ptr(), bytes.len()) } == RLC_OK {
            CtOption::new(
                Self(wrapper),
                Choice::from(unsafe { wrapper_g2_is_valid(&wrapper) } as u8),
            )
        } else {
            CtOption::new(Self(wrapper), 0.into())
        }
    }

    fn from_bytes_unchecked(bytes: &Self::Repr) -> CtOption<Self> {
        let mut wrapper = new_wrapper();
        if unsafe { wrapper_g2_read_bin(&mut wrapper, bytes.as_ptr(), bytes.len()) } == RLC_OK {
            CtOption::new(Self(wrapper), 1.into())
        } else {
            CtOption::new(Self(wrapper), 0.into())
        }
    }

    fn to_bytes(&self) -> Self::Repr {
        GenericArray::from_array(self.into())
    }
}

impl Group for G2Projective {
    type Scalar = Scalar;

    fn random(_rng: impl RngCore) -> Self {
        let mut g2 = new_wrapper();
        unsafe {
            wrapper_g2_rand(&mut g2);
        }
        Self(g2)
    }

    fn identity() -> Self {
        Self::default()
    }

    fn generator() -> Self {
        let mut value = new_wrapper();
        unsafe {
            wrapper_g2_generator(&mut value);
        }
        Self(value)
    }

    fn is_identity(&self) -> Choice {
        Choice::from(unsafe { wrapper_g2_is_neutral(&self.0) } as u8)
    }

    fn double(&self) -> Self {
        let mut ret = new_wrapper();
        unsafe {
            wrapper_g2_double(&mut ret, &self.0);
        }
        Self(ret)
    }
}

impl PrimeGroup for G2Projective {}

/// The affine representation of G2.
pub type G2Affine = Affine<G2Projective>;

impl Curve for G2Projective {
    type AffineRepr = Affine<Self>;

    fn to_affine(&self) -> Self::AffineRepr {
        let mut g2 = new_wrapper();
        unsafe {
            wrapper_g2_norm(&mut g2, &self.0);
        }
        Affine(Self(g2))
    }
}

impl PrimeCurve for G2Projective {
    type Affine = Affine<Self>;
}

impl Add<Affine<G2Projective>> for G2Projective {
    type Output = G2Projective;

    fn add(self, rhs: Affine<G2Projective>) -> Self::Output {
        self + rhs.0
    }
}

impl Add<&Affine<G2Projective>> for G2Projective {
    type Output = G2Projective;

    fn add(self, rhs: &Affine<G2Projective>) -> Self::Output {
        self + rhs.0
    }
}

impl Sub<Affine<G2Projective>> for G2Projective {
    type Output = G2Projective;

    fn sub(self, rhs: Affine<G2Projective>) -> Self::Output {
        self - rhs.0
    }
}

impl Sub<&Affine<G2Projective>> for G2Projective {
    type Output = G2Projective;

    fn sub(self, rhs: &Affine<G2Projective>) -> Self::Output {
        self - rhs.0
    }
}

impl AddAssign<Affine<G2Projective>> for G2Projective {
    fn add_assign(&mut self, rhs: Affine<G2Projective>) {
        *self += rhs.0;
    }
}

impl AddAssign<&Affine<G2Projective>> for G2Projective {
    fn add_assign(&mut self, rhs: &Affine<G2Projective>) {
        *self += rhs.0;
    }
}

impl SubAssign<Affine<G2Projective>> for G2Projective {
    fn sub_assign(&mut self, rhs: Affine<G2Projective>) {
        *self -= rhs.0;
    }
}

impl SubAssign<&Affine<G2Projective>> for G2Projective {
    fn sub_assign(&mut self, rhs: &Affine<G2Projective>) {
        *self -= rhs.0;
    }
}

impl From<Affine<G2Projective>> for G2Projective {
    fn from(value: Affine<G2Projective>) -> Self {
        value.0
    }
}

impl From<&Affine<G2Projective>> for G2Projective {
    fn from(value: &Affine<G2Projective>) -> Self {
        value.0
    }
}

impl From<G2Projective> for Affine<G2Projective> {
    fn from(mut value: G2Projective) -> Self {
        unsafe {
            wrapper_g2_norm(&mut value.0, &value.0);
        }
        Self(value)
    }
}

impl From<&G2Projective> for Affine<G2Projective> {
    fn from(value: &G2Projective) -> Self {
        let mut g2 = new_wrapper();
        unsafe {
            wrapper_g2_norm(&mut g2, &value.0);
        }
        Self(g2.into())
    }
}

impl GroupEncoding for Affine<G2Projective> {
    type Repr = <G2Projective as GroupEncoding>::Repr;

    fn from_bytes(bytes: &Self::Repr) -> CtOption<Self> {
        let mut wrapper = new_wrapper();
        if unsafe { wrapper_g2_read_bin(&mut wrapper, bytes.as_ptr(), bytes.len()) } == RLC_OK {
            CtOption::new(
                Self(wrapper.into()),
                Choice::from(unsafe { wrapper_g2_is_valid(&wrapper) } as u8),
            )
        } else {
            CtOption::new(Self(wrapper.into()), 0.into())
        }
    }

    fn from_bytes_unchecked(bytes: &Self::Repr) -> CtOption<Self> {
        let mut wrapper = new_wrapper();
        if unsafe { wrapper_g2_read_bin(&mut wrapper, bytes.as_ptr(), bytes.len()) } == RLC_OK {
            CtOption::new(Self(wrapper.into()), 1.into())
        } else {
            CtOption::new(Self(wrapper.into()), 0.into())
        }
    }

    fn to_bytes(&self) -> Self::Repr {
        self.0.to_bytes()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn generator() {
        let generator = G2Projective::generator();
        let identity = G2Projective::identity();
        assert_ne!(generator, identity);
    }

    #[test]
    fn add() {
        let mut rng = rand::thread_rng();
        let v1 = G2Projective::random(&mut rng);
        let v2 = G2Projective::random(&mut rng);
        assert_eq!(v1 + v2, v2 + v1);
    }
}
