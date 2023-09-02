use criterion::{black_box, criterion_group, criterion_main, Criterion};

use chaikin::chaikin_n;

const INITIAL_POINTS: [(f32, f32); 3] = [(0.0, 0.0), (1.0, 1.0), (2.0, 0.0)];

const N: usize = 20;

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function(&format!("chaikin_{}", N), |b| {
        b.iter(|| chaikin_n(black_box(&INITIAL_POINTS), N))
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
