use criterion::{black_box, criterion_group, criterion_main, Criterion};
use modularity::{ModularInt, MontgomeryArithmetic, MontgomeryContext};

fn bench_modular_addition(c: &mut Criterion) {
    let mut group = c.benchmark_group("ModularAddition");

    // Benchmark for small modulus
    group.bench_function("small_modulus", |b| {
        let a = ModularInt::<u64>::new(5u64, 17);
        let b = ModularInt::<u64>::new(7u64, 17);
        b.iter(|| black_box(a.add_mod(&b)));
    });

    // Benchmark for large modulus
    let large_prime = 0xFFFFFFFFFFFFFFFBu64; // 2^64 - 5
    group.bench_function("large_modulus", |b| {
        let a = ModularInt::<u64>::new(0xABCDEF0123456789u64, large_prime);
        let b = ModularInt::<u64>::new(0x123456789ABCDEFu64, large_prime);
        b.iter(|| black_box(a.add_mod(&b)));
    });

    group.finish();
}

fn bench_modular_multiplication(c: &mut Criterion) {
    let mut group = c.benchmark_group("ModularMultiplication");

    // Benchmark for small modulus
    group.bench_function("small_modulus", |b| {
        let a = ModularInt::<u64>::new(5u64, 17);
        let b = ModularInt::<u64>::new(7u64, 17);
        b.iter(|| black_box(a.mul_mod(&b)));
    });

    // Benchmark for large modulus
    let large_prime = 0xFFFFFFFFFFFFFFFBu64; // 2^64 - 5
    group.bench_function("large_modulus", |b| {
        let a = ModularInt::<u64>::new(0xABCDEF0123456789u64, large_prime);
        let b = ModularInt::<u64>::new(0x123456789ABCDEFu64, large_prime);
        b.iter(|| black_box(a.mul_mod(&b)));
    });

    group.finish();
}

fn bench_montgomery_multiplication(c: &mut Criterion) {
    let mut group = c.benchmark_group("MontgomeryMultiplication");

    // Benchmark for small modulus
    group.bench_function("small_modulus", |b| {
        let modulus = 17u64;
        let ctx = MontgomeryContext::new(modulus);
        let a = ModularInt::<u64>::new(5u64, modulus).to_montgomery(&ctx);
        let b = ModularInt::<u64>::new(7u64, modulus).to_montgomery(&ctx);
        b.iter(|| black_box(a.montgomery_mul(&b, &ctx)));
    });

    // Benchmark for large modulus
    let large_prime = 0xFFFFFFFFFFFFFFFBu64; // 2^64 - 5
    group.bench_function("large_modulus", |b| {
        let ctx = MontgomeryContext::new(large_prime);
        let a = ModularInt::<u64>::new(0xABCDEF0123456789u64, large_prime).to_montgomery(&ctx);
        let b = ModularInt::<u64>::new(0x123456789ABCDEFu64, large_prime).to_montgomery(&ctx);
        b.iter(|| black_box(a.montgomery_mul(&b, &ctx)));
    });

    group.finish();
}

criterion_group!(
    benches,
    bench_modular_addition,
    bench_modular_multiplication,
    bench_montgomery_multiplication
);
criterion_main!(benches);
