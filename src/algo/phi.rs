use crate::backend::{BigInt, BigFloat};


type Matrix = Vec<Vec<BigInt>>;

fn matrix_mul(a: &Matrix, b: &Matrix) -> Matrix {
    vec![
        vec![&a[0][0] * &b[0][0] + &a[0][1] * &b[1][0], &a[0][0] * &b[0][1] + &a[0][1] * &b[1][1]],
        vec![&a[1][0] * &b[0][0] + &a[1][1] * &b[1][0], &a[1][0] * &b[0][1] + &a[1][1] * &b[1][1]]
    ]
}

fn binary_splitting(mut t_mx: Matrix, mut power: u32) -> Matrix {
    let mut res: Matrix = vec![vec![1.into(), 0.into()], vec![0.into(), 1.into()]];
    while power > 0 {
        if power % 2 == 1 {
            res = matrix_mul(&res, &t_mx);
        }
        t_mx = matrix_mul(&t_mx, &t_mx);
        power /= 2;
    }
    res
}

pub fn compute(prec: u32) -> BigFloat {
    let terms = ((prec as f64 * 4.79) + 10.) as u32;
    let t_mx = vec![vec![1.into(), 1.into()], vec![1.into(), 0.into()]];
    let t_out = binary_splitting(t_mx, terms);

    let binary_prec = ((prec as f64 * std::f64::consts::LOG2_10).ceil() as u32) + 32;

    let mut out = BigFloat::with_val(binary_prec, &t_out[0][0]);
    out /= &t_out[1][0];
    out
}