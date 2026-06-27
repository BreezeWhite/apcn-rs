use std::time::Instant;

use apcn::backend::BigFloat;
use apcn::cli::{Actions, Cli};
use apcn::{e, gamma, log, phi, pi, sqrt};
use clap::Parser;

#[cfg(feature = "rug")]
const BACKEND: &str = "rug";

#[cfg(feature = "dashu")]
const BACKEND: &str = "dashu";

fn get_func(action: Actions, parallel: bool) -> fn(u32) -> BigFloat {
    match action {
        Actions::Pi => {
            if parallel {
                pi::compute_parallel
            } else {
                pi::compute
            }
        }
        Actions::E => {
            if parallel {
                e::compute_parallel
            } else {
                e::compute
            }
        }
        Actions::Ln2 => {
            if parallel {
                log::ln2_parallel
            } else {
                log::ln2
            }
        }
        Actions::Ln3 => {
            if parallel {
                log::ln3_parallel
            } else {
                log::ln3
            }
        }
        Actions::Ln5 => {
            if parallel {
                log::ln5_parallel
            } else {
                log::ln5
            }
        }
        Actions::Sqrt2 => sqrt::sqrt2,
        Actions::Sqrt3 => sqrt::sqrt3,
        Actions::Sqrt5 => sqrt::sqrt5,
        Actions::Phi => {
            if parallel {
                phi::compute_parallel
            } else {
                phi::compute_phi
            }
        }
        Actions::Gamma => {
            if parallel {
                gamma::compute_parallel
            } else {
                gamma::compute
            }
        }
    }
}

fn main() {
    let arg = Cli::parse();

    if arg.backend {
        println!("{BACKEND}");
        return;
    }

    let action = match arg.action {
        Some(action) => action,
        None => {
            return;
        }
    };

    let func = get_func(action, arg.parallel);

    let start = Instant::now();
    let out = func(arg.digits);

    if !arg.no_print {
        let out_str = out.to_fixed_string();
        println!("{}", &out_str[..(arg.digits as usize + 2)]);
        // println!("{}", out_str);
    }

    if arg.bench {
        println!("Elapsed: {:#?}", start.elapsed());
    }
}

fn main_test() {
    let prec = 100;
    let binary_prec = ((prec as f64 * std::f64::consts::LOG2_10).ceil() as u32) + 32;

    let x = 2;

    let start = Instant::now();
    // let val = log::compute(prec);
    // let val = BigFloat::with_val(binary_prec, x).ln();
    // let val = compute_ln_parallel(x as f64, prec);
    // let val = pi::compute(prec);
    // let val = phi::compute_parallel(prec);
    let val = gamma::compute_parallel(prec);
    let dura = start.elapsed();
    println!("{}", val.to_fixed_string());
    println!("Compute parallel duration: {dura:#?}");

    let start = Instant::now();
    let _ = val.to_string();
    let dura = start.elapsed();
    println!("Format duration: {dura:#?}");

    // -----
    // let start = Instant::now();
    // let val = compute_ln(x as f64, prec);
    // let val = compute_phi(binary_prec);
    // let val = 2f64.ln();
    // let val = rug::Float::with_val(binary_prec, rug::float::Constant::Euler);
    // let dura = start.elapsed();
    // println!("{val}");
    // // println!("{}", val.to_fixed_string());
    // println!("Compute generic duration: {dura:#?}");

    // let start = Instant::now();
    // let _ = val.to_string();
    // let dura = start.elapsed();
    // println!("Format duration: {dura:#?}");
}
