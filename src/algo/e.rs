use crate::backend::{BigFloat, BigInt};
use crate::algo::bs_utils::{
    BSMergeType, BinarySplit, BinarySplitGeneric, binary_splitting,
    binary_splitting_parallel, sub_binary_splitting_generic,
    binary_splitting_generic_parallel,
};

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

// ----------------- General Exp Implementation ----------------- //

struct ExpBS {
    a: BigInt,
    b: BigInt,
}

impl BinarySplitGeneric for ExpBS {
    type Value = (BigInt, BigInt, BigInt);

    fn compute_base(&self, k: u64) -> Self::Value {
        let q_val = self.b.clone() * BigInt::from(k + 1);
        (
            self.a.clone(),
            q_val.clone(),
            q_val,
        )
    }

    fn merge(&self, left: Self::Value, right: Self::Value) -> Self::Value {
        let (pl, ql, tl) = left;
        let (pr, qr, tr) = right;

        let mut p_new = pl.clone();
        p_new *= &pr;

        let mut q_new = ql.clone();
        q_new *= &qr;

        // T = T_L * Q_R + P_L * T_R
        let mut term1 = tl;
        term1 *= &qr;

        let mut term2 = tr;
        term2 *= &pl;

        let t_new = term1 + term2;

        (p_new, q_new, t_new)
    }
}

pub fn exp(x: f64, prec: u32) -> BigFloat {
    let binary_prec = (prec as f64 * std::f64::consts::LOG2_10).round() as u32 + 32;
    if x == 0.0 {
        return BigFloat::with_val(binary_prec, 1);
    }

    let m = (x / std::f64::consts::LN_2).round() as i32;
    let ln2 = crate::algo::log::ln2(prec);
    
    let mut m_ln2 = ln2.clone();
    m_ln2 *= m;

    let x_bf = BigFloat::with_val(binary_prec, x);
    let r = x_bf - m_ln2;

    let b_int = BigInt::from(2).pow(binary_prec);
    let mut a_float = r.clone();
    a_float *= &BigFloat::with_val(binary_prec, &b_int);
    let a_int = a_float.to_int();

    let terms = estimate_terms(prec);
    let context = ExpBS {
        a: a_int,
        b: b_int,
    };

    let (_p, q, t) = sub_binary_splitting_generic(0, terms, &context);

    let mut val = BigFloat::with_val(binary_prec, &t);
    val /= &q;

    if m != 0 {
        if m >= 0 {
            let p2 = BigInt::from(2).pow(m as u32);
            let factor = BigFloat::with_val(binary_prec, &p2);
            val *= &factor;
        } else {
            let p2 = BigInt::from(2).pow(m.abs() as u32);
            let factor = BigFloat::with_val(binary_prec, &p2);
            val /= &factor;
        }
    }

    val
}

pub fn exp_parallel(x: f64, prec: u32) -> BigFloat {
    let binary_prec = (prec as f64 * std::f64::consts::LOG2_10).round() as u32 + 32;
    if x == 0.0 {
        return BigFloat::with_val(binary_prec, 1);
    }

    let m = (x / std::f64::consts::LN_2).round() as i32;
    let ln2 = crate::algo::log::ln2_parallel(prec);
    
    let mut m_ln2 = ln2.clone();
    m_ln2 *= m;

    let x_bf = BigFloat::with_val(binary_prec, x);
    let r = x_bf - m_ln2;

    let b_int = BigInt::from(2).pow(binary_prec);
    let mut a_float = r.clone();
    a_float *= &BigFloat::with_val(binary_prec, &b_int);
    let a_int = a_float.to_int();

    let terms = estimate_terms(prec);
    let context = ExpBS {
        a: a_int,
        b: b_int,
    };

    let (_p, q, t) = binary_splitting_generic_parallel(&context, terms);

    let mut val = BigFloat::with_val(binary_prec, &t);
    val /= &q;

    if m != 0 {
        if m >= 0 {
            let p2 = BigInt::from(2).pow(m as u32);
            let factor = BigFloat::with_val(binary_prec, &p2);
            val *= &factor;
        } else {
            let p2 = BigInt::from(2).pow(m.abs() as u32);
            let factor = BigFloat::with_val(binary_prec, &p2);
            val /= &factor;
        }
    }

    val
}
