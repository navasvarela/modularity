use modularity::{ModularInt, MontgomeryArithmetic, MontgomeryContext};

fn main() {
    // Example of basic modular arithmetic
    let modulus = 17u64;
    let a = ModularInt::<u64>::new(5u64, modulus);
    let b = ModularInt::<u64>::new(7u64, modulus);

    // Basic operations
    let c = a.add_mod(&b);
    println!("5 + 7 mod 17 = {}", c.value()); // Output: 12

    let d = a.sub_mod(&b);
    println!("5 - 7 mod 17 = {}", d.value()); // Output: 15

    let e = a.mul_mod(&b);
    println!("5 * 7 mod 17 = {}", e.value()); // Output: 1

    // Montgomery multiplication (requires specific implementation in MontgomeryContext)
    // This will panic with todo!() until implemented
    if false {
        // Guard to prevent panic
        let mont_ctx = MontgomeryContext::new(modulus);
        let a_mont = a.to_montgomery(&mont_ctx);
        let b_mont = b.to_montgomery(&mont_ctx);
        let c_mont = a_mont.montgomery_mul(&b_mont, &mont_ctx);
        let result = c_mont.from_montgomery(&mont_ctx);
        println!("5 * 7 mod 17 using Montgomery = {}", result.value()); // Should be 1
    }

    // Large modulus example
    let large_prime = 0xFFFFFFFFFFFFFFFBu64; // 2^64 - 5
    let x = ModularInt::<u64>::new(0xABCDEF0123456789u64, large_prime);
    let y = ModularInt::<u64>::new(0x123456789ABCDEFu64, large_prime);

    let z = x.add_mod(&y);
    println!("x + y mod p = {:#x}", z.value());

    let w = x.mul_mod(&y);
    println!("x * y mod p = {:#x}", w.value());

    println!("\nThis example shows basic functionality of the modularity library.");
    println!("Note that many advanced features are marked with todo!() and need implementation.");
}
