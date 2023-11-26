use criterion::{black_box, criterion_group, criterion_main, Criterion};
use rand::rngs::SmallRng;
use rustfall_engine::pixel::steam::Steam;
use rustfall_engine::pixel::water::Water;
use rustfall_engine::sandbox::Sandbox;

pub fn liquid_benchmark(c: &mut Criterion) {
    c.bench_function("liquid tick", |b| {
        b.iter_custom(|iters| {
            let mut duration = std::time::Duration::new(0, 0);
            for _ in 0..iters {
                let mut sandbox = black_box(Sandbox::<SmallRng>::new(200, 200));
                for i in 50..150 {
                    sandbox.place_pixel_force(Water::default(), i, 0);
                }

                let start = std::time::Instant::now();
                for _ in 0..200 {
                    sandbox.tick();
                }
                duration += start.elapsed();
            }
            duration
        })
    });
}
pub fn gas_benchmark(c: &mut Criterion) {
    c.bench_function("gas tick", |b| {
        b.iter_custom(|iters| {
            let mut duration = std::time::Duration::new(0, 0);
            for _ in 0..iters {
                let mut sandbox = black_box(Sandbox::<SmallRng>::new(200, 200));
                for i in 50..150 {
                    sandbox.place_pixel_force(Steam::default(), i, 199);
                }

                let start = std::time::Instant::now();
                for _ in 0..200 {
                    sandbox.tick();
                }
                duration += start.elapsed();
            }
            duration
        })
    });
}

criterion_group!(benches, liquid_benchmark, gas_benchmark);
criterion_main!(benches);
