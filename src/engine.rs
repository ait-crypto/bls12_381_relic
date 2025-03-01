//! Pairing-engine based on relic
//!
//! The `pairing` crate defines an [Engine] that collects the scalar field,
//! projective and affine representations of the source groups and the target
//! groups. Most importantly, it also provides the [Engine::pairing] function to
//! compute the pairing.
//!
//! In addition to the engine, this module also provides some additional
//! functions that evaluate the pairing on projective coordinations and sums of
//! pairings (as `Gt` is also using additive notation).

#[cfg(feature = "alloc")]
extern crate alloc;
#[cfg(feature = "alloc")]
use alloc::vec::Vec;

use librelic_sys::wrapper_pc_map;
#[cfg(feature = "alloc")]
use librelic_sys::wrapper_pc_map_sim;
#[cfg(not(feature = "alloc"))]
use pairing::group::Group;
use pairing::{Engine, MillerLoopResult, MultiMillerLoop, PairingCurveAffine};

use crate::{G1Affine, G1Projective, G2Affine, G2Projective, Gt, Scalar, gt::new_wrapper};

/// Relic-based [Engine]
///
/// The only purpose of this struct is to implement the [Engine] to use with
/// relic's implementation of the pairing-friendly BLS12-381 curve.
/// Additionally, it also provides the multi-Miller-loop ([MultiMillerLoop])
/// functionality whereas the speed-up is only provided if the `alloc` feature
/// is enabled.`
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
    /// Compute pairing of a point in group `G1` a point in group `G2`
    #[inline]
    pub fn projective_pairing(p: &G1Projective, q: &G2Projective) -> Gt {
        let mut gt = new_wrapper();
        unsafe {
            wrapper_pc_map(&mut gt, &p.0, &q.0);
        }
        gt.into()
    }

    /// Compute multiple pairings and their sum
    pub fn projective_multi_miller_loop(terms: &[(&G1Projective, &G2Projective)]) -> Gt {
        #[cfg(feature = "alloc")]
        {
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

        #[cfg(not(feature = "alloc"))]
        {
            terms.iter().fold(Gt::identity(), |a, (g1, g2)| {
                a + Self::projective_pairing(g1, g2)
            })
        }
    }
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

impl MultiMillerLoop for RelicEngine {
    // there is no prepared version
    type G2Prepared = G2Affine;

    type Result = Gt;

    #[cfg(feature = "alloc")]
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

    #[cfg(not(feature = "alloc"))]
    fn multi_miller_loop(terms: &[(&Self::G1Affine, &Self::G2Prepared)]) -> Self::Result {
        terms
            .iter()
            .fold(Gt::identity(), |a, (g1, g2)| a + super::pair(*g1, *g2))
    }
}

impl MillerLoopResult for Gt {
    type Gt = Gt;

    #[inline]
    fn final_exponentiation(&self) -> Self::Gt {
        *self
    }
}

#[cfg(test)]
mod test {
    use crate::{group::Group, pairing_sum};

    use super::*;

    #[test]
    fn projective_pairing() {
        let mut rng = rand::thread_rng();
        let g1 = G1Projective::random(&mut rng);
        let g2 = G2Projective::random(&mut rng);

        assert_eq!(
            RelicEngine::projective_pairing(&g1, &g2),
            RelicEngine::pairing(&G1Affine::from(g1), &G2Affine::from(g2))
        );
    }

    #[test]
    fn pair_with() {
        let mut rng = rand::thread_rng();
        let g1 = G1Affine::from(G1Projective::random(&mut rng));
        let g2 = G2Affine::from(G2Projective::random(&mut rng));

        assert_eq!(g1.pairing_with(&g2), g2.pairing_with(&g1));
        assert_eq!(RelicEngine::pairing(&g1, &g2), g2.pairing_with(&g1));
    }

    #[test]
    fn multi_miller_loop() {
        let mut rng = rand::thread_rng();
        let g1s = [
            G1Affine::from(G1Projective::random(&mut rng)),
            G1Affine::from(G1Projective::random(&mut rng)),
        ];
        let g2s = [
            G2Affine::from(G2Projective::random(&mut rng)),
            G2Affine::from(G2Projective::random(&mut rng)),
        ];
        let terms = [(&g1s[0], &g2s[0]), (&g1s[1], &g2s[1])];

        let mml = RelicEngine::multi_miller_loop(&terms).final_exponentiation();
        let check = pairing_sum(terms);

        assert_eq!(check, mml);
    }
}
