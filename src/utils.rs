use ndarray::Array2;
use num_complex::Complex;

// used for printing matrices better bc ndarray ugly
pub fn print_matrix(matrix: &Array2<Complex<f64>>, label: &str) {
    println!("🔹 {} ({}×{}):", label, matrix.nrows(), matrix.ncols());
    for row in matrix.rows() {
        print!("  [");
        for (j, val) in row.iter().enumerate() {
            
            let re = if val.re.abs() < 1e-10 { 0.0 } else { val.re };
            let im = if val.im.abs() < 1e-10 { 0.0 } else { val.im };

            if im == 0.0 {
                print!("{:>7.3}", re);
            } else if re == 0.0 {
                print!("{:>7.3}i", im);
            } else {
                print!("{:>7.3}+{:>7.3}i", re, im);
            }

            if j < row.len() - 1 {
                print!(", ");
            }
        }
        println!(" ]");
    }
}
