pub struct Timestep {
    pub gates: Vec<GateInstance>,
}
impl Timestep {

    //compile the simulataneous gates
    pub fn compile(&self, num_qubits: usize) -> Array2<Complex<f64>> {
        
        // start out w identity matrix
        let mut compiled = Array2::<Complex<f64>>::eye(2_usize.pow(num_qubits as u32));

        for g in &self.gates {

            // build full matrix for each gate instance
            let full_unitary = expand_gate(g, num_qubits);
            compiled = full_matrix.dot(&compiled);
        }

        compiled
    }
}

/// build 2^n by 2^n gate for each gate
fn expand_gate(_g: &GateInstance, num_qubits: usize) -> Array2<Complex<f64>> {
    
}
