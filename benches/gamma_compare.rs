use std::hint::black_box;

use criterion::{BenchmarkId, Criterion};

use apcn::gamma;

fn bench_gamma(c: &mut Criterion) {
    let mut group = c.benchmark_group("gamma");
    for digits in [1_000, 10_000, 100_000] {
        group.bench_with_input(BenchmarkId::new("apcn", digits), &digits, |b, &digits| {
            b.iter(|| black_box(gamma::compute(digits)))
        });

        group.bench_with_input(
            BenchmarkId::new("apcn_parallel", digits),
            &digits,
            |b, &digits| b.iter(|| black_box(gamma::compute_parallel(digits))),
        );
    }
    group.finish();
}

pub fn bench(c: &mut Criterion) {
    bench_gamma(c);
}
