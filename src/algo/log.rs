use std::f64;

use crate::backend::{BigFloat, BigInt};
use crate::bs_utils::{
    BSMergeType, BinarySplit, BinarySplitGeneric, binary_splitting, binary_splitting_parallel,
};
use crate::pi;

#[allow(dead_code)]
mod agm_ln {
    // AGM implementation to compute log value.
    // Slower than binary splitting approach, thus not used.
    use super::*;

    fn agm(mut a: BigFloat, mut b: BigFloat, prec: u32) -> BigFloat {
        let iters = (prec as f64).log2().ceil() as u32 + 12;
        let binary_prec = ((prec as f64 * std::f64::consts::LOG2_10).ceil() as u32) + 32;
        let mut tmp = BigFloat::with_val(binary_prec, &a);
        for _ in 0..iters {
            tmp.assign(&a);
            a += &b;
            a /= 2;
            b *= &tmp;
            b.sqrt_mut();
        }
        (a + b) / 2
    }

    fn compute_ln(prec: u32) -> BigFloat {
        let binary_prec = ((prec as f64 * f64::consts::LOG2_10).ceil() as u32) + 32;
        let pi_v = pi::compute(prec);
        let m = ((prec as f64) * f64::consts::LOG2_10 / 2. + 10.).ceil() as u32;
        let s2 = BigInt::from(2).pow(m);
        let sx = BigInt::from(&s2) * 2;

        let log_x2m = pi_v.clone()
            / (2_u32
                * agm(
                    BigFloat::with_val(binary_prec, 1),
                    BigFloat::with_val(binary_prec, 4) / sx,
                    prec,
                ));

        let log_2 = pi_v
            / (2_u32
                * m
                * agm(
                    BigFloat::with_val(binary_prec, 1),
                    BigFloat::with_val(binary_prec, 4) / s2,
                    prec,
                ));

        log_x2m - m * log_2
    }
}

struct Ln2;

impl BinarySplit for Ln2 {
    fn compute_bs_base(a: u64) -> (BigInt, BigInt, BigInt) {
        (BigInt::from(2 * a + 1), BigInt::from(9), BigInt::from(9))
    }

    fn bs_finalize(binary_prec: u32, p: BigInt, q: BigInt, t: BigInt) -> BigFloat {
        let deno = 3_u32 * q * p;
        let mut nume = BigFloat::with_val(binary_prec, 2_u32 * t);
        nume /= &deno;
        nume
    }

    fn merge_type() -> BSMergeType {
        BSMergeType::LOG
    }
}

pub fn ln2(prec: u32) -> BigFloat {
    let binary_prec = ((prec as f64 * std::f64::consts::LOG2_10).ceil() as u32) + 32;
    let terms = ((prec as f64 + 5.) / 0.9542425094393249) as u64;
    binary_splitting::<Ln2>(binary_prec, terms)
}

pub fn ln2_parallel(prec: u32) -> BigFloat {
    let binary_prec = ((prec as f64 * std::f64::consts::LOG2_10).ceil() as u32) + 32;
    let terms = ((prec as f64 + 5.) / 0.9542425094393249) as u64;
    binary_splitting_parallel::<Ln2>(binary_prec, terms)
}

// Alias to ln2
pub fn compute(prec: u32) -> BigFloat {
    ln2(prec)
}

// Alias to ln2_parallel
pub fn compute_parallel(prec: u32) -> BigFloat {
    ln2_parallel(prec)
}

// ----------------- Generic Log Computation ------------------ //

pub fn ln3(prec: u32) -> BigFloat {
    generic_log::compute_ln(3., prec)
}

pub fn ln3_parallel(prec: u32) -> BigFloat {
    generic_log::compute_ln_parallel(3., prec)
}

pub fn ln5(prec: u32) -> BigFloat {
    generic_log::compute_ln(5., prec)
}

pub fn ln5_parallel(prec: u32) -> BigFloat {
    generic_log::compute_ln_parallel(5., prec)
}

pub mod generic_log {
    use super::*;

    fn log10_estimate(val: &BigInt) -> f64 {
        if *val == BigInt::from(0) {
            return f64::NEG_INFINITY;
        }
        let s = val.to_string();
        if s.len() <= 15 {
            let f: f64 = s.parse().unwrap_or(1.0);
            f.log10()
        } else {
            let first_digits: f64 = s[..15].parse().unwrap_or(1.0);
            first_digits.log10() + (s.len() - 15) as f64
        }
    }

    /// Converts a finite `f64` into its exact rational fraction `(numerator, denominator)` represented as `BigInt`.
    /// Returns `None` for NaN or Infinite values.
    fn f64_to_big_fraction(val: f64) -> Option<(BigInt, BigInt)> {
        if !val.is_finite() {
            return None;
        }
        if val == 0.0 {
            return Some((BigInt::from(0), BigInt::from(1)));
        }
        let bits = val.to_bits();
        let is_negative = (bits >> 63) == 1;
        let exponent_raw = ((bits >> 52) & 0x7ff) as i16;
        let mantissa_raw = bits & 0xfffffffffffff;

        let (mut mantissa, exponent) = if exponent_raw == 0 {
            // Subnormal numbers
            (mantissa_raw, -1022 - 52)
        } else {
            // Normalized numbers (include the implicit leading 1 bit)
            (mantissa_raw | 0x10000000000000, exponent_raw - 1023 - 52)
        };

        let mut num: BigInt;
        let den: BigInt;

        if exponent >= 0 {
            num = BigInt::from(mantissa) * BigInt::from(2).pow(exponent as u32);
            den = BigInt::from(1);
        } else {
            let k = (-exponent) as u32;
            let tz = mantissa.trailing_zeros();
            let p = tz.min(k);
            mantissa >>= p;
            num = BigInt::from(mantissa);
            den = BigInt::from(2).pow(k - p);
        }

        if is_negative {
            num *= -1_i32;
        }

        Some((num, den))
    }

    struct GenericLogBS {
        u2: BigInt,
        v2: BigInt,
    }

    impl BinarySplitGeneric for GenericLogBS {
        type Value = (BigInt, BigInt, BigInt, BigInt);

        fn compute_base(&self, a: u64) -> Self::Value {
            (
                self.u2.clone(),
                self.v2.clone(),
                BigInt::from((2 * a + 1) as u64),
                self.v2.clone(),
            )
        }

        fn merge(&self, left: Self::Value, right: Self::Value) -> Self::Value {
            let (pl, mut ql, mut bl, mut tl) = left;
            let (mut pr, qr, br, mut tr) = right;

            pr *= &pl;
            ql *= &qr;
            tl *= &br;
            tl *= &qr;
            tr *= &pl;
            tr *= &bl;
            bl *= br;

            (pr, ql, bl, tl + tr)
        }
    }

    fn calculate_log_f(u: &BigInt, v: &BigInt, prec: u32) -> (BigInt, BigInt) {
        if *u == BigInt::from(0) {
            return (BigInt::from(0), BigInt::from(1));
        }

        let u2 = u.clone() * u.clone();
        let v2 = v.clone() * v.clone();
        let u_abs = u.abs();
        let u_log = log10_estimate(&u_abs);
        let v_log = log10_estimate(v);
        let term_ratio = v_log - u_log;
        let mut terms = ((prec as f64 + 5.) / 2. / term_ratio).ceil() as u32;
        terms = terms.max(1);

        let context = GenericLogBS { u2, v2 };
        let (_p, q, b, t) =
            crate::bs_utils::sub_binary_splitting_generic(0, terms as u64, &context);

        let num_log_f = t * u.clone() * 2_u64;
        let den_log_f = b * q * v.clone();
        (num_log_f, den_log_f)
    }

    fn calculate_log_f_parallel(u: &BigInt, v: &BigInt, prec: u32) -> (BigInt, BigInt) {
        if *u == BigInt::from(0) {
            return (BigInt::from(0), BigInt::from(1));
        }

        let u2 = u.clone() * u.clone();
        let v2 = v.clone() * v.clone();
        let u_abs = u.abs();
        let u_log = log10_estimate(&u_abs);
        let v_log = log10_estimate(v);
        let term_ratio = v_log - u_log;
        let mut terms = ((prec as f64 + 5.) / 2. / term_ratio).ceil() as u32;
        terms = terms.max(1);

        let context = GenericLogBS { u2, v2 };
        let (_p, q, b, t) =
            crate::bs_utils::binary_splitting_generic_parallel(&context, terms as u64);

        let num_log_f = t * u.clone() * 2_u64;
        let den_log_f = b * q * v.clone();
        (num_log_f, den_log_f)
    }

    pub fn compute_ln(x: f64, prec: u32) -> BigFloat {
        if x <= 0.0 {
            panic!("Logarithm argument must be positive.");
        }
        let binary_prec = ((prec as f64 * std::f64::consts::LOG2_10).ceil() as u32) + 32;
        if x == 1.0 {
            return BigFloat::with_val(binary_prec, 0);
        }

        let mut m = x.log2().round() as i32;
        let (num_x_big, den_x_big) = f64_to_big_fraction(x).unwrap();

        let mut num_f: BigInt;
        let mut den_f: BigInt;
        if m >= 0 {
            num_f = num_x_big;
            den_f = den_x_big * BigInt::from(2).pow(m as u32);
        } else {
            num_f = num_x_big * BigInt::from(2).pow(m.abs() as u32);
            den_f = den_x_big;
        }

        let mut f_approx = x;
        let mut temp_m = m;
        while temp_m > 1000 {
            f_approx *= 2.0_f64.powi(-1000);
            temp_m -= 1000;
        }
        while temp_m < -1000 {
            f_approx *= 2.0_f64.powi(1000);
            temp_m += 1000;
        }
        f_approx *= 2.0_f64.powi(-temp_m);

        while f_approx < 0.7071067811865475 {
            m -= 1;
            num_f *= 2_u64;
            f_approx *= 2.0;
        }
        while f_approx >= 1.414213562373095 {
            m += 1;
            den_f *= 2_u64;
            f_approx /= 2.0;
        }

        let uf = num_f.clone() - den_f.clone();
        let vf = num_f + den_f;

        let (num_log_f, den_log_f) = calculate_log_f(&uf, &vf, prec);

        let mut num_total = num_log_f.clone();
        let mut den_total = den_log_f.clone();
        if m != 0 {
            let (num_log2, den_log2) = calculate_log_f(&BigInt::from(1), &BigInt::from(3), prec);
            let mut term1 = num_log2;
            term1 *= &den_log_f;
            term1 *= m;

            let mut term2 = num_log_f;
            term2 *= &den_log2;

            num_total = term1 + term2;
            den_total *= den_log2;
        }

        let is_negative = num_total < BigInt::from(0);
        let num_abs = num_total.abs();
        let mut val = BigFloat::with_val(binary_prec, &num_abs);
        val /= den_total;
        if is_negative {
            val *= -1_i32;
        }

        val
    }

    pub fn compute_ln_parallel(x: f64, prec: u32) -> BigFloat {
        if x <= 0.0 {
            panic!("Logarithm argument must be positive.");
        }
        let binary_prec = ((prec as f64 * std::f64::consts::LOG2_10).ceil() as u32) + 32;
        if x == 1.0 {
            return BigFloat::with_val(binary_prec, 0);
        }

        let mut m = x.log2().round() as i32;
        let (num_x_big, den_x_big) = f64_to_big_fraction(x).unwrap();

        let mut num_f: BigInt;
        let mut den_f: BigInt;
        if m >= 0 {
            num_f = num_x_big;
            den_f = den_x_big * BigInt::from(2).pow(m as u32);
        } else {
            num_f = num_x_big * BigInt::from(2).pow(m.abs() as u32);
            den_f = den_x_big;
        }

        let mut f_approx = x;
        let mut temp_m = m;
        while temp_m > 1000 {
            f_approx *= 2.0_f64.powi(-1000);
            temp_m -= 1000;
        }
        while temp_m < -1000 {
            f_approx *= 2.0_f64.powi(1000);
            temp_m += 1000;
        }
        f_approx *= 2.0_f64.powi(-temp_m);

        while f_approx < 0.7071067811865475 {
            m -= 1;
            num_f *= 2_u64;
            f_approx *= 2.0;
        }
        while f_approx >= 1.414213562373095 {
            m += 1;
            den_f *= 2_u64;
            f_approx /= 2.0;
        }

        let uf = num_f.clone() - den_f.clone();
        let vf = num_f + den_f;

        let (num_log_f, den_log_f) = calculate_log_f_parallel(&uf, &vf, prec);

        let mut num_total = num_log_f.clone();
        let mut den_total = den_log_f.clone();
        if m != 0 {
            let (num_log2, den_log2) =
                calculate_log_f_parallel(&BigInt::from(1), &BigInt::from(3), prec);
            let mut term1 = num_log2;
            term1 *= &den_log_f;
            term1 *= m;

            let mut term2 = num_log_f;
            term2 *= &den_log2;

            num_total = term1 + term2;
            den_total *= den_log2;
        }

        let is_negative = num_total < BigInt::from(0);
        let num_abs = num_total.abs();
        let mut val = BigFloat::with_val(binary_prec, &num_abs);
        val /= den_total;
        if is_negative {
            val *= -1_i32;
        }

        val
    }
}
