use std::{fmt::Debug, iter::Sum};

use bls12_381_relic::{ff::Field, pairing_sum, G1Projective, G2Projective, Gt, Scalar};
use pairing::group::Group;

trait Signer<Signature, Message> {
    type Error: Debug;

    fn try_sign(&self, msg: &Message) -> Result<Signature, Self::Error>;

    fn sign(&self, msg: &Message) -> Signature {
        self.try_sign(msg).unwrap()
    }
}

trait Verifier<Signature, Message> {
    type Error: Debug;

    fn verify(&self, msg: &Message, sigma: &Signature) -> Result<(), Self::Error>;
}

struct PrivateKey<const N: usize>([Scalar; N]);

struct PublicKey<const N: usize>([G2Projective; N]);

struct Signature {
    z: G1Projective,
    y: G1Projective,
    yhat: G2Projective,
}

impl<const N: usize> PrivateKey<N> {
    fn new() -> Self {
        let mut rng = rand::thread_rng();
        let mut scalars = [Scalar::ZERO; N];
        scalars.iter_mut().for_each(|s| {
            *s = Scalar::random(&mut rng);
        });

        Self(scalars)
    }

    fn to_public_key(&self) -> PublicKey<N> {
        let mut pks = [G2Projective::default(); N];
        (0..N).for_each(|i| {
            pks[i] = G2Projective::generator() * self.0[i];
        });
        PublicKey(pks)
    }
}

impl<const N: usize> Signer<Signature, [G1Projective; N]> for PrivateKey<N> {
    type Error = ();

    fn try_sign(&self, msg: &[G1Projective; N]) -> Result<Signature, Self::Error> {
        let mut rng = rand::thread_rng();
        let y = Scalar::random(&mut rng);

        let z = G1Projective::sum(msg.iter().zip(self.0.iter())) * y;
        let yinv = y.invert().unwrap();
        Ok(Signature {
            z,
            y: G1Projective::generator() * yinv,
            yhat: G2Projective::generator() * yinv,
        })
    }
}

impl<const N: usize> Verifier<Signature, [G1Projective; N]> for PublicKey<N> {
    type Error = ();

    fn verify(&self, msg: &[G1Projective; N], sigma: &Signature) -> Result<(), Self::Error> {
        let base = [(&-sigma.z, &sigma.yhat)];
        let first = pairing_sum(base.into_iter().chain(msg.iter().zip(self.0.iter())));
        let second = pairing_sum([
            (sigma.y, G2Projective::generator()),
            (-G1Projective::generator(), sigma.yhat),
        ]);

        if first == Gt::identity() && second == first {
            Ok(())
        } else {
            Err(())
        }
    }
}

#[test]
fn spseq() {
    const N: usize = 16;

    let sk = PrivateKey::<N>::new();
    let pk = sk.to_public_key();

    let mut msg = [G1Projective::default(); N];
    let mut rng = rand::thread_rng();
    msg.iter_mut().for_each(|m| {
        *m = G1Projective::random(&mut rng);
    });

    let sigma = sk.sign(&msg);
    assert!(pk.verify(&msg, &sigma).is_ok());
}
