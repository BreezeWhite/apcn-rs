use std::error::Error;

use apcn::log;

mod test_ln2 {
    use super::*;

    #[test]
    pub fn test_ln2_binary_splitting() -> Result<(), Box<dyn Error>> {
        let ans_ln2 = std::fs::read_to_string("./tests/data/ln2_million.txt")?;

        let test_len = 1_000_000;
        let ln2_out = log::ln2(test_len).to_fixed_string();
        assert_eq!(
            &ln2_out[..(test_len as usize + 2)],
            &ans_ln2[..(test_len as usize + 2)]
        );
        Ok(())
    }

    #[test]
    pub fn test_ln2_parallel_to_a_million_digits() -> Result<(), Box<dyn Error>> {
        let ans_ln2 = std::fs::read_to_string("./tests/data/ln2_million.txt")?;

        let test_len = 1_000_000;
        let ln2_out = log::ln2_parallel(test_len).to_fixed_string();
        assert_eq!(
            &ln2_out[..(test_len as usize + 2)],
            &ans_ln2[..(test_len as usize + 2)]
        );
        Ok(())
    }
}

mod test_ln3 {
    use super::*;

    #[test]
    pub fn test_ln3_binary_splitting() -> Result<(), Box<dyn Error>> {
        let ans_ln3 = std::fs::read_to_string("./tests/data/ln3_million.txt")?;

        let test_len = 1_000_000;
        let ln3_out = log::ln3(test_len).to_fixed_string();
        assert_eq!(
            &ln3_out[..(test_len as usize + 2)],
            &ans_ln3[..(test_len as usize + 2)]
        );
        Ok(())
    }

    #[test]
    pub fn test_ln3_parallel_to_a_million_digits() -> Result<(), Box<dyn Error>> {
        let ans_ln3 = std::fs::read_to_string("./tests/data/ln3_million.txt")?;

        let test_len = 1_000_000;
        let ln3_out = log::ln3_parallel(test_len).to_fixed_string();
        assert_eq!(
            &ln3_out[..(test_len as usize + 2)],
            &ans_ln3[..(test_len as usize + 2)]
        );
        Ok(())
    }
}

mod test_ln5 {
    use super::*;

    #[test]
    pub fn test_ln5_binary_splitting() -> Result<(), Box<dyn Error>> {
        let ans_ln5 = std::fs::read_to_string("./tests/data/ln5_million.txt")?;

        let test_len = 1_000_000;
        let ln5_out = log::ln5(test_len).to_fixed_string();
        assert_eq!(
            &ln5_out[..(test_len as usize + 2)],
            &ans_ln5[..(test_len as usize + 2)]
        );
        Ok(())
    }

    #[test]
    pub fn test_ln5_parallel_to_a_million_digits() -> Result<(), Box<dyn Error>> {
        let ans_ln5 = std::fs::read_to_string("./tests/data/ln5_million.txt")?;

        let test_len = 1_000_000;
        let ln5_out = log::ln5_parallel(test_len).to_fixed_string();
        assert_eq!(
            &ln5_out[..(test_len as usize + 2)],
            &ans_ln5[..(test_len as usize + 2)]
        );
        Ok(())
    }
}

#[test]
fn test_ln_generic_extreme_inputs() -> Result<(), Box<dyn Error>> {
    let prec = 50;

    // Large value
    let x_large = 1e300_f64;
    let expected_large = x_large.ln();
    let actual_seq_large: f64 = log::generic_log::compute_ln(x_large, prec)
        .to_fixed_string()
        .parse()?;
    let actual_par_large: f64 = log::generic_log::compute_ln_parallel(x_large, prec)
        .to_fixed_string()
        .parse()?;
    assert!((actual_seq_large - expected_large).abs() / expected_large < 1e-13);
    assert_eq!(actual_seq_large, actual_par_large);

    // Subnormal value
    let x_subnormal = 1e-320_f64;
    let expected_subnormal = x_subnormal.ln();
    let actual_seq_sub: f64 = log::generic_log::compute_ln(x_subnormal, prec)
        .to_fixed_string()
        .parse()?;
    let actual_par_sub: f64 = log::generic_log::compute_ln_parallel(x_subnormal, prec)
        .to_fixed_string()
        .parse()?;
    assert!((actual_seq_sub - expected_subnormal).abs() / expected_subnormal.abs() < 1e-13);
    assert_eq!(actual_seq_sub, actual_par_sub);

    // Boundary value
    let actual_seq_one = log::generic_log::compute_ln(1.0, prec).to_fixed_string();
    let actual_par_one = log::generic_log::compute_ln_parallel(1.0, prec).to_fixed_string();
    assert_eq!(
        actual_seq_one.trim_end_matches(|c| c == '0' || c == '.'),
        ""
    );
    assert_eq!(
        actual_par_one.trim_end_matches(|c| c == '0' || c == '.'),
        ""
    );

    Ok(())
}
