#![feature(test)]

extern crate num_complex;
extern crate rand;
extern crate test;

use num_complex::Complex32;
use rand::prelude::*;
use std::time::{Duration, SystemTime};
use std::f32;

mod matrixmult;
use matrixmult::gemm;

static MATRIX_SIZE: usize = 64;

fn main() {
    // add your code here
}

// fn find_average_latency() -> u64 {
//     let num_iter: u64 = 1000;
//     let start = SystemTime::now();
//     for _ in 0..num_iter {
//         run_matrixmult();
//     }
//     let duration = start.elapsed();
    
//     // Return latency
//     // duration / num_iter
//     duration.as_secs() / num_iter
// }
    
fn print_matrix(mat: &Vec<Complex32>, rows: usize, cols: usize) {
    for i in 0..rows {
        for j in 0..cols{
            print!("{} + {}i, ", mat[i*cols + j].re, mat[i*cols + j].im);
        }
        println!("");
    }
    println!("");
}

// generates a randomly intialized matrix in row major order
fn generate_random_matrix(size: usize) -> Vec<Complex32> {
    let mut a: Vec<Complex32> = Vec::with_capacity(size * size);
    let mut rng = rand::thread_rng();

    for _ in 0..size*size {
        a.push(Complex32{re: rng.gen(), im: rng.gen()});
    }

    a
}

// generates a zeroed matrix in row major order
fn generate_zeroed_matrix(size: usize) -> Vec<Complex32> {
    let mut a: Vec<Complex32> = Vec::with_capacity(size * size);

    for _ in 0..size*size {
        a.push(Complex32{re: 0.0, im: 0.0});
    }
    
    a
}

/// checks that 2 mxn complex matrices are equal by taking the square of the euclidean distance between the elements
fn check_matrix_equality(a: &Vec<Complex32>, b: &Vec<Complex32>, m: usize, n:usize) -> bool {
    let mut equal = true;
    let eps = 0.0001;

    for i in 0..m {
        for j in 0..n {
            if (a[i*n + j].re - b[i*n + j].re).powf(2.0) + (a[i*n + j].im - b[i*n + j].im).powf(2.0) > eps {
                equal = false;
            }
        }
    }

    equal
}

fn run_matrixmult() {
    let m = MATRIX_SIZE;
    let a = generate_random_matrix(m);
    let b = generate_random_matrix(m);
    let mut c = generate_zeroed_matrix(m);

    let _ = gemm(&a, m, m, &b, m, m, &mut c);

}

#[cfg(test)]
mod tests {
    use super::*;
    use super::matrixmult::*;
    use test::Bencher;

    #[test]
    fn it_works() {
        let m = MATRIX_SIZE;

        //generate input matrices
        let a = generate_random_matrix(m);
        let b = generate_random_matrix(m);

        // find result with unoptimized version
        let mut c_unopt = generate_zeroed_matrix(m);
        let _ = gemm_unopt(&a, m, m, &b, m , m, &mut c_unopt);

        // find result with optimized version
        let mut c = generate_zeroed_matrix(m);
        let _ = gemm(&a, m, m, &b, m , m, &mut c);

        // check that results are equal
        let result = check_matrix_equality(&c_unopt, &c, m, m);

        assert_eq!(result, true);
    }

    #[bench]
    fn bench_matmul(bench: &mut Bencher) {
        bench.iter(|| run_matrixmult());
    }
}