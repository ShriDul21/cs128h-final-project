use std::collections::HashMap;
use crate::gateinstance::GateInstance;
use crate::timestep::Timestep;

use ndarray::Array2;
use num_complex::Complex;
pub struct Circuit {
    pub num_qubits: usize,
    pub timesteps: Vec<Timestep>,
}

impl Circuit {
    pub fn new(num_qubits: usize) -> Self {
        Self { num_qubits, timesteps: vec![] }
    }
    pub fn compute(&mut self, gates: Vec<GateInstance>) -> Array2<Complex<f64>> {
        // grouping gates by time <time, gates>
        let mut map: HashMap<usize, Vec<GateInstance>> = HashMap::new();
        for g in gates {
            map.entry(g.time).or_default().push(g);
        }

        let mut times: Vec<usize> = map.keys().cloned().collect();
        times.sort();

        self.timesteps = times.iter().map(|t| Timestep { gates: map[t].clone() }).collect();
        println!("Time map keys: {:?}", map.keys());
        for (t, gs) in &map {
            println!("timestep {}: {:?}", t, gs.iter().map(|g| g.gate.name()).collect::<Vec<_>>());
        }       

        // compile each timestep sequentially
        let mut final_unitary = Array2::<Complex<f64>>::eye(2_usize.pow(self.num_qubits as u32));
        for ts in &self.timesteps {
            let u = ts.compile(self.num_qubits);
            final_unitary = u.dot(&final_unitary);
          
        }

        final_unitary
    }
}

