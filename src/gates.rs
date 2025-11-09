use ndarray::{Array2};
use num_complex::Complex;

pub trait Gate {
    // matrix representation of gate
    fn matrix(&self) -> Array2<Complex<f64>>;

    //how many qubits the gate operates on
    fn num_qubits(&self) -> usize;

    // string name of gate
    fn name(&self) -> &'static str;
}

// identity gate
pub struct I;
impl Gate for I {
    fn matrix(&self) -> Array2<Complex<f64>> {
        Array2::eye(2).mapv(|x| Complex::new(x,0.0))
    }
    fn num_qubits(&self) -> usize { 1 }
    fn name(&self) -> &'static str { "I" }
}

// CNOT gate
pub struct CNOT;
impl Gate for CNOT {
    fn matrix(&self) -> Array2<Complex<f64>> {

        ndarray::array![
            
            [Complex::new(1.0,0.0), Complex::new(0.0,0.0), Complex::new(0.0,0.0), Complex::new(0.0,0.0)],
            [Complex::new(0.0,0.0), Complex::new(1.0,0.0), Complex::new(0.0,0.0), Complex::new(0.0,0.0)],
            [Complex::new(0.0,0.0), Complex::new(0.0,0.0), Complex::new(0.0,0.0), Complex::new(1.0,0.0)],
            [Complex::new(0.0,0.0), Complex::new(0.0,0.0), Complex::new(1.0,0.0), Complex::new(0.0,0.0)]
        ]
    }
    fn num_qubits(&self) -> usize { 2 }
    fn name(&self) -> &'static str { "CNOT" }
}

// H gate
pub struct H;
impl Gate for H {
    fn matrix(&self) -> Array2<Complex<f64>> {
        let sqrt_2 = 2.0.sqrt();
        ndarray::array![
           [Complex::new(sqrt_2, 0.0), Complex::new(sqrt_2, 0.0)],
           [Complex::new(sqrt_2, 0.0), Complex::new(-1 * sqrt_2, 0.0)]
        ]
    }
    fn num_qubits(&self) -> usize { 1 }
    fn name(&self) -> &'static str { "H" }
}