use uintxx::{I256, U128, U256};

#[test]
fn test_div() {
    let case_list = [
        [
            I256::from(U256 {
                lo: U128(0x00000000000000000000000000000001),
                hi: U128(0x00000000000000000000000000000000),
            }),
            I256::from(U256 {
                lo: U128(0x00000000000000000000000000000000),
                hi: U128(0x00000000000000000000000000000000),
            }),
            I256::from(U256 {
                lo: U128(0xffffffffffffffffffffffffffffffff),
                hi: U128(0xffffffffffffffffffffffffffffffff),
            }),
        ],
        [
            I256::from(U256 {
                lo: U128(0x00000000000000000000000000000000),
                hi: U128(0x80000000000000000000000000000000),
            }),
            I256::from(U256 {
                lo: U128(0xffffffffffffffffffffffffffffffff),
                hi: U128(0xffffffffffffffffffffffffffffffff),
            }),
            I256::from(U256 {
                lo: U128(0x00000000000000000000000000000000),
                hi: U128(0x80000000000000000000000000000000),
            }),
        ],
        [
            I256::from(U256 {
                lo: U128(0x2c1fb5204d24891731a7445bdf8bcb5c),
                hi: U128(0xea2177d8d51000000000000000000000),
            }),
            I256::from(U256 {
                lo: U128(0x686f332000000000000000000dd2966b),
                hi: U128(0x00000bea6a6af75538be984c83ce8648),
            }),
            I256::from(U256 {
                lo: U128(0xfffffffffffffffffffffffffffe2a27),
                hi: U128(0xffffffffffffffffffffffffffffffff),
            }),
        ],
    ];
    for case in &case_list {
        let lhs = case[0];
        let rhs = case[1];
        let e = case[2];
        let r = lhs.wrapping_div(rhs);
        assert_eq!(r.uint, e.uint);
    }
}

#[test]
fn test_rem() {
    let case_list = [
        [
            I256::from(U256 {
                lo: U128(0x00000000000000000000000000000001),
                hi: U128(0x00000000000000000000000000000000),
            }),
            I256::from(U256 {
                lo: U128(0x00000000000000000000000000000000),
                hi: U128(0x00000000000000000000000000000000),
            }),
            I256::from(U256 {
                lo: U128(0x00000000000000000000000000000001),
                hi: U128(0x00000000000000000000000000000000),
            }),
        ],
        [
            I256::from(U256 {
                lo: U128(0x00000000000000000000000000000000),
                hi: U128(0x80000000000000000000000000000000),
            }),
            I256::from(U256 {
                lo: U128(0xffffffffffffffffffffffffffffffff),
                hi: U128(0xffffffffffffffffffffffffffffffff),
            }),
            I256::from(U256 {
                lo: U128(0x00000000000000000000000000000000),
                hi: U128(0x00000000000000000000000000000000),
            }),
        ],
    ];
    for case in &case_list {
        let lhs = case[0];
        let rhs = case[1];
        let e = case[2];
        let r = lhs.wrapping_rem(rhs);
        assert_eq!(r.uint, e.uint);
    }
}
