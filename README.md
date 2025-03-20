# Modularity

A Rust library for performant modular arithmetic.

## Overview

Modularity is a high-performance library for modular arithmetic operations in Rust, with a focus on efficiency and hardware acceleration. It provides various reduction techniques such as Montgomery and Barrett reduction for optimized modular operations.

## Features

- **Modular Integer Representation**: A `ModularInt` type that represents integers modulo a given modulus.
- **Basic Arithmetic Operations**: Efficient implementations of addition, subtraction, multiplication, and exponentiation.
- **Reduction Techniques**:
  - Montgomery Reduction for efficient modular multiplication
  - Barrett Reduction for efficient modular division and remainder operations
- **Hardware Acceleration**: Utilizes CPU intrinsics where available for maximum performance.
- **Benchmarking**: Comprehensive benchmarking suite to measure performance.

## Usage

```rust
use modularity::{ModularInt, MontgomeryContext, MontgomeryArithmetic};

fn main() {
    let modulus = 17;
    let a = ModularInt::new(5, modulus);
    let b = ModularInt::new(7, modulus);

    let c = a + b;
    println!("5 + 7 mod 17 = {}", c.value()); // Output: 12

    let d = a * b;
    println!("5 * 7 mod 17 = {}", d.value()); // Output: 1

    // Using Montgomery reduction for efficient multiplication
    let mont_ctx = MontgomeryContext::new(modulus);
    let a_mont = a.to_montgomery(&mont_ctx);
    let b_mont = b.to_montgomery(&mont_ctx);
    let c_mont = a_mont.montgomery_mul(&b_mont, &mont_ctx);
    println!("5 * 7 mod 17 using Montgomery = {}", c_mont.from_montgomery(&mont_ctx).value());
}
```

## Optional Features

- **arbitrary-precision**: Enables support for arbitrary-precision arithmetic using the `num-bigint` crate.
- **hardware-acceleration**: Enables hardware acceleration using CPU intrinsics.

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
modularity = "0.1.0"

# Optional: To enable arbitrary-precision arithmetic
modularity = { version = "0.1.0", features = ["arbitrary-precision"] }

# Optional: To enable hardware acceleration
modularity = { version = "0.1.0", features = ["hardware-acceleration"] }
```

## Performance

The library includes a comprehensive benchmarking suite to measure the performance of various operations. To run the benchmarks:

```bash
cargo bench
```

## License

This project is licensed under the MIT License - see the LICENSE file for details.
