use crate::error::Error;
pub use uintxx::{I1024, I256, I512, U1024, U256, U512};

#[derive(Clone, Copy, Debug)]
pub enum VRegister {
    U1024([U1024; 2]),
    U512([U512; 4]),
    U256([U256; 8]),
    U128([u128; 16]),
    U64([u64; 32]),
    U32([u32; 64]),
    U16([u16; 128]),
    U8([u8; 256]),
}

pub fn vfunc_add_vv(
    lhs: &VRegister,
    rhs: &VRegister,
    result: &mut VRegister,
    num: usize,
) -> Result<(), Error> {
    match (lhs, rhs, result) {
        (VRegister::U1024(a), VRegister::U1024(b), VRegister::U1024(ref mut r)) => {
            for i in 0..num {
                r[i] = a[i].wrapping_add(b[i]);
            }
        }
        (VRegister::U512(a), VRegister::U512(b), VRegister::U512(ref mut r)) => {
            for i in 0..num {
                r[i] = a[i].wrapping_add(b[i]);
            }
        }
        (VRegister::U256(a), VRegister::U256(b), VRegister::U256(ref mut r)) => {
            for i in 0..num {
                r[i] = a[i].wrapping_add(b[i]);
            }
        }
        (VRegister::U128(a), VRegister::U128(b), VRegister::U128(ref mut r)) => {
            for i in 0..num {
                r[i] = a[i].wrapping_add(b[i]);
            }
        }
        (VRegister::U64(a), VRegister::U64(b), VRegister::U64(ref mut r)) => {
            for i in 0..num {
                r[i] = a[i].wrapping_add(b[i]);
            }
        }
        (VRegister::U32(a), VRegister::U32(b), VRegister::U32(ref mut r)) => {
            for i in 0..num {
                r[i] = a[i].wrapping_add(b[i]);
            }
        }
        (VRegister::U16(a), VRegister::U16(b), VRegister::U16(ref mut r)) => {
            for i in 0..num {
                r[i] = a[i].wrapping_add(b[i]);
            }
        }
        (VRegister::U8(a), VRegister::U8(b), VRegister::U8(ref mut r)) => {
            for i in 0..num {
                r[i] = a[i].wrapping_add(b[i]);
            }
        }
        _ => return Err(Error::Unexpected),
    }
    Ok(())
}

pub fn vfunc_add_vx(
    lhs: &VRegister,
    rhs: u64,
    result: &mut VRegister,
    num: usize,
) -> Result<(), Error> {
    match (lhs, result) {
        (VRegister::U1024(a), VRegister::U1024(ref mut r)) => {
            for i in 0..num {
                r[i] = a[i].wrapping_add(U1024::from(rhs as i64));
            }
        }
        (VRegister::U512(a), VRegister::U512(ref mut r)) => {
            for i in 0..num {
                r[i] = a[i].wrapping_add(U512::from(rhs as i64));
            }
        }
        (VRegister::U256(a), VRegister::U256(ref mut r)) => {
            for i in 0..num {
                r[i] = a[i].wrapping_add(U256::from(rhs as i64));
            }
        }
        (VRegister::U128(a), VRegister::U128(ref mut r)) => {
            for i in 0..num {
                r[i] = a[i].wrapping_add(rhs as i64 as i128 as u128);
            }
        }
        (VRegister::U64(a), VRegister::U64(ref mut r)) => {
            for i in 0..num {
                r[i] = a[i].wrapping_add(rhs);
            }
        }
        (VRegister::U32(a), VRegister::U32(ref mut r)) => {
            for i in 0..num {
                r[i] = a[i].wrapping_add(rhs as u32);
            }
        }
        (VRegister::U16(a), VRegister::U16(ref mut r)) => {
            for i in 0..num {
                r[i] = a[i].wrapping_add(rhs as u16);
            }
        }
        (VRegister::U8(a), VRegister::U8(ref mut r)) => {
            for i in 0..num {
                r[i] = a[i].wrapping_add(rhs as u8);
            }
        }
        _ => return Err(Error::Unexpected),
    }
    Ok(())
}

pub fn vfunc_add_vi(
    lhs: &VRegister,
    imm: i32,
    result: &mut VRegister,
    num: usize,
) -> Result<(), Error> {
    match (lhs, result) {
        (VRegister::U1024(a), VRegister::U1024(ref mut r)) => {
            for i in 0..num {
                r[i] = a[i].wrapping_add(U1024::from(imm));
            }
        }
        (VRegister::U512(a), VRegister::U512(ref mut r)) => {
            for i in 0..num {
                r[i] = a[i].wrapping_add(U512::from(imm));
            }
        }
        (VRegister::U256(a), VRegister::U256(ref mut r)) => {
            for i in 0..num {
                r[i] = a[i].wrapping_add(U256::from(imm));
            }
        }
        (VRegister::U128(a), VRegister::U128(ref mut r)) => {
            for i in 0..num {
                r[i] = a[i].wrapping_add(imm as i128 as u128);
            }
        }
        (VRegister::U64(a), VRegister::U64(ref mut r)) => {
            for i in 0..num {
                r[i] = a[i].wrapping_add(imm as i64 as u64);
            }
        }
        (VRegister::U32(a), VRegister::U32(ref mut r)) => {
            for i in 0..num {
                r[i] = a[i].wrapping_add(imm as u32);
            }
        }
        (VRegister::U16(a), VRegister::U16(ref mut r)) => {
            for i in 0..num {
                r[i] = a[i].wrapping_add(imm as i16 as u16);
            }
        }
        (VRegister::U8(a), VRegister::U8(ref mut r)) => {
            for i in 0..num {
                r[i] = a[i].wrapping_add(imm as i8 as u8);
            }
        }
        _ => return Err(Error::Unexpected),
    }
    Ok(())
}

pub fn vfunc_sub_vv(
    lhs: &VRegister,
    rhs: &VRegister,
    result: &mut VRegister,
    num: usize,
) -> Result<(), Error> {
    match (lhs, rhs, result) {
        (VRegister::U1024(a), VRegister::U1024(b), VRegister::U1024(ref mut r)) => {
            for i in 0..num {
                r[i] = a[i].wrapping_sub(b[i]);
            }
        }
        (VRegister::U512(a), VRegister::U512(b), VRegister::U512(ref mut r)) => {
            for i in 0..num {
                r[i] = a[i].wrapping_sub(b[i]);
            }
        }
        (VRegister::U256(a), VRegister::U256(b), VRegister::U256(ref mut r)) => {
            for i in 0..num {
                r[i] = a[i].wrapping_sub(b[i]);
            }
        }
        (VRegister::U128(a), VRegister::U128(b), VRegister::U128(ref mut r)) => {
            for i in 0..num {
                r[i] = a[i].wrapping_sub(b[i]);
            }
        }
        (VRegister::U64(a), VRegister::U64(b), VRegister::U64(ref mut r)) => {
            for i in 0..num {
                r[i] = a[i].wrapping_sub(b[i]);
            }
        }
        (VRegister::U32(a), VRegister::U32(b), VRegister::U32(ref mut r)) => {
            for i in 0..num {
                r[i] = a[i].wrapping_sub(b[i]);
            }
        }
        (VRegister::U16(a), VRegister::U16(b), VRegister::U16(ref mut r)) => {
            for i in 0..num {
                r[i] = a[i].wrapping_sub(b[i]);
            }
        }
        (VRegister::U8(a), VRegister::U8(b), VRegister::U8(ref mut r)) => {
            for i in 0..num {
                r[i] = a[i].wrapping_sub(b[i]);
            }
        }
        _ => return Err(Error::Unexpected),
    }
    Ok(())
}

pub fn vfunc_sub_vx(
    lhs: &VRegister,
    rhs: u64,
    result: &mut VRegister,
    num: usize,
) -> Result<(), Error> {
    match (lhs, result) {
        (VRegister::U1024(a), VRegister::U1024(ref mut r)) => {
            for i in 0..num {
                r[i] = a[i].wrapping_sub(U1024::from(rhs as i64));
            }
        }
        (VRegister::U512(a), VRegister::U512(ref mut r)) => {
            for i in 0..num {
                r[i] = a[i].wrapping_sub(U512::from(rhs as i64));
            }
        }
        (VRegister::U256(a), VRegister::U256(ref mut r)) => {
            for i in 0..num {
                r[i] = a[i].wrapping_sub(U256::from(rhs as i64));
            }
        }
        (VRegister::U128(a), VRegister::U128(ref mut r)) => {
            for i in 0..num {
                r[i] = a[i].wrapping_sub(rhs as i64 as i128 as u128);
            }
        }
        (VRegister::U64(a), VRegister::U64(ref mut r)) => {
            for i in 0..num {
                r[i] = a[i].wrapping_sub(rhs);
            }
        }
        (VRegister::U32(a), VRegister::U32(ref mut r)) => {
            for i in 0..num {
                r[i] = a[i].wrapping_sub(rhs as u32);
            }
        }
        (VRegister::U16(a), VRegister::U16(ref mut r)) => {
            for i in 0..num {
                r[i] = a[i].wrapping_sub(rhs as u16);
            }
        }
        (VRegister::U8(a), VRegister::U8(ref mut r)) => {
            for i in 0..num {
                r[i] = a[i].wrapping_sub(rhs as u8);
            }
        }
        _ => return Err(Error::Unexpected),
    }
    Ok(())
}

pub fn vfunc_rsub_vx(
    lhs: &VRegister,
    rhs: u64,
    result: &mut VRegister,
    num: usize,
) -> Result<(), Error> {
    match (lhs, result) {
        (VRegister::U1024(a), VRegister::U1024(ref mut r)) => {
            for i in 0..num {
                r[i] = U1024::from(rhs as i64).wrapping_sub(a[i]);
            }
        }
        (VRegister::U512(a), VRegister::U512(ref mut r)) => {
            for i in 0..num {
                r[i] = U512::from(rhs as i64).wrapping_sub(a[i]);
            }
        }
        (VRegister::U256(a), VRegister::U256(ref mut r)) => {
            for i in 0..num {
                r[i] = U256::from(rhs as i64).wrapping_sub(a[i]);
            }
        }
        (VRegister::U128(a), VRegister::U128(ref mut r)) => {
            for i in 0..num {
                r[i] = (rhs as i128 as u128).wrapping_sub(a[i]);
            }
        }
        (VRegister::U64(a), VRegister::U64(ref mut r)) => {
            for i in 0..num {
                r[i] = (rhs).wrapping_sub(a[i]);
            }
        }
        (VRegister::U32(a), VRegister::U32(ref mut r)) => {
            for i in 0..num {
                r[i] = (rhs as u32).wrapping_sub(a[i]);
            }
        }
        (VRegister::U16(a), VRegister::U16(ref mut r)) => {
            for i in 0..num {
                r[i] = (rhs as u16).wrapping_sub(a[i]);
            }
        }
        (VRegister::U8(a), VRegister::U8(ref mut r)) => {
            for i in 0..num {
                r[i] = (rhs as u8).wrapping_sub(a[i]);
            }
        }
        _ => return Err(Error::Unexpected),
    }
    Ok(())
}

pub fn vfunc_rsub_vi(
    lhs: &VRegister,
    rhs: i32,
    result: &mut VRegister,
    num: usize,
) -> Result<(), Error> {
    match (lhs, result) {
        (VRegister::U1024(a), VRegister::U1024(ref mut r)) => {
            for i in 0..num {
                r[i] = U1024::from(rhs).wrapping_sub(a[i]);
            }
        }
        (VRegister::U512(a), VRegister::U512(ref mut r)) => {
            for i in 0..num {
                r[i] = U512::from(rhs).wrapping_sub(a[i]);
            }
        }
        (VRegister::U256(a), VRegister::U256(ref mut r)) => {
            for i in 0..num {
                r[i] = U256::from(rhs).wrapping_sub(a[i]);
            }
        }
        (VRegister::U128(a), VRegister::U128(ref mut r)) => {
            for i in 0..num {
                r[i] = (rhs as i128 as u128).wrapping_sub(a[i]);
            }
        }
        (VRegister::U64(a), VRegister::U64(ref mut r)) => {
            for i in 0..num {
                r[i] = (rhs as i64 as u64).wrapping_sub(a[i]);
            }
        }
        (VRegister::U32(a), VRegister::U32(ref mut r)) => {
            for i in 0..num {
                r[i] = (rhs as u32).wrapping_sub(a[i]);
            }
        }
        (VRegister::U16(a), VRegister::U16(ref mut r)) => {
            for i in 0..num {
                r[i] = (rhs as i16 as u16).wrapping_sub(a[i]);
            }
        }
        (VRegister::U8(a), VRegister::U8(ref mut r)) => {
            for i in 0..num {
                r[i] = (rhs as i8 as u8).wrapping_sub(a[i]);
            }
        }
        _ => return Err(Error::Unexpected),
    }
    Ok(())
}

pub fn vfunc_mul_vv(
    lhs: &VRegister,
    rhs: &VRegister,
    result: &mut VRegister,
    num: usize,
) -> Result<(), Error> {
    match (lhs, rhs, result) {
        (VRegister::U1024(a), VRegister::U1024(b), VRegister::U1024(ref mut r)) => {
            for i in 0..num {
                r[i] = a[i].wrapping_mul(b[i]);
            }
        }
        (VRegister::U512(a), VRegister::U512(b), VRegister::U512(ref mut r)) => {
            for i in 0..num {
                r[i] = a[i].wrapping_mul(b[i]);
            }
        }
        (VRegister::U256(a), VRegister::U256(b), VRegister::U256(ref mut r)) => {
            for i in 0..num {
                r[i] = a[i].wrapping_mul(b[i]);
            }
        }
        (VRegister::U128(a), VRegister::U128(b), VRegister::U128(ref mut r)) => {
            for i in 0..num {
                r[i] = a[i].wrapping_mul(b[i]);
            }
        }
        (VRegister::U64(a), VRegister::U64(b), VRegister::U64(ref mut r)) => {
            for i in 0..num {
                r[i] = a[i].wrapping_mul(b[i]);
            }
        }
        (VRegister::U32(a), VRegister::U32(b), VRegister::U32(ref mut r)) => {
            for i in 0..num {
                r[i] = a[i].wrapping_mul(b[i]);
            }
        }
        (VRegister::U16(a), VRegister::U16(b), VRegister::U16(ref mut r)) => {
            for i in 0..num {
                r[i] = a[i].wrapping_mul(b[i]);
            }
        }
        (VRegister::U8(a), VRegister::U8(b), VRegister::U8(ref mut r)) => {
            for i in 0..num {
                r[i] = a[i].wrapping_mul(b[i]);
            }
        }
        _ => return Err(Error::Unexpected),
    }
    Ok(())
}

pub fn vfunc_mul_vx(
    lhs: &VRegister,
    rhs: u64,
    result: &mut VRegister,
    num: usize,
) -> Result<(), Error> {
    match (lhs, result) {
        (VRegister::U1024(a), VRegister::U1024(ref mut r)) => {
            for i in 0..num {
                r[i] = a[i].wrapping_mul(U1024::from(rhs as i64));
            }
        }
        (VRegister::U512(a), VRegister::U512(ref mut r)) => {
            for i in 0..num {
                r[i] = a[i].wrapping_mul(U512::from(rhs as i64));
            }
        }
        (VRegister::U256(a), VRegister::U256(ref mut r)) => {
            for i in 0..num {
                r[i] = a[i].wrapping_mul(U256::from(rhs as i64));
            }
        }
        (VRegister::U128(a), VRegister::U128(ref mut r)) => {
            for i in 0..num {
                r[i] = a[i].wrapping_mul(rhs as i64 as i128 as u128);
            }
        }
        (VRegister::U64(a), VRegister::U64(ref mut r)) => {
            for i in 0..num {
                r[i] = a[i].wrapping_mul(rhs);
            }
        }
        (VRegister::U32(a), VRegister::U32(ref mut r)) => {
            for i in 0..num {
                r[i] = a[i].wrapping_mul(rhs as u32);
            }
        }
        (VRegister::U16(a), VRegister::U16(ref mut r)) => {
            for i in 0..num {
                r[i] = a[i].wrapping_mul(rhs as u16);
            }
        }
        (VRegister::U8(a), VRegister::U8(ref mut r)) => {
            for i in 0..num {
                r[i] = a[i].wrapping_mul(rhs as u8);
            }
        }
        _ => return Err(Error::Unexpected),
    }
    Ok(())
}

pub fn vfunc_sll_vv(
    lhs: &VRegister,
    rhs: &VRegister,
    result: &mut VRegister,
    num: usize,
) -> Result<(), Error> {
    match (lhs, rhs, result) {
        (VRegister::U1024(a), VRegister::U1024(b), VRegister::U1024(ref mut r)) => {
            for i in 0..num {
                r[i] = a[i] << (b[i].u32() % 1024);
            }
        }
        (VRegister::U512(a), VRegister::U512(b), VRegister::U512(ref mut r)) => {
            for i in 0..num {
                r[i] = a[i] << (b[i].u32() % 512);
            }
        }
        (VRegister::U256(a), VRegister::U256(b), VRegister::U256(ref mut r)) => {
            for i in 0..num {
                r[i] = a[i] << (b[i].u32() % 256);
            }
        }
        (VRegister::U128(a), VRegister::U128(b), VRegister::U128(ref mut r)) => {
            for i in 0..num {
                r[i] = a[i] << (b[i] as u32 % 128);
            }
        }
        (VRegister::U64(a), VRegister::U64(b), VRegister::U64(ref mut r)) => {
            for i in 0..num {
                r[i] = a[i] << (b[i] as u32 % 64);
            }
        }
        (VRegister::U32(a), VRegister::U32(b), VRegister::U32(ref mut r)) => {
            for i in 0..num {
                r[i] = a[i] << (b[i] as u32 % 32);
            }
        }
        (VRegister::U16(a), VRegister::U16(b), VRegister::U16(ref mut r)) => {
            for i in 0..num {
                r[i] = a[i] << (b[i] as u32 % 16);
            }
        }
        (VRegister::U8(a), VRegister::U8(b), VRegister::U8(ref mut r)) => {
            for i in 0..num {
                r[i] = a[i] << (b[i] as u32 % 8);
            }
        }
        _ => return Err(Error::Unexpected),
    }
    Ok(())
}

pub fn vfunc_sll_vi(
    lhs: &VRegister,
    rhs: u32,
    result: &mut VRegister,
    num: usize,
) -> Result<(), Error> {
    match (lhs, result) {
        (VRegister::U1024(a), VRegister::U1024(ref mut r)) => {
            for i in 0..num {
                r[i] = a[i] << (rhs % 1024);
            }
        }
        (VRegister::U512(a), VRegister::U512(ref mut r)) => {
            for i in 0..num {
                r[i] = a[i] << (rhs % 512);
            }
        }
        (VRegister::U256(a), VRegister::U256(ref mut r)) => {
            for i in 0..num {
                r[i] = a[i] << (rhs % 256);
            }
        }
        (VRegister::U128(a), VRegister::U128(ref mut r)) => {
            for i in 0..num {
                r[i] = a[i] << (rhs % 128);
            }
        }
        (VRegister::U64(a), VRegister::U64(ref mut r)) => {
            for i in 0..num {
                r[i] = a[i] << (rhs % 64);
            }
        }
        (VRegister::U32(a), VRegister::U32(ref mut r)) => {
            for i in 0..num {
                r[i] = a[i] << (rhs % 32);
            }
        }
        (VRegister::U16(a), VRegister::U16(ref mut r)) => {
            for i in 0..num {
                r[i] = a[i] << (rhs % 16);
            }
        }
        (VRegister::U8(a), VRegister::U8(ref mut r)) => {
            for i in 0..num {
                r[i] = a[i] << (rhs % 8);
            }
        }
        _ => return Err(Error::Unexpected),
    }
    Ok(())
}

pub fn vfunc_srl_vv(
    lhs: &VRegister,
    rhs: &VRegister,
    result: &mut VRegister,
    num: usize,
) -> Result<(), Error> {
    match (lhs, rhs, result) {
        (VRegister::U1024(a), VRegister::U1024(b), VRegister::U1024(ref mut r)) => {
            for i in 0..num {
                r[i] = a[i] >> (b[i].u32() % 1024);
            }
        }
        (VRegister::U512(a), VRegister::U512(b), VRegister::U512(ref mut r)) => {
            for i in 0..num {
                r[i] = a[i] >> (b[i].u32() % 512);
            }
        }
        (VRegister::U256(a), VRegister::U256(b), VRegister::U256(ref mut r)) => {
            for i in 0..num {
                r[i] = a[i] >> (b[i].u32() % 256);
            }
        }
        (VRegister::U128(a), VRegister::U128(b), VRegister::U128(ref mut r)) => {
            for i in 0..num {
                r[i] = a[i] >> (b[i] as u32 % 128);
            }
        }
        (VRegister::U64(a), VRegister::U64(b), VRegister::U64(ref mut r)) => {
            for i in 0..num {
                r[i] = a[i] >> (b[i] as u32 % 64);
            }
        }
        (VRegister::U32(a), VRegister::U32(b), VRegister::U32(ref mut r)) => {
            for i in 0..num {
                r[i] = a[i] >> (b[i] as u32 % 32);
            }
        }
        (VRegister::U16(a), VRegister::U16(b), VRegister::U16(ref mut r)) => {
            for i in 0..num {
                r[i] = a[i] >> (b[i] as u32 % 16);
            }
        }
        (VRegister::U8(a), VRegister::U8(b), VRegister::U8(ref mut r)) => {
            for i in 0..num {
                r[i] = a[i] >> (b[i] as u32 % 8);
            }
        }
        _ => return Err(Error::Unexpected),
    }
    Ok(())
}

pub fn vfunc_srl_vi(
    lhs: &VRegister,
    rhs: u32,
    result: &mut VRegister,
    num: usize,
) -> Result<(), Error> {
    match (lhs, result) {
        (VRegister::U1024(a), VRegister::U1024(ref mut r)) => {
            for i in 0..num {
                r[i] = a[i] >> (rhs % 1024);
            }
        }
        (VRegister::U512(a), VRegister::U512(ref mut r)) => {
            for i in 0..num {
                r[i] = a[i] >> (rhs % 512);
            }
        }
        (VRegister::U256(a), VRegister::U256(ref mut r)) => {
            for i in 0..num {
                r[i] = a[i] >> (rhs % 256);
            }
        }
        (VRegister::U128(a), VRegister::U128(ref mut r)) => {
            for i in 0..num {
                r[i] = a[i] >> (rhs % 128);
            }
        }
        (VRegister::U64(a), VRegister::U64(ref mut r)) => {
            for i in 0..num {
                r[i] = a[i] >> (rhs % 64);
            }
        }
        (VRegister::U32(a), VRegister::U32(ref mut r)) => {
            for i in 0..num {
                r[i] = a[i] >> (rhs % 32);
            }
        }
        (VRegister::U16(a), VRegister::U16(ref mut r)) => {
            for i in 0..num {
                r[i] = a[i] >> (rhs % 16);
            }
        }
        (VRegister::U8(a), VRegister::U8(ref mut r)) => {
            for i in 0..num {
                r[i] = a[i] >> (rhs % 8);
            }
        }
        _ => return Err(Error::Unexpected),
    }
    Ok(())
}

pub fn vfunc_sra_vv(
    lhs: &VRegister,
    rhs: &VRegister,
    result: &mut VRegister,
    num: usize,
) -> Result<(), Error> {
    match (lhs, rhs, result) {
        (VRegister::U1024(a), VRegister::U1024(b), VRegister::U1024(ref mut r)) => {
            for i in 0..num {
                r[i] = a[i].wrapping_sra(b[i].u32() % 1024)
            }
        }
        (VRegister::U512(a), VRegister::U512(b), VRegister::U512(ref mut r)) => {
            for i in 0..num {
                r[i] = a[i].wrapping_sra(b[i].u32() % 512)
            }
        }
        (VRegister::U256(a), VRegister::U256(b), VRegister::U256(ref mut r)) => {
            for i in 0..num {
                r[i] = a[i].wrapping_sra(b[i].u32() % 256)
            }
        }
        (VRegister::U128(a), VRegister::U128(b), VRegister::U128(ref mut r)) => {
            for i in 0..num {
                r[i] = ((a[i] as i128) >> (b[i] % 128)) as u128;
            }
        }
        (VRegister::U64(a), VRegister::U64(b), VRegister::U64(ref mut r)) => {
            for i in 0..num {
                r[i] = ((a[i] as i64) >> (b[i] % 64)) as u64;
            }
        }
        (VRegister::U32(a), VRegister::U32(b), VRegister::U32(ref mut r)) => {
            for i in 0..num {
                r[i] = ((a[i] as i32) >> (b[i] % 32)) as u32;
            }
        }
        (VRegister::U16(a), VRegister::U16(b), VRegister::U16(ref mut r)) => {
            for i in 0..num {
                r[i] = ((a[i] as i16) >> (b[i] % 16)) as u16;
            }
        }
        (VRegister::U8(a), VRegister::U8(b), VRegister::U8(ref mut r)) => {
            for i in 0..num {
                r[i] = ((a[i] as i8) >> (b[i] % 8)) as u8;
            }
        }
        _ => return Err(Error::Unexpected),
    }
    Ok(())
}

pub fn vfunc_sra_vi(
    lhs: &VRegister,
    rhs: u32,
    result: &mut VRegister,
    num: usize,
) -> Result<(), Error> {
    match (lhs, result) {
        (VRegister::U1024(a), VRegister::U1024(ref mut r)) => {
            for i in 0..num {
                r[i] = a[i].wrapping_sra(rhs % 1024)
            }
        }
        (VRegister::U512(a), VRegister::U512(ref mut r)) => {
            for i in 0..num {
                r[i] = a[i].wrapping_sra(rhs % 512)
            }
        }
        (VRegister::U256(a), VRegister::U256(ref mut r)) => {
            for i in 0..num {
                r[i] = a[i].wrapping_sra(rhs % 256)
            }
        }
        (VRegister::U128(a), VRegister::U128(ref mut r)) => {
            for i in 0..num {
                r[i] = (a[i] as i128).wrapping_shr(rhs % 128) as u128
            }
        }
        (VRegister::U64(a), VRegister::U64(ref mut r)) => {
            for i in 0..num {
                r[i] = (a[i] as i64).wrapping_shr(rhs % 64) as u64
            }
        }
        (VRegister::U32(a), VRegister::U32(ref mut r)) => {
            for i in 0..num {
                r[i] = (a[i] as i32).wrapping_shr(rhs % 32) as u32
            }
        }
        (VRegister::U16(a), VRegister::U16(ref mut r)) => {
            for i in 0..num {
                r[i] = (a[i] as i16).wrapping_shr(rhs % 16) as u16
            }
        }
        (VRegister::U8(a), VRegister::U8(ref mut r)) => {
            for i in 0..num {
                r[i] = (a[i] as i8).wrapping_shr(rhs % 8) as u8
            }
        }
        _ => return Err(Error::Unexpected),
    }
    Ok(())
}

impl Default for VRegister {
    fn default() -> Self {
        VRegister::U8([0x00; 256])
    }
}

pub fn vfunc_divu_vv(
    lhs: &VRegister,
    rhs: &VRegister,
    result: &mut VRegister,
    num: usize,
) -> Result<(), Error> {
    match (lhs, rhs, result) {
        (VRegister::U1024(a), VRegister::U1024(b), VRegister::U1024(ref mut r)) => {
            for i in 0..num {
                r[i] = a[i].wrapping_div(b[i]);
            }
        }
        (VRegister::U512(a), VRegister::U512(b), VRegister::U512(ref mut r)) => {
            for i in 0..num {
                r[i] = a[i].wrapping_div(b[i]);
            }
        }
        (VRegister::U256(a), VRegister::U256(b), VRegister::U256(ref mut r)) => {
            for i in 0..num {
                r[i] = a[i].wrapping_div(b[i]);
            }
        }
        (VRegister::U128(a), VRegister::U128(b), VRegister::U128(ref mut r)) => {
            for i in 0..num {
                r[i] = a[i].wrapping_div(b[i]);
            }
        }
        (VRegister::U64(a), VRegister::U64(b), VRegister::U64(ref mut r)) => {
            for i in 0..num {
                r[i] = a[i].wrapping_div(b[i]);
            }
        }
        (VRegister::U32(a), VRegister::U32(b), VRegister::U32(ref mut r)) => {
            for i in 0..num {
                r[i] = a[i].wrapping_div(b[i]);
            }
        }
        (VRegister::U16(a), VRegister::U16(b), VRegister::U16(ref mut r)) => {
            for i in 0..num {
                r[i] = a[i].wrapping_div(b[i]);
            }
        }
        (VRegister::U8(a), VRegister::U8(b), VRegister::U8(ref mut r)) => {
            for i in 0..num {
                r[i] = a[i].wrapping_div(b[i]);
            }
        }
        _ => return Err(Error::Unexpected),
    }
    Ok(())
}

pub fn vfunc_divu_vx(
    lhs: &VRegister,
    rhs: u64,
    result: &mut VRegister,
    num: usize,
) -> Result<(), Error> {
    match (lhs, result) {
        (VRegister::U1024(a), VRegister::U1024(ref mut r)) => {
            for i in 0..num {
                r[i] = a[i].wrapping_div(U1024::from(rhs as i64));
            }
        }
        (VRegister::U512(a), VRegister::U512(ref mut r)) => {
            for i in 0..num {
                r[i] = a[i].wrapping_div(U512::from(rhs as i64));
            }
        }
        (VRegister::U256(a), VRegister::U256(ref mut r)) => {
            for i in 0..num {
                r[i] = a[i].wrapping_div(U256::from(rhs as i64));
            }
        }
        (VRegister::U128(a), VRegister::U128(ref mut r)) => {
            for i in 0..num {
                r[i] = a[i].wrapping_div(rhs as i128 as u128);
            }
        }
        (VRegister::U64(a), VRegister::U64(ref mut r)) => {
            for i in 0..num {
                r[i] = a[i].wrapping_div(rhs);
            }
        }
        (VRegister::U32(a), VRegister::U32(ref mut r)) => {
            for i in 0..num {
                r[i] = a[i].wrapping_div(rhs as u32);
            }
        }
        (VRegister::U16(a), VRegister::U16(ref mut r)) => {
            for i in 0..num {
                r[i] = a[i].wrapping_div(rhs as u16);
            }
        }
        (VRegister::U8(a), VRegister::U8(ref mut r)) => {
            for i in 0..num {
                r[i] = a[i].wrapping_div(rhs as u8);
            }
        }
        _ => return Err(Error::Unexpected),
    }
    Ok(())
}

pub fn vfunc_remu_vv(
    lhs: &VRegister,
    rhs: &VRegister,
    result: &mut VRegister,
    num: usize,
) -> Result<(), Error> {
    match (lhs, rhs, result) {
        (VRegister::U1024(a), VRegister::U1024(b), VRegister::U1024(ref mut r)) => {
            for i in 0..num {
                r[i] = a[i].wrapping_rem(b[i]);
            }
        }
        (VRegister::U512(a), VRegister::U512(b), VRegister::U512(ref mut r)) => {
            for i in 0..num {
                r[i] = a[i].wrapping_rem(b[i]);
            }
        }
        (VRegister::U256(a), VRegister::U256(b), VRegister::U256(ref mut r)) => {
            for i in 0..num {
                r[i] = a[i].wrapping_rem(b[i]);
            }
        }
        (VRegister::U128(a), VRegister::U128(b), VRegister::U128(ref mut r)) => {
            for i in 0..num {
                r[i] = a[i].wrapping_rem(b[i]);
            }
        }
        (VRegister::U64(a), VRegister::U64(b), VRegister::U64(ref mut r)) => {
            for i in 0..num {
                r[i] = a[i].wrapping_rem(b[i]);
            }
        }
        (VRegister::U32(a), VRegister::U32(b), VRegister::U32(ref mut r)) => {
            for i in 0..num {
                r[i] = a[i].wrapping_rem(b[i]);
            }
        }
        (VRegister::U16(a), VRegister::U16(b), VRegister::U16(ref mut r)) => {
            for i in 0..num {
                r[i] = a[i].wrapping_rem(b[i]);
            }
        }
        (VRegister::U8(a), VRegister::U8(b), VRegister::U8(ref mut r)) => {
            for i in 0..num {
                r[i] = a[i].wrapping_rem(b[i]);
            }
        }
        _ => return Err(Error::Unexpected),
    }
    Ok(())
}

pub fn vfunc_remu_vx(
    lhs: &VRegister,
    rhs: u64,
    result: &mut VRegister,
    num: usize,
) -> Result<(), Error> {
    match (lhs, result) {
        (VRegister::U1024(a), VRegister::U1024(ref mut r)) => {
            for i in 0..num {
                r[i] = a[i].wrapping_rem(U1024::from(rhs as i64));
            }
        }
        (VRegister::U512(a), VRegister::U512(ref mut r)) => {
            for i in 0..num {
                r[i] = a[i].wrapping_rem(U512::from(rhs as i64));
            }
        }
        (VRegister::U256(a), VRegister::U256(ref mut r)) => {
            for i in 0..num {
                r[i] = a[i].wrapping_rem(U256::from(rhs as i64));
            }
        }
        (VRegister::U128(a), VRegister::U128(ref mut r)) => {
            for i in 0..num {
                r[i] = a[i].wrapping_rem(rhs as i128 as u128);
            }
        }
        (VRegister::U64(a), VRegister::U64(ref mut r)) => {
            for i in 0..num {
                r[i] = a[i].wrapping_rem(rhs);
            }
        }
        (VRegister::U32(a), VRegister::U32(ref mut r)) => {
            for i in 0..num {
                r[i] = a[i].wrapping_rem(rhs as u32);
            }
        }
        (VRegister::U16(a), VRegister::U16(ref mut r)) => {
            for i in 0..num {
                r[i] = a[i].wrapping_rem(rhs as u16);
            }
        }
        (VRegister::U8(a), VRegister::U8(ref mut r)) => {
            for i in 0..num {
                r[i] = a[i].wrapping_rem(rhs as u8);
            }
        }
        _ => return Err(Error::Unexpected),
    }
    Ok(())
}

pub fn vfunc_mseq_vv(
    lhs: &VRegister,
    rhs: &VRegister,
    result: &mut VRegister,
    num: usize,
) -> Result<(), Error> {
    match (lhs, rhs, result) {
        (VRegister::U1024(a), VRegister::U1024(b), VRegister::U1024(ref mut r)) => {
            for i in 0..num {
                r[i] = if a[i] == b[i] {
                    U1024::from(1u32)
                } else {
                    U1024::MIN
                };
            }
        }
        (VRegister::U512(a), VRegister::U512(b), VRegister::U512(ref mut r)) => {
            for i in 0..num {
                r[i] = if a[i] == b[i] {
                    U512::from(1u32)
                } else {
                    U512::MIN
                };
            }
        }
        (VRegister::U256(a), VRegister::U256(b), VRegister::U256(ref mut r)) => {
            for i in 0..num {
                r[i] = if a[i] == b[i] {
                    U256::from(1u32)
                } else {
                    U256::MIN
                };
            }
        }
        (VRegister::U128(a), VRegister::U128(b), VRegister::U128(ref mut r)) => {
            for i in 0..num {
                r[i] = if a[i] == b[i] { 1 } else { 0 };
            }
        }
        (VRegister::U64(a), VRegister::U64(b), VRegister::U64(ref mut r)) => {
            for i in 0..num {
                r[i] = if a[i] == b[i] { 1 } else { 0 };
            }
        }
        (VRegister::U32(a), VRegister::U32(b), VRegister::U32(ref mut r)) => {
            for i in 0..num {
                r[i] = if a[i] == b[i] { 1 } else { 0 };
            }
        }
        (VRegister::U16(a), VRegister::U16(b), VRegister::U16(ref mut r)) => {
            for i in 0..num {
                r[i] = if a[i] == b[i] { 1 } else { 0 };
            }
        }
        (VRegister::U8(a), VRegister::U8(b), VRegister::U8(ref mut r)) => {
            for i in 0..num {
                r[i] = if a[i] == b[i] { 1 } else { 0 };
            }
        }
        _ => return Err(Error::Unexpected),
    }
    Ok(())
}

pub fn vfunc_mseq_vx(
    lhs: &VRegister,
    rhs: u64,
    result: &mut VRegister,
    num: usize,
) -> Result<(), Error> {
    match (lhs, result) {
        (VRegister::U1024(a), VRegister::U1024(ref mut r)) => {
            for i in 0..num {
                r[i] = if a[i] == U1024::from(rhs as i64) {
                    U1024::from(1u32)
                } else {
                    U1024::MIN
                };
            }
        }
        (VRegister::U512(a), VRegister::U512(ref mut r)) => {
            for i in 0..num {
                r[i] = if a[i] == U512::from(rhs as i64) {
                    U512::from(1u32)
                } else {
                    U512::MIN
                };
            }
        }
        (VRegister::U256(a), VRegister::U256(ref mut r)) => {
            for i in 0..num {
                r[i] = if a[i] == U256::from(rhs as i64) {
                    U256::from(1u32)
                } else {
                    U256::MIN
                };
            }
        }
        (VRegister::U128(a), VRegister::U128(ref mut r)) => {
            for i in 0..num {
                r[i] = if a[i] == rhs as i64 as i128 as u128 {
                    1
                } else {
                    0
                };
            }
        }
        (VRegister::U64(a), VRegister::U64(ref mut r)) => {
            for i in 0..num {
                r[i] = if a[i] == rhs { 1 } else { 0 };
            }
        }
        (VRegister::U32(a), VRegister::U32(ref mut r)) => {
            for i in 0..num {
                r[i] = if a[i] == rhs as u32 { 1 } else { 0 };
            }
        }
        (VRegister::U16(a), VRegister::U16(ref mut r)) => {
            for i in 0..num {
                r[i] = if a[i] == rhs as u16 { 1 } else { 0 };
            }
        }
        (VRegister::U8(a), VRegister::U8(ref mut r)) => {
            for i in 0..num {
                r[i] = if a[i] == rhs as u8 { 1 } else { 0 };
            }
        }
        _ => return Err(Error::Unexpected),
    }
    Ok(())
}

pub fn vfunc_mseq_vi(
    lhs: &VRegister,
    rhs: i32,
    result: &mut VRegister,
    num: usize,
) -> Result<(), Error> {
    match (lhs, result) {
        (VRegister::U1024(a), VRegister::U1024(ref mut r)) => {
            for i in 0..num {
                r[i] = if a[i] == U1024::from(rhs) {
                    U1024::from(1u32)
                } else {
                    U1024::MIN
                };
            }
        }
        (VRegister::U512(a), VRegister::U512(ref mut r)) => {
            for i in 0..num {
                r[i] = if a[i] == U512::from(rhs) {
                    U512::from(1u32)
                } else {
                    U512::MIN
                };
            }
        }
        (VRegister::U256(a), VRegister::U256(ref mut r)) => {
            for i in 0..num {
                r[i] = if a[i] == U256::from(rhs) {
                    U256::from(1u32)
                } else {
                    U256::MIN
                };
            }
        }
        (VRegister::U128(a), VRegister::U128(ref mut r)) => {
            for i in 0..num {
                r[i] = if a[i] == rhs as i128 as u128 { 1 } else { 0 };
            }
        }
        (VRegister::U64(a), VRegister::U64(ref mut r)) => {
            for i in 0..num {
                r[i] = if a[i] == rhs as i64 as u64 { 1 } else { 0 };
            }
        }
        (VRegister::U32(a), VRegister::U32(ref mut r)) => {
            for i in 0..num {
                r[i] = if a[i] == rhs as u32 { 1 } else { 0 };
            }
        }
        (VRegister::U16(a), VRegister::U16(ref mut r)) => {
            for i in 0..num {
                r[i] = if a[i] == rhs as i16 as u16 { 1 } else { 0 };
            }
        }
        (VRegister::U8(a), VRegister::U8(ref mut r)) => {
            for i in 0..num {
                r[i] = if a[i] == rhs as i8 as u8 { 1 } else { 0 };
            }
        }
        _ => return Err(Error::Unexpected),
    }
    Ok(())
}

pub fn vfunc_msne_vv(
    lhs: &VRegister,
    rhs: &VRegister,
    result: &mut VRegister,
    num: usize,
) -> Result<(), Error> {
    match (lhs, rhs, result) {
        (VRegister::U1024(a), VRegister::U1024(b), VRegister::U1024(ref mut r)) => {
            for i in 0..num {
                r[i] = if a[i] != b[i] {
                    U1024::from(1u32)
                } else {
                    U1024::MIN
                };
            }
        }
        (VRegister::U512(a), VRegister::U512(b), VRegister::U512(ref mut r)) => {
            for i in 0..num {
                r[i] = if a[i] != b[i] {
                    U512::from(1u32)
                } else {
                    U512::MIN
                };
            }
        }
        (VRegister::U256(a), VRegister::U256(b), VRegister::U256(ref mut r)) => {
            for i in 0..num {
                r[i] = if a[i] != b[i] {
                    U256::from(1u32)
                } else {
                    U256::MIN
                };
            }
        }
        (VRegister::U128(a), VRegister::U128(b), VRegister::U128(ref mut r)) => {
            for i in 0..num {
                r[i] = if a[i] != b[i] { 1 } else { 0 };
            }
        }
        (VRegister::U64(a), VRegister::U64(b), VRegister::U64(ref mut r)) => {
            for i in 0..num {
                r[i] = if a[i] != b[i] { 1 } else { 0 };
            }
        }
        (VRegister::U32(a), VRegister::U32(b), VRegister::U32(ref mut r)) => {
            for i in 0..num {
                r[i] = if a[i] != b[i] { 1 } else { 0 };
            }
        }
        (VRegister::U16(a), VRegister::U16(b), VRegister::U16(ref mut r)) => {
            for i in 0..num {
                r[i] = if a[i] != b[i] { 1 } else { 0 };
            }
        }
        (VRegister::U8(a), VRegister::U8(b), VRegister::U8(ref mut r)) => {
            for i in 0..num {
                r[i] = if a[i] != b[i] { 1 } else { 0 };
            }
        }
        _ => return Err(Error::Unexpected),
    }
    Ok(())
}

pub fn vfunc_msne_vx(
    lhs: &VRegister,
    rhs: u64,
    result: &mut VRegister,
    num: usize,
) -> Result<(), Error> {
    match (lhs, result) {
        (VRegister::U1024(a), VRegister::U1024(ref mut r)) => {
            for i in 0..num {
                r[i] = if a[i] != U1024::from(rhs as i64) {
                    U1024::from(1u32)
                } else {
                    U1024::MIN
                };
            }
        }
        (VRegister::U512(a), VRegister::U512(ref mut r)) => {
            for i in 0..num {
                r[i] = if a[i] != U512::from(rhs as i64) {
                    U512::from(1u32)
                } else {
                    U512::MIN
                };
            }
        }
        (VRegister::U256(a), VRegister::U256(ref mut r)) => {
            for i in 0..num {
                r[i] = if a[i] != U256::from(rhs as i64) {
                    U256::from(1u32)
                } else {
                    U256::MIN
                };
            }
        }
        (VRegister::U128(a), VRegister::U128(ref mut r)) => {
            for i in 0..num {
                r[i] = if a[i] != rhs as i64 as i128 as u128 {
                    1
                } else {
                    0
                };
            }
        }
        (VRegister::U64(a), VRegister::U64(ref mut r)) => {
            for i in 0..num {
                r[i] = if a[i] != rhs { 1 } else { 0 };
            }
        }
        (VRegister::U32(a), VRegister::U32(ref mut r)) => {
            for i in 0..num {
                r[i] = if a[i] != rhs as u32 { 1 } else { 0 };
            }
        }
        (VRegister::U16(a), VRegister::U16(ref mut r)) => {
            for i in 0..num {
                r[i] = if a[i] != rhs as u16 { 1 } else { 0 };
            }
        }
        (VRegister::U8(a), VRegister::U8(ref mut r)) => {
            for i in 0..num {
                r[i] = if a[i] != rhs as u8 { 1 } else { 0 };
            }
        }
        _ => return Err(Error::Unexpected),
    }
    Ok(())
}

pub fn vfunc_msne_vi(
    lhs: &VRegister,
    rhs: i32,
    result: &mut VRegister,
    num: usize,
) -> Result<(), Error> {
    match (lhs, result) {
        (VRegister::U1024(a), VRegister::U1024(ref mut r)) => {
            for i in 0..num {
                r[i] = if a[i] != U1024::from(rhs) {
                    U1024::from(1u32)
                } else {
                    U1024::MIN
                };
            }
        }
        (VRegister::U512(a), VRegister::U512(ref mut r)) => {
            for i in 0..num {
                r[i] = if a[i] != U512::from(rhs) {
                    U512::from(1u32)
                } else {
                    U512::MIN
                };
            }
        }
        (VRegister::U256(a), VRegister::U256(ref mut r)) => {
            for i in 0..num {
                r[i] = if a[i] != U256::from(rhs) {
                    U256::from(1u32)
                } else {
                    U256::MIN
                };
            }
        }
        (VRegister::U128(a), VRegister::U128(ref mut r)) => {
            for i in 0..num {
                r[i] = if a[i] != rhs as i128 as u128 { 1 } else { 0 };
            }
        }
        (VRegister::U64(a), VRegister::U64(ref mut r)) => {
            for i in 0..num {
                r[i] = if a[i] != rhs as i64 as u64 { 1 } else { 0 };
            }
        }
        (VRegister::U32(a), VRegister::U32(ref mut r)) => {
            for i in 0..num {
                r[i] = if a[i] != rhs as u32 { 1 } else { 0 };
            }
        }
        (VRegister::U16(a), VRegister::U16(ref mut r)) => {
            for i in 0..num {
                r[i] = if a[i] != rhs as i16 as u16 { 1 } else { 0 };
            }
        }
        (VRegister::U8(a), VRegister::U8(ref mut r)) => {
            for i in 0..num {
                r[i] = if a[i] != rhs as i8 as u8 { 1 } else { 0 };
            }
        }
        _ => return Err(Error::Unexpected),
    }
    Ok(())
}

pub fn vfunc_msltu_vv(
    lhs: &VRegister,
    rhs: &VRegister,
    result: &mut VRegister,
    num: usize,
) -> Result<(), Error> {
    match (lhs, rhs, result) {
        (VRegister::U1024(a), VRegister::U1024(b), VRegister::U1024(ref mut r)) => {
            for i in 0..num {
                r[i] = if a[i] < b[i] {
                    U1024::from(1u32)
                } else {
                    U1024::MIN
                };
            }
        }
        (VRegister::U512(a), VRegister::U512(b), VRegister::U512(ref mut r)) => {
            for i in 0..num {
                r[i] = if a[i] < b[i] {
                    U512::from(1u32)
                } else {
                    U512::MIN
                };
            }
        }
        (VRegister::U256(a), VRegister::U256(b), VRegister::U256(ref mut r)) => {
            for i in 0..num {
                r[i] = if a[i] < b[i] {
                    U256::from(1u32)
                } else {
                    U256::MIN
                };
            }
        }
        (VRegister::U128(a), VRegister::U128(b), VRegister::U128(ref mut r)) => {
            for i in 0..num {
                r[i] = if a[i] < b[i] { 1 } else { 0 };
            }
        }
        (VRegister::U64(a), VRegister::U64(b), VRegister::U64(ref mut r)) => {
            for i in 0..num {
                r[i] = if a[i] < b[i] { 1 } else { 0 };
            }
        }
        (VRegister::U32(a), VRegister::U32(b), VRegister::U32(ref mut r)) => {
            for i in 0..num {
                r[i] = if a[i] < b[i] { 1 } else { 0 };
            }
        }
        (VRegister::U16(a), VRegister::U16(b), VRegister::U16(ref mut r)) => {
            for i in 0..num {
                r[i] = if a[i] < b[i] { 1 } else { 0 };
            }
        }
        (VRegister::U8(a), VRegister::U8(b), VRegister::U8(ref mut r)) => {
            for i in 0..num {
                r[i] = if a[i] < b[i] { 1 } else { 0 };
            }
        }
        _ => return Err(Error::Unexpected),
    }
    Ok(())
}

pub fn vfunc_msltu_vx(
    lhs: &VRegister,
    rhs: u64,
    result: &mut VRegister,
    num: usize,
) -> Result<(), Error> {
    match (lhs, result) {
        (VRegister::U1024(a), VRegister::U1024(ref mut r)) => {
            for i in 0..num {
                r[i] = if a[i] < U1024::from(rhs) {
                    U1024::from(1u32)
                } else {
                    U1024::MIN
                };
            }
        }
        (VRegister::U512(a), VRegister::U512(ref mut r)) => {
            for i in 0..num {
                r[i] = if a[i] < U512::from(rhs) {
                    U512::from(1u32)
                } else {
                    U512::MIN
                };
            }
        }
        (VRegister::U256(a), VRegister::U256(ref mut r)) => {
            for i in 0..num {
                r[i] = if a[i] < U256::from(rhs) {
                    U256::from(1u32)
                } else {
                    U256::MIN
                };
            }
        }
        (VRegister::U128(a), VRegister::U128(ref mut r)) => {
            for i in 0..num {
                r[i] = if a[i] < rhs as u128 { 1 } else { 0 };
            }
        }
        (VRegister::U64(a), VRegister::U64(ref mut r)) => {
            for i in 0..num {
                r[i] = if a[i] < rhs { 1 } else { 0 };
            }
        }
        (VRegister::U32(a), VRegister::U32(ref mut r)) => {
            for i in 0..num {
                r[i] = if a[i] < rhs as u32 { 1 } else { 0 };
            }
        }
        (VRegister::U16(a), VRegister::U16(ref mut r)) => {
            for i in 0..num {
                r[i] = if a[i] < rhs as u16 { 1 } else { 0 };
            }
        }
        (VRegister::U8(a), VRegister::U8(ref mut r)) => {
            for i in 0..num {
                r[i] = if a[i] < rhs as u8 { 1 } else { 0 };
            }
        }
        _ => return Err(Error::Unexpected),
    }
    Ok(())
}

pub fn vfunc_msleu_vv(
    lhs: &VRegister,
    rhs: &VRegister,
    result: &mut VRegister,
    num: usize,
) -> Result<(), Error> {
    match (lhs, rhs, result) {
        (VRegister::U1024(a), VRegister::U1024(b), VRegister::U1024(ref mut r)) => {
            for i in 0..num {
                r[i] = if a[i] <= b[i] {
                    U1024::from(1u32)
                } else {
                    U1024::MIN
                };
            }
        }
        (VRegister::U512(a), VRegister::U512(b), VRegister::U512(ref mut r)) => {
            for i in 0..num {
                r[i] = if a[i] <= b[i] {
                    U512::from(1u32)
                } else {
                    U512::MIN
                };
            }
        }
        (VRegister::U256(a), VRegister::U256(b), VRegister::U256(ref mut r)) => {
            for i in 0..num {
                r[i] = if a[i] <= b[i] {
                    U256::from(1u32)
                } else {
                    U256::MIN
                };
            }
        }
        (VRegister::U128(a), VRegister::U128(b), VRegister::U128(ref mut r)) => {
            for i in 0..num {
                r[i] = if a[i] <= b[i] { 1 } else { 0 };
            }
        }
        (VRegister::U64(a), VRegister::U64(b), VRegister::U64(ref mut r)) => {
            for i in 0..num {
                r[i] = if a[i] <= b[i] { 1 } else { 0 };
            }
        }
        (VRegister::U32(a), VRegister::U32(b), VRegister::U32(ref mut r)) => {
            for i in 0..num {
                r[i] = if a[i] <= b[i] { 1 } else { 0 };
            }
        }
        (VRegister::U16(a), VRegister::U16(b), VRegister::U16(ref mut r)) => {
            for i in 0..num {
                r[i] = if a[i] <= b[i] { 1 } else { 0 };
            }
        }
        (VRegister::U8(a), VRegister::U8(b), VRegister::U8(ref mut r)) => {
            for i in 0..num {
                r[i] = if a[i] <= b[i] { 1 } else { 0 };
            }
        }
        _ => return Err(Error::Unexpected),
    }
    Ok(())
}

pub fn vfunc_msleu_vx(
    lhs: &VRegister,
    rhs: u64,
    result: &mut VRegister,
    num: usize,
) -> Result<(), Error> {
    match (lhs, result) {
        (VRegister::U1024(a), VRegister::U1024(ref mut r)) => {
            for i in 0..num {
                r[i] = if a[i] <= U1024::from(rhs as i64) {
                    U1024::from(1u32)
                } else {
                    U1024::MIN
                };
            }
        }
        (VRegister::U512(a), VRegister::U512(ref mut r)) => {
            for i in 0..num {
                r[i] = if a[i] <= U512::from(rhs as i64) {
                    U512::from(1u32)
                } else {
                    U512::MIN
                };
            }
        }
        (VRegister::U256(a), VRegister::U256(ref mut r)) => {
            for i in 0..num {
                r[i] = if a[i] <= U256::from(rhs as i64) {
                    U256::from(1u32)
                } else {
                    U256::MIN
                };
            }
        }
        (VRegister::U128(a), VRegister::U128(ref mut r)) => {
            for i in 0..num {
                r[i] = if a[i] <= rhs as i64 as i128 as u128 {
                    1
                } else {
                    0
                };
            }
        }
        (VRegister::U64(a), VRegister::U64(ref mut r)) => {
            for i in 0..num {
                r[i] = if a[i] <= rhs { 1 } else { 0 };
            }
        }
        (VRegister::U32(a), VRegister::U32(ref mut r)) => {
            for i in 0..num {
                r[i] = if a[i] <= rhs as u32 { 1 } else { 0 };
            }
        }
        (VRegister::U16(a), VRegister::U16(ref mut r)) => {
            for i in 0..num {
                r[i] = if a[i] <= rhs as u16 { 1 } else { 0 };
            }
        }
        (VRegister::U8(a), VRegister::U8(ref mut r)) => {
            for i in 0..num {
                r[i] = if a[i] <= rhs as u8 { 1 } else { 0 };
            }
        }
        _ => return Err(Error::Unexpected),
    }
    Ok(())
}

pub fn vfunc_msleu_vi(
    lhs: &VRegister,
    rhs: i32,
    result: &mut VRegister,
    num: usize,
) -> Result<(), Error> {
    match (lhs, result) {
        (VRegister::U1024(a), VRegister::U1024(ref mut r)) => {
            for i in 0..num {
                r[i] = if a[i] <= U1024::from(rhs) {
                    U1024::from(1u32)
                } else {
                    U1024::MIN
                };
            }
        }
        (VRegister::U512(a), VRegister::U512(ref mut r)) => {
            for i in 0..num {
                r[i] = if a[i] <= U512::from(rhs) {
                    U512::from(1u32)
                } else {
                    U512::MIN
                };
            }
        }
        (VRegister::U256(a), VRegister::U256(ref mut r)) => {
            for i in 0..num {
                r[i] = if a[i] <= U256::from(rhs) {
                    U256::from(1u32)
                } else {
                    U256::MIN
                };
            }
        }
        (VRegister::U128(a), VRegister::U128(ref mut r)) => {
            for i in 0..num {
                r[i] = if a[i] <= rhs as i128 as u128 { 1 } else { 0 };
            }
        }
        (VRegister::U64(a), VRegister::U64(ref mut r)) => {
            for i in 0..num {
                r[i] = if a[i] <= rhs as i64 as u64 { 1 } else { 0 };
            }
        }
        (VRegister::U32(a), VRegister::U32(ref mut r)) => {
            for i in 0..num {
                r[i] = if a[i] <= rhs as u32 { 1 } else { 0 };
            }
        }
        (VRegister::U16(a), VRegister::U16(ref mut r)) => {
            for i in 0..num {
                r[i] = if a[i] <= rhs as i16 as u16 { 1 } else { 0 };
            }
        }
        (VRegister::U8(a), VRegister::U8(ref mut r)) => {
            for i in 0..num {
                r[i] = if a[i] <= rhs as i8 as u8 { 1 } else { 0 };
            }
        }
        _ => return Err(Error::Unexpected),
    }
    Ok(())
}

pub fn vfunc_msgtu_vx(
    lhs: &VRegister,
    rhs: u64,
    result: &mut VRegister,
    num: usize,
) -> Result<(), Error> {
    match (lhs, result) {
        (VRegister::U1024(a), VRegister::U1024(ref mut r)) => {
            for i in 0..num {
                r[i] = if a[i] > U1024::from(rhs as i64) {
                    U1024::from(1u32)
                } else {
                    U1024::MIN
                };
            }
        }
        (VRegister::U512(a), VRegister::U512(ref mut r)) => {
            for i in 0..num {
                r[i] = if a[i] > U512::from(rhs as i64) {
                    U512::from(1u32)
                } else {
                    U512::MIN
                };
            }
        }
        (VRegister::U256(a), VRegister::U256(ref mut r)) => {
            for i in 0..num {
                r[i] = if a[i] > U256::from(rhs as i64) {
                    U256::from(1u32)
                } else {
                    U256::MIN
                };
            }
        }
        (VRegister::U128(a), VRegister::U128(ref mut r)) => {
            for i in 0..num {
                r[i] = if a[i] > rhs as i64 as i128 as u128 {
                    1
                } else {
                    0
                };
            }
        }
        (VRegister::U64(a), VRegister::U64(ref mut r)) => {
            for i in 0..num {
                r[i] = if a[i] > rhs { 1 } else { 0 };
            }
        }
        (VRegister::U32(a), VRegister::U32(ref mut r)) => {
            for i in 0..num {
                r[i] = if a[i] > rhs as u32 { 1 } else { 0 };
            }
        }
        (VRegister::U16(a), VRegister::U16(ref mut r)) => {
            for i in 0..num {
                r[i] = if a[i] > rhs as u16 { 1 } else { 0 };
            }
        }
        (VRegister::U8(a), VRegister::U8(ref mut r)) => {
            for i in 0..num {
                r[i] = if a[i] > rhs as u8 { 1 } else { 0 };
            }
        }
        _ => return Err(Error::Unexpected),
    }
    Ok(())
}

pub fn vfunc_msgtu_vi(
    lhs: &VRegister,
    rhs: i32,
    result: &mut VRegister,
    num: usize,
) -> Result<(), Error> {
    match (lhs, result) {
        (VRegister::U1024(a), VRegister::U1024(ref mut r)) => {
            for i in 0..num {
                r[i] = if a[i] > U1024::from(rhs) {
                    U1024::from(1u32)
                } else {
                    U1024::MIN
                };
            }
        }
        (VRegister::U512(a), VRegister::U512(ref mut r)) => {
            for i in 0..num {
                r[i] = if a[i] > U512::from(rhs) {
                    U512::from(1u32)
                } else {
                    U512::MIN
                };
            }
        }
        (VRegister::U256(a), VRegister::U256(ref mut r)) => {
            for i in 0..num {
                r[i] = if a[i] > U256::from(rhs) {
                    U256::from(1u32)
                } else {
                    U256::MIN
                };
            }
        }
        (VRegister::U128(a), VRegister::U128(ref mut r)) => {
            for i in 0..num {
                r[i] = if a[i] > rhs as i128 as u128 { 1 } else { 0 };
            }
        }
        (VRegister::U64(a), VRegister::U64(ref mut r)) => {
            for i in 0..num {
                r[i] = if a[i] > rhs as i64 as u64 { 1 } else { 0 };
            }
        }
        (VRegister::U32(a), VRegister::U32(ref mut r)) => {
            for i in 0..num {
                r[i] = if a[i] > rhs as u32 { 1 } else { 0 };
            }
        }
        (VRegister::U16(a), VRegister::U16(ref mut r)) => {
            for i in 0..num {
                r[i] = if a[i] > rhs as i16 as u16 { 1 } else { 0 };
            }
        }
        (VRegister::U8(a), VRegister::U8(ref mut r)) => {
            for i in 0..num {
                r[i] = if a[i] > rhs as i8 as u8 { 1 } else { 0 };
            }
        }
        _ => return Err(Error::Unexpected),
    }
    Ok(())
}

pub fn vfunc_msgt_vx(
    lhs: &VRegister,
    rhs: u64,
    result: &mut VRegister,
    num: usize,
) -> Result<(), Error> {
    match (lhs, result) {
        (VRegister::U1024(a), VRegister::U1024(ref mut r)) => {
            for i in 0..num {
                r[i] = if I1024::from(a[i]) > I1024::from(U1024::from(rhs as i64)) {
                    U1024::from(1u32)
                } else {
                    U1024::MIN
                };
            }
        }
        (VRegister::U512(a), VRegister::U512(ref mut r)) => {
            for i in 0..num {
                r[i] = if I512::from(a[i]) > I512::from(U512::from(rhs as i64)) {
                    U512::from(1u32)
                } else {
                    U512::MIN
                };
            }
        }
        (VRegister::U256(a), VRegister::U256(ref mut r)) => {
            for i in 0..num {
                r[i] = if I256::from(a[i]) > I256::from(U256::from(rhs as i64)) {
                    U256::from(1u32)
                } else {
                    U256::MIN
                };
            }
        }
        (VRegister::U128(a), VRegister::U128(ref mut r)) => {
            for i in 0..num {
                r[i] = if a[i] as i128 > rhs as i64 as i128 {
                    1
                } else {
                    0
                };
            }
        }
        (VRegister::U64(a), VRegister::U64(ref mut r)) => {
            for i in 0..num {
                r[i] = if a[i] as i64 > rhs as i64 { 1 } else { 0 };
            }
        }
        (VRegister::U32(a), VRegister::U32(ref mut r)) => {
            for i in 0..num {
                r[i] = if a[i] as i32 > rhs as u32 as i32 {
                    1
                } else {
                    0
                };
            }
        }
        (VRegister::U16(a), VRegister::U16(ref mut r)) => {
            for i in 0..num {
                r[i] = if a[i] as i16 > rhs as u16 as i16 {
                    1
                } else {
                    0
                };
            }
        }
        (VRegister::U8(a), VRegister::U8(ref mut r)) => {
            for i in 0..num {
                r[i] = if a[i] as i8 > rhs as u8 as i8 { 1 } else { 0 };
            }
        }
        _ => return Err(Error::Unexpected),
    }
    Ok(())
}

pub fn vfunc_msgt_vi(
    lhs: &VRegister,
    rhs: i32,
    result: &mut VRegister,
    num: usize,
) -> Result<(), Error> {
    match (lhs, result) {
        (VRegister::U1024(a), VRegister::U1024(ref mut r)) => {
            for i in 0..num {
                r[i] = if I1024::from(a[i]) > I1024::from(U1024::from(rhs)) {
                    U1024::from(1u32)
                } else {
                    U1024::MIN
                };
            }
        }
        (VRegister::U512(a), VRegister::U512(ref mut r)) => {
            for i in 0..num {
                r[i] = if I512::from(a[i]) > I512::from(U512::from(rhs)) {
                    U512::from(1u32)
                } else {
                    U512::MIN
                };
            }
        }
        (VRegister::U256(a), VRegister::U256(ref mut r)) => {
            for i in 0..num {
                r[i] = if I256::from(a[i]) > I256::from(U256::from(rhs)) {
                    U256::from(1u32)
                } else {
                    U256::MIN
                };
            }
        }
        (VRegister::U128(a), VRegister::U128(ref mut r)) => {
            for i in 0..num {
                r[i] = if a[i] as i128 > rhs as i128 { 1 } else { 0 };
            }
        }
        (VRegister::U64(a), VRegister::U64(ref mut r)) => {
            for i in 0..num {
                r[i] = if a[i] as i64 > rhs as i64 { 1 } else { 0 };
            }
        }
        (VRegister::U32(a), VRegister::U32(ref mut r)) => {
            for i in 0..num {
                r[i] = if a[i] as i32 > rhs { 1 } else { 0 };
            }
        }
        (VRegister::U16(a), VRegister::U16(ref mut r)) => {
            for i in 0..num {
                r[i] = if a[i] as i16 > rhs as i16 { 1 } else { 0 };
            }
        }
        (VRegister::U8(a), VRegister::U8(ref mut r)) => {
            for i in 0..num {
                r[i] = if a[i] as i8 > rhs as i8 { 1 } else { 0 };
            }
        }
        _ => return Err(Error::Unexpected),
    }
    Ok(())
}
