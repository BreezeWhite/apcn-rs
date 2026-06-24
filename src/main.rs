use std::fs::File;
use std::io::{Result, Write};
use std::time::Instant;

use apcn::backend::BigFloat;
use apcn::cli::{Actions, Cli};
use apcn::log::generic_log::{compute_ln, compute_ln_parallel};
use apcn::{e, log, pi, sqrt};
use clap::Parser;

fn gen_test_data() -> Result<()> {
    let prec = 2_000_000;
    let binary_prec = ((prec as f64 * std::f64::consts::LOG2_10).ceil() as u32) + 32;

    let x = 5;
    let start = Instant::now();
    let ss = BigFloat::with_val(binary_prec, x).ln().to_fixed_string();
    let elapsed = start.elapsed();
    println!("Elapsed: {elapsed:#?}");

    let out_path = format!("./tests/data/ln{x}_million.txt");
    let mut f = File::create(out_path)?;
    f.write_all(&ss.as_bytes()[..prec + 2])?;
    Ok(())
}

fn _main() -> Result<()> {
    let _ = gen_test_data()?;
    Ok(())
}

fn main() {
    let arg = Cli::parse();
    let func = match arg.action {
        Actions::Pi => {
            if arg.parallel {
                pi::compute_parallel
            } else {
                pi::compute
            }
        }
        Actions::E => {
            if arg.parallel {
                e::compute_parallel
            } else {
                e::compute
            }
        }
        Actions::Ln2 => {
            if arg.parallel {
                log::ln2_parallel
            } else {
                log::ln2
            }
        }
        Actions::Ln3 => {
            if arg.parallel {
                log::ln3_parallel
            } else {
                log::ln3
            }
        }
        Actions::Ln5 => {
            if arg.parallel {
                log::ln5_parallel
            } else {
                log::ln5
            }
        }
        Actions::Sqrt2 => sqrt::sqrt2,
        Actions::Sqrt3 => sqrt::sqrt3,
        Actions::Sqrt5 => sqrt::sqrt5,
    };

    let start = Instant::now();
    let out = func(arg.digits);

    let out_str = out.to_fixed_string();
    println!("{}", &out_str[..(arg.digits as usize + 2)]);
    
    if arg.bench {
        println!("Elapsed: {:#?}", start.elapsed());
    }
}

fn main_sub() {
    let prec = 1_000_000;
    let binary_prec = ((prec as f64 * std::f64::consts::LOG2_10).ceil() as u32) + 32;

    let x = 2;

    let start = Instant::now();
    // let val = log::compute(prec);
    // let val = BigFloat::with_val(binary_prec, x).ln();
    // let val = compute_ln_parallel(x as f64, prec);
    let val = pi::compute_parallel(prec);
    let dura = start.elapsed();
    // println!("{}", val.to_string());
    println!("Compute parallel duration: {dura:#?}");

    let start = Instant::now();
    let _ = val.to_string();
    let dura = start.elapsed();
    println!("Format duration: {dura:#?}");

    // -----
    let start = Instant::now();
    let val = compute_ln(x as f64, prec);
    let dura = start.elapsed();
    // println!("{}", val.to_fixed_string());
    println!("Compute generic duration: {dura:#?}");

    let start = Instant::now();
    let _ = val.to_string();
    let dura = start.elapsed();
    println!("Format duration: {dura:#?}");
}
