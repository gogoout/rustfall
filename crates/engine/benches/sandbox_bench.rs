use criterion::{criterion_group, criterion_main, Criterion};
use rustfall_engine::pixel::steam::Steam;
use rustfall_engine::pixel::water::Water;
use rustfall_engine::sandbox::sandbox::Sandbox;

pub fn liquid_benchmark(c: &mut Criterion) {
    c.bench_function("liquid tick", |b| {
        b.iter_batched_ref(
            || {
                let mut sandbox = Sandbox::new(300, 300);
                for i in 50..150 {
                    sandbox.place_pixel_force(Water::default(), (i, 0));
                }
                sandbox
            },
            |sandbox| {
                sandbox.tick();
            },
            criterion::BatchSize::NumIterations(200),
        );
    });
}
pub fn gas_benchmark(c: &mut Criterion) {
    c.bench_function("gas tick", |b| {
        b.iter_batched_ref(
            || {
                let mut sandbox = Sandbox::new(300, 300);
                for i in 50..150 {
                    sandbox.place_pixel_force(Steam::default(), (i, 199));
                }
                sandbox
            },
            |sandbox| {
                sandbox.tick();
            },
            criterion::BatchSize::NumIterations(200),
        );
    });
}

criterion_group!(benches, liquid_benchmark, gas_benchmark);
criterion_main!(benches);
