#[cfg(feature = "hardware-acceleration")]
use modularity::intrinsics::ModularArithmeticAccelerated;
use modularity::ModularInt;
use std::time::{Duration, Instant};

fn main() {
    // This example demonstrates the use of hardware acceleration for modular arithmetic
    // It requires the `hardware-acceleration` feature to be enabled

    println!("Modular Arithmetic with Hardware Acceleration Example");
    println!("====================================================");

    #[cfg(feature = "hardware-acceleration")]
    {
        // Check if hardware acceleration is available
        let accel = ModularArithmeticAccelerated::new();
        if accel.use_acceleration {
            println!("Hardware acceleration is available and will be used.");
        } else {
            println!("Hardware acceleration is not available on this system.");
            println!("Falling back to software implementation.");
        }

        // Define test parameters
        let modulus = 0xFFFFFFFFFFFFFFFBu64; // 2^64 - 5, a large prime
        let a = 0xABCDEF0123456789u64;
        let b = 0x123456789ABCDEFu64;

        // Benchmark hardware-accelerated multiplication
        println!("\nBenchmarking hardware-accelerated multiplication...");
        let start = Instant::now();
        let mut result = 0;
        for _ in 0..1_000_000 {
            result = accel.mul_mod(a, b, modulus);
        }
        let hw_duration = start.elapsed();
        println!("Result: {:#x}", result);
        println!("Time: {:?}", hw_duration);

        // Compare with standard implementation
        println!("\nBenchmarking standard multiplication...");
        let start = Instant::now();
        let mut std_result = 0;
        for _ in 0..1_000_000 {
            std_result = ((a as u128 * b as u128) % modulus as u128) as u64;
        }
        let std_duration = start.elapsed();
        println!("Result: {:#x}", std_result);
        println!("Time: {:?}", std_duration);

        // Compare results
        assert_eq!(result, std_result, "Results must match!");

        // Print speedup
        if hw_duration < std_duration {
            let speedup = std_duration.as_nanos() as f64 / hw_duration.as_nanos() as f64;
            println!(
                "\nHardware acceleration provides a {:.2}x speedup.",
                speedup
            );
        } else {
            let slowdown = hw_duration.as_nanos() as f64 / std_duration.as_nanos() as f64;
            println!(
                "\nHardware acceleration is {:.2}x slower than standard implementation.",
                slowdown
            );
            println!("This might be due to overhead of feature detection or other factors.");
        }
    }

    #[cfg(not(feature = "hardware-acceleration"))]
    {
        println!("This example requires the 'hardware-acceleration' feature to be enabled.");
        println!("Please compile with:");
        println!("cargo run --example hardware_acceleration --features hardware-acceleration");
    }

    // Show comparison with regular ModularInt
    println!("\nUsing ModularInt for comparison:");
    let modulus = 17u64;
    let a = ModularInt::<u64>::new(5u64, modulus);
    let b = ModularInt::<u64>::new(7u64, modulus);

    let c = a.mul_mod(&b);
    println!("5 * 7 mod 17 = {}", c.value()); // Should output 1
}
