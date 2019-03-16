extern crate lin_alg;
extern crate num_complex;
extern crate rand;

use num_complex::Complex32;
use lin_alg::csvd::csvd;
use lin_alg::find_pinv_from_svd;
use rand::prelude::*;
use std::time::SystemTime;

fn main() {
    let n = 64;
    let m = 64;
    let num_iter = 1_000;

    // create input matrix and initiaze with random numbers
    let mut input_mat: Vec<Complex32> = Vec::with_capacity(m*n);
    let mut rng = rand::thread_rng();

    for _ in 0..m*n {
        input_mat.push(Complex32{re: rng.gen(), im: rng.gen()})
    }

    //create inverse matrix with dimension nxm
    let mut inverse_mat: Vec<Complex32> = Vec::with_capacity(n*m);
    for _ in 0..n*m {
        inverse_mat.push(Complex32{re: 0.0, im: 0.0});
}
    //create S vector with dimension n
    let mut s: Vec<f32> = Vec::with_capacity(n);
    for _ in 0..n {
        s.push(0.0);
    }

    //create U matrix dimension mxm
    let mut u: Vec<Complex32> = Vec::with_capacity(m*m);
    for _ in 0..m*m {
        u.push(Complex32{re: 0.0, im: 0.0});
    }

    //create v matrix with dimension nxn
    let mut v: Vec<Complex32> = Vec::with_capacity(n*n);
    for _ in 0..n*n {
        v.push(Complex32{re: 0.0, im: 0.0});
    }

    //now run pinv in a loop and note time
    let start = SystemTime::now();
    for _ in 0..num_iter {
        // csvd(&mut input_mat, m, n, n, m, 0, m, n, &mut s, &mut u, &mut v);
        find_pinv_from_svd(&mut s, &u, &v, m, n, &mut inverse_mat);
    }
    let duration = start.elapsed();

    println!("time = {:?}", duration.unwrap());
}
