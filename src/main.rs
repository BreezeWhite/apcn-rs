use std::time::Instant;

use apcn::backend::BigFloat;
use apcn::cli::{Actions, Cli};
use apcn::{e, gamma, log, phi, pi, sqrt};
use clap::Parser;

#[cfg(feature = "rug")]
const BACKEND: &str = "rug";

#[cfg(feature = "dashu")]
const BACKEND: &str = "dashu";

fn get_func(action: Actions, parallel: bool) -> Box<dyn Fn(u32) -> BigFloat> {
    match action {
        Actions::Pi => {
            if parallel {
                Box::new(pi::compute_parallel)
            } else {
                Box::new(pi::compute)
            }
        }
        Actions::E => {
            if parallel {
                Box::new(e::compute_parallel)
            } else {
                Box::new(e::compute)
            }
        }
        Actions::Ln2 => {
            if parallel {
                Box::new(log::ln2_parallel)
            } else {
                Box::new(log::ln2)
            }
        }
        Actions::Ln3 => {
            if parallel {
                Box::new(log::ln3_parallel)
            } else {
                Box::new(log::ln3)
            }
        }
        Actions::Ln5 => {
            if parallel {
                Box::new(log::ln5_parallel)
            } else {
                Box::new(log::ln5)
            }
        }
        Actions::Sqrt2 => Box::new(sqrt::sqrt2),
        Actions::Sqrt3 => Box::new(sqrt::sqrt3),
        Actions::Sqrt5 => Box::new(sqrt::sqrt5),
        Actions::Phi => {
            if parallel {
                Box::new(phi::compute_parallel)
            } else {
                Box::new(phi::compute_phi)
            }
        }
        Actions::Gamma => {
            if parallel {
                Box::new(gamma::compute_parallel)
            } else {
                Box::new(gamma::compute)
            }
        }
        Actions::Exp { x } => {
            if parallel {
                Box::new(move |prec| e::exp_parallel(x, prec))
            } else {
                Box::new(move |prec| e::exp(x, prec))
            }
        }
        Actions::Ln { x } => {
            if parallel {
                Box::new(move |prec| log::generic_log::compute_ln_parallel(x, prec))
            } else {
                Box::new(move |prec| log::generic_log::compute_ln(x, prec))
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
        let n = out_str.len();
        println!("{}", &out_str[..(n.min(arg.digits as usize + 2))]);
        // println!("{}", out_str);
    }

    if arg.bench {
        println!("Elapsed: {:#?}", start.elapsed());
    }
}

fn main_test() {
    let prec = 100000;
    let binary_prec = ((prec as f64 * std::f64::consts::LOG2_10).ceil() as u32) + 32;

    let start = Instant::now();
    let val = gamma::compute_parallel(prec);
    let dura = start.elapsed();
    // println!("{}", val.to_fixed_string());
    println!("Compute parallel duration: {dura:#?}");

    let start = Instant::now();
    let _ = val.to_string();
    let dura = start.elapsed();
    println!("Format duration: {dura:#?}");

    // -----
    let start = Instant::now();
    let val = rug::Float::with_val(binary_prec, rug::float::Constant::Euler);
    let dura = start.elapsed();
    // println!("{val}");
    // println!("{}", val.to_fixed_string());
    println!("Compute generic duration: {dura:#?}");

    let start = Instant::now();
    let _ = val.to_string();
    let dura = start.elapsed();
    println!("Format duration: {dura:#?}");
}
