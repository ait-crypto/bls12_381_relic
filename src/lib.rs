pub mod g1;
pub mod scalar;
mod utils;

// re-exports
pub use pairing;
pub use pairing::group;
pub use pairing::group::ff;
pub use subtle;

pub use g1::{G1Affine, G1};
pub use scalar::Scalar;
pub(crate) use utils::Affine;

pub enum Error {
    RelicError(i32),
    InvalidBytesRepresentation,
}
