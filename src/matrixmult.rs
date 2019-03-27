use num_complex::Complex32;

/// Un-optimized Implementation of general matrix multiplication
/// mat_c = mat_a * mat_b
pub fn gemm_unopt(mat_a: &[Complex32], a_rows: usize, a_cols: usize, mat_b: &[Complex32], b_rows: usize, b_cols: usize, mat_c: &mut[Complex32]) -> Result< (), &'static str> {
    if a_cols != b_rows {
        return Err("Matrix dimension not compatible!");
    }

    for i in 0..a_rows {
        for j in 0..b_cols {
            for k in 0..a_cols{
                mat_c[i * b_cols  + j] += mat_a[i*a_cols + k] * mat_b[k*b_cols + j]; 
            }
        }
    }

    Ok(())
}

/// Implementation of general matrix multiplication that you need to edit
/// mat_c = mat_a * mat_b
pub fn gemm(mat_a: &[Complex32], a_rows: usize, a_cols: usize, mat_b: &[Complex32], b_rows: usize, b_cols: usize, mat_c: &mut[Complex32]) -> Result< (), &'static str> {
    if a_cols != b_rows {
        return Err("Matrix dimension not compatible!");
    }

    println!("{}, {}", mat_a[0], mat_a[20]);

    for i in 0..a_rows {
        for j in 0..b_cols {
            for k in 0..a_cols{
                mat_c[i * b_cols  + j] += mat_a[i*a_cols + k] * mat_b[k*b_cols + j]; 
            }
        }
    }

    Ok(())
}