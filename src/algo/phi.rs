use crate::algo::sqrt::sqrt5;
use crate::backend::{BigFloat, BigInt};
use rayon;

type Matrix = Vec<Vec<BigInt>>;

fn matrix_mul(a: &Matrix, b: &Matrix) -> Matrix {
    vec![
        vec![
            &a[0][0] * &b[0][0] + &a[0][1] * &b[1][0],
            &a[0][0] * &b[0][1] + &a[0][1] * &b[1][1],
        ],
        vec![
            &a[1][0] * &b[0][0] + &a[1][1] * &b[1][0],
            &a[1][0] * &b[0][1] + &a[1][1] * &b[1][1],
        ],
    ]
}

fn matrix_mul_para(a: &Matrix, b: &Matrix) -> Matrix {
    let (((m0, m1), (m2, m3)), ((m4, m5), (m6, m7))) = rayon::join(
        || {
            rayon::join(
                || rayon::join(|| &a[0][0] * &b[0][0], || &a[0][1] * &b[1][0]),
                || rayon::join(|| &a[0][0] * &b[0][1], || &a[0][1] * &b[1][1]),
            )
        },
        || {
            rayon::join(
                || rayon::join(|| &a[1][0] * &b[0][0], || &a[1][1] * &b[1][0]),
                || rayon::join(|| &a[1][0] * &b[0][1], || &a[1][1] * &b[1][1]),
            )
        },
    );
    vec![vec![m0 + m1, m2 + m3], vec![m4 + m5, m6 + m7]]
}

fn binary_splitting(
    mut t_mx: Matrix,
    mut power: u32,
    op: fn(&Matrix, &Matrix) -> Matrix,
) -> Matrix {
    let mut res: Matrix = vec![vec![1.into(), 0.into()], vec![0.into(), 1.into()]];
    while power > 0 {
        if power % 2 == 1 {
            res = op(&res, &t_mx);
        }
        t_mx = op(&t_mx, &t_mx);
        power /= 2;
    }
    res
}

// Use Fibonacci series to calculate phi.
pub fn compute(prec: u32) -> BigFloat {
    let terms = ((prec as f64 * 4.79) + 10.) as u32;
    let t_mx = vec![vec![1.into(), 1.into()], vec![1.into(), 0.into()]];
    let t_out = binary_splitting(t_mx, terms, matrix_mul);

    let binary_prec = ((prec as f64 * std::f64::consts::LOG2_10).ceil() as u32) + 32;

    let mut out = BigFloat::with_val(binary_prec, &t_out[0][0]);
    out /= &t_out[1][0];
    out
}

pub fn compute_parallel(prec: u32) -> BigFloat {
    // Use half of the CPU cores to compute.
    let thread_cnt = std::thread::available_parallelism()
        .map(|n| n.get())
        .unwrap()
        / 2;
    let thread_cnt = thread_cnt.max(1);

    // Configure rayon thread pool.
    rayon::ThreadPoolBuilder::new()
        .num_threads(thread_cnt)
        .build_global()
        .unwrap_or(());

    let terms = ((prec as f64 * 4.79) + 10.) as u32;
    let t_mx = vec![vec![1.into(), 1.into()], vec![1.into(), 0.into()]];
    let t_out = binary_splitting(t_mx, terms, matrix_mul_para);

    let binary_prec = ((prec as f64 * std::f64::consts::LOG2_10).ceil() as u32) + 32;

    let mut out = BigFloat::with_val(binary_prec, &t_out[0][0]);
    out /= &t_out[1][0];
    out
}

// Direct calculation to phi.
pub fn compute_phi(prec: u32) -> BigFloat {
    // phi = (1 + sqrt(5)) / 2
    let binary_prec = ((prec as f64 * std::f64::consts::LOG2_10).ceil() as u32) + 32;

    let mut phi = sqrt5(prec);
    phi += BigFloat::with_val(binary_prec, 1);
    phi /= 2;
    phi
}
