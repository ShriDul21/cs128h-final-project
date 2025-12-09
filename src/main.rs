pub mod circuit;
pub mod gateinstance;
pub mod gates;
pub mod timestep;
pub mod utils;
pub mod app; 
pub mod circuitvisualizer;

use app::App;
use num_complex::Complex;

use crate::{circuit::Circuit, gateinstance::GateInstance, gates::{CNOT, H, X, RX}, utils::{print_matrix, print_statevector}};

fn main() {

    //let h = gates::RX{theta : 90.0};
    let mut c = Circuit::new(2);
let gates = vec![

GateInstance::new(0, vec![0], Box::new(RX{theta: 3.1415})),

GateInstance::new(1,vec![0], Box::new(X)),

];

  

let final_unitary = c.compute(gates);

print_matrix(&final_unitary, "final unitary");

  

//|00> state

let state_vector = ndarray::array![

[Complex::new(1.0, 0.0), Complex::new(0.0, 0.0), Complex::new(0.0, 0.0), Complex::new(0.0,0.0),

]

];

  

print_statevector(&state_vector.dot(&final_unitary));

    yew::Renderer::<App>::new().render();
}
