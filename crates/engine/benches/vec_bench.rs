use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};

fn vec_bench(c: &mut Criterion) {
    let mut group = c.benchmark_group("Vec vs Matrix");
    const WIDTH: usize = 200;
    const HEIGHT: usize = 200;
    const IDXS: [(usize, usize); 5] = [(0, 10), (0, 199), (100, 100), (199, 10), (199, 199)];

    for (x, y) in IDXS.iter() {
        let vec = vec![0; WIDTH * HEIGHT];
        let matrix = vec![vec![0; HEIGHT]; WIDTH];
        let name = format!("(x:{} y:{})", x, y);

        group.bench_with_input(BenchmarkId::new("Vec", &name), &(*x, *y), |b, (x, y)| {
            b.iter(|| {
                let var = vec[*x + *y * WIDTH];
            })
        });
        group.bench_with_input(BenchmarkId::new("Matrix", &name), &(*x, *y), |b, (x, y)| {
            b.iter(|| {
                let var = matrix[*x][*y];
            })
        });
    }
    group.finish();
}

criterion_group!(benches, vec_bench);
criterion_main!(benches);
