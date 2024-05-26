use core::ops::{Add, AddAssign};

use librelic_sys::{wrapper_pc_map, wrapper_pc_map_sim};
use pairing::{Engine, MillerLoopResult, MultiMillerLoop, PairingCurveAffine};

use crate::{gt::new_wrapper, G1Affine, G1Projective, G2Affine, G2Projective, Gt, Scalar};

#[derive(Debug, Clone)]
pub struct BLS12Engine;

impl Engine for BLS12Engine {
    type Fr = Scalar;

    type G1 = G1Projective;

    type G1Affine = G1Affine;

    type G2 = G2Projective;

    type G2Affine = G2Affine;

    type Gt = Gt;

    #[inline]
    fn pairing(p: &Self::G1Affine, q: &Self::G2Affine) -> Self::Gt {
        Self::projective_pairing(p.as_ref(), q.as_ref())
    }
}

impl BLS12Engine {
    #[inline]
    pub fn projective_pairing(p: &G1Projective, q: &G2Projective) -> Gt {
        let mut gt = new_wrapper();
        unsafe {
            wrapper_pc_map(&mut gt, &p.0, &q.0);
        }
        gt.into()
    }

    pub fn projective_multi_miller_loop(terms: &[(&G1Projective, &G2Projective)]) -> Gt {
        let mut g1s = Vec::with_capacity(terms.len());
        let mut g2s = Vec::with_capacity(terms.len());
        terms.iter().for_each(|(g1, g2)| {
            g1s.push((*g1).into());
            g2s.push((*g2).into());
        });

        let mut gt = new_wrapper();
        unsafe {
            wrapper_pc_map_sim(&mut gt, g1s.as_ptr(), g2s.as_ptr(), terms.len());
        }
        gt.into()
    }
}

pub fn pairing(p: &G1Affine, q: &G2Affine) -> Gt {
    BLS12Engine::pairing(p, q)
}

impl PairingCurveAffine for G1Affine {
    type Pair = G2Affine;

    type PairingResult = Gt;

    fn pairing_with(&self, other: &Self::Pair) -> Self::PairingResult {
        BLS12Engine::pairing(self, other)
    }
}

impl PairingCurveAffine for G2Affine {
    type Pair = G1Affine;

    type PairingResult = Gt;

    fn pairing_with(&self, other: &Self::Pair) -> Self::PairingResult {
        BLS12Engine::pairing(other, self)
    }
}

impl MultiMillerLoop for BLS12Engine {
    type G2Prepared = G2Affine;

    type Result = MultiMillerLoopResult;

    fn multi_miller_loop(terms: &[(&Self::G1Affine, &Self::G2Prepared)]) -> Self::Result {
        let mut g1s = Vec::with_capacity(terms.len());
        let mut g2s = Vec::with_capacity(terms.len());
        terms.iter().for_each(|(g1, g2)| {
            g1s.push((*g1).into());
            g2s.push((*g2).into());
        });

        let mut gt = new_wrapper();
        unsafe {
            wrapper_pc_map_sim(&mut gt, g1s.as_ptr(), g2s.as_ptr(), terms.len());
        }
        MultiMillerLoopResult(gt.into())
    }
}

#[derive(Debug, Clone, Copy, Default)]
pub struct MultiMillerLoopResult(Gt);

impl MillerLoopResult for MultiMillerLoopResult {
    type Gt = Gt;

    fn final_exponentiation(&self) -> Self::Gt {
        self.0
    }
}

impl Add for MultiMillerLoopResult {
    type Output = MultiMillerLoopResult;

    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0)
    }
}

impl Add<&Self> for MultiMillerLoopResult {
    type Output = MultiMillerLoopResult;

    fn add(self, rhs: &Self) -> Self::Output {
        Self(self.0 + rhs.0)
    }
}

impl AddAssign for MultiMillerLoopResult {
    fn add_assign(&mut self, rhs: Self) {
        self.0 += rhs.0;
    }
}

impl AddAssign<&Self> for MultiMillerLoopResult {
    fn add_assign(&mut self, rhs: &Self) {
        self.0 += rhs.0;
    }
}

#[cfg(test)]
mod test {
    use pairing::group::Group;

    use super::*;

    #[test]
    fn pair() {
        let mut rng = rand::thread_rng();
        let g1: G1Affine = G1Projective::random(&mut rng).into();
        let g2: G2Affine = G2Projective::random(&mut rng).into();

        assert_eq!(g1.pairing_with(&g2), g2.pairing_with(&g1));
    }
}
