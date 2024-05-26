use core::ops::{Add, AddAssign, Mul, Neg, Sub, SubAssign};

use pairing::group::{
    prime::{PrimeCurve, PrimeCurveAffine},
    GroupEncoding, UncompressedEncoding,
};
use subtle::{Choice, CtOption};

use crate::Scalar;

/// Affine representation of curve points
///
/// This is a fake "affine" representation since relic does not support them explicitly.
#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
pub struct Affine<G>(pub(crate) G);

impl<G> AsRef<G> for Affine<G> {
    fn as_ref(&self) -> &G {
        &self.0
    }
}

impl<G> Add for Affine<G>
where
    G: Add<Output = G>,
{
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0)
    }
}

impl<G> Add<&Affine<G>> for Affine<G>
where
    for<'a> G: Add<&'a G, Output = G>,
{
    type Output = Self;

    #[inline]
    fn add(self, rhs: &Self) -> Self::Output {
        Self(self.0 + &rhs.0)
    }
}

impl<G> AddAssign for Affine<G>
where
    G: AddAssign,
{
    #[inline]
    fn add_assign(&mut self, rhs: Self) {
        self.0 += rhs.0;
    }
}

impl<G> AddAssign<&Affine<G>> for Affine<G>
where
    for<'a> G: AddAssign<&'a G>,
{
    #[inline]
    fn add_assign(&mut self, rhs: &Self) {
        self.0 += &rhs.0;
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

impl<G> Sub for Affine<G>
where
    G: Sub<Output = G>,
{
    type Output = Self;

    #[inline]
    fn sub(self, rhs: Self) -> Self::Output {
        Self(self.0 - rhs.0)
    }
}

impl<G> Sub<&Affine<G>> for Affine<G>
where
    for<'a> G: Sub<&'a G, Output = G>,
{
    type Output = Self;

    #[inline]
    fn sub(self, rhs: &Self) -> Self::Output {
        Self(self.0 - &rhs.0)
    }
}

impl<G> SubAssign for Affine<G>
where
    G: SubAssign,
{
    #[inline]
    fn sub_assign(&mut self, rhs: Self) {
        self.0 -= rhs.0;
    }
}

impl<G> SubAssign<&Affine<G>> for Affine<G>
where
    for<'a> G: SubAssign<&'a G>,
{
    #[inline]
    fn sub_assign(&mut self, rhs: &Self) {
        self.0 -= &rhs.0;
    }
}

impl<G> Mul<Scalar> for Affine<G>
where
    G: Mul<Scalar, Output = G>,
{
    type Output = G;

    #[inline]
    fn mul(self, rhs: Scalar) -> Self::Output {
        self.0 * rhs
    }
}

impl<G> Mul<&Scalar> for Affine<G>
where
    for<'a> G: Mul<&'a Scalar, Output = G>,
{
    type Output = G;

    #[inline]
    fn mul(self, rhs: &Scalar) -> Self::Output {
        self.0 * rhs
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

impl<G> UncompressedEncoding for Affine<G>
where
    Self: GroupEncoding,
{
    type Uncompressed = <Self as GroupEncoding>::Repr;

    #[inline]
    fn from_uncompressed(bytes: &Self::Uncompressed) -> CtOption<Self> {
        Self::from_bytes(bytes)
    }

    #[inline]
    fn from_uncompressed_unchecked(bytes: &Self::Uncompressed) -> CtOption<Self> {
        Self::from_bytes_unchecked(bytes)
    }

    #[inline]
    fn to_uncompressed(&self) -> Self::Uncompressed {
        self.to_bytes()
    }
}