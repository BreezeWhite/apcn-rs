use crate::backend::{BigFloat, BigInt};
use crate::algo::bs_utils::{
    BinarySplitGeneric, binary_splitting_generic_parallel, sub_binary_splitting_generic,
};

struct GammaBS {
    n2: BigInt,
}

impl BinarySplitGeneric for GammaBS {
    type Value = (BigInt, BigInt, BigInt, BigInt, BigInt); // P, V, T, W, U

    fn compute_base(&self, a: u64) -> Self::Value {
        let k_val = BigInt::from(a + 1);
        (
            self.n2.clone(),
            k_val,
            self.n2.clone(),
            BigInt::from(1),
            self.n2.clone(),
        )
    }

    fn merge(&self, left: Self::Value, right: Self::Value) -> Self::Value {
        let (pl, vl, tl, wl, ul) = left;
        let (pr, vr, tr, wr, ur) = right;

        // Since Q = V^2, we only need to compute Q_R for the merge step.
        // We use clones and MulAssign for compatibility with both backends.
        let mut qr = vr.clone();
        qr *= &vr;

        let mut p = pl.clone();
        p *= &pr;

        let mut v = vl.clone();
        v *= &vr;

        // T = T_L * Q_R + P_L * T_R
        let mut t = tl;
        t *= &qr;
        let mut p_l_t_r = pl.clone();
        p_l_t_r *= &tr;
        t += &p_l_t_r;

        // W = W_L * V_R + W_R * V_L
        let mut w_l_v_r = wl.clone();
        w_l_v_r *= &vr;
        let mut w_r_v_l = wr;
        w_r_v_l *= &vl;
        let w = w_l_v_r.clone() + w_r_v_l;

        // U = (U_L * Q_R + W_L * (P_L * T_R)) * V_R + (P_L * U_R) * V_L
        let mut u_l_q_r = ul;
        u_l_q_r *= &qr;

        let mut wl_p_l_t_r = wl;
        wl_p_l_t_r *= &p_l_t_r;

        let mut term1_3 = u_l_q_r + wl_p_l_t_r;
        term1_3 *= &vr;

        let mut term2 = pl;
        term2 *= &ur;
        term2 *= &vl;

        let u = term1_3 + term2;

        (p, v, t, w, u)
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
    let k = (4.32 * n as f64).ceil() as u64 + 5;

    let n2 = BigInt::from(n * n);
    let bessel_ctx = GammaBS { n2 };

    let (_p, v, t, _wl, ul) = if parallel {
        binary_splitting_generic_parallel(&bessel_ctx, k - 1)
    } else {
        sub_binary_splitting_generic(0, k - 1, &bessel_ctx)
    };

    // Recover Q = V^2 at the top level
    let mut q = v.clone();
    q *= &v;

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

    // Use native, highly-optimized ln implementation of the backend
    let log_n = BigFloat::with_val(binary_prec, n as u32).ln();

    gamma_bessel -= &log_n;
    gamma_bessel
}
