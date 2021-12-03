use uintxx::{I256, U256};

#[test]
fn test_signed_div_special_case_0() {
    let a = I256::from(U256::ONE);
    let b = I256::from(U256::MIN);
    let r = a / b;
    assert_eq!(r.uint, U256::MAX);
}

#[test]
fn test_signed_rem_special_case_0() {
    let a = I256::from(U256::ONE);
    let b = I256::from(U256::MIN);
    let r = a % b;
    assert_eq!(r.uint, a.uint);
}

#[test]
fn test_signed_div_special_case_1() {
    let a = I256::from(U256::ONE << 255);
    let b = I256::from(U256::MAX);
    let r = a / b;
    assert_eq!(r.uint, a.uint);
}

#[test]
fn test_signed_rem_special_case_1() {
    let a = I256::from(U256::ONE << 255);
    let b = I256::from(U256::MAX);
    let r = a % b;
    assert_eq!(r.uint, U256::MIN);
}
