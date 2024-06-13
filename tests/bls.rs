use bls12_381_relic::{ff::Field, pair, G1Projective, G2Projective, Scalar};
use pairing::group::Group;
use signature::{Error, Signer, Verifier};

struct PrivateKey(Scalar);

impl PrivateKey {
    fn new() -> Self {
        Self(Scalar::random(rand::thread_rng()))
    }

    fn to_public_key(&self) -> PublicKey {
        PublicKey(G2Projective::identity() * self.0)
    }
}

impl Signer<Signature> for PrivateKey {
    fn try_sign(&self, msg: &[u8]) -> Result<Signature, Error> {
        let base_point = G1Projective::hash_to_curve(msg, b"BLS");
        Ok(Signature(base_point * self.0))
    }
}

struct PublicKey(G2Projective);

impl Verifier<Signature> for PublicKey {
    fn verify(&self, msg: &[u8], signature: &Signature) -> Result<(), Error> {
        let base_point = G1Projective::hash_to_curve(msg, b"BLS");
        if pair(base_point, self.0) == pair(signature.0, G2Projective::identity()) {
            Ok(())
        } else {
            Err(Error::new())
        }
    }
}

struct Signature(G1Projective);

#[test]
fn bls_signature() {
    let sk = PrivateKey::new();
    let pk = sk.to_public_key();

    let sigma = sk.sign(b"this is the message");
    assert!(pk.verify(b"this is the message", &sigma).is_ok());
}
