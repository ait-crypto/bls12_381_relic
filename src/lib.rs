//! # BLS12-381 from [relic]
//!
//! This crate provides a [pairing]-compatible wrapper for BLS12-381 provided as
//! by relic.
//!
//! ```
//! use bls12_381_relic::{G1Projective, G2Projective, Scalar, pair};
//! use bls12_381_relic::{group::Group, ff::Field};
//!
//! let base = G1Projective::hash_to_curve(b"my message", b"public parameters");
//! let secret = Scalar::random(rand::thread_rng());
//! let pk = G2Projective::generator() * secret;
//!
//! let sigma = base * secret;
//! assert_eq!(pair(sigma, G2Projective::generator()), pair(base, pk));
//! ```
//!
//! The goal is to be as compatible with the interface defined by [pairing] and
//! implemented by [bls12_381] crate as possible. There are however some notable
//! differences where concepts of [pairing] have no mapping in [relic]. Some
//! examples of the differences include:
//! * [G1Affine] and [G2Affine] are thin wrappers of their projective
//!   counterparts since [relic] does not have separate types for affine
//!   representations and associated functions.
//! * There is no "prepared" variant of elements in `G2` for multi-miller-loops.
//!
//! ## Additional features
//!
//! The crate provides multi-product sums for pairs of group elements and
//! scalars that is faster then evaluating the scalar multiplications and
//! additions separately.
//!
//! ```
//! use bls12_381_relic::{G1Projective, Scalar};
//! use bls12_381_relic::{group::Group, ff::Field};
//! use core::iter::Sum;
//!
//! let mut rng = rand::thread_rng();
//! let v1 = G1Projective::random(&mut rng);
//! let v2 = G1Projective::random(&mut rng);
//! let v3 = G1Projective::random(&mut rng);
//! let s1 = Scalar::random(&mut rng);
//! let s2 = Scalar::random(&mut rng);
//! let s3 = Scalar::random(&mut rng);
//! assert_eq!(
//!     G1Projective::sum([(v1, s1), (v2, s2), (v3, s3)].iter()),
//!     v1 * s1 + v2 * s2 + v3 * s3
//! );
//! ```
//!
//! This speed-up is only available if the `alloc` feature is enabled.
//!
//! ## Notation
//!
//! The [pairing] crate uses additive notation for all groups, thus this crate
//! follows the same convention. This is especially noticeable in the names of
//! some functions. Instead of talking about pairing products, the same idea is
//! referred to as pairing sums or sums of pairings.
//!
//! [relic]: https://github.com/relic-toolkit/relic
//! [bls12_381]: https://crates.io/crates/bls12_381

#![cfg_attr(all(not(feature = "std"), not(test)), no_std)]
#![warn(missing_docs)]

#[cfg(feature = "alloc")]
extern crate alloc;
#[cfg(feature = "alloc")]
use alloc::vec::Vec;

/// Re-exports of used crates
pub use pairing;
pub use pairing::group;
pub use pairing::group::ff;
pub use subtle;

pub mod affine;
pub mod engine;
pub mod g1;
pub mod g2;
pub mod gt;
pub mod scalar;
#[cfg(feature = "serde")]
mod serde_helpers;

pub(crate) use affine::Affine;
pub use engine::RelicEngine;
pub use g1::{G1Affine, G1Projective};
pub use g2::{G2Affine, G2Projective};
pub use gt::Gt;
pub use scalar::Scalar;

/// Error type
///
/// This enum covers all errors that are produced by this crate.
#[derive(Debug)]
#[cfg_attr(feature = "std", derive(thiserror::Error))]
pub enum Error {
    /// Relic failure
    #[cfg_attr(feature = "std", error("Error from relic: {0}"))]
    RelicError(i32),
    /// Invalid byte representation of group elements or scalars
    #[cfg_attr(feature = "std", error("Invalid representation as bytes."))]
    InvalidBytesRepresentation,
}

/// Compute pairing of a point in `G1` and one in `G2`
///
/// `G1` can be elements from [G1Projective] or [G1Affine] (or references) and
/// `G2` can be elements from [G2Projective] or [G2Affine] (or references).
///
/// ```
/// use bls12_381_relic::{G1Affine, G2Affine, G1Projective, G2Projective, pair};
/// use bls12_381_relic::group::Group;
///
/// let g1 = G1Projective::generator();
/// let g2 = G2Projective::generator();
///
/// assert_eq!(pair(g1, g2), pair(G1Affine::from(&g1), G2Affine::from(&g2)));
/// ```
#[inline]
pub fn pair<G1, G2>(p: G1, q: G2) -> Gt
where
    G1: AsRef<G1Projective>,
    G2: AsRef<G2Projective>,
{
    RelicEngine::projective_pairing(p.as_ref(), q.as_ref())
}

/// Compute sum of multiple pairings
///
/// ```
/// use bls12_381_relic::{G1Affine, G2Affine, G1Projective, G2Projective, pair, Scalar, pairing_sum};
/// use bls12_381_relic::group::Group;
///
/// let g1 = G1Projective::generator();
/// let g2 = G2Projective::generator();
///
/// let elements = [(g1, g2), (g1 * Scalar::from(2), g2 * Scalar::from(7))];
///
/// assert_eq!(
///     pair(elements[0].0, elements[0].1) + pair(elements[1].0, elements[1].1),
///     pairing_sum(elements)
/// );
/// ```
pub fn pairing_sum<I, G1, G2>(iter: I) -> Gt
where
    I: IntoIterator<Item = (G1, G2)>,
    G1: AsRef<G1Projective>,
    G2: AsRef<G2Projective>,
{
    #[cfg(feature = "alloc")]
    {
        use gt::new_wrapper;
        use librelic_sys::wrapper_pc_map_sim;

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

    #[cfg(not(feature = "alloc"))]
    {
        use pairing::group::Group;

        iter.into_iter()
            .fold(Gt::identity(), |a, (g1, g2)| a + pair(g1, g2))
    }
}

pub(crate) const RANDOM_DOMAIN_SEPERATOR: &[u8; 32] = b"randrandrandrandrandrandrandrand";

#[cfg(test)]
mod test {
    use pairing::group::Group;

    use super::*;

    #[test]
    fn pair_generators() {
        let g1 = G1Projective::generator();
        let g2 = G2Projective::generator();
        let s = Scalar::from(127);

        assert_eq!(pair(g1, g2), Gt::generator());
        assert_eq!(pair(g1 * s, g2), Gt::generator() * s);
        assert_eq!(pair(g1, g2 * s), Gt::generator() * s);
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

        let check = pair(elements[0].0, elements[0].1) + pair(elements[1].0, elements[1].1);
        let pp = pairing_sum(elements);
        assert_eq!(check, pp);

        let elements = elements.map(|(g1, g2)| (G1Affine::from(g1), G2Affine::from(g2)));
        let pp = pairing_sum(elements);
        assert_eq!(check, pp);
    }
}
