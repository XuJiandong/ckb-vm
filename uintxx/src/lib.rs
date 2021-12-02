#[derive(Copy, Clone, Default, PartialEq, Eq)]
pub struct U128(pub u128);

impl U128 {
    /// The size of this integer type in bits.
    pub const BITS: u32 = 128;

    /// The smallest value that can be represented by this integer type.
    pub const MIN: Self = Self(u128::MIN);

    /// The largest value that can be represented by this integer type.
    pub const MAX: Self = Self(u128::MAX);

    /// The one value that can be represented by this integer type.
    pub const ONE: Self = Self(1);
}

impl std::fmt::LowerHex for U128 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:032x}", self.0)
    }
}

impl std::fmt::Debug for U128 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:032x}", self.0)
    }
}

impl std::fmt::Display for U128 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:032x}", self.0)
    }
}

impl std::ops::BitAnd for U128 {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self::Output {
        Self(self.0 & rhs.0)
    }
}

impl std::ops::BitAndAssign for U128 {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0 &= rhs.0
    }
}

impl std::ops::BitOr for U128 {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self::Output {
        Self(self.0 | rhs.0)
    }
}

impl std::ops::BitOrAssign for U128 {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0 |= rhs.0
    }
}

impl std::ops::BitXor for U128 {
    type Output = Self;
    fn bitxor(self, rhs: Self) -> Self::Output {
        Self(self.0 ^ rhs.0)
    }
}

impl std::ops::BitXorAssign for U128 {
    fn bitxor_assign(&mut self, rhs: Self) {
        self.0 ^= rhs.0
    }
}

impl std::ops::Not for U128 {
    type Output = Self;
    fn not(self) -> Self::Output {
        Self(!self.0)
    }
}

impl std::ops::Neg for U128 {
    type Output = Self;
    fn neg(self) -> Self::Output {
        Self((!self.0).wrapping_add(1))
    }
}

impl std::cmp::PartialOrd for U128 {
    fn partial_cmp(&self, rhs: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(rhs))
    }
}

impl std::cmp::Ord for U128 {
    fn cmp(&self, rhs: &Self) -> std::cmp::Ordering {
        self.0.cmp(&rhs.0)
    }
}

impl std::ops::Add for U128 {
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        self.wrapping_add(rhs)
    }
}

impl std::ops::AddAssign for U128 {
    fn add_assign(&mut self, rhs: Self) {
        self.0 = self.0.wrapping_add(rhs.0)
    }
}

impl std::ops::Sub for U128 {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        self.wrapping_sub(rhs)
    }
}

impl std::ops::SubAssign for U128 {
    fn sub_assign(&mut self, rhs: Self) {
        self.0 = self.0.wrapping_sub(rhs.0)
    }
}

impl std::ops::Mul for U128 {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self::Output {
        self.wrapping_mul(rhs)
    }
}

impl std::ops::MulAssign for U128 {
    fn mul_assign(&mut self, rhs: Self) {
        self.0 = self.0.wrapping_mul(rhs.0)
    }
}

impl std::ops::Div for U128 {
    type Output = Self;
    fn div(self, rhs: Self) -> Self::Output {
        self.wrapping_div(rhs)
    }
}

impl std::ops::DivAssign for U128 {
    fn div_assign(&mut self, rhs: Self) {
        self.0 = self.0.wrapping_div(rhs.0);
    }
}

impl std::ops::Rem for U128 {
    type Output = Self;
    fn rem(self, rhs: Self) -> Self::Output {
        self.wrapping_rem(rhs)
    }
}

impl std::ops::RemAssign for U128 {
    fn rem_assign(&mut self, rhs: Self) {
        self.0 = self.0.wrapping_rem(rhs.0);
    }
}

impl std::ops::Shl<u32> for U128 {
    type Output = Self;
    fn shl(self, rhs: u32) -> Self::Output {
        self.wrapping_shl(rhs)
    }
}

impl std::ops::Shr<u32> for U128 {
    type Output = Self;
    fn shr(self, rhs: u32) -> Self::Output {
        self.wrapping_shr(rhs)
    }
}

impl U128 {
    /// Returns true if self is positive and false if the number is zero or negative.
    pub fn is_positive(self) -> bool {
        (self.0 as i128).is_positive()
    }

    /// Returns true if self is negative and false if the number is zero or positive.
    pub fn is_negative(self) -> bool {
        (self.0 as i128).is_negative()
    }

    /// Returns the lower 32 bits.
    pub fn u32(self) -> u32 {
        self.0 as u32
    }

    /// Returns the lower 64 bits.
    pub fn u64(self) -> u64 {
        self.0 as u64
    }

    /// Create a native endian integer value from its representation as a byte array in big endian.
    pub const fn from_be_bytes(bytes: [u8; 16]) -> Self {
        Self(u128::from_be_bytes(bytes))
    }

    /// Create a native endian integer value from its representation as a byte array in little endian.
    pub const fn from_le_bytes(bytes: [u8; 16]) -> Self {
        Self(u128::from_le_bytes(bytes))
    }

    /// Return the memory representation of this integer as a byte array in big-endian (network) byte order.
    pub fn to_be_bytes(self) -> [u8; 16] {
        self.0.to_be_bytes()
    }

    /// Return the memory representation of this integer as a byte array in little-endian byte order.
    pub fn to_le_bytes(self) -> [u8; 16] {
        self.0.to_le_bytes()
    }

    /// Returns the number of leading zeros in the binary representation of self.
    pub fn leading_zeros(self) -> u32 {
        self.0.leading_zeros()
    }

    /// Calculates self + rhs
    ///
    /// Returns a tuple of the addition along with a boolean indicating whether an arithmetic overflow would
    /// occur.
    /// If an overflow would have occurred then the wrapped value is returned.
    pub fn overflowing_add(self, rhs: Self) -> (Self, bool) {
        let (r, b) = self.0.overflowing_add(rhs.0);
        (Self(r), b)
    }

    /// Calculates self - rhs
    ///
    /// Returns a tuple of the subtraction along with a boolean indicating whether an arithmetic overflow would
    /// occur.
    /// If an overflow would have occurred then the wrapped value is returned.
    pub fn overflowing_sub(self, rhs: Self) -> (Self, bool) {
        let (r, b) = self.0.overflowing_sub(rhs.0);
        (Self(r), b)
    }

    /// Calculates the multiplication of self and rhs.
    ///
    /// Returns a tuple of the multiplication along with a boolean indicating whether an arithmetic overflow would
    /// occur. If an overflow would have occurred then the wrapped value is returned.
    pub fn overflowing_mul(self, rhs: Self) -> (Self, bool) {
        let (r, b) = self.0.overflowing_mul(rhs.0);
        (Self(r), b)
    }

    /// Calculates the divisor when self is divided by rhs.
    ///
    /// Returns a tuple of the divisor along with a boolean indicating whether an arithmetic overflow would occur. Note
    /// that for unsigned integers overflow never occurs, so the second value is always false.
    pub fn overflowing_div(self, rhs: Self) -> (Self, bool) {
        let (r, b) = self.0.overflowing_div(rhs.0);
        (Self(r), b)
    }

    /// Calculates the divisor when self is divided by rhs.
    ///
    /// Returns a tuple of the divisor along with a boolean indicating whether an arithmetic overflow would occur. Note
    /// that for unsigned integers overflow never occurs, so the second value is always false.
    pub fn overflowing_rem(self, rhs: Self) -> (Self, bool) {
        let (r, b) = self.0.overflowing_rem(rhs.0);
        (Self(r), b)
    }

    /// Wrapping (modular) addition. Computes self + rhs, wrapping around at the boundary of the type.
    pub fn wrapping_add(self, rhs: Self) -> Self {
        Self(self.0.wrapping_add(rhs.0))
    }

    /// Wrapping (modular) subtraction. Computes self - rhs, wrapping around at the boundary of the type.
    pub fn wrapping_sub(self, rhs: Self) -> Self {
        Self(self.0.wrapping_sub(rhs.0))
    }

    /// Wrapping (modular) multiplication. Computes self * rhs, wrapping around at the boundary of the type.
    pub fn wrapping_mul(self, rhs: Self) -> Self {
        Self(self.0.wrapping_mul(rhs.0))
    }

    /// Wrapping (modular) division. Computes self / rhs. Wrapped division on unsigned types is just normal division.
    /// There’s no way wrapping could ever happen. This function exists, so that all operations are accounted for in
    /// the wrapping operations.
    pub fn wrapping_div(self, rhs: Self) -> Self {
        Self(self.0.wrapping_div(rhs.0))
    }

    /// Wrapping (modular) remainder. Computes self % rhs. Wrapped remainder calculation on unsigned types is just the
    /// regular remainder calculation. There’s no way wrapping could ever happen. This function exists, so that all
    /// operations are accounted for in the wrapping operations.
    pub fn wrapping_rem(self, rhs: Self) -> Self {
        Self(self.0.wrapping_rem(rhs.0))
    }

    /// Panic-free bitwise shift-left; yields self << mask(rhs), where mask removes any high-order bits of rhs
    /// that would cause the shift to exceed the bitwidth of the type.
    ///
    /// Note that this is not the same as a rotate-left; the RHS of a wrapping shift-left is restricted to the
    /// range of the type, rather than the bits shifted out of the LHS being returned to the other end. The
    /// primitive integer types all implement a rotate_left function, which may be what you want instead.
    pub fn wrapping_shl(self, rhs: u32) -> Self {
        Self(self.0.wrapping_shl(rhs))
    }

    /// Panic-free bitwise shift-right; yields self >> mask(rhs), where mask removes any high-order bits of rhs
    /// that would cause the shift to exceed the bitwidth of the type.
    ///
    /// Note that this is not the same as a rotate-right; the RHS of a wrapping shift-right is restricted to
    /// the range of the type, rather than the bits shifted out of the LHS being returned to the other end. The
    /// primitive integer types all implement a rotate_right function, which may be what you want instead.
    pub fn wrapping_shr(self, rhs: u32) -> Self {
        Self(self.0.wrapping_shr(rhs))
    }

    /// Panic-free bitwise sign shift-right.
    pub fn wrapping_sra(self, rhs: u32) -> Self {
        Self((self.0 as i128).wrapping_shr(rhs) as u128)
    }

    /// Function mul_full returns the 256-bit product of x and y: (lo, hi) = x * y
    /// with the product bits' upper half returned in hi and the lower half returned in lo.
    ///
    /// Inspired by https://pkg.go.dev/math/bits@go1.17.2#Mul64
    pub fn widening_mul(self, rhs: Self) -> (Self, Self) {
        let lo = |x: u128| x & 0xffffffffffffffff;
        let hi = |x: u128| x >> 64;

        let x0 = lo(self.0);
        let x1 = hi(self.0);
        let y0 = lo(rhs.0);
        let y1 = hi(rhs.0);
        let w0 = x0 * y0;
        let t = x1 * y0 + hi(w0);
        let w1 = lo(t);
        let w2 = hi(t);
        let w1 = w1 + x0 * y1;
        let hi = x1 * y1 + w2 + hi(w1);
        let lo = self.0.wrapping_mul(rhs.0);
        (Self(lo), Self(hi))
    }
}

macro_rules! u128_impl_from {
    ($from:ty) => {
        impl std::convert::From<$from> for U128 {
            fn from(small: $from) -> Self {
                Self(small as u128)
            }
        }
    };
}

u128_impl_from!(bool);
u128_impl_from!(u8);
u128_impl_from!(u16);
u128_impl_from!(u32);
u128_impl_from!(u64);
u128_impl_from!(u128);
u128_impl_from!(i8);
u128_impl_from!(i16);
u128_impl_from!(i32);
u128_impl_from!(i64);
u128_impl_from!(i128);

macro_rules! uint_impl {
    ($name:ident, $half:ty, $bits:expr) => {
        #[derive(Copy, Clone, Default, PartialEq, Eq)]
        pub struct $name {
            pub lo: $half,
            pub hi: $half,
        }

        impl $name {
            /// The size of this integer type in bits.
            pub const BITS: u32 = $bits;

            /// The smallest value that can be represented by this integer type.
            pub const MIN: Self = Self {
                lo: <$half>::MIN,
                hi: <$half>::MIN,
            };

            /// The largest value that can be represented by this integer type.
            pub const MAX: Self = Self {
                lo: <$half>::MAX,
                hi: <$half>::MAX,
            };

            /// The one value that can be represented by this integer type.
            pub const ONE: Self = Self {
                lo: <$half>::ONE,
                hi: <$half>::MIN,
            };
        }

        impl std::fmt::LowerHex for $name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "{:x}{:x}", self.hi, self.lo)
            }
        }

        impl std::fmt::Debug for $name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "{:x}{:x}", self.hi, self.lo)
            }
        }

        impl std::fmt::Display for $name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "{:x}{:x}", self.hi, self.lo)
            }
        }

        impl std::ops::BitAnd for $name {
            type Output = Self;
            fn bitand(self, rhs: Self) -> Self::Output {
                Self {
                    lo: self.lo & rhs.lo,
                    hi: self.hi & rhs.hi,
                }
            }
        }

        impl std::ops::BitAndAssign for $name {
            fn bitand_assign(&mut self, rhs: Self) {
                self.lo &= rhs.lo;
                self.hi &= rhs.hi;
            }
        }

        impl std::ops::BitOr for $name {
            type Output = Self;
            fn bitor(self, rhs: Self) -> Self::Output {
                Self {
                    lo: self.lo | rhs.lo,
                    hi: self.hi | rhs.hi,
                }
            }
        }

        impl std::ops::BitOrAssign for $name {
            fn bitor_assign(&mut self, rhs: Self) {
                self.lo |= rhs.lo;
                self.hi |= rhs.hi;
            }
        }

        impl std::ops::BitXor for $name {
            type Output = Self;
            fn bitxor(self, rhs: Self) -> Self::Output {
                Self {
                    lo: self.lo ^ rhs.lo,
                    hi: self.hi ^ rhs.hi,
                }
            }
        }

        impl std::ops::BitXorAssign for $name {
            fn bitxor_assign(&mut self, rhs: Self) {
                self.lo ^= rhs.lo;
                self.hi ^= rhs.hi;
            }
        }

        impl std::ops::Not for $name {
            type Output = Self;
            fn not(self) -> Self::Output {
                Self {
                    lo: !self.lo,
                    hi: !self.hi,
                }
            }
        }

        impl std::ops::Neg for $name {
            type Output = Self;
            fn neg(self) -> Self::Output {
                (!self).wrapping_add(<$name>::ONE)
            }
        }

        impl std::cmp::PartialOrd for $name {
            fn partial_cmp(&self, rhs: &Self) -> Option<std::cmp::Ordering> {
                Some(self.cmp(rhs))
            }
        }

        impl std::cmp::Ord for $name {
            fn cmp(&self, rhs: &Self) -> std::cmp::Ordering {
                let hi_cmp = self.hi.cmp(&rhs.hi);
                if hi_cmp != std::cmp::Ordering::Equal {
                    hi_cmp
                } else {
                    self.lo.cmp(&rhs.lo)
                }
            }
        }

        impl std::ops::Add for $name {
            type Output = Self;
            fn add(self, rhs: Self) -> Self {
                self.wrapping_add(rhs)
            }
        }

        impl std::ops::AddAssign for $name {
            fn add_assign(&mut self, rhs: Self) {
                *self = self.wrapping_add(rhs)
            }
        }

        impl std::ops::Sub for $name {
            type Output = Self;
            fn sub(self, rhs: Self) -> Self::Output {
                self.wrapping_sub(rhs)
            }
        }

        impl std::ops::SubAssign for $name {
            fn sub_assign(&mut self, rhs: Self) {
                *self = self.wrapping_sub(rhs)
            }
        }

        impl std::ops::Mul for $name {
            type Output = Self;
            fn mul(self, rhs: Self) -> Self::Output {
                self.wrapping_mul(rhs)
            }
        }

        impl std::ops::MulAssign for $name {
            fn mul_assign(&mut self, rhs: Self) {
                *self = self.wrapping_mul(rhs)
            }
        }

        impl std::ops::Div for $name {
            type Output = Self;
            fn div(self, rhs: Self) -> Self::Output {
                self.wrapping_div(rhs)
            }
        }

        impl std::ops::DivAssign for $name {
            fn div_assign(&mut self, rhs: Self) {
                *self = self.wrapping_div(rhs)
            }
        }

        impl std::ops::Shl<u32> for $name {
            type Output = Self;
            fn shl(self, rhs: u32) -> Self::Output {
                self.wrapping_shl(rhs)
            }
        }

        impl std::ops::Shr<u32> for $name {
            type Output = Self;
            fn shr(self, rhs: u32) -> Self::Output {
                self.wrapping_shr(rhs)
            }
        }

        impl $name {
            /// Returns true if self is positive and false if the number is zero or negative.
            pub fn is_positive(self) -> bool {
                self != <$name>::MIN && self.wrapping_shr(Self::BITS - 1) == <$name>::MIN
            }

            /// Returns true if self is negative and false if the number is zero or positive.
            pub fn is_negative(self) -> bool {
                self != <$name>::MIN && self.wrapping_shr(Self::BITS - 1) == <$name>::ONE
            }

            /// Returns the lower 32 bits.
            pub fn u32(self) -> u32 {
                self.lo.u32()
            }

            /// Returns the lower 64 bits.
            pub fn u64(self) -> u64 {
                self.lo.u64()
            }

            /// Create a native endian integer value from its representation as a byte array in big endian.
            pub fn from_be_bytes(bytes: [u8; $bits / 8]) -> Self {
                let mut t = [0u8; $bits / 8 / 2];
                let a = 0x00;
                let b = $bits / 8 / 2;
                let c = b;
                let d = $bits / 8;
                t.copy_from_slice(&bytes[a..b]);
                let hi = <$half>::from_be_bytes(t);
                t.copy_from_slice(&bytes[c..d]);
                let lo = <$half>::from_be_bytes(t);
                Self { lo, hi }
            }

            /// Create a native endian integer value from its representation as a byte array in little endian.
            pub fn from_le_bytes(bytes: [u8; $bits / 8]) -> Self {
                let mut t = [0u8; $bits / 8 / 2];
                let a = 0x00;
                let b = $bits / 8 / 2;
                let c = b;
                let d = $bits / 8;
                t.copy_from_slice(&bytes[a..b]);
                let lo = <$half>::from_le_bytes(t);
                t.copy_from_slice(&bytes[c..d]);
                let hi = <$half>::from_le_bytes(t);
                Self { lo, hi }
            }

            /// Return the memory representation of this integer as a byte array in big-endian (network) byte order.
            pub fn to_be_bytes(self) -> [u8; $bits / 8] {
                let mut r = [0u8; $bits / 8];
                let a = 0x00;
                let b = $bits / 8 / 2;
                let c = b;
                let d = $bits / 8;
                r[a..b].copy_from_slice(&self.hi.to_be_bytes());
                r[c..d].copy_from_slice(&self.lo.to_be_bytes());
                return r;
            }

            /// Return the memory representation of this integer as a byte array in little-endian byte order.
            pub fn to_le_bytes(self) -> [u8; $bits / 8] {
                let mut r = [0u8; $bits / 8];
                let a = 0x00;
                let b = $bits / 8 / 2;
                let c = b;
                let d = $bits / 8;
                r[a..b].copy_from_slice(&self.lo.to_le_bytes());
                r[c..d].copy_from_slice(&self.hi.to_le_bytes());
                return r;
            }

            /// Returns the number of leading zeros in the binary representation of self.
            pub fn leading_zeros(self) -> u32 {
                if self.hi == <$half>::MIN {
                    Self::BITS / 2 + self.lo.leading_zeros()
                } else {
                    self.hi.leading_zeros()
                }
            }

            /// Calculates self + rhs
            ///
            /// Returns a tuple of the addition along with a boolean indicating whether an arithmetic overflow would
            /// occur.
            /// If an overflow would have occurred then the wrapped value is returned.
            pub fn overflowing_add(self, rhs: Self) -> (Self, bool) {
                let (lo, lo_carry) = self.lo.overflowing_add(rhs.lo);
                let (hi, hi_carry_1) = self.hi.overflowing_add(<$half>::from(lo_carry));
                let (hi, hi_carry_2) = hi.overflowing_add(rhs.hi);
                (Self { lo, hi }, hi_carry_1 || hi_carry_2)
            }

            /// Calculates self - rhs
            ///
            /// Returns a tuple of the subtraction along with a boolean indicating whether an arithmetic overflow would
            /// occur.
            /// If an overflow would have occurred then the wrapped value is returned.
            pub fn overflowing_sub(self, rhs: Self) -> (Self, bool) {
                let (lo, lo_borrow) = self.lo.overflowing_sub(rhs.lo);
                let (hi, hi_borrow_1) = self.hi.overflowing_sub(<$half>::from(lo_borrow));
                let (hi, hi_borrow_2) = hi.overflowing_sub(rhs.hi);
                (Self { lo, hi }, hi_borrow_1 || hi_borrow_2)
            }

            /// Calculates the multiplication of self and rhs.
            ///
            /// Returns a tuple of the multiplication along with a boolean indicating whether an arithmetic overflow
            /// would occur. If an overflow would have occurred then the wrapped value is returned.
            pub fn overflowing_mul(self, rhs: Self) -> (Self, bool) {
                let (hi, hi_overflow_mul) = match (self.hi, rhs.hi) {
                    (_, <$half>::MIN) => self.hi.overflowing_mul(rhs.lo),
                    (<$half>::MIN, _) => rhs.hi.overflowing_mul(self.lo),
                    _ => (
                        self.hi
                            .wrapping_mul(rhs.lo)
                            .wrapping_add(rhs.hi.wrapping_mul(self.lo)),
                        true,
                    ),
                };
                let lo = self.lo.widening_mul(rhs.lo);
                let lo = Self { lo: lo.0, hi: lo.1 };
                let (hi, hi_overflow_add) = lo.hi.overflowing_add(hi);
                let lo = Self { lo: lo.lo, hi: hi };
                (lo, hi_overflow_mul || hi_overflow_add)
            }

            /// Calculates the divisor when self is divided by rhs.
            ///
            /// Returns a tuple of the divisor along with a boolean indicating whether an arithmetic overflow would
            /// occur. Note that for unsigned integers overflow never occurs, so the second value is always false.
            pub fn overflowing_div(self, rhs: Self) -> (Self, bool) {
                (self.wrapping_div(rhs), false)
            }

            /// Calculates the divisor when self is divided by rhs.
            ///
            /// Returns a tuple of the divisor along with a boolean indicating whether an arithmetic overflow would
            /// occur. Note that for unsigned integers overflow never occurs, so the second value is always false.
            pub fn overflowing_rem(self, rhs: Self) -> (Self, bool) {
                (self.wrapping_rem(rhs), false)
            }

            /// Wrapping (modular) addition. Computes self + rhs, wrapping around at the boundary of the type.
            pub fn wrapping_add(self, rhs: Self) -> Self {
                let (lo, carry) = self.lo.overflowing_add(rhs.lo);
                let hi = self.hi.wrapping_add(rhs.hi).wrapping_add(<$half>::from(carry));
                Self { lo, hi }
            }

            /// Wrapping (modular) subtraction. Computes self - rhs, wrapping around at the boundary of the type.
            pub fn wrapping_sub(self, rhs: Self) -> Self {
                let (lo, borrow) = self.lo.overflowing_sub(rhs.lo);
                let hi = self.hi.wrapping_sub(rhs.hi).wrapping_sub(<$half>::from(borrow));
                Self { lo, hi }
            }

            /// Wrapping (modular) multiplication. Computes self * rhs, wrapping around at the boundary of the type.
            pub fn wrapping_mul(self, rhs: Self) -> Self {
                let (lo, hi) = self.lo.widening_mul(rhs.lo);
                let hi = hi
                    .wrapping_add(self.lo.wrapping_mul(rhs.hi))
                    .wrapping_add(self.hi.wrapping_mul(rhs.lo));
                Self { lo, hi }
            }

            /// div_half_0 returns the quotient and remainder of (hi, lo) divided by y: quo = (hi, lo)/y,
            /// rem = (hi, lo)%y with the dividend bits' upper half in parameter hi and the lower half in parameter lo.
            /// div_half_0 panics for y == 0 (division by zero) or y <= hi (quotient overflow).
            ///
            /// Inspired by https://cs.opensource.google/go/go/+/refs/tags/go1.17.3:src/math/bits/bits.go;l=512
            fn div_half_0(self, y: $half) -> ($half, $half) {
                let twos = <$half>::ONE << (Self::BITS / 4);
                let mask = twos - <$half>::ONE;
                assert!(y != <$half>::MIN);
                assert!(y > self.hi);
                let s = y.leading_zeros();
                let y = y << s;
                let yn1 = y >> (Self::BITS / 4);
                let yn0 = y & mask;
                let un32 = (self.hi << s) | (self.lo >> (Self::BITS / 2 - s));
                let un10 = self.lo << s;
                let un1 = un10 >> (Self::BITS / 4);
                let un0 = un10 & mask;
                let mut q1 = un32 / yn1;
                let mut rhat = un32 - q1 * yn1;
                while q1 >= twos || q1 * yn0 > twos * rhat + un1 {
                    q1 -= <$half>::ONE;
                    rhat += yn1;
                    if rhat >= twos {
                        break;
                    }
                }
                let un21 = un32 * twos + un1 - q1 * y;
                let mut q0 = un21 / yn1;
                rhat = un21 - q0 * yn1;
                while q0 >= twos || q0 * yn0 > twos * rhat + un0 {
                    q0 -= <$half>::ONE;
                    rhat += yn1;
                    if rhat >= twos {
                        break;
                    }
                }
                (q1 * twos + q0, (un21 * twos + un0 - q0 * y) >> s)
            }

            fn div_half_1(self, y: $half) -> (Self, $half) {
                if self.hi < y {
                    let (lo, r) = self.div_half_0(y);
                    (Self::from(lo), r)
                } else {
                    let (hi, r) = Self::from(self.hi).div_half_0(y);
                    let (lo, r) = Self { hi: r, lo: self.lo }.div_half_0(y);
                    (Self { lo: lo, hi: hi }, r)
                }
            }

            /// Inspired by https://github.com/Pilatuz/bigx/blob/8615506d17c5/uint128.go#L291
            fn div(self, rhs: Self) -> (Self, Self) {
                if rhs.hi == <$half>::MIN {
                    let (q, r) = self.div_half_1(rhs.lo);
                    return (q, Self::from(r));
                }
                let n = rhs.hi.leading_zeros();
                let u1 = self >> 1;
                let v1 = self << n;
                let (tq, _) = u1.div_half_0(v1.hi);
                let mut tq = tq >> ((Self::BITS / 2 - 1) - n);
                if tq != <$half>::MIN {
                    tq -= <$half>::ONE;
                }
                let mut q = Self::from(tq);
                let mut r = self - rhs * q;
                if r >= rhs {
                    q += Self::ONE;
                    r = r - rhs;
                }
                (q, r)
            }

            /// Wrapping (modular) division. Computes self / rhs. Wrapped division on unsigned types is just normal
            /// division. There’s no way wrapping could ever happen. This function exists, so that all operations are
            /// accounted for in the wrapping operations.
            pub fn wrapping_div(self, rhs: Self) -> Self {
                if rhs == Self::MIN {
                    Self::MAX
                } else {
                    self.div(rhs).0
                }
            }

            /// Wrapping (modular) remainder. Computes self % rhs. Wrapped remainder calculation on unsigned types is
            /// just the regular remainder calculation. There’s no way wrapping could ever happen. This function exists,
            /// so that all operations are accounted for in the wrapping operations.
            pub fn wrapping_rem(self, rhs: Self) -> Self {
                if rhs == Self::MIN {
                    self
                } else {
                    self.div(rhs).1
                }
            }

            /// Panic-free bitwise shift-left; yields self << mask(rhs), where mask removes any high-order bits of rhs
            /// that would cause the shift to exceed the bitwidth of the type.
            ///
            /// Note that this is not the same as a rotate-left; the RHS of a wrapping shift-left is restricted to the
            /// range of the type, rather than the bits shifted out of the LHS being returned to the other end. The
            /// primitive integer types all implement a rotate_left function, which may be what you want instead.
            pub fn wrapping_shl(self, rhs: u32) -> Self {
                if rhs < Self::BITS / 2 {
                    Self {
                        lo: self.lo.wrapping_shl(rhs),
                        hi: self.hi.wrapping_shl(rhs)
                            | self.lo.wrapping_shr(1).wrapping_shr((Self::BITS / 2) - 1 - rhs),
                    }
                } else if rhs < Self::BITS {
                    Self {
                        lo: <$half>::MIN,
                        hi: self.lo.wrapping_shl(rhs - Self::BITS / 2),
                    }
                } else {
                    Self::MIN
                }
            }

            /// Panic-free bitwise shift-right; yields self >> mask(rhs), where mask removes any high-order bits of rhs
            /// that would cause the shift to exceed the bitwidth of the type.
            ///
            /// Note that this is not the same as a rotate-right; the RHS of a wrapping shift-right is restricted to
            /// the range of the type, rather than the bits shifted out of the LHS being returned to the other end. The
            /// primitive integer types all implement a rotate_right function, which may be what you want instead.
            pub fn wrapping_shr(self, rhs: u32) -> Self {
                if rhs < Self::BITS / 2 {
                    Self {
                        lo: self.lo.wrapping_shr(rhs)
                            | self.hi.wrapping_shl(1).wrapping_shl((Self::BITS / 2) - 1 - rhs),
                        hi: self.hi.wrapping_shr(rhs),
                    }
                } else if rhs < Self::BITS {
                    Self {
                        lo: self.hi.wrapping_shr(rhs - Self::BITS / 2),
                        hi: <$half>::MIN,
                    }
                } else {
                    Self::MIN
                }
            }

            /// Panic-free bitwise sign shift-right.
            pub fn wrapping_sra(self, rhs: u32) -> Self {
                if self.is_negative() {
                    if rhs < Self::BITS / 2 {
                        Self {
                            lo: self.lo.wrapping_shr(rhs)
                                | self.hi.wrapping_shl(1).wrapping_shl(Self::BITS / 2 - 1 - rhs),
                            hi: self.hi.wrapping_sra(rhs),
                        }
                    } else if rhs < Self::BITS {
                        Self {
                            lo: self.hi.wrapping_sra(rhs - Self::BITS / 2),
                            hi: <$half>::MAX,
                        }
                    } else {
                        Self::MAX
                    }
                } else {
                    self.wrapping_shr(rhs)
                }
            }

            /// Calculates the complete product self * rhs without the possibility to overflow.
            ///
            /// This returns the low-order (wrapping) bits and the high-order (overflow) bits of the result as two
            /// separate values, in that order.
            pub fn widening_mul(self, rhs: Self) -> (Self, Self) {
                let lo = |x: Self| Self::from(x.lo);
                let hi = |x: Self| Self::from(x.hi);

                let x0 = lo(self);
                let x1 = hi(self);
                let y0 = lo(rhs);
                let y1 = hi(rhs);
                let w0 = x0 * y0;
                let t = x1 * y0 + hi(w0);
                let w1 = lo(t);
                let w2 = hi(t);
                let w1 = w1 + x0 * y1;
                let hi = x1 * y1 + w2 + hi(w1);
                let lo = self.wrapping_mul(rhs);
                (lo, hi)
            }
        }
    };
}

macro_rules! uint_impl_from_u {
    ($name:ident, $half:ty) => {
        impl std::convert::From<$half> for $name {
            fn from(small: $half) -> Self {
                Self {
                    lo: small,
                    hi: <$half>::MIN,
                }
            }
        }
    };
    ($name:ident, $half:ty, $from:ty) => {
        impl std::convert::From<$from> for $name {
            fn from(small: $from) -> Self {
                Self {
                    lo: <$half>::from(small),
                    hi: <$half>::MIN,
                }
            }
        }
    };
}

macro_rules! uint_impl_from_i {
    ($name:ident, $half:ty, $from:ty) => {
        impl std::convert::From<$from> for $name {
            fn from(small: $from) -> Self {
                Self {
                    lo: <$half>::from(small),
                    hi: if small > 0 { <$half>::MIN } else { <$half>::MAX },
                }
            }
        }
    };
}

uint_impl!(U256, U128, 256);
uint_impl_from_u!(U256, U128, bool);
uint_impl_from_u!(U256, U128, u8);
uint_impl_from_u!(U256, U128, u16);
uint_impl_from_u!(U256, U128, u32);
uint_impl_from_u!(U256, U128, u64);
uint_impl_from_u!(U256, U128, u128);
uint_impl_from_u!(U256, U128);
uint_impl_from_i!(U256, U128, i8);
uint_impl_from_i!(U256, U128, i16);
uint_impl_from_i!(U256, U128, i32);
uint_impl_from_i!(U256, U128, i64);
uint_impl_from_i!(U256, U128, i128);
uint_impl!(U512, U256, 512);
uint_impl_from_u!(U512, U256, bool);
uint_impl_from_u!(U512, U256, u8);
uint_impl_from_u!(U512, U256, u16);
uint_impl_from_u!(U512, U256, u32);
uint_impl_from_u!(U512, U256, u64);
uint_impl_from_u!(U512, U256, u128);
uint_impl_from_u!(U512, U256, U128);
uint_impl_from_u!(U512, U256);
uint_impl_from_i!(U512, U256, i8);
uint_impl_from_i!(U512, U256, i16);
uint_impl_from_i!(U512, U256, i32);
uint_impl_from_i!(U512, U256, i64);
uint_impl_from_i!(U512, U256, i128);
uint_impl!(U1024, U512, 1024);
uint_impl_from_u!(U1024, U512, bool);
uint_impl_from_u!(U1024, U512, u8);
uint_impl_from_u!(U1024, U512, u16);
uint_impl_from_u!(U1024, U512, u32);
uint_impl_from_u!(U1024, U512, u64);
uint_impl_from_u!(U1024, U512, u128);
uint_impl_from_u!(U1024, U512, U128);
uint_impl_from_u!(U1024, U512, U256);
uint_impl_from_u!(U1024, U512);
uint_impl_from_i!(U1024, U512, i8);
uint_impl_from_i!(U1024, U512, i16);
uint_impl_from_i!(U1024, U512, i32);
uint_impl_from_i!(U1024, U512, i64);
uint_impl_from_i!(U1024, U512, i128);

macro_rules! sint_impl {
    ($name:ident, $uint:ty) => {
        #[derive(Copy, Clone, Default, PartialEq, Eq)]
        pub struct $name {
            pub uint: $uint,
        }

        impl std::convert::From<$uint> for $name {
            fn from(uint: $uint) -> Self {
                Self { uint }
            }
        }

        impl std::cmp::PartialOrd for $name {
            fn partial_cmp(&self, rhs: &Self) -> Option<std::cmp::Ordering> {
                Some(self.cmp(rhs))
            }
        }

        impl std::cmp::Ord for $name {
            fn cmp(&self, rhs: &Self) -> std::cmp::Ordering {
                let lhssign = self.uint.is_negative();
                let rhssign = rhs.uint.is_negative();
                match (lhssign, rhssign) {
                    (false, false) => self.uint.cmp(&rhs.uint),
                    (false, true) => std::cmp::Ordering::Greater,
                    (true, false) => std::cmp::Ordering::Less,
                    (true, true) => rhs.uint.cmp(&self.uint),
                }
            }
        }

        impl $name {
            /// Panic-free bitwise shift-left; yields self << mask(rhs), where mask removes any high-order bits of rhs
            /// that would cause the shift to exceed the bitwidth of the type.
            ///
            /// Note that this is not the same as a rotate-left; the RHS of a wrapping shift-left is restricted to the
            /// range of the type, rather than the bits shifted out of the LHS being returned to the other end. The
            /// primitive integer types all implement a rotate_left function, which may be what you want instead.
            pub fn wrapping_shl(self, rhs: u32) -> Self {
                Self {
                    uint: self.uint.wrapping_shl(rhs),
                }
            }

            /// Panic-free bitwise sign shift-right.
            pub fn wrapping_shr(self, rhs: u32) -> Self {
                Self {
                    uint: self.uint.wrapping_sra(rhs),
                }
            }
        }
    };
}

sint_impl!(I256, U256);
sint_impl!(I512, U512);
sint_impl!(I1024, U1024);

mod internal {
    use uint::construct_uint;

    construct_uint! {
        pub struct U256(4);
    }

    construct_uint! {
        pub struct U512(8);
    }

    construct_uint! {
        pub struct U1024(16);
    }
}
