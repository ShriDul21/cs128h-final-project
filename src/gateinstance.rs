use crate::gates::Gate;
#[derive(Clone)]

pub struct GateInstance {
    //the time it acts on
    pub time: usize,           

    // which qubits the gate acts on
    pub targets: Vec<usize>,   

    // the gate itself
    pub gate: Box<dyn Gate>,   
}

impl GateInstance {
    pub fn new(time: usize, targets: Vec<usize>, gate: Box<dyn Gate>) -> Self {
        Self { time, targets, gate }
    }
}
