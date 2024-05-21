use core::{
    fmt,
    iter::Sum,
    mem::MaybeUninit,
    ops::{Add, AddAssign, Mul, MulAssign, Neg, Sub, SubAssign},
};

use generic_array::{typenum::U97, GenericArray};
use librelic_sys::{
    wrapper_g1_add, wrapper_g1_add_assign, wrapper_g1_double, wrapper_g1_generator,
    wrapper_g1_init, wrapper_g1_is_equal, wrapper_g1_is_neutral, wrapper_g1_is_valid,
    wrapper_g1_mul, wrapper_g1_mul_assign, wrapper_g1_neg, wrapper_g1_neutral, wrapper_g1_norm,
    wrapper_g1_rand, wrapper_g1_read_bin, wrapper_g1_sub, wrapper_g1_sub_assign, wrapper_g1_t,
    wrapper_g1_write_bin, RLC_OK,
};
use pairing::group::{
    prime::{PrimeCurve, PrimeGroup},
    Curve, Group, GroupEncoding,
};
use subtle::{Choice, ConditionallySelectable, CtOption};

use crate::{Affine, Error, Scalar};
use rand_core::RngCore;

fn new_wrapper() -> wrapper_g1_t {
    let mut g1 = MaybeUninit::uninit();
    unsafe {
        wrapper_g1_init(g1.as_mut_ptr());
        g1.assume_init()
    }
}

#[derive(Clone, Copy)]
#[allow(clippy::large_enum_variant)]
pub struct G1(wrapper_g1_t);

impl Default for G1 {
    fn default() -> Self {
        let mut value = new_wrapper();
        unsafe {
            wrapper_g1_neutral(&mut value);
        }
        Self(value)
    }
}

impl From<wrapper_g1_t> for G1 {
    fn from(value: wrapper_g1_t) -> Self {
        Self(value)
    }
}

impl From<&wrapper_g1_t> for G1 {
    fn from(value: &wrapper_g1_t) -> Self {
        Self(*value)
    }
}

impl TryFrom<[u8; 97]> for G1 {
    type Error = Error;

    fn try_from(value: [u8; 97]) -> Result<Self, Self::Error> {
        Self::try_from(&value)
    }
}

impl TryFrom<&[u8; 97]> for G1 {
    type Error = Error;

    fn try_from(value: &[u8; 97]) -> Result<Self, Self::Error> {
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

impl From<G1> for wrapper_g1_t {
    fn from(value: G1) -> Self {
        value.0
    }
}

impl From<&G1> for wrapper_g1_t {
    fn from(value: &G1) -> Self {
        value.0
    }
}

impl From<G1> for [u8; 97] {
    fn from(value: G1) -> Self {
        let mut ret = [0u8; 97];
        unsafe {
            wrapper_g1_write_bin(ret.as_mut_ptr(), ret.len(), &value.0);
        }
        ret
    }
}

impl From<&G1> for [u8; 97] {
    fn from(value: &G1) -> Self {
        let mut ret = [0u8; 97];
        unsafe {
            wrapper_g1_write_bin(ret.as_mut_ptr(), ret.len(), &value.0);
        }
        ret
    }
}

impl TryFrom<&[u8]> for G1 {
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

impl Add for G1 {
    type Output = G1;

    fn add(mut self, rhs: Self) -> Self::Output {
        unsafe {
            wrapper_g1_add_assign(&mut self.0, &rhs.into());
        }
        self
    }
}

impl Add<&G1> for G1 {
    type Output = G1;

    fn add(mut self, rhs: &Self) -> Self::Output {
        unsafe { wrapper_g1_add_assign(&mut self.0, &rhs.0) };
        self
    }
}

impl Add for &G1 {
    type Output = G1;

    fn add(self, rhs: Self) -> Self::Output {
        let mut ret = new_wrapper();
        unsafe {
            wrapper_g1_add(&mut ret, &self.0, &rhs.0);
        }
        ret.into()
    }
}

impl Add<G1> for &G1 {
    type Output = G1;

    fn add(self, rhs: G1) -> Self::Output {
        rhs + self
    }
}

impl AddAssign for G1 {
    fn add_assign(&mut self, rhs: Self) {
        unsafe { wrapper_g1_add_assign(&mut self.0, &rhs.0) };
    }
}

impl AddAssign<&G1> for G1 {
    fn add_assign(&mut self, rhs: &Self) {
        unsafe { wrapper_g1_add_assign(&mut self.0, &rhs.0) };
    }
}

impl Neg for G1 {
    type Output = G1;

    fn neg(mut self) -> Self::Output {
        unsafe {
            wrapper_g1_neg(&mut self.0);
        }
        self
    }
}

impl Neg for &G1 {
    type Output = G1;

    fn neg(self) -> Self::Output {
        let mut ret = self.into();
        unsafe {
            wrapper_g1_neg(&mut ret);
        }
        G1(ret)
    }
}

impl Sub for G1 {
    type Output = G1;

    fn sub(mut self, rhs: Self) -> Self::Output {
        unsafe {
            wrapper_g1_sub_assign(&mut self.0, &rhs.into());
        }
        self
    }
}

impl Sub<&G1> for G1 {
    type Output = G1;

    fn sub(mut self, rhs: &Self) -> Self::Output {
        unsafe { wrapper_g1_sub_assign(&mut self.0, &rhs.0) };
        self
    }
}

impl Sub for &G1 {
    type Output = G1;

    fn sub(self, rhs: Self) -> Self::Output {
        let mut ret = new_wrapper();
        unsafe {
            wrapper_g1_sub(&mut ret, &self.0, &rhs.0);
        }
        ret.into()
    }
}

impl Sub<G1> for &G1 {
    type Output = G1;

    fn sub(self, rhs: G1) -> Self::Output {
        let mut ret = new_wrapper();
        unsafe {
            wrapper_g1_sub(&mut ret, &self.0, &rhs.0);
        }
        ret.into()
    }
}

impl SubAssign for G1 {
    fn sub_assign(&mut self, rhs: Self) {
        unsafe { wrapper_g1_sub_assign(&mut self.0, &rhs.0) };
    }
}

impl SubAssign<&G1> for G1 {
    fn sub_assign(&mut self, rhs: &Self) {
        unsafe { wrapper_g1_sub_assign(&mut self.0, &rhs.0) };
    }
}

impl Sum for G1 {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        let mut start = new_wrapper();
        unsafe {
            wrapper_g1_neutral(&mut start);
        }
        Self(iter.fold(start, |mut sum, v| {
            unsafe {
                wrapper_g1_add_assign(&mut sum, &v.0);
            }
            sum
        }))
    }
}

impl<'a> Sum<&'a G1> for G1 {
    fn sum<I: Iterator<Item = &'a Self>>(iter: I) -> Self {
        let mut start = new_wrapper();
        unsafe {
            wrapper_g1_neutral(&mut start);
        }
        Self(iter.fold(start, |mut sum, v| {
            unsafe {
                wrapper_g1_add_assign(&mut sum, &v.0);
            }
            sum
        }))
    }
}

// TODO: Scalar * G!

impl Mul<Scalar> for G1 {
    type Output = G1;

    fn mul(self, rhs: Scalar) -> Self::Output {
        self * &rhs
    }
}

impl Mul<&Scalar> for G1 {
    type Output = G1;

    fn mul(mut self, rhs: &Scalar) -> Self::Output {
        match rhs {
            Scalar::Relic(bn) => unsafe {
                wrapper_g1_mul_assign(&mut self.0, bn);
            },
            _ => {
                let bn = rhs.into();
                unsafe {
                    wrapper_g1_mul_assign(&mut self.0, &bn);
                }
            }
        }
        self
    }
}

impl Mul<Scalar> for &G1 {
    type Output = G1;

    fn mul(self, rhs: Scalar) -> Self::Output {
        let mut g1 = new_wrapper();
        match rhs {
            Scalar::Relic(bn) => unsafe {
                wrapper_g1_mul(&mut g1, &self.0, &bn);
            },
            _ => {
                let bn = rhs.into();
                unsafe {
                    wrapper_g1_mul(&mut g1, &self.0, &bn);
                }
            }
        }
        G1(g1)
    }
}

impl Mul<&Scalar> for &G1 {
    type Output = G1;

    fn mul(self, rhs: &Scalar) -> Self::Output {
        let mut g1 = new_wrapper();
        match rhs {
            Scalar::Relic(bn) => unsafe {
                wrapper_g1_mul(&mut g1, &self.0, bn);
            },
            _ => {
                let bn = rhs.into();
                unsafe {
                    wrapper_g1_mul(&mut g1, &self.0, &bn);
                }
            }
        }
        G1(g1)
    }
}

impl MulAssign<Scalar> for G1 {
    fn mul_assign(&mut self, rhs: Scalar) {
        *self *= &rhs;
    }
}

impl MulAssign<&Scalar> for G1 {
    fn mul_assign(&mut self, rhs: &Scalar) {
        match rhs {
            Scalar::Relic(bn) => unsafe {
                wrapper_g1_mul_assign(&mut self.0, bn);
            },
            _ => {
                let bn = rhs.into();
                unsafe {
                    wrapper_g1_mul_assign(&mut self.0, &bn);
                }
            }
        }
    }
}

impl PartialEq for G1 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { wrapper_g1_is_equal(&self.0, &other.0) }
    }
}

impl Eq for G1 {}

impl fmt::Debug for G1 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let bytes: [u8; 97] = self.into();
        f.debug_tuple("Relic").field(&bytes).finish()
    }
}

impl GroupEncoding for G1 {
    type Repr = GenericArray<u8, U97>;

    fn from_bytes(bytes: &Self::Repr) -> CtOption<Self> {
        let mut wrapper = new_wrapper();
        if unsafe { wrapper_g1_read_bin(&mut wrapper, bytes.as_ptr(), bytes.len()) } == RLC_OK {
            CtOption::new(
                Self(wrapper),
                Choice::from(unsafe { wrapper_g1_is_valid(&wrapper) } as u8),
            )
        } else {
            CtOption::new(Self(wrapper), 0.into())
        }
    }

    fn from_bytes_unchecked(bytes: &Self::Repr) -> CtOption<Self> {
        let mut wrapper = new_wrapper();
        if unsafe { wrapper_g1_read_bin(&mut wrapper, bytes.as_ptr(), bytes.len()) } == RLC_OK {
            CtOption::new(Self(wrapper), 1.into())
        } else {
            CtOption::new(Self(wrapper), 0.into())
        }
    }

    fn to_bytes(&self) -> Self::Repr {
        GenericArray::from_array(self.into())
    }
}

impl Group for G1 {
    type Scalar = Scalar;

    fn random(_rng: impl RngCore) -> Self {
        let mut g1 = new_wrapper();
        unsafe {
            wrapper_g1_rand(&mut g1);
        }
        Self(g1)
        /*
                let mut bytes = [0u8; 97];
                // unpacked representation
                bytes[0] = 4;
                loop {
                    rng.fill_bytes(&mut bytes[1..]);
                }
        */
    }

    fn identity() -> Self {
        Self::default()
    }

    fn generator() -> Self {
        let mut value = new_wrapper();
        unsafe {
            wrapper_g1_generator(&mut value);
        }
        Self(value)
    }

    fn is_identity(&self) -> Choice {
        Choice::from(unsafe { wrapper_g1_is_neutral(&self.0) } as u8)
    }

    fn double(&self) -> Self {
        let mut ret = new_wrapper();
        unsafe {
            wrapper_g1_double(&mut ret, &self.0);
        }
        Self(ret)
    }
}

impl PrimeGroup for G1 {}

/// The affine representation of G1.
pub type G1Affine = Affine<G1>;

impl Curve for G1 {
    type AffineRepr = Affine<Self>;

    fn to_affine(&self) -> Self::AffineRepr {
        let mut g1 = new_wrapper();
        unsafe {
            wrapper_g1_norm(&mut g1, &self.0);
        }
        Affine(Self(g1))
    }
}

impl PrimeCurve for G1 {
    type Affine = Affine<Self>;
}

impl Add<Affine<G1>> for G1 {
    type Output = G1;

    fn add(self, rhs: Affine<G1>) -> Self::Output {
        self + rhs.0
    }
}

impl Add<&Affine<G1>> for G1 {
    type Output = G1;

    fn add(self, rhs: &Affine<G1>) -> Self::Output {
        self + rhs.0
    }
}

impl Sub<Affine<G1>> for G1 {
    type Output = G1;

    fn sub(self, rhs: Affine<G1>) -> Self::Output {
        self - rhs.0
    }
}

impl Sub<&Affine<G1>> for G1 {
    type Output = G1;

    fn sub(self, rhs: &Affine<G1>) -> Self::Output {
        self - rhs.0
    }
}

impl AddAssign<Affine<G1>> for G1 {
    fn add_assign(&mut self, rhs: Affine<G1>) {
        *self += rhs.0;
    }
}

impl AddAssign<&Affine<G1>> for G1 {
    fn add_assign(&mut self, rhs: &Affine<G1>) {
        *self += rhs.0;
    }
}

impl SubAssign<Affine<G1>> for G1 {
    fn sub_assign(&mut self, rhs: Affine<G1>) {
        *self -= rhs.0;
    }
}

impl SubAssign<&Affine<G1>> for G1 {
    fn sub_assign(&mut self, rhs: &Affine<G1>) {
        *self -= rhs.0;
    }
}

impl ConditionallySelectable for G1 {
    fn conditional_select(a: &Self, b: &Self, choice: Choice) -> Self {
        // TODO: implement constant tinme
        if choice.unwrap_u8() == 1 {
            *b
        } else {
            *a
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn generator() {
        let generator = G1::generator();
        let identity = G1::identity();
        assert_ne!(generator, identity);
    }

    #[test]
    fn add() {
        let mut rng = rand::thread_rng();
        let v1 = G1::random(&mut rng);
        let v2 = G1::random(&mut rng);
        assert_eq!(v1 + v2, v2 + v1);
    }
}
