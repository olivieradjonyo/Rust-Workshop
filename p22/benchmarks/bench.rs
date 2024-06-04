// bench.rs
#![feature(test)]

extern crate test;

use std::mem::replace;
use test::Bencher;

// bench: find the `BENCH_SIZE` first terms of the fibonacci sequence
static BENCH_SIZE: usize = 20;


#[bench]
fn recursive_fibonacci(b: &mut Bencher) {
    b.iter(|| (0..BENCH_SIZE).map(fibonacci_rec).collect::<Vec<u32>>())
}

#[bench]
fn recursive_fibonacci(b: &mut Bencher) {
    b.iter(|| (0..BENCH_SIZE).map(fibonacci_loop).collect::<Vec<u32>>())
}
