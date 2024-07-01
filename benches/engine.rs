use bls12_381_relic::{ff::Field, G1Projective, G2Projective, RelicEngine};
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use pairing::{
    group::{prime::PrimeCurve, Curve, Group},
    Engine, MillerLoopResult, MultiMillerLoop,
};

fn bench_engine<E>(c: &mut Criterion, name: &str)
where
    E: Engine + MultiMillerLoop,
{
    let mut rng = rand::thread_rng();
    let g = E::G1::random(&mut rng).to_affine();
    let h = E::G2::random(&mut rng).to_affine();
    c.bench_function(&format!("{}: pairing (affine)", name), move |b| {
        b.iter(|| black_box(E::pairing(black_box(&g), black_box(&h))))
    });

    let terms: Vec<_> = (0..8)
        .map(|_| {
            (
                E::G1::random(&mut rng).to_affine(),
                E::G2Prepared::from(E::G2::random(&mut rng).to_affine()),
            )
        })
        .collect();
    let ref_terms: Vec<_> = terms.iter().map(|(g1, g2)| (g1, g2)).collect();

    c.bench_function(&format!("{}: multi miller loop (8)", name), move |b| {
        b.iter(|| black_box(E::multi_miller_loop(black_box(&ref_terms)).final_exponentiation()))
    });
}

fn bench_pairings(c: &mut Criterion) {
    bench_engine::<RelicEngine>(c, "RelicEngine");

    let mut rng = rand::thread_rng();
    let g = <RelicEngine as Engine>::G1::random(&mut rng);
    let h = <RelicEngine as Engine>::G2::random(&mut rng);
    c.bench_function("RelicEngine: pairing (projective)", move |b| {
        b.iter(|| {
            black_box(RelicEngine::projective_pairing(
                black_box(&g),
                black_box(&h),
            ))
        })
    });
}

fn bench_bls12_381_pairings(c: &mut Criterion) {
    bench_engine::<bls12_381::Bls12>(c, "Bls12");
}

fn bench_group<T>(c: &mut Criterion, name: &str)
where
    T: PrimeCurve,
{
    let mut rng = rand::thread_rng();

    let a = T::random(&mut rng);
    let s = T::Scalar::random(&mut rng) + T::Scalar::ONE;

    c.bench_function(&format!("{}: addition", name), move |b| {
        b.iter(|| black_box(black_box(a) + black_box(a)))
    });
    c.bench_function(&format!("{}: double", name), move |b| {
        b.iter(|| black_box(black_box(a).double()))
    });

    c.bench_function(&format!("{}: scalar multiplication", name), move |b| {
        b.iter(|| black_box(black_box(a) * black_box(s)))
    });
}

fn bench_g1_projective(c: &mut Criterion) {
    bench_group::<G1Projective>(c, "G1Projective");
}

fn bench_g2_projective(c: &mut Criterion) {
    bench_group::<G2Projective>(c, "G2Projective");
}

fn bench_bls12_381_g1_projective(c: &mut Criterion) {
    bench_group::<bls12_381::G1Projective>(c, "bls12_381::G1Projective");
}

fn bench_bls12_381_g2_projective(c: &mut Criterion) {
    bench_group::<bls12_381::G2Projective>(c, "bls12_381::G2Projective");
}

criterion_group!(
    benches,
    bench_g1_projective,
    bench_g2_projective,
    bench_pairings,
    bench_bls12_381_g1_projective,
    bench_bls12_381_g2_projective,
    bench_bls12_381_pairings,
);
criterion_main!(benches);
