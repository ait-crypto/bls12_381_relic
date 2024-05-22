use librelic_sys::wrapper_pc_map;
use pairing::{Engine, PairingCurveAffine};

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

    fn pairing(p: &Self::G1Affine, q: &Self::G2Affine) -> Self::Gt {
        let mut gt = new_wrapper();
        unsafe {
            wrapper_pc_map(&mut gt, &p.0 .0, &q.0 .0);
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
