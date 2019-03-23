extern crate num_complex;
extern crate rand;

use num_complex::Complex32;
use rand::prelude::*;
use std::time::{Duration, SystemTime};
use std::thread::sleep;
use std::sync::atomic::{AtomicUsize, Ordering};

mod matrixmult;
use matrixmult::gemm;

static MULT_DONE: AtomicUsize = AtomicUsize::new(0);

fn main() {

    /*** Part 1: Latency - how fast can you make matrix multiplication ***/
    let num_iter = 1000;

    let start = SystemTime::now();
    for _ in 0..num_iter {
        run_matrixmult();
    }
    let duration = start.elapsed();

    // note down average latency
    // latency = duration / num_iter



    /*** Part 2: Throughput - how many matrix multiplications can you perform in parallel ***/

    // Num of seconds to run the evaulation
    let time = 30;

    // we sleep for 30 seconds
    sleep(Duration::new(30, 0));

    // Note down values of measurement thread and total throughput
    // throughput = MULT_DONE / time
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

fn run_matrixmult() {
    let m = 64;
    let a = generate_random_matrix(m);
    let b = generate_random_matrix(m);
    let mut c = generate_zeroed_matrix(m);

    let _ = gemm(&a, m, m, &b, m, m, &mut c);

    MULT_DONE.fetch_add(1, Ordering::Relaxed);
}