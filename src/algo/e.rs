use crate::backend::{BigFloat, BigInt};

use crate::bs_utils::{BSMergeType, BinarySplit, binary_splitting, binary_splitting_parallel};

struct Euler;

impl BinarySplit for Euler {
    fn compute_bs_base(a: u64) -> (BigInt, BigInt, BigInt) {
        if a == 0 {
            return (BigInt::from(1), BigInt::from(1), BigInt::from(1));
        }
        (BigInt::from(1), BigInt::from(a), BigInt::from(1))
    }

    fn bs_finalize(binary_prec: u32, p: BigInt, q: BigInt, t: BigInt) -> BigFloat {
        let _p = p;
        let mut e = BigFloat::with_val(binary_prec, t);
        e /= q;
        e
    }

    fn merge_type() -> BSMergeType {
        BSMergeType::DEFAULT
    }
}

fn estimate_terms(prec: u32) -> u64 {
    let d = prec as f64 * std::f64::consts::LN_10;

    // Initial estimate via inverse Stirling
    let ln_d = d.ln();
    let mut n = d / (ln_d - ln_d.ln());

    // One Newton-Raphson refinement: solve N·ln(N/e) = D
    n = n - (n * n.ln() - n - d) / n.ln();

    n.ceil() as u64 + 2
}

pub fn compute(prec: u32) -> BigFloat {
    let binary_prec = (prec as f64 * std::f64::consts::LOG2_10).round() as u32 + 5;
    let terms = estimate_terms(prec);

    binary_splitting::<Euler>(binary_prec, terms)
}

pub fn compute_parallel(prec: u32) -> BigFloat {
    let binary_prec = (prec as f64 * std::f64::consts::LOG2_10).round() as u32 + 5;
    let terms = estimate_terms(prec);
    binary_splitting_parallel::<Euler>(binary_prec, terms)
}
