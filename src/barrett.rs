// Barrett reduction implementation

use crate::ModularInt;
use num_traits::{One, Zero};
use std::fmt::Debug;
use std::ops::{Add, Div, Mul, Sub};

/// Barrett context for efficient modular reduction.
///
/// Barrett reduction is a technique for efficiently computing modular reduction
/// without performing division by the modulus. This context stores precomputed values
/// needed for Barrett reduction.
#[derive(Debug, Clone)]
pub struct BarrettContext<T> {
    modulus: T,
    mu: T, // Precomputed value for Barrett reduction
}

// Extension trait for ModularInt to use Barrett reduction
pub trait BarrettReduction<T> {
    fn barrett_reduce(&self, ctx: &BarrettContext<T>) -> ModularInt<T>;
    fn barrett_mul(&self, other: &ModularInt<T>, ctx: &BarrettContext<T>) -> ModularInt<T>;
}

// Generic implementation with placeholder methods
impl<T> BarrettContext<T>
where
    T: Copy
        + PartialEq
        + PartialOrd
        + Eq
        + Zero
        + One
        + Add<Output = T>
        + Sub<Output = T>
        + Mul<Output = T>
        + Div<Output = T>
        + Debug,
{
    /// Returns the modulus used in this Barrett context.
    pub fn modulus(&self) -> T {
        self.modulus
    }
}

// Implementation for u64
impl BarrettContext<u64> {
    /// Creates a new Barrett context for a u64 modulus.
    ///
    /// Precomputes values needed for efficient Barrett reduction.
    pub fn new(modulus: u64) -> Self {
        assert!(modulus > 0, "Modulus must be positive");

        // Compute mu = floor(2^128 / modulus)
        // This is used to estimate division by the modulus
        let mu = if modulus == 1 {
            0 // Special case, though not very useful
        } else {
            // Calculate floor(2^128 / modulus)
            // To avoid 128-bit division, compute it as (2^128 - 1) / modulus
            let max_u128 = u128::MAX;
            ((max_u128 / modulus as u128) + 1) as u64
        };

        Self { modulus, mu }
    }

    /// Performs Barrett reduction on the given value.
    ///
    /// This efficiently computes value % modulus without using the expensive
    /// modulo operation.
    pub fn reduce_u64(&self, value: u64) -> u64 {
        // If value < modulus, no reduction needed
        if value < self.modulus {
            return value;
        }

        // For small values, direct modulo is faster
        if value < self.modulus * 2 {
            return value - self.modulus;
        }

        // Barrett reduction algorithm
        // 1. q = floor(value * mu / 2^64)
        // 2. r = value - q * modulus
        // 3. if r >= modulus, r = r - modulus
        let q = ((value as u128 * self.mu as u128) >> 64) as u64;
        let mut r = value.wrapping_sub(q.wrapping_mul(self.modulus));

        // Final correction step
        if r >= self.modulus {
            r -= self.modulus;
        }

        r
    }

    /// Performs efficient modular multiplication using Barrett reduction.
    pub fn mul_mod_u64(&self, a: u64, b: u64) -> u64 {
        let product = a as u128 * b as u128;
        if product < self.modulus as u128 {
            return product as u64;
        }

        // For products that fit in u128, we can do reduction directly
        let product_hi = (product >> 64) as u64;
        let product_lo = product as u64;

        // Barrett reduction for the product
        // 1. q = floor(product * mu / 2^64)
        // 2. r = product - q * modulus
        // 3. if r >= modulus, r = r - modulus

        // Compute q for the product
        let q_hi = ((product_hi as u128 * self.mu as u128) >> 64) as u64;
        let q_lo_part1 = ((product_lo as u128 * self.mu as u128) >> 64) as u64;
        let q_lo_part2 = (product_lo as u128 * self.mu as u128) as u64;

        // q = q_hi * 2^64 + q_lo
        // We only need q_hi and q_lo_part1 for the computation

        // Compute r = product - q * modulus
        let qm = q_hi
            .wrapping_mul(self.modulus)
            .wrapping_add(q_lo_part1.wrapping_mul(self.modulus));
        let mut r = product_lo.wrapping_sub(qm);

        // Final correction
        if r >= self.modulus {
            r = r.wrapping_sub(self.modulus);
        }

        r
    }
}

// Specific implementation of Barrett reduction for ModularInt<u64>
impl BarrettReduction<u64> for ModularInt<u64> {
    fn barrett_reduce(&self, ctx: &BarrettContext<u64>) -> ModularInt<u64> {
        assert_eq!(self.modulus(), ctx.modulus(), "Modulus mismatch");
        ModularInt::new(ctx.reduce_u64(self.value()), self.modulus())
    }

    fn barrett_mul(&self, other: &ModularInt<u64>, ctx: &BarrettContext<u64>) -> ModularInt<u64> {
        assert_eq!(self.modulus(), ctx.modulus(), "Modulus mismatch for self");
        assert_eq!(other.modulus(), ctx.modulus(), "Modulus mismatch for other");
        ModularInt::new(ctx.mul_mod_u64(self.value(), other.value()), self.modulus())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_barrett_reduction_u64() {
        let modulus = 17u64;
        let ctx = BarrettContext::new(modulus);

        assert_eq!(ctx.reduce_u64(5), 5); // 5 mod 17 = 5
        assert_eq!(ctx.reduce_u64(17), 0); // 17 mod 17 = 0
        assert_eq!(ctx.reduce_u64(35), 1); // 35 mod 17 = 1
    }

    #[test]
    fn test_barrett_multiplication_u64() {
        let modulus = 17u64;
        let ctx = BarrettContext::new(modulus);

        assert_eq!(ctx.mul_mod_u64(5, 7), 1); // 5 * 7 mod 17 = 35 mod 17 = 1

        let a = ModularInt::new(5u64, modulus);
        let b = ModularInt::new(7u64, modulus);

        let result = a.barrett_mul(&b, &ctx);
        assert_eq!(result.value(), 1);
    }

    #[test]
    fn test_barrett_large_modulus() {
        let large_prime = 0xFFFFFFFFFFFFFFFBu64; // 2^64 - 5
        let ctx = BarrettContext::new(large_prime);

        let a = ModularInt::new(0xABCDEF0123456789u64, large_prime);
        let b = ModularInt::new(0x123456789ABCDEFu64, large_prime);

        let result = a.barrett_mul(&b, &ctx);

        // Verify using regular multiplication
        let expected = (a.value() as u128 * b.value() as u128 % large_prime as u128) as u64;
        assert_eq!(result.value(), expected);
    }
}
