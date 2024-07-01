//! Test based on the BLS signature scheme

use bls12_381_relic::{
    ff::Field,
    group::{prime::PrimeCurveAffine, Curve, Group},
    pairing_sum, G1Affine, G1Projective, G2Affine, G2Projective, Gt, Scalar,
};
use signature::{Error, Signer, Verifier};

const HASH_SEPERATOR: &[u8] = b"BLS";

/// BLS private key
#[derive(Debug)]
struct PrivateKey(Scalar);

impl PrivateKey {
    fn new() -> Self {
        Self(Scalar::random(rand::thread_rng()))
    }

    fn to_public_key(&self) -> PublicKey {
        PublicKey(G2Projective::generator() * self.0)
    }

    fn to_affine_public_key(&self) -> AffinePublicKey {
        AffinePublicKey((G2Projective::generator() * self.0).to_affine())
    }
}

impl Signer<Signature> for PrivateKey {
    fn try_sign(&self, msg: &[u8]) -> Result<Signature, Error> {
        Ok(self.sign(msg))
    }

    fn sign(&self, msg: &[u8]) -> Signature {
        Signature(G1Projective::hash_to_curve(msg, HASH_SEPERATOR) * self.0)
    }
}

impl Signer<AffineSignature> for PrivateKey {
    fn try_sign(&self, msg: &[u8]) -> Result<AffineSignature, Error> {
        Ok(self.sign(msg))
    }

    fn sign(&self, msg: &[u8]) -> AffineSignature {
        AffineSignature((G1Projective::hash_to_curve(msg, HASH_SEPERATOR) * self.0).to_affine())
    }
}

/// BLS public key
#[derive(Debug)]
struct PublicKey(G2Projective);

/// BLS public key
#[derive(Debug)]
struct AffinePublicKey(G2Affine);

impl Verifier<Signature> for PublicKey {
    fn verify(&self, msg: &[u8], signature: &Signature) -> Result<(), Error> {
        // Instead of comparing the results of two pairings compute a pairing-sum and check if it the identity in Gt.
        // e(H(msg), pk) == e(sigma, h) <=> e(H(msg), pk) - e(sigma, h) == 0 <=> e(-H(msg), pk) + e(sigma, h) == 0
        let base_point = -G1Projective::hash_to_curve(msg, HASH_SEPERATOR);
        if pairing_sum([
            (base_point, self.0),
            (signature.0, G2Projective::generator()),
        ]) == Gt::identity()
        {
            Ok(())
        } else {
            Err(Error::new())
        }
    }
}

impl Verifier<AffineSignature> for AffinePublicKey {
    fn verify(&self, msg: &[u8], signature: &AffineSignature) -> Result<(), Error> {
        // Instead of comparing the results of two pairings compute a pairing-sum and check if it the identity in Gt.
        // e(H(msg), pk) == e(sigma, h) <=> e(H(msg), pk) - e(sigma, h) == 0 <=> e(-H(msg), pk) + e(sigma, h) == 0
        let base_point = (-G1Projective::hash_to_curve(msg, HASH_SEPERATOR)).to_affine();
        if pairing_sum([(base_point, self.0), (signature.0, G2Affine::generator())])
            == Gt::identity()
        {
            Ok(())
        } else {
            Err(Error::new())
        }
    }
}

/// BLS signature
#[derive(Debug)]
struct Signature(G1Projective);

/// BLS signature
#[derive(Debug)]
struct AffineSignature(G1Affine);

#[test]
fn bls_signature() {
    let sk = PrivateKey::new();
    let pk = sk.to_public_key();

    let sigma: Signature = sk.sign(b"this is the message");
    assert!(
        pk.verify(b"this is the message", &sigma).is_ok(),
        "valid signature failed to verify"
    );
    assert!(
        pk.verify(b"this is another message", &sigma).is_err(),
        "invalid signature verified"
    );

    let pk = PrivateKey::new().to_public_key();
    assert!(
        pk.verify(b"this is the message", &sigma).is_err(),
        "invalid signature verified"
    );
}

#[test]
fn affine_bls_signature() {
    let sk = PrivateKey::new();
    let pk = sk.to_affine_public_key();

    let sigma: AffineSignature = sk.sign(b"this is the message");
    assert!(
        pk.verify(b"this is the message", &sigma).is_ok(),
        "valid signature failed to verify"
    );
    assert!(
        pk.verify(b"this is another message", &sigma).is_err(),
        "invalid signature verified"
    );

    let pk = PrivateKey::new().to_affine_public_key();
    assert!(
        pk.verify(b"this is the message", &sigma).is_err(),
        "invalid signature verified"
    );
}
