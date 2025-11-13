pub mod circuit;
pub mod gateinstance;
pub mod gates;
pub mod timestep;
pub mod utils;

use crate::circuit::Circuit;
use crate::gateinstance::GateInstance;
use crate::gates::{H,CNOT};
use num_complex::Complex;
use crate::utils::print_matrix;
fn main() {
    let mut c = Circuit::new(2);

    let gates = vec![
        GateInstance::new(1, vec![0], Box::new(H)),
        GateInstance::new(0, vec![0,1], Box::new(CNOT)),
    ];

    let final_unitary = c.compute(gates);
    print_matrix(&final_unitary, "final unitary");
    let state_vector = ndarray::array![
           [Complex::new(1.0, 0.0), Complex::new(0.0, 0.0), Complex::new(0.0, 0.0), Complex::new(0.0,0.0)] 
    ];
    print_matrix(&state_vector.dot(&final_unitary), "final state");
   
}
