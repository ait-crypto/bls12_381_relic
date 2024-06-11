//! # BLS12-381 from [relic]
//!
//! This crate provides a [pairing]-compatible wrapper for BLS12-381 provided as
//! by relic.
//!
//! ```
//! use relic_rs::{G1Projective, G2Projective, Scalar, pairing};
//! use relic_rs::exports::{group::Group, ff::Field};
//!
//! let base = G1Projective::hash_to_curve(b"my message", b"public parameters");
//! let secret = Scalar::random(rand::thread_rng());
//! let pk = G2Projective::generator() * secret;
//!
//! let sigma = base * secret;
//! assert_eq!(pairing(sigma, G2Projective::generator()), pairing(base, pk));
//! ```
//!
//! ## Additional feature
//!
//! ## Notation
//!
//! The [pairing] uses additive notation for all groups, this crate follows the
//! same convention. This is especially noticeable in the names of some
//! functions. Instead of talking about pairing products, the same idea is
//! referred to as pairing sums or sums of pairings.
//!
//! [relic]: https://github.com/relic-toolkit/relic

#![cfg_attr(all(not(feature = "std"), not(test)), no_std)]
#![warn(missing_docs)]

mod affine;
pub mod engine;
pub mod g1;
pub mod g2;
pub mod gt;
pub mod scalar;

/// Re-exports of used crates
pub mod exports {
    pub use pairing;
    pub use pairing::group;
    pub use pairing::group::ff;
    pub use subtle;
}

pub(crate) use affine::Affine;
#[cfg(feature = "alloc")]
pub use engine::multi_pairing;
pub use engine::{pairing, RelicEngine};
pub use g1::{G1Affine, G1Projective};
pub use g2::{G2Affine, G2Projective};
pub use gt::Gt;
pub use scalar::Scalar;

/// Error type
///
/// This enum covers all errors that are produced by the crate.
#[derive(Debug)]
#[cfg_attr(feature = "std", derive(thiserror::Error))]
pub enum Error {
    /// Relic failure
    #[cfg_attr(feature = "std", error("Relic failure: {0}"))]
    RelicError(i32),
    /// Invalid byte representation of group elements or scalars
    #[cfg_attr(feature = "std", error("Invalid representation as bytes."))]
    InvalidBytesRepresentation,
}
