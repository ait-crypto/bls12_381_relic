use core::ops::{Add, Mul, Neg, Sub};

use pairing::group::{
    prime::{PrimeCurve, PrimeCurveAffine},
    GroupEncoding,
};
use subtle::Choice;

use crate::Scalar;

/// Affine representation of curve points
///
/// This is a fake "affine" representation since relic does not support them explicitly.
#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
#[repr(transparent)]
pub struct Affine<G>(pub(crate) G);

impl<G> AsRef<G> for Affine<G> {
    fn as_ref(&self) -> &G {
        &self.0
    }
}

impl<G, Gp> Add<Gp> for Affine<G>
where
    G: Add<Gp, Output = G>,
{
    type Output = G;

    fn add(self, rhs: Gp) -> Self::Output {
        self.0 + rhs
    }
}

impl<'a, G, Gp> Add<Gp> for &'a Affine<G>
where
    &'a G: Add<Gp, Output = G>,
{
    type Output = G;

    fn add(self, rhs: Gp) -> Self::Output {
        &self.0 + rhs
    }
}

impl<G> Neg for Affine<G>
where
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
    &'a G: Sub<Gp, Output = G>,
{
    type Output = G;

    fn sub(self, rhs: Gp) -> Self::Output {
        &self.0 - rhs
    }
}

impl<S, G> Mul<S> for Affine<G>
where
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
    G: zeroize::Zeroize,
{
    fn zeroize(&mut self) {
        self.0.zeroize();
    }
}
