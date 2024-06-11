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
///
/// `G1` can be elements from [G1Projective] or [G1Affine] (or references) and
/// `G2` can be elements from [G2Projective] or [G2Affine] (or references).
///
/// ```
/// use relic_rs::{G1Affine, G2Affine, G1Projective, G2Projective, pairing};
/// use relic_rs::exports::group::Group;
///
/// let g1 = G1Projective::generator();
/// let g2 = G2Projective::generator();
///
/// assert_eq!(pairing(g1, g2), pairing(G1Affine::from(&g1), G2Affine::from(&g2)));
/// ```
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

    #[inline]
    fn final_exponentiation(&self) -> Self::Gt {
        *self
    }
}

/// Compute sum of multiple pairings
///
/// ```
/// use relic_rs::{G1Affine, G2Affine, G1Projective, G2Projective, pairing, Scalar, multi_pairing};
/// use relic_rs::exports::group::Group;
///
/// let g1 = G1Projective::generator();
/// let g2 = G2Projective::generator();
///
/// let elements = [(g1, g2), (g1 * Scalar::from(2), g2 * Scalar::from(7))];
///
/// assert_eq!(pairing(elements[0].0, elements[0].1) + pairing(elements[1].0, elements[1].1), multi_pairing(elements));
/// ```
#[cfg(feature = "alloc")]
pub fn multi_pairing<I, G1, G2>(iter: I) -> Gt
where
    I: IntoIterator<Item = (G1, G2)>,
    G1: AsRef<G1Projective>,
    G2: AsRef<G2Projective>,
{
    let iter = iter.into_iter();
    let iter_len = iter.size_hint().0;

    let mut g1s = Vec::with_capacity(iter_len);
    let mut g2s = Vec::with_capacity(iter_len);
    iter.for_each(|(g1, g2)| {
        g1s.push(g1.as_ref().into());
        g2s.push(g2.as_ref().into());
    });

    let mut gt = new_wrapper();
    unsafe {
        wrapper_pc_map_sim(&mut gt, g1s.as_ptr(), g2s.as_ptr(), g1s.len());
    }
    gt.into()
}

#[cfg(test)]
mod test {
    use pairing::group::Group;

    use super::*;

    #[test]
    fn pair() {
        let mut rng = rand::thread_rng();
        let g1 = G1Affine::from(G1Projective::random(&mut rng));
        let g2 = G2Affine::from(G2Projective::random(&mut rng));

        assert_eq!(g1.pairing_with(&g2), g2.pairing_with(&g1));
    }

    #[cfg(feature = "alloc")]
    #[test]
    fn multi_pair() {
        let mut rng = rand::thread_rng();
        let elements = [
            (
                G1Projective::random(&mut rng),
                G2Projective::random(&mut rng),
            ),
            (
                G1Projective::random(&mut rng),
                G2Projective::random(&mut rng),
            ),
        ];

        let check = pairing(elements[0].0, elements[0].1) + pairing(elements[1].0, elements[1].1);
        let pp = multi_pairing(elements);
        assert_eq!(check, pp);

        let elements = elements.map(|(g1, g2)| (G1Affine::from(g1), G2Affine::from(g2)));
        let pp = multi_pairing(elements);
        assert_eq!(check, pp);
    }
}
