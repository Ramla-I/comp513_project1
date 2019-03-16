extern crate num_complex;
extern crate rand;

use num_complex::Complex32;
use rand::prelude::*;
use std::time::SystemTime;

mod matrixmult;

use matrixmult::gemm;

fn main() {
    let m = 64;
    let num_iter = 1_000;

    // create input matrix A and initialize with random numbers
    let mut a: Vec<Complex32> = Vec::with_capacity(m*m);
    let mut rng = rand::thread_rng();

    for _ in 0..m*m {
        a.push(Complex32{re: rng.gen(), im: rng.gen()});
    }

    // create input matrix B and initialize with random numbers
    let mut b: Vec<Complex32> = Vec::with_capacity(m*m);
    for i in 0..m*m {
        b.push(Complex32{re: rng.gen(), im: rng.gen()});
    }

    // create output matrix C and clear
    let mut c: Vec<Complex32> = Vec::with_capacity(m*m);

    for _ in 0..m*m {
        c.push(Complex32{re: 0.0, im: 0.0})
    }

    // now run matrix mult in a loop and note time
    let start = SystemTime::now();
    for _ in 0..num_iter {
        let _ = gemm(&mut a, m, m, &mut b, m, m, &mut c);
    }
    let duration = start.elapsed();

    println!("time = {:?}", duration.unwrap());
}

fn print_matrix(mat: &Vec<Complex32>, rows: usize, cols: usize) {
    for i in 0..rows {
        for j in 0..cols{
            print!("{} + {}i, ", mat[i*cols + j].re, mat[i*cols + j].im);
        }
        println!("");
    }
    println!("");
}