use std::error::Error;
use apcn::phi;

#[test]
pub fn test_phi_sequential() -> Result<(), Box<dyn Error>> {
    let ans_phi = std::fs::read_to_string("./tests/data/phi_million.txt")?;

    let test_len = 100_000;
    let phi_out = phi::compute(test_len).to_fixed_string();
    assert_eq!(
        &phi_out[..(test_len as usize + 2)],
        &ans_phi[..(test_len as usize + 2)]
    );
    Ok(())
}

#[test]
pub fn test_phi_parallel() -> Result<(), Box<dyn Error>> {
    let ans_phi = std::fs::read_to_string("./tests/data/phi_million.txt")?;

    let test_len = 100_000;
    let phi_out = phi::compute_parallel(test_len).to_fixed_string();
    assert_eq!(
        &phi_out[..(test_len as usize + 2)],
        &ans_phi[..(test_len as usize + 2)]
    );
    Ok(())
}

#[test]
pub fn test_phi_direct() -> Result<(), Box<dyn Error>> {
    let ans_phi = std::fs::read_to_string("./tests/data/phi_million.txt")?;

    let test_len = 100_000;
    let phi_out = phi::compute_phi(test_len).to_fixed_string();
    assert_eq!(
        &phi_out[..(test_len as usize + 2)],
        &ans_phi[..(test_len as usize + 2)]
    );
    Ok(())
}
