use std::hint::black_box;

use apcn::backend::BigFloat;
use criterion::{BenchmarkId, Criterion};

use apcn::e;

fn bench_euler(c: &mut Criterion) {
    let mut group = c.benchmark_group("euler");
    for digits in [1_000, 10_000, 100_000, 1_000_000] {
        let bits = (digits as f64 * std::f64::consts::LOG2_10).ceil() as u32;

        group.bench_with_input(
            BenchmarkId::new("backend_default", digits),
            &bits,
            |b, &bits| b.iter(|| black_box(BigFloat::with_val(bits, 1).exp())),
        );

        group.bench_with_input(BenchmarkId::new("apcn", digits), &digits, |b, &digits| {
            b.iter(|| black_box(e::compute(digits)))
        });

        group.bench_with_input(
            BenchmarkId::new("apcn_parallel", digits),
            &digits,
            |b, &digits| b.iter(|| black_box(e::compute_parallel(digits))),
        );
    }
    group.finish();
}

pub fn bench(c: &mut Criterion) {
    bench_euler(c);
}
