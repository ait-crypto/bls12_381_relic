use core::ops::{Add, AddAssign, Mul, Neg, Sub, SubAssign};

use pairing::group::{
    prime::{PrimeCurve, PrimeCurveAffine},
    GroupEncoding, UncompressedEncoding,
};
use subtle::{Choice, ConditionallySelectable, CtOption};

use crate::Scalar;

/// Affine representation of curve points
///
/// This is a fake "affine" representation since relic does not support them explicitly.
#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
pub struct Affine<G>(pub(crate) G);

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

    fn add(self, rhs: &Self) -> Self::Output {
        Self(self.0 + &rhs.0)
    }
}

impl<G> AddAssign for Affine<G>
where
    G: AddAssign,
{
    fn add_assign(&mut self, rhs: Self) {
        self.0 += rhs.0;
    }
}

impl<G> AddAssign<&Affine<G>> for Affine<G>
where
    for<'a> G: AddAssign<&'a G>,
{
    fn add_assign(&mut self, rhs: &Self) {
        self.0 += &rhs.0;
    }
}

impl<G> Neg for Affine<G>
where
    G: Neg<Output = G>,
{
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self(-self.0)
    }
}

impl<G> Sub for Affine<G>
where
    G: Sub<Output = G>,
{
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self(self.0 - rhs.0)
    }
}

impl<G> Sub<&Affine<G>> for Affine<G>
where
    for<'a> G: Sub<&'a G, Output = G>,
{
    type Output = Self;

    fn sub(self, rhs: &Self) -> Self::Output {
        Self(self.0 - &rhs.0)
    }
}

impl<G> SubAssign for Affine<G>
where
    G: SubAssign,
{
    fn sub_assign(&mut self, rhs: Self) {
        self.0 -= rhs.0;
    }
}

impl<G> SubAssign<&Affine<G>> for Affine<G>
where
    for<'a> G: SubAssign<&'a G>,
{
    fn sub_assign(&mut self, rhs: &Self) {
        self.0 -= &rhs.0;
    }
}

impl<G> Mul<Scalar> for Affine<G>
where
    G: Mul<Scalar, Output = G>,
{
    type Output = G;

    fn mul(self, rhs: Scalar) -> Self::Output {
        self.0 * rhs
    }
}

impl<G> Mul<&Scalar> for Affine<G>
where
    for<'a> G: Mul<&'a Scalar, Output = G>,
{
    type Output = G;

    fn mul(self, rhs: &Scalar) -> Self::Output {
        self.0 * rhs
    }
}

impl<G> GroupEncoding for Affine<G>
where
    G: GroupEncoding + Default + ConditionallySelectable,
{
    type Repr = G::Repr;

    fn from_bytes(bytes: &Self::Repr) -> CtOption<Self> {
        G::from_bytes(bytes).map(|g| Self(g))
    }

    fn from_bytes_unchecked(bytes: &Self::Repr) -> CtOption<Self> {
        G::from_bytes_unchecked(bytes).map(|g| Self(g))
    }

    fn to_bytes(&self) -> Self::Repr {
        self.0.to_bytes()
    }
}

impl<G> PrimeCurveAffine for Affine<G>
where
    G: PrimeCurve<Affine = Self, Scalar = Scalar> + Default + ConditionallySelectable,
{
    type Scalar = Scalar;

    type Curve = G;

    fn identity() -> Self {
        Self(G::identity())
    }

    fn generator() -> Self {
        Self(G::generator())
    }

    fn is_identity(&self) -> Choice {
        self.0.is_identity()
    }

    fn to_curve(&self) -> Self::Curve {
        self.0
    }
}

impl<G> UncompressedEncoding for Affine<G>
where
    Self: GroupEncoding,
{
    type Uncompressed = <Self as GroupEncoding>::Repr;

    fn from_uncompressed(bytes: &Self::Uncompressed) -> CtOption<Self> {
        Self::from_bytes(bytes)
    }

    fn from_uncompressed_unchecked(bytes: &Self::Uncompressed) -> CtOption<Self> {
        Self::from_bytes_unchecked(bytes)
    }

    fn to_uncompressed(&self) -> Self::Uncompressed {
        self.to_bytes()
    }
}
