use ndarray::linalg::kron;
use num_complex::Complex;
use crate::gateinstance::GateInstance;
use ndarray::Array2;
use crate::utils::print_matrix;
pub struct Timestep {
    pub gates: Vec<GateInstance>,
}
impl Timestep {

    //compile the simulataneous gates
    pub fn compile(&self, num_qubits: usize) -> Array2<Complex<f64>> {
        let dim = 1 << num_qubits; // 2^num_qubits
        // start out w identity matrix
        let mut compiled = Array2::<Complex<f64>>::eye(dim);

        

        for g in &self.gates {
            println!("Compiling gate {} on targets {:?}", g.gate.name(), g.targets);

            // build full matrix for each gate instance
            let full_unitary = gate_instance_matrix(g, num_qubits);
            print_matrix(&g.gate.matrix(), "Gate matrix");
            print_matrix(&full_unitary, "Expanded to full matrix");
            
            compiled = full_unitary.dot(&compiled);
            print_matrix(&compiled,  "Compiled matrix");

        }

        compiled
    }
}

/// Build a full 2^n x 2^n matrix for a gate instance (naive rn)
fn gate_instance_matrix(_g: &GateInstance, num_qubits: usize) -> Array2<Complex<f64>> {
    
    let mut mats: Vec<Array2<Complex<f64>>> = Vec::new();
    let mut i: usize = 0;
    let start = *_g.targets.iter().min().unwrap();
    let num = _g.targets.len();
    while i < num_qubits {
        if i == start {
            mats.push(_g.gate.matrix().clone());
            i += num;
        } else {
            mats.push(Array2::eye(2).mapv(|x| Complex::new(x, 0.0)));
            i += 1;
        }
    }
    // for i in 0..num_qubits {
    //     if _g.targets.contains(&i) {
    //         mats.push(_g.gate.matrix());
    //     } else {
    //         mats.push(Array2::eye(2).mapv(|x| Complex::new(x, 0.0)));
    //     }
    // }

    let mut full = mats[0].clone();
    for m in mats.iter().skip(1) {
        full = kron(&full, m);
    }

    // TODO: implement CNOT target switching + multi qubit gate support

    
    full
}

