use std::error::Error;

use apcn::e;

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
