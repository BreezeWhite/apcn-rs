use criterion::{Criterion, criterion_group, criterion_main};

mod e_compare;
mod ln_compare;
mod phi_compare;
mod pi_compare;
mod sqrt_compare;
mod gamma_compare;

criterion_group!(
    name = benches;
    config = Criterion::default().sample_size(10); // fewer samples for slow high-digit runs
    targets =
        e_compare::bench,
        ln_compare::bench,
        pi_compare::bench,
        sqrt_compare::bench,
        phi_compare::bench,
        gamma_compare::bench,
);

criterion_main!(benches);
