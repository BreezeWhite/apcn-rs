use std::hint::black_box;

use criterion::{BenchmarkId, Criterion};

use apcn::gamma;

#[cfg(feature = "rug")]
mod bench_rug {
    use super::*;
    use rug::Float;
    use rug::float::Constant;

    pub fn bench_gamma(c: &mut Criterion) {
        let mut group = c.benchmark_group("gamma");
        for digits in [1_000, 10_000, 100_000] {
            let bits = (digits as f64 * std::f64::consts::LOG2_10).ceil() as u32;

            group.bench_with_input(
                BenchmarkId::new("backend_default", digits),
                &bits,
                |b, &bits| {
                    b.iter_batched(
                        || rug::float::free_cache(rug::float::FreeCache::All),
                        |_| {
                            let gamma = Float::with_val(bits, Constant::Euler);
                            black_box(gamma)
                        },
                        criterion::BatchSize::PerIteration,
                    )
                },
            );

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
}

#[cfg(feature = "dashu")]
mod bench_dashu {
    use super::*;
    use dashu::float::FBig;
    use dashu::float::round::mode::HalfEven;

    pub fn bench_gamma(c: &mut Criterion) {
        let mut group = c.benchmark_group("gamma");
        for digits in [1_000, 10_000, 100_000] {
            let bits = (digits as f64 * std::f64::consts::LOG2_10).ceil() as usize;

            group.bench_with_input(BenchmarkId::new("apcn", digits), &digits, |b, &digits| {
                b.iter(|| black_box(gamma::compute(digits as u32)))
            });

            group.bench_with_input(
                BenchmarkId::new("apcn_parallel", digits),
                &digits,
                |b, &digits| b.iter(|| black_box(gamma::compute_parallel(digits as u32))),
            );
        }
        group.finish();
    }
}

pub fn bench(c: &mut Criterion) {
    #[cfg(feature = "rug")]
    bench_rug::bench_gamma(c);

    #[cfg(feature = "dashu")]
    bench_dashu::bench_gamma(c);
}
