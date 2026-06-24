use crate::backend::BigFloat;

pub fn sqrt2(prec: u32) -> BigFloat {
    sqrt(prec, 2)
}

pub fn sqrt3(prec: u32) -> BigFloat {
    sqrt(prec, 3)
}

pub fn sqrt5(prec: u32) -> BigFloat {
    sqrt(prec, 5)
}

fn sqrt(prec: u32, v: u32) -> BigFloat {
    // 1 decimal digit ≈ 3.32 bits. Add guard bits to prevent rounding errors.
    let binary_prec = ((prec as f64 * std::f64::consts::LOG2_10).ceil() as u32) + 32;

    // Start with a highly accurate 64-bit seed for 1/sqrt(2)
    let mut cur_prec = 64;
    let mut y = BigFloat::with_val(cur_prec, 1. / (v as f64).sqrt());
    let mut bi_pro = BigFloat::with_val(cur_prec, 1_i32);
    let c1 = BigFloat::with_val(binary_prec, 1.5);
    let c2 = BigFloat::with_val(binary_prec, v as f64 / 2.);

    while cur_prec < binary_prec {
        cur_prec = u32::min(cur_prec * 2, binary_prec);

        // Mutate precision in-place to allocate only what is needed for this step
        bi_pro.set_prec(cur_prec);
        y.set_prec(cur_prec);

        // Newton step for inverse sqrt: y = y * (1.5 - y^2)
        bi_pro.assign(&y);
        bi_pro.pow_assign(3);
        bi_pro *= &c2;
        y *= &c1;
        y -= &bi_pro;
    }

    y * v
}
