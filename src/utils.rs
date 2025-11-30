use ndarray::{Array2};
use num_complex::Complex;

// used for printing matrices better bc ndarray ugly
pub fn print_matrix(matrix: &Array2<Complex<f64>>, label: &str) {
    println!("{} ({}×{}):", label, matrix.nrows(), matrix.ncols());
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

// qiskit style printing for easier verification lol
pub fn print_statevector(state: &Array2<Complex<f64>>) {
    let (rows, cols) = state.dim();
    let num_states = if rows == 1 { cols } else { rows };
    let num_qubits = (num_states as f64).log2().round() as usize;

    println!("\n Current state: ({} qubits, {} amplitudes):", num_qubits, num_states);
    println!("--- Amplitudes ---");

    let mut probs = Vec::new();

    for i in 0..num_states {
        let amp = if rows == 1 {
            state[[0, i]]
        } else {
            state[[i, 0]]
        };

        let prob = amp.norm_sqr();
        if prob < 1e-10 {
            continue;
        }

        let bitstring = format!("{:0width$b}", i, width = num_qubits);
        let magnitude = amp.norm();
        let phase_deg = amp.im.atan2(amp.re).to_degrees();

        println!(
            "|{}⟩ : {:.3} ∠ {:.1}°  ({:.3} + {:.3}i)   P = {:.3}",
            bitstring, magnitude, phase_deg, amp.re, amp.im, prob
        );

        probs.push((bitstring, prob));
    }

    println!("\n--- Probabilities ---");
    print!("{{ ");
    for (i, (bitstring, prob)) in probs.iter().enumerate() {
        print!("|{}⟩: {:.3}", bitstring, prob);
        if i < probs.len() - 1 {
            print!(", ");
        }
    }
    println!(" }}\n");
}
