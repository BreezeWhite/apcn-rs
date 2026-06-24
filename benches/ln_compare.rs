use std::hint::black_box;

use criterion::{BenchmarkId, Criterion};

use apcn::backend::BigFloat;
use apcn::log;

fn bench_ln2(c: &mut Criterion) {
    let mut group = c.benchmark_group("ln2");
    for digits in [1_000, 10_000, 100_000, 1_000_000] {
        let bits = (digits as f64 * std::f64::consts::LOG2_10).ceil() as u32;

        group.bench_with_input(
            BenchmarkId::new("backend_default", digits),
            &bits,
            |b, &bits| b.iter(|| black_box(BigFloat::with_val(bits, 2).ln())),
        );

        group.bench_with_input(BenchmarkId::new("apcn", digits), &digits, |b, &digits| {
            b.iter(|| black_box(log::ln2(digits)))
        });

        group.bench_with_input(
            BenchmarkId::new("apcn_parallel", digits),
            &digits,
            |b, &digits| b.iter(|| black_box(log::ln2_parallel(digits))),
        );
    }
    group.finish();
}


fn bench_ln3(c: &mut Criterion) {
    let mut group = c.benchmark_group("ln3");
    for digits in [1_000, 10_000, 100_000, 1_000_000] {
        let bits = (digits as f64 * std::f64::consts::LOG2_10).ceil() as u32;

        group.bench_with_input(
            BenchmarkId::new("backend_default", digits),
            &bits,
            |b, &bits| b.iter(|| black_box(BigFloat::with_val(bits, 3).ln())),
        );

        group.bench_with_input(BenchmarkId::new("apcn", digits), &digits, |b, &digits| {
            b.iter(|| black_box(log::ln3(digits)))
        });

        group.bench_with_input(
            BenchmarkId::new("apcn_parallel", digits),
            &digits,
            |b, &digits| b.iter(|| black_box(log::ln3_parallel(digits))),
        );
    }
    group.finish();
}

pub fn bench(c: &mut Criterion) {
    bench_ln2(c);
    bench_ln3(c);
}
