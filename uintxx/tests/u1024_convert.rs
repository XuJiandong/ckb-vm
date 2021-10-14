use uintxx::{U1024, U512};

#[test]
fn test_u1024_convert() {
    let a: i32 = -1;
    let r = U1024::from(a);
    assert_eq!(r.lo, U512::MAX);
    assert_eq!(r.hi, U512::MAX);
}
