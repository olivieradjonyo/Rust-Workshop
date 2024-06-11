use std::fmt;
use std::ops::{Add, Sub};

const NUM_LIMBS: usize = 64;

#[derive(Copy, Clone, Eq, PartialEq)]
pub struct BigUint4096 {
    limbs: [u64; NUM_LIMBS],
}

impl BigUint4096 {
    pub fn new() -> Self {
        BigUint4096 { limbs: [0; NUM_LIMBS] }
    }

    pub fn from_u64(val: u64) -> Self {
        let mut limbs = [0; NUM_LIMBS];
        limbs[0] = val;
        BigUint4096 { limbs }
    }

    pub fn from_hex_str(s: &str) -> Result<Self, &'static str> {
        if s.len() > 1024 {
            return Err("Hex string too long");
        }

        let mut limbs = [0u64; NUM_LIMBS];
        let hex = if s.len() % 16 == 0 { s.to_string() } else { format!("{:0>width$}", s, width = ((s.len() + 15) / 16) * 16) };
        let bytes = hex.as_bytes();

        for (i, chunk) in bytes.chunks(16).rev().enumerate() {
            let chunk_str = std::str::from_utf8(chunk).map_err(|_| "Invalid UTF-8 sequence")?;
            limbs[i] = u64::from_str_radix(chunk_str, 16).map_err(|_| "Invalid hex string")?;
        }

        Ok(BigUint4096 { limbs })
    }

    pub fn to_hex_string(&self) -> String {
        self.limbs.iter().rev().map(|limb| format!("{:016x}", limb)).collect::<Vec<_>>().concat()
    }
}

// Implement the Add trait for BigUint4096
impl Add for BigUint4096 {
    type Output = BigUint4096;

    fn add(self, other: BigUint4096) -> BigUint4096 {
        let mut result = BigUint4096::new();
        let mut carry = 0u64;

        for i in 0..NUM_LIMBS {
            let (sum, carry_out) = self.limbs[i].overflowing_add(other.limbs[i]);
            let (sum, carry_out2) = sum.overflowing_add(carry);
            result.limbs[i] = sum;
            carry = (carry_out as u64) + (carry_out2 as u64);
        }

        result
    }
}

// Implement the Sub trait for BigUint4096
impl Sub for BigUint4096 {
    type Output = BigUint4096;

    fn sub(self, other: BigUint4096) -> BigUint4096 {
        let mut result = BigUint4096::new();
        let mut borrow = false;

        for i in 0..NUM_LIMBS {
            let (diff, borrow_out) = self.limbs[i].overflowing_sub(other.limbs[i]);
            let (diff, borrow_out2) = if borrow {
                diff.overflowing_sub(1)
            } else {
                (diff, false)
            };
            result.limbs[i] = diff;
            borrow = borrow_out || borrow_out2;
        }

        result
    }
}

// Implement Debug trait for BigUint4096
impl fmt::Debug for BigUint4096 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "BigUint4096(")?;
        for &limb in self.limbs.iter().rev() {
            write!(f, "{:016x}", limb)?;
        }
        write!(f, ")")
    }
}

// Implement Display trait for BigUint4096
impl fmt::Display for BigUint4096 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "0x")?;
        for &limb in self.limbs.iter().rev() {
            write!(f, "{:016x}", limb)?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_addition() {
        let a = BigUint4096::from_u64(1);
        let b = BigUint4096::from_u64(2);
        let c = a + b;
        assert_eq!(c, BigUint4096::from_u64(3));
    }

    #[test]
    fn test_subtraction() {
        let a = BigUint4096::from_u64(3);
        let b = BigUint4096::from_u64(2);
        let c = a - b;
        let n = 0x1Au64;
        assert_eq!(u64::from_le(n), n);
        assert_eq!(c, BigUint4096::from_u64(1));
    }

    #[test]
    fn test_from_hex_str() {
        let hex_str = "1";
        let bigint = BigUint4096::from_hex_str(hex_str).unwrap();
        assert_eq!(bigint, BigUint4096::from_u64(1));

        let hex_str = "1234567890abcdef1234567890abcdef";
        let bigint = BigUint4096::from_hex_str(hex_str).unwrap();
        assert_eq!(bigint.limbs[0], 0x1234567890abcdef);
        assert_eq!(bigint.limbs[1], 0x1234567890abcdef);
    }

    #[test]
    fn test_to_hex_string() {
        let bigint = BigUint4096::from_u64(1);
        assert_eq!(bigint.to_hex_string(), "0000000000000001");

        let bigint = BigUint4096::from_hex_str("1234567890abcdef1234567890abcdef").unwrap();
        assert_eq!(bigint.to_hex_string(), "1234567890abcdef1234567890abcdef000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000");
    }
}
