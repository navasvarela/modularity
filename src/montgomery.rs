// Montgomery reduction implementation

use crate::ModularInt;
use num_traits::{One, Zero};
use std::fmt::Debug;
use std::ops::{Add, Mul, Sub};

/// Montgomery context for efficient modular multiplication.
///
/// Montgomery reduction is a technique for efficiently implementing modular multiplication
/// without performing division by the modulus. This context stores precomputed values
/// needed for Montgomery reduction.
#[derive(Debug, Clone)]
pub struct MontgomeryContext<T> {
    modulus: T,
    r_squared: T, // R² mod N
    r_inverse: T, // R⁻¹ mod N
    n_prime: T,   // -N⁻¹ mod R
}

// Generic trait for Montgomery arithmetic operations
pub trait MontgomeryArithmetic<T> {
    fn to_montgomery(&self, ctx: &MontgomeryContext<T>) -> ModularInt<T>;
    fn from_montgomery(&self, ctx: &MontgomeryContext<T>) -> ModularInt<T>;
    fn montgomery_mul(&self, other: &ModularInt<T>, ctx: &MontgomeryContext<T>) -> ModularInt<T>;
}

// Generic implementation with placeholder methods
impl<T> MontgomeryContext<T>
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
        + Debug,
{
    /// Returns the modulus used in this Montgomery context.
    pub fn modulus(&self) -> T {
        self.modulus
    }
}

// Implementation for u64
impl MontgomeryContext<u64> {
    /// Creates a new Montgomery context for the given modulus.
    ///
    /// Precomputes required values for Montgomery arithmetic.
    pub fn new(modulus: u64) -> Self {
        assert!(
            modulus % 2 == 1,
            "Modulus must be odd for Montgomery reduction"
        );
        assert!(modulus > 0, "Modulus must be positive");

        // Compute R = 2^64 mod n
        // For u64, R is implicitly 2^64, which is congruent to 0 mod 2^64
        // So we don't store R explicitly

        // Compute R^2 mod n
        let r_squared = Self::compute_r_squared(modulus);

        // Compute n' such that n * n' ≡ -1 (mod R)
        // This is equivalent to finding n' such that n * n' ≡ -1 (mod 2^64)
        let n_prime = Self::compute_n_prime(modulus);

        Self {
            modulus,
            r_squared,
            r_inverse: 1, // Not actually used in the implementation
            n_prime,
        }
    }

    /// Computes R^2 mod n where R = 2^64
    fn compute_r_squared(modulus: u64) -> u64 {
        // Start with 1 (which is 1 * R^0)
        // Square it 64 times, each time multiplying by 2^2 mod n
        // This gives us (2^2)^64 mod n = 2^128 mod n = R^2 mod n
        let mut result = 1u128;
        for _ in 0..128 {
            result = (result << 1) % modulus as u128;
        }
        result as u64
    }

    /// Computes n' such that n * n' ≡ -1 (mod 2^64)
    fn compute_n_prime(modulus: u64) -> u64 {
        // Extended Binary GCD to compute the modular inverse
        let mut t = 0u64;
        let mut r = 0u64;
        let mut new_t = 1u64;
        let mut new_r = modulus;
        let mut k = 0;
        let mut q = 0;
        let mut temp = 0;

        while new_r != 0 {
            temp = r;
            q = temp / new_r;
            r = new_r;
            new_r = temp - q * new_r;

            temp = t;
            t = new_t;
            new_t = temp.wrapping_sub(q.wrapping_mul(new_t));

            k += 1;
        }

        t.wrapping_neg()
    }

    /// Performs the Montgomery reduction.
    ///
    /// Given T = a * b, computes T * R^(-1) mod n efficiently.
    fn montgomery_reduction(&self, t: u128) -> u64 {
        // Compute m = (T mod R) * n' mod R
        let m = ((t as u64).wrapping_mul(self.n_prime)) as u128;

        // Compute t = (T + m * n) / R
        let t = (t + m * self.modulus as u128) >> 64;

        // If t >= n, return t - n; else return t
        if t >= self.modulus as u128 {
            (t - self.modulus as u128) as u64
        } else {
            t as u64
        }
    }
}

// Implementation of MontgomeryArithmetic for u64
impl MontgomeryArithmetic<u64> for ModularInt<u64> {
    fn to_montgomery(&self, ctx: &MontgomeryContext<u64>) -> ModularInt<u64> {
        assert_eq!(self.modulus(), ctx.modulus(), "Modulus mismatch");
        // To convert to Montgomery form, multiply by R^2 mod n and then reduce
        let mont_value = ctx.montgomery_reduction(self.value() as u128 * ctx.r_squared as u128);
        ModularInt::<u64>::new(mont_value, self.modulus())
    }

    fn from_montgomery(&self, ctx: &MontgomeryContext<u64>) -> ModularInt<u64> {
        assert_eq!(self.modulus(), ctx.modulus(), "Modulus mismatch");
        // To convert from Montgomery form, apply Montgomery reduction with 1
        let regular_value = ctx.montgomery_reduction(self.value() as u128);
        ModularInt::<u64>::new(regular_value, self.modulus())
    }

    fn montgomery_mul(
        &self,
        other: &ModularInt<u64>,
        ctx: &MontgomeryContext<u64>,
    ) -> ModularInt<u64> {
        assert_eq!(self.modulus(), ctx.modulus(), "Modulus mismatch for self");
        assert_eq!(other.modulus(), ctx.modulus(), "Modulus mismatch for other");

        // Montgomery multiplication is just regular multiplication followed by Montgomery reduction
        let result = ctx.montgomery_reduction(self.value() as u128 * other.value() as u128);
        ModularInt::<u64>::new(result, self.modulus())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_montgomery_multiplication_u64() {
        let modulus = 17u64;
        let ctx = MontgomeryContext::new(modulus);

        let a = ModularInt::<u64>::new(5u64, modulus);
        let b = ModularInt::<u64>::new(7u64, modulus);

        let a_mont = a.to_montgomery(&ctx);
        let b_mont = b.to_montgomery(&ctx);

        let result_mont = a_mont.montgomery_mul(&b_mont, &ctx);
        let result = result_mont.from_montgomery(&ctx);

        assert_eq!(result.value(), 1); // 5 * 7 mod 17 = 35 mod 17 = 1
    }

    #[test]
    fn test_montgomery_large_modulus() {
        let large_prime = 0xFFFFFFFFFFFFFFFBu64; // 2^64 - 5
        let ctx = MontgomeryContext::new(large_prime);

        let a = ModularInt::<u64>::new(0xABCDEF0123456789u64, large_prime);
        let b = ModularInt::<u64>::new(0x123456789ABCDEFu64, large_prime);

        let a_mont = a.to_montgomery(&ctx);
        let b_mont = b.to_montgomery(&ctx);

        let result_mont = a_mont.montgomery_mul(&b_mont, &ctx);
        let result = result_mont.from_montgomery(&ctx);

        // Verify using regular multiplication
        let expected = (a.value() as u128 * b.value() as u128 % large_prime as u128) as u64;
        assert_eq!(result.value(), expected);
    }
}
