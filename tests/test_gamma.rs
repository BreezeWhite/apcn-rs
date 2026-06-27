use apcn::gamma;

#[test]
fn test_gamma_sequential() {
    let ans_gamma = std::fs::read_to_string("./tests/data/gamma_million.txt").unwrap();
    let test_len = 200_000;
    let gamma_out = gamma::compute(test_len).to_fixed_string();
    assert_eq!(
        &gamma_out[..(test_len as usize + 2)],
        &ans_gamma[..(test_len as usize + 2)]
    );
}

#[test]
fn test_gamma_parallel() {
    let ans_gamma = std::fs::read_to_string("./tests/data/gamma_million.txt").unwrap();
    let test_len = 200_000;
    let gamma_out = gamma::compute_parallel(test_len).to_fixed_string();
    assert_eq!(
        &gamma_out[..(test_len as usize + 2)],
        &ans_gamma[..(test_len as usize + 2)]
    );
}
