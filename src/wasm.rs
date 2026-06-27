use wasm_bindgen::prelude::*;

use crate::algo::*;

fn process_string(s: String, prec: u32) -> String {
    s[..(prec + 2) as usize].to_string()
}

#[wasm_bindgen]
pub fn compute_pi(prec: u32) -> String {
    process_string(pi::compute(prec).to_fixed_string(), prec)
}

#[wasm_bindgen]
pub fn compute_e(prec: u32) -> String {
    process_string(e::compute(prec).to_fixed_string(), prec)
}

#[wasm_bindgen]
pub fn compute_sqrt2(prec: u32) -> String {
    process_string(sqrt::sqrt2(prec).to_fixed_string(), prec)
}

#[wasm_bindgen]
pub fn sqrt(x: u32, prec: u32) -> String {
    process_string(sqrt::sqrt(prec, x).to_fixed_string(), prec)
}

#[wasm_bindgen]
pub fn ln(x: f64, prec: u32) -> String {
    process_string(
        log::generic_log::compute_ln(x, prec).to_fixed_string(),
        prec,
    )
}

#[wasm_bindgen]
pub fn compute_ln2(prec: u32) -> String {
    process_string(log::ln2(prec).to_fixed_string(), prec)
}

#[wasm_bindgen]
pub fn compute_phi(prec: u32) -> String {
    process_string(phi::compute_phi(prec).to_fixed_string(), prec)
}

#[wasm_bindgen]
pub fn compute_gamma(prec: u32) -> String {
    process_string(gamma::compute(prec).to_fixed_string(), prec)
}
