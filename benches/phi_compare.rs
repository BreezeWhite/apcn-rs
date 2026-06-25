use std::hint::black_box;

use apcn::backend::BigFloat;
use criterion::{BenchmarkId, Criterion};

use apcn::phi;

fn bench_phi(c: &mut Criterion) {
    let mut group = c.benchmark_group("phi");
    for digits in [1_000, 10_000, 100_000, 1_000_000, 3_000_000] {
        let bits = (digits as f64 * std::f64::consts::LOG2_10).ceil() as u32;

        group.bench_with_input(BenchmarkId::new("apcn", digits), &digits, |b, &digits| {
            b.iter(|| black_box(phi::compute_phi(digits)))
        });

        group.bench_with_input(
            BenchmarkId::new("apcn_bs", digits),
            &digits,
            |b, &digits| b.iter(|| black_box(phi::compute(digits))),
        );

        group.bench_with_input(
            BenchmarkId::new("apcn_bs_parallel", digits),
            &digits,
            |b, &digits| b.iter(|| black_box(phi::compute_parallel(digits))),
        );
    }
    group.finish();
}

pub fn bench(c: &mut Criterion) {
    bench_phi(c);
}
