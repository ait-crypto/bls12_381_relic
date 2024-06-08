//! BLS12-381 from relic
//!
//! This crate provides a `pairing`-compatible wrapper for BLS12-381 provided by relic.

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
pub use engine::{pairing, RelicEngine};
pub use g1::{G1Affine, G1Projective};
pub use g2::{G2Affine, G2Projective};
pub use gt::Gt;
pub use scalar::Scalar;

/// Error type
///
/// This enum covers all errors that are produced by the crate.
#[derive(thiserror::Error, Debug)]
pub enum Error {
    /// Relic failure
    #[error("Relic failure: {0}")]
    RelicError(i32),
    /// Invalid byte representation of group elements or scalars
    #[error("Invalid representation as bytes.")]
    InvalidBytesRepresentation,
}
