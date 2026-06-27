use crate::backend::{BigFloat, BigInt};
use crate::algo::bs_utils::{
    BinarySplitGeneric, binary_splitting_generic_parallel, sub_binary_splitting_generic,
};
use crate::algo::log::generic_log;

struct GammaBS {
    n2: BigInt,
}

impl BinarySplitGeneric for GammaBS {
    type Value = (BigInt, BigInt, BigInt, BigInt, BigInt, BigInt); // P, Q, V, T, W, U

    fn compute_base(&self, a: u64) -> Self::Value {
        let k = BigInt::from(a + 1);
        let k2 = k.clone() * k;
        let k_val = BigInt::from(a + 1);
        (
            self.n2.clone(),
            k2,
            k_val,
            self.n2.clone(),
            BigInt::from(1),
            self.n2.clone(),
        )
    }

    fn merge(&self, left: Self::Value, right: Self::Value) -> Self::Value {
        let (pl, ql, vl, tl, wl, ul) = left;
        let (pr, qr, vr, tr, wr, ur) = right;

        let mut p = pl.clone();
        p *= &pr;

        let mut q = ql.clone();
        q *= &qr;

        let mut v = vl.clone();
        v *= &vr;

        // T = T_L * Q_R + P_L * T_R
        let mut t = tl;
        t *= &qr;
        let mut p_tr = pl.clone();
        p_tr *= &tr;
        t += p_tr;

        // W = W_L * V_R + W_R * V_L
        let mut w = wl.clone();
        w *= &vr;
        let mut w_vl = wr;
        w_vl *= &vl;
        w += w_vl;

        // U = U_L * V_R * Q_R + P_L * U_R * V_L + W_L * P_L * T_R * V_R
        let mut term1 = ul;
        term1 *= &vr;
        term1 *= &qr;

        let mut term2 = pl.clone();
        term2 *= &ur;
        term2 *= &vl;

        let mut term3 = wl;
        term3 *= &pl;
        term3 *= &tr;
        term3 *= &vr;

        let u = term1 + term2 + term3;

        (p, q, v, t, w, u)
    }
}

struct AsymptoticA {
    thirty_two_n2: BigInt,
}

impl BinarySplitGeneric for AsymptoticA {
    type Value = (BigInt, BigInt, BigInt); // P, Q, T

    fn compute_base(&self, a: u64) -> Self::Value {
        let k = a + 1;
        let mut term = BigInt::from(2 * k - 1);
        let term_cloned = term.clone();
        term *= &term_cloned;
        term *= &term_cloned; // (2k - 1)^3

        let mut q = self.thirty_two_n2.clone();
        q *= k;

        (term.clone(), q, term)
    }

    fn merge(&self, left: Self::Value, right: Self::Value) -> Self::Value {
        let (pl, ql, tl) = left;
        let (pr, qr, tr) = right;

        let mut p = pl.clone();
        p *= &pr;

        let mut q = ql;
        q *= &qr;

        let mut t = tl;
        t *= &qr;
        let mut pl_tr = pl;
        pl_tr *= &tr;
        t += pl_tr;

        (p, q, t)
    }
}

pub fn compute(prec: u32) -> BigFloat {
    calculate_euler_gamma(prec, false)
}

pub fn compute_parallel(prec: u32) -> BigFloat {
    calculate_euler_gamma(prec, true)
}

fn calculate_euler_gamma(prec: u32, parallel: bool) -> BigFloat {
    if prec < 1 {
        panic!("Precision must be at least 1.");
    }

    let binary_prec = ((prec as f64 * std::f64::consts::LOG2_10).ceil() as u32) + 64;

    let n = ((prec as f64 + 15.0) / 1.7371779276).ceil() as u64 + 5;
    let k = (4.9711 * n as f64).ceil() as u64 + 20;

    let n2 = BigInt::from(n * n);
    let bessel_ctx = GammaBS { n2 };

    let (_p, q, v, t, _wl, ul) = if parallel {
        binary_splitting_generic_parallel(&bessel_ctx, k - 1)
    } else {
        sub_binary_splitting_generic(0, k - 1, &bessel_ctx)
    };

    let thirty_two_n2_u64 = 32 * n * n;

    let mut log_term = 0.0;
    let mut m = 0;
    let target = -(prec as f64 + 15.0);
    for k_idx in 1..=(2 * n) {
        let ratio = (2 * k_idx - 1) as f64;
        let ratio_cubed = ratio * ratio * ratio;
        let denom = 32.0 * k_idx as f64 * (n * n) as f64;
        log_term += (ratio_cubed / denom).log10();
        if log_term < target {
            m = k_idx;
            break;
        }
    }
    if m == 0 {
        m = 2 * n;
    }
    let m = m.max(2);

    let thirty_two_n2_big = BigInt::from(thirty_two_n2_u64);
    let asymptotic_ctx = AsymptoticA {
        thirty_two_n2: thirty_two_n2_big,
    };

    let (_p_a, q_a, t_a) = if parallel {
        binary_splitting_generic_parallel(&asymptotic_ctx, m - 1)
    } else {
        sub_binary_splitting_generic(0, m - 1, &asymptotic_ctx)
    };

    let mut a_num = q_a.clone();
    a_num += &t_a;
    let a_den = q_a;

    let mut q_plus_t = q.clone();
    q_plus_t += &t;

    let four_n = BigInt::from(4 * n);

    let mut term1 = ul;
    term1 *= &four_n;
    term1 *= &a_den;
    term1 *= &q_plus_t;

    let mut q2 = q.clone();
    q2 *= &q;

    let mut term2 = a_num;
    term2 *= &q2;
    term2 *= &v;

    let n_common = term1 - term2;

    let mut q_plus_t_2 = q_plus_t.clone();
    q_plus_t_2 *= &q_plus_t;

    let mut d_common = four_n;
    d_common *= &v;
    d_common *= &a_den;
    d_common *= &q_plus_t_2;

    let mut gamma_bessel = BigFloat::with_val(binary_prec, &n_common);
    let d_common_f = BigFloat::with_val(binary_prec, &d_common);
    gamma_bessel /= d_common_f;

    let log_n = if parallel {
        generic_log::compute_ln_parallel(n as f64, prec)
    } else {
        generic_log::compute_ln(n as f64, prec)
    };

    gamma_bessel -= &log_n;
    gamma_bessel
}
