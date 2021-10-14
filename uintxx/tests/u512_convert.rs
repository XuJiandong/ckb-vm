use uintxx::{U256, U512};

#[test]
fn test_u512_convert() {
    let a: i32 = -1;
    let r = U512::from(a);
    assert_eq!(r.lo, U256::MAX);
    assert_eq!(r.hi, U256::MAX);
}
