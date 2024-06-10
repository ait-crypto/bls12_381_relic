#[cfg(feature = "alloc")]
extern crate alloc;
#[cfg(feature = "alloc")]
use alloc::vec::Vec;

use librelic_sys::wrapper_pc_map;
#[cfg(feature = "alloc")]
use librelic_sys::wrapper_pc_map_sim;
use pairing::{Engine, PairingCurveAffine};
#[cfg(feature = "alloc")]
use pairing::{MillerLoopResult, MultiMillerLoop};

use crate::{gt::new_wrapper, G1Affine, G1Projective, G2Affine, G2Projective, Gt, Scalar};

/// Relic-based [Engine]
#[derive(Debug, Clone)]
pub struct RelicEngine;

impl Engine for RelicEngine {
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

impl RelicEngine {
    /// Compute pairing of a point in group `G1` a point in group `G2``
    #[inline]
    pub fn projective_pairing(p: &G1Projective, q: &G2Projective) -> Gt {
        let mut gt = new_wrapper();
        unsafe {
            wrapper_pc_map(&mut gt, &p.0, &q.0);
        }
        gt.into()
    }

    /// Compute multiple pairings and their sum
    #[cfg(feature = "alloc")]
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

/// Compute pairing of a point in `G1` and one in `G2`
#[inline]
pub fn pairing<G1, G2>(p: G1, q: G2) -> Gt
where
    G1: AsRef<G1Projective>,
    G2: AsRef<G2Projective>,
{
    RelicEngine::projective_pairing(p.as_ref(), q.as_ref())
}

impl PairingCurveAffine for G1Affine {
    type Pair = G2Affine;

    type PairingResult = Gt;

    #[inline]
    fn pairing_with(&self, other: &Self::Pair) -> Self::PairingResult {
        RelicEngine::pairing(self, other)
    }
}

impl PairingCurveAffine for G2Affine {
    type Pair = G1Affine;

    type PairingResult = Gt;

    #[inline]
    fn pairing_with(&self, other: &Self::Pair) -> Self::PairingResult {
        RelicEngine::pairing(other, self)
    }
}

#[cfg(feature = "alloc")]
impl MultiMillerLoop for RelicEngine {
    type G2Prepared = G2Affine;

    type Result = Gt;

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
        gt.into()
    }
}

#[cfg(feature = "alloc")]
impl MillerLoopResult for Gt {
    type Gt = Gt;

    fn final_exponentiation(&self) -> Self::Gt {
        *self
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
