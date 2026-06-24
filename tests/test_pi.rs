use std::error::Error;

use apcn::pi;

#[test]
pub fn test_pi_to_a_million_digits() -> Result<(), Box<dyn Error>> {
    let ans_pi = std::fs::read_to_string("./tests/data/pi_million.txt")?;

    let test_len = 1_000_000;
    let pi_out = pi::compute(test_len).to_string();
    assert_eq!(
        &pi_out[..(test_len as usize + 2)],
        &ans_pi[..(test_len as usize + 2)]
    );
    Ok(())
}

#[test]
pub fn test_pi_parallel_to_a_million_digits() -> Result<(), Box<dyn Error>> {
    let ans_pi = std::fs::read_to_string("./tests/data/pi_million.txt")?;

    let test_len = 1_000_000;
    let pi_out = pi::compute_parallel(test_len).to_string();
    assert_eq!(
        &pi_out[..(test_len as usize + 2)],
        &ans_pi[..(test_len as usize + 2)]
    );
    Ok(())
}
