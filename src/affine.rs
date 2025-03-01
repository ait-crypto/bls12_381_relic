//! Affine representation of curve points
//!
//! Relic does not implement specific types or functions that make use of affine
//! representations of group elements. Hence, this crate provides a generic
//! wrapper around default representation whereas the elements are initially
//! normalized. Whenever an operation is performed on the group elements, the
//! default representation is returned as result.

use core::ops::{Add, Mul, Neg, Sub};

use pairing::group::{
    GroupEncoding,
    prime::{PrimeCurve, PrimeCurveAffine},
};
use subtle::Choice;

use crate::Scalar;

pub(crate) mod private {
    /// Internal trait to make instantiations of [super::Affine] impossible for
    /// types other than [crate::G1Projective] and [crate::G2Projective].
    pub trait Sealed {}
}

/// Affine representation of curve points
///
/// This is a fake "affine" representation since relic does not support them
/// explicitly. The implementation ensures that the wrapped element is
/// normalized.
///
/// ```
/// use bls12_381_relic::G1Projective;
/// use bls12_381_relic::group::Curve;
///
/// let g1 = G1Projective::hash_to_curve(b"a point", b"public parameters");
/// let affine = g1.to_affine();
/// ```
#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
#[repr(transparent)]
pub struct Affine<G>(pub(crate) G)
where
    G: private::Sealed;

impl<G> AsRef<G> for Affine<G>
where
    G: private::Sealed,
{
    fn as_ref(&self) -> &G {
        &self.0
    }
}

impl<'a, G> TryFrom<&'a [u8]> for Affine<G>
where
    G: private::Sealed,
    G: TryFrom<&'a [u8]>,
    Affine<G>: From<G>,
{
    type Error = <G as TryFrom<&'a [u8]>>::Error;

    fn try_from(value: &'a [u8]) -> Result<Self, Self::Error> {
        G::try_from(value).map(Self::from)
    }
}

impl<G, Gp> Add<Gp> for Affine<G>
where
    G: private::Sealed,
    G: Add<Gp, Output = G>,
{
    type Output = G;

    fn add(self, rhs: Gp) -> Self::Output {
        self.0 + rhs
    }
}

impl<'a, G, Gp> Add<Gp> for &'a Affine<G>
where
    G: private::Sealed,
    &'a G: Add<Gp, Output = G>,
{
    type Output = G;

    fn add(self, rhs: Gp) -> Self::Output {
        &self.0 + rhs
    }
}

impl<G> Neg for Affine<G>
where
    G: private::Sealed,
    G: Neg<Output = G>,
{
    type Output = Self;

    #[inline]
    fn neg(self) -> Self::Output {
        Self(-self.0)
    }
}

impl<G, Gp> Sub<Gp> for Affine<G>
where
    G: private::Sealed,
    G: Sub<Gp, Output = G>,
{
    type Output = G;

    #[inline]
    fn sub(self, rhs: Gp) -> Self::Output {
        self.0 - rhs
    }
}

impl<'a, G, Gp> Sub<Gp> for &'a Affine<G>
where
    G: private::Sealed,
    &'a G: Sub<Gp, Output = G>,
{
    type Output = G;

    fn sub(self, rhs: Gp) -> Self::Output {
        &self.0 - rhs
    }
}

impl<S, G> Mul<S> for Affine<G>
where
    G: private::Sealed,
    G: Mul<S, Output = G>,
{
    type Output = G;

    #[inline]
    fn mul(self, rhs: S) -> Self::Output {
        self.0 * rhs
    }
}

impl<'a, S, G> Mul<S> for &'a Affine<G>
where
    G: private::Sealed,
    &'a G: Mul<S, Output = G>,
{
    type Output = G;

    #[inline]
    fn mul(self, rhs: S) -> Self::Output {
        &self.0 * rhs
    }
}

impl<G> PrimeCurveAffine for Affine<G>
where
    G: private::Sealed,
    G: PrimeCurve<Affine = Self, Scalar = Scalar>,
    Self: GroupEncoding,
{
    type Scalar = Scalar;

    type Curve = G;

    #[inline]
    fn identity() -> Self {
        Self(G::identity())
    }

    #[inline]
    fn generator() -> Self {
        Self(G::generator())
    }

    #[inline]
    fn is_identity(&self) -> Choice {
        self.0.is_identity()
    }

    #[inline]
    fn to_curve(&self) -> Self::Curve {
        self.0
    }
}

#[cfg(feature = "zeroize")]
impl<G> zeroize::Zeroize for Affine<G>
where
    G: private::Sealed,
    G: zeroize::Zeroize,
{
    fn zeroize(&mut self) {
        self.0.zeroize();
    }
}

#[cfg(feature = "serde")]
impl<G> serde::Serialize for Affine<G>
where
    G: private::Sealed,
    G: serde::Serialize,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.0.serialize(serializer)
    }
}

#[cfg(feature = "serde")]
impl<'de, G> serde::Deserialize<'de> for Affine<G>
where
    G: private::Sealed,
    G: serde::Deserialize<'de>,
    Self: From<G>,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        G::deserialize(deserializer).map(|g| Self::from(g))
    }
}
