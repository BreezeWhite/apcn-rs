use apcn::sqrt;
use std::error::Error;

#[test]
pub fn test_sqrt2_to_million_digits() -> Result<(), Box<dyn Error>> {
    let ans_pi = std::fs::read_to_string("./tests/data/sqrt2_million.txt")?;

    let test_len = 1_000_000;
    let pi_out = sqrt::sqrt2(test_len).to_string();
    assert_eq!(
        &pi_out[..(test_len as usize + 2)],
        &ans_pi[..(test_len as usize + 2)]
    );
    Ok(())
}

#[test]
pub fn test_sqrt3_to_million_digits() -> Result<(), Box<dyn Error>> {
    let ans_pi = std::fs::read_to_string("./tests/data/sqrt3_million.txt")?;

    let test_len = 1_000_000;
    let pi_out = sqrt::sqrt3(test_len).to_string();
    assert_eq!(
        &pi_out[..(test_len as usize + 2)],
        &ans_pi[..(test_len as usize + 2)]
    );
    Ok(())
}

#[test]
pub fn test_sqrt5_to_million_digits() -> Result<(), Box<dyn Error>> {
    let ans_pi = std::fs::read_to_string("./tests/data/sqrt5_million.txt")?;

    let test_len = 1_000_000;
    let pi_out = sqrt::sqrt5(test_len).to_string();
    assert_eq!(
        &pi_out[..(test_len as usize + 2)],
        &ans_pi[..(test_len as usize + 2)]
    );
    Ok(())
}
