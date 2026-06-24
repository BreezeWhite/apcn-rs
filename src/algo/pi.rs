use crate::backend::{BigFloat, BigInt};

use crate::bs_utils::{BSMergeType, BinarySplit, binary_splitting, binary_splitting_parallel};

struct Pi;

impl BinarySplit for Pi {
    fn compute_bs_base(a: u64) -> (BigInt, BigInt, BigInt) {
        let mut pab = BigInt::from(1);
        let mut qab = BigInt::from(1);
        if a != 0 {
            pab.assign(6 * a - 5);
            pab *= 2 * a - 1;
            pab *= 6 * a - 1;
            qab.assign(a);
            qab = qab.pow(3);
            qab *= 10939058860032000_u64;
        }

        let mut tab = BigInt::from(13591409 + 545140134 * a);
        tab *= &pab;
        if a % 2 == 1 {
            tab *= -1;
        }

        (pab, qab, tab)
    }

    fn bs_finalize(binary_prec: u32, p: BigInt, q: BigInt, t: BigInt) -> BigFloat {
        let _p = p;
        let mut pi = BigFloat::with_val(binary_prec, q);
        pi /= t;
        pi *= 426880_u32;
        pi *= BigFloat::with_val(binary_prec, 10005).sqrt();
        pi
    }

    fn merge_type() -> BSMergeType {
        BSMergeType::DEFAULT
    }
}

pub fn compute(prec: u32) -> BigFloat {
    let binary_prec = (prec as f64 * std::f64::consts::LOG2_10).round() as u32 + 5;
    let terms = prec as u64 / 14 + 1;
    binary_splitting::<Pi>(binary_prec, terms)
}

pub fn compute_parallel(prec: u32) -> BigFloat {
    let binary_prec = (prec as f64 * std::f64::consts::LOG2_10).round() as u32 + 10;
    let terms = prec as u64 / 14 + 1;
    binary_splitting_parallel::<Pi>(binary_prec, terms)
}
