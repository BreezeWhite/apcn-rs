use std::error::Error;

use apcn::phi;

#[test]
pub fn test_phi_to_a_million_digits() -> Result<(), Box<dyn Error>> {
    let ans_phi = std::fs::read_to_string("./tests/data/phi_million.txt")?;

    let test_len = 1_000_000;
    let phi_out = phi::compute(test_len).to_string();
    assert_eq!(
        &phi_out[..(test_len as usize + 2)],
        &ans_phi[..(test_len as usize + 2)]
    );
    Ok(())
}

#[test]
pub fn test_phi_parallel_to_a_million_digits() -> Result<(), Box<dyn Error>> {
    let ans_phi = std::fs::read_to_string("./tests/data/phi_million.txt")?;

    let test_len = 1_000_000;
    let phi_out = phi::compute_parallel(test_len).to_string();
    assert_eq!(
        &phi_out[..(test_len as usize + 2)],
        &ans_phi[..(test_len as usize + 2)]
    );
    Ok(())
}

#[test]
pub fn test_phi_algebra() -> Result<(), Box<dyn Error>> {
    let ans_phi = std::fs::read_to_string("./tests/data/phi_million.txt")?;

    let test_len = 1_000_000;
    let phi_out = phi::compute_phi(test_len).to_string();
    assert_eq!(
        &phi_out[..(test_len as usize + 2)],
        &ans_phi[..(test_len as usize + 2)]
    );
    Ok(())
}
