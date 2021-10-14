use uintxx::{U256, U512};

#[test]
fn test_u256_convert() {
    let a: i32 = -1;
    let r = U256::from(a);
    assert_eq!(r.lo, u128::MAX);
    assert_eq!(r.hi, u128::MAX);
}

#[test]
fn test_u256_mul() {
    let a = U256 {
        lo: 0xdfd3a0870f60e072996d1b60923c18a6,
        hi: 0x7c6bcb08155fac3844a705073f90be80,
    };
    let b = U256 {
        lo: 0x8b4eb00000000000314320aa7da5b1ef,
        hi: 0xf5bad73c74be6d8a7dc2ae94e4000000,
    };
    let r = a * b;
    let e = U256 {
        lo: 0xa764ffd5218bf79d51df0b450423c8fa,
        hi: 0xda2cf9b53da8fbd586ae6e35b7bd2f02,
    };
    assert_eq!(r, e);
}

#[test]
fn test_u256_rem() {
    let a = U512 {
        lo: U256 {
            lo: 0x00000000000000000000000000000000,
            hi: 0x00000000000000000000000000000000,
        },
        hi: U256 {
            lo: 0x0000000000000000000000000000000a,
            hi: 0x00000000000000000000000000000000,
        },
    };
    let b = U512::from(19u32);
    let c = a % b;
    assert_eq!(c, U512::from(8u32));
}
