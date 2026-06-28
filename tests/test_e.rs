use apcn::backend::BigFloat;
use apcn::e;
use std::error::Error;

#[test]
pub fn test_e_binary_splitting() -> Result<(), Box<dyn Error>> {
    let ans_e = std::fs::read_to_string("./tests/data/e_million.txt")?;

    let test_len = 1_000_000;
    let e_out = e::compute(test_len).to_string();
    assert_eq!(
        &e_out[..(test_len as usize + 2)],
        &ans_e[..(test_len as usize + 2)]
    );
    Ok(())
}

#[test]
pub fn test_e_parallel_to_a_million_digits() -> Result<(), Box<dyn Error>> {
    let ans_e = std::fs::read_to_string("./tests/data/e_million.txt")?;

    let test_len = 1_000_000;
    let e_out = e::compute_parallel(test_len).to_string();
    assert_eq!(
        &e_out[..(test_len as usize + 2)],
        &ans_e[..(test_len as usize + 2)]
    );
    Ok(())
}

mod generic_exp {
    use super::*;

    fn check_exp(x: f64, prec: u32) -> Result<(), Box<dyn Error>> {
        let binary_prec = (prec as f64 * std::f64::consts::LOG2_10).round() as u32 + 32;

        let seq_out = e::exp(x, prec);
        let par_out = e::exp_parallel(x, prec);

        // Compute native exp using the backend's native arbitrary-precision exp
        let native_bf = BigFloat::with_val(binary_prec, x);
        let native_out = native_bf.exp();

        let seq_str = seq_out.to_fixed_string();
        let par_str = par_out.to_fixed_string();
        let native_str = native_out.to_fixed_string();

        // Sequential and parallel implementations must match exactly
        assert_eq!(seq_str, par_str);

        // Compare prefix of the strings (excluding last few digits to avoid minor rounding/ulp differences)
        // We compare up to prec - 2 characters to be safe.
        let compare_len = (prec as usize).min(seq_str.len()).min(native_str.len()) - 2;
        assert_eq!(
            &seq_str[..compare_len],
            &native_str[..compare_len],
            "Mismatch for exp({}) at precision {}: ours={}, native={}",
            x,
            prec,
            seq_str,
            native_str
        );

        // Sanity check primitive f64 value
        let actual_f64: f64 = seq_str.parse()?;
        let expected_f64 = x.exp();
        if expected_f64.is_finite() && actual_f64.is_finite() {
            if expected_f64 == 0.0 {
                assert!(actual_f64 < 1e-300);
            } else {
                let relative_diff = (actual_f64 - expected_f64).abs() / expected_f64;
                assert!(
                    relative_diff < 1e-13,
                    "Relative difference too large for exp({}): ours={}, expected={}",
                    x,
                    actual_f64,
                    expected_f64
                );
            }
        }

        Ok(())
    }

    #[test]
    fn test_exp_zero() -> Result<(), Box<dyn Error>> {
        check_exp(0.0, 100)?;
        check_exp(0.0, 1000)?;
        Ok(())
    }

    #[test]
    fn test_exp_one() -> Result<(), Box<dyn Error>> {
        check_exp(1.0, 100)?;
        check_exp(1.0, 1000)?;
        Ok(())
    }

    #[test]
    fn test_exp_negative_one() -> Result<(), Box<dyn Error>> {
        check_exp(-1.0, 100)?;
        check_exp(-1.0, 1000)?;
        Ok(())
    }

    #[test]
    fn test_exp_fractions() -> Result<(), Box<dyn Error>> {
        check_exp(0.5, 100)?;
        check_exp(-0.5, 100)?;
        check_exp(0.25, 200)?;
        check_exp(-0.25, 200)?;
        Ok(())
    }

    #[test]
    fn test_exp_larger() -> Result<(), Box<dyn Error>> {
        check_exp(2.5, 100)?;
        check_exp(-2.5, 100)?;
        check_exp(12.34, 150)?;
        check_exp(-12.34, 150)?;
        Ok(())
    }
}
