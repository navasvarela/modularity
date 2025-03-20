// Modularity: A Rust library for performant modular arithmetic

use num_traits::{One, Zero};
use std::fmt::Debug;
use std::ops::{Add, AddAssign, Mul, MulAssign, Neg, Sub, SubAssign};

mod barrett;
#[cfg(feature = "hardware-acceleration")]
mod intrinsics;
mod montgomery;

pub use barrett::BarrettContext;
pub use barrett::BarrettReduction;
#[cfg(feature = "hardware-acceleration")]
pub use intrinsics;
pub use montgomery::MontgomeryArithmetic;
pub use montgomery::MontgomeryContext;

/// Represents an integer modulo a given modulus.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ModularInt<T> {
    value: T,
    modulus: T,
}

/// Common methods shared by all ModularInt types
impl<T> ModularInt<T>
where
    T: Copy
        + PartialEq
        + PartialOrd
        + Eq
        + Zero
        + Add<Output = T>
        + Sub<Output = T>
        + Mul<Output = T>
        + Debug,
{
    /// Returns the value of the modular integer.
    pub fn value(&self) -> T {
        self.value
    }

    /// Returns the modulus of the modular integer.
    pub fn modulus(&self) -> T {
        self.modulus
    }
}

// Implement Add trait for ModularInt
impl<T> Add for ModularInt<T>
where
    T: Copy
        + PartialEq
        + PartialOrd
        + Eq
        + Zero
        + Add<Output = T>
        + Sub<Output = T>
        + Mul<Output = T>
        + Debug,
{
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        assert_eq!(self.modulus, other.modulus, "Modulus mismatch in addition");
        let mut result = self.clone();
        result.value = self.value + other.value;
        // Note: reduction is type-specific and needs to be handled by specialized implementations
        // The generic implementation without reduction is incomplete
        todo!("Implement addition for specific numeric types");
    }
}

// Implement AddAssign trait for ModularInt
impl<T> AddAssign for ModularInt<T>
where
    T: Copy
        + PartialEq
        + PartialOrd
        + Eq
        + Zero
        + Add<Output = T>
        + Sub<Output = T>
        + Mul<Output = T>
        + Debug,
{
    fn add_assign(&mut self, other: Self) {
        assert_eq!(
            self.modulus, other.modulus,
            "Modulus mismatch in add_assign"
        );
        self.value = self.value + other.value;
        // Note: reduction is type-specific and needs to be handled by specialized implementations
        todo!("Implement add_assign for specific numeric types");
    }
}

// Implement Sub trait for ModularInt
impl<T> Sub for ModularInt<T>
where
    T: Copy
        + PartialEq
        + PartialOrd
        + Eq
        + Zero
        + Add<Output = T>
        + Sub<Output = T>
        + Mul<Output = T>
        + Debug,
{
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        assert_eq!(
            self.modulus, other.modulus,
            "Modulus mismatch in subtraction"
        );
        // Handle subtraction based on the type
        // This is a simplified implementation
        todo!("Implement subtraction for specific numeric types");
    }
}

// Implement SubAssign trait for ModularInt
impl<T> SubAssign for ModularInt<T>
where
    T: Copy
        + PartialEq
        + PartialOrd
        + Eq
        + Zero
        + Add<Output = T>
        + Sub<Output = T>
        + Mul<Output = T>
        + Debug,
{
    fn sub_assign(&mut self, other: Self) {
        assert_eq!(
            self.modulus, other.modulus,
            "Modulus mismatch in sub_assign"
        );
        // Handle subtraction based on the type
        // This is a simplified implementation
        todo!("Implement subtraction_assign for specific numeric types");
    }
}

// Implement Mul trait for ModularInt
impl<T> Mul for ModularInt<T>
where
    T: Copy
        + PartialEq
        + PartialOrd
        + Eq
        + Zero
        + Add<Output = T>
        + Sub<Output = T>
        + Mul<Output = T>
        + Debug,
{
    type Output = Self;

    fn mul(self, other: Self) -> Self::Output {
        assert_eq!(
            self.modulus, other.modulus,
            "Modulus mismatch in multiplication"
        );
        let mut result = self.clone();
        result.value = self.value * other.value;
        // Note: reduction is type-specific and needs to be handled by specialized implementations
        todo!("Implement multiplication for specific numeric types");
    }
}

// Implement MulAssign trait for ModularInt
impl<T> MulAssign for ModularInt<T>
where
    T: Copy
        + PartialEq
        + PartialOrd
        + Eq
        + Zero
        + Add<Output = T>
        + Sub<Output = T>
        + Mul<Output = T>
        + Debug,
{
    fn mul_assign(&mut self, other: Self) {
        assert_eq!(
            self.modulus, other.modulus,
            "Modulus mismatch in mul_assign"
        );
        self.value = self.value * other.value;
        // Note: reduction is type-specific and needs to be handled by specialized implementations
        todo!("Implement mul_assign for specific numeric types");
    }
}

// Implement Neg trait for ModularInt
impl<T> Neg for ModularInt<T>
where
    T: Copy
        + PartialEq
        + PartialOrd
        + Eq
        + Zero
        + Add<Output = T>
        + Sub<Output = T>
        + Mul<Output = T>
        + Debug,
{
    type Output = Self;

    fn neg(self) -> Self::Output {
        // Implementation depends on the type
        todo!("Implement negation for specific numeric types");
    }
}

// Implementation for u64
impl ModularInt<u64> {
    /// Creates a new ModularInt with the given value and modulus.
    ///
    /// # Arguments
    ///
    /// * `value` - The integer value.
    /// * `modulus` - The modulus.
    ///
    /// # Panics
    ///
    /// Panics if the modulus is zero.
    pub fn new(value: u64, modulus: u64) -> Self {
        assert!(modulus > 0, "Modulus cannot be zero");
        let mut result = Self { value, modulus };
        result.reduce();
        result
    }

    /// Reduces the value to be within the range [0, modulus).
    fn reduce(&mut self) {
        if self.value >= self.modulus {
            self.value %= self.modulus;
        }
    }

    /// Performs modular addition.
    pub fn add_mod(&self, other: &Self) -> Self {
        assert_eq!(self.modulus, other.modulus, "Modulus mismatch in add_mod");
        let sum = self.value.wrapping_add(other.value);
        if sum >= self.modulus || sum < self.value {
            // Check for overflow
            Self::new(sum % self.modulus, self.modulus)
        } else {
            Self::new(sum, self.modulus)
        }
    }

    /// Performs modular subtraction.
    pub fn sub_mod(&self, other: &Self) -> Self {
        assert_eq!(self.modulus, other.modulus, "Modulus mismatch in sub_mod");
        if self.value >= other.value {
            Self::new(self.value - other.value, self.modulus)
        } else {
            Self::new(self.modulus - (other.value - self.value), self.modulus)
        }
    }

    /// Performs modular multiplication.
    pub fn mul_mod(&self, other: &Self) -> Self {
        assert_eq!(self.modulus, other.modulus, "Modulus mismatch in mul_mod");
        Self::new(
            (self.value as u128 * other.value as u128 % self.modulus as u128) as u64,
            self.modulus,
        )
    }

    /// Computes the modular exponentiation: self^exponent mod modulus.
    ///
    /// Uses the square-and-multiply algorithm for efficient computation.
    pub fn pow_mod(&self, exponent: u64) -> Self {
        if exponent == 0 {
            return Self::new(1, self.modulus);
        }

        let mut base = *self;
        let mut result = Self::new(1, self.modulus);
        let mut exp = exponent;

        while exp > 0 {
            if exp & 1 == 1 {
                result = result.mul_mod(&base);
            }
            base = base.mul_mod(&base);
            exp >>= 1;
        }

        result
    }

    /// Computes the modular inverse: self^(-1) mod modulus.
    ///
    /// Uses the extended Euclidean algorithm.
    ///
    /// # Panics
    ///
    /// Panics if the inverse does not exist (i.e., if gcd(self.value, modulus) != 1).
    pub fn inverse_mod(&self) -> Self {
        use num_integer::gcd;

        if self.value == 0 {
            panic!("Cannot compute the inverse of 0");
        }

        if gcd(self.value, self.modulus) != 1 {
            panic!("The inverse does not exist because gcd(value, modulus) != 1");
        }

        // Extended Euclidean Algorithm
        let (mut s, mut old_s) = (0i64, 1i64);
        let (mut t, mut old_t) = (1i64, 0i64);
        let (mut r, mut old_r) = (self.modulus as i64, self.value as i64);

        while r != 0 {
            let quotient = old_r / r;

            let temp = old_r;
            old_r = r;
            r = temp - quotient * r;

            let temp = old_s;
            old_s = s;
            s = temp - quotient * s;

            let temp = old_t;
            old_t = t;
            t = temp - quotient * t;
        }

        // Ensure we have the correct result
        if old_r != 1 {
            panic!("The inverse does not exist");
        }

        // Convert result to positive
        let result = if old_s < 0 {
            old_s + self.modulus as i64
        } else {
            old_s
        };

        Self::new(result as u64, self.modulus)
    }
}

// Implementation for u32
impl ModularInt<u32> {
    /// Creates a new ModularInt with the given value and modulus.
    ///
    /// # Arguments
    ///
    /// * `value` - The integer value.
    /// * `modulus` - The modulus.
    ///
    /// # Panics
    ///
    /// Panics if the modulus is zero.
    pub fn new(value: u32, modulus: u32) -> Self {
        assert!(modulus > 0, "Modulus cannot be zero");
        let mut result = Self { value, modulus };
        result.reduce();
        result
    }

    fn reduce(&mut self) {
        if self.value >= self.modulus {
            self.value %= self.modulus;
        }
    }

    pub fn add_mod(&self, other: &Self) -> Self {
        assert_eq!(self.modulus, other.modulus, "Modulus mismatch in add_mod");
        let sum = self.value.wrapping_add(other.value);
        if sum >= self.modulus || sum < self.value {
            Self::new(sum % self.modulus, self.modulus)
        } else {
            Self::new(sum, self.modulus)
        }
    }

    pub fn sub_mod(&self, other: &Self) -> Self {
        assert_eq!(self.modulus, other.modulus, "Modulus mismatch in sub_mod");
        if self.value >= other.value {
            Self::new(self.value - other.value, self.modulus)
        } else {
            Self::new(self.modulus - (other.value - self.value), self.modulus)
        }
    }

    pub fn mul_mod(&self, other: &Self) -> Self {
        assert_eq!(self.modulus, other.modulus, "Modulus mismatch in mul_mod");
        Self::new(
            (self.value as u64 * other.value as u64 % self.modulus as u64) as u32,
            self.modulus,
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_modular_int_creation() {
        let a = ModularInt::<u64>::new(5u64, 17);
        assert_eq!(a.value(), 5);
        assert_eq!(a.modulus(), 17);

        let b = ModularInt::<u64>::new(20u64, 17);
        assert_eq!(b.value(), 3); // 20 % 17 = 3
    }

    #[test]
    fn test_modular_arithmetic_u64() {
        let a = ModularInt::<u64>::new(5u64, 17);
        let b = ModularInt::<u64>::new(7u64, 17);

        let c = a.add_mod(&b);
        assert_eq!(c.value(), 12); // (5 + 7) % 17 = 12

        let d = a.sub_mod(&b);
        assert_eq!(d.value(), 15); // (5 - 7) % 17 = 15 (or 17-2=15)

        let e = a.mul_mod(&b);
        assert_eq!(e.value(), 1); // (5 * 7) % 17 = 35 % 17 = 1
    }

    #[test]
    fn test_modular_exponentiation() {
        let a = ModularInt::<u64>::new(2u64, 17);
        let b = a.pow_mod(4);
        assert_eq!(b.value(), 16); // 2^4 % 17 = 16

        let c = a.pow_mod(8);
        assert_eq!(c.value(), 1); // 2^8 % 17 = 256 % 17 = 1
    }

    #[test]
    fn test_modular_inverse() {
        let a = ModularInt::<u64>::new(3u64, 17);
        let b = a.inverse_mod();
        assert_eq!(b.value(), 6); // 3 * 6 % 17 = 18 % 17 = 1

        let c = ModularInt::<u64>::new(5u64, 17);
        let d = c.inverse_mod();
        assert_eq!(d.value(), 7); // 5 * 7 % 17 = 35 % 17 = 1
    }
}
