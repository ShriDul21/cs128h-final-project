use ndarray::{Array2};
use num_complex::Complex;
use std::fmt::Debug;

pub trait Gate: GateClone + Debug {
    // matrix representation of gate
    fn matrix(&self) -> Array2<Complex<f64>>;

    //how many qubits the gate operates on
    fn num_qubits(&self) -> usize;

    // string name of gate
    fn name(&self) -> &'static str;
}

pub trait GateClone {
    fn clone_box(&self) -> Box<dyn Gate>;
}

impl<T> GateClone for T
where
    T: 'static + Gate + Clone,
{
    fn clone_box(&self) -> Box<dyn Gate> {
        Box::new(self.clone())
    }
}

impl Clone for Box<dyn Gate> {
    fn clone(&self) -> Box<dyn Gate> {
        self.clone_box()
    }
}

// identity gate
#[derive(Clone, Debug)]
pub struct I{
    theta : f64
}

impl Gate for I {
    fn matrix(&self) -> Array2<Complex<f64>> {
        Array2::eye(2).mapv(|x| Complex::new(x,0.0))
    }
    fn num_qubits(&self) -> usize { 1 }
    fn name(&self) -> &'static str { "I" }
}

// CNOT gate
#[derive(Clone, Debug)]
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
#[derive(Clone, Debug)]
pub struct H;

impl Gate for H {
    fn matrix(&self) -> Array2<Complex<f64>> {
        let sqrt_2 = 2.0_f64.sqrt()/ 2.0;
        ndarray::array![
           [Complex::new(sqrt_2, 0.0), Complex::new(sqrt_2, 0.0)],
           [Complex::new(sqrt_2, 0.0), Complex::new(-1.0 * sqrt_2, 0.0)]
        ]
    }
    fn num_qubits(&self) -> usize { 1 }
    fn name(&self) -> &'static str { "H" }
}

#[derive(Clone, Debug)]
pub struct X;

impl Gate for X{
    fn matrix(&self) -> Array2<Complex<f64>> {
        
        ndarray::array![
           [Complex::new(0.0, 0.0), Complex::new(1.0, 0.0)],
           [Complex::new(1.0, 0.0), Complex::new(0.0, 0.0)]
        ]
    }
    fn num_qubits(&self) -> usize { 1 }
    fn name(&self) -> &'static str { "X" }
}

#[derive(Clone, Debug)]
pub struct CCZ;
impl Gate for CCZ{
    fn matrix(&self) -> Array2<Complex<f64>>{

        let diag = ndarray::arr1(&[Complex::new(1.0,0.0),Complex::new(1.0,0.0),
                                                                            Complex::new(1.0,0.0),Complex::new(1.0,0.0),
                                                                            Complex::new(1.0,0.0),Complex::new(1.0,0.0),
                                                                            Complex::new(1.0,0.0),Complex::new(-1.0,0.0)]);

        return Array2::from_diag(&diag);
        
    }
    fn num_qubits(&self) -> usize {3}
    fn name(&self) -> &'static str { "CCZ" }
}

#[derive(Clone, Debug)]
pub struct Y;
impl Gate for Y{
    fn matrix(&self) -> Array2<Complex<f64>>{

        ndarray::array![
           [Complex::new(0.0, 0.0), Complex::new(0.0, -1.0)],
           [Complex::new(0.0, 1.0), Complex::new(0.0, 0.0)]
        ]
        
    }
    fn num_qubits(&self) -> usize {1}
    fn name(&self) -> &'static str { "Y" }
}

#[derive(Clone, Debug)]
pub struct CY;
impl Gate for CY{
    fn matrix(&self) -> Array2<Complex<f64>>{

        ndarray::array![
           [Complex::new(1.0, 0.0), Complex::new(0.0, 0.0), Complex::new(0.0,0.0), Complex::new(0.0,0.0)],
           [Complex::new(0.0, 0.0), Complex::new(1.0, 0.0), Complex::new(0.0, 0.0), Complex::new(0.0, 0.0)], 
           [Complex::new(0.0, 0.0), Complex::new(0.0, 0.0), Complex::new(0.0, 0.0), Complex::new(0.0, -1.0)], 
           [Complex::new(0.0, 0.0), Complex::new(0.0, 0.0), Complex::new(1.0, 0.0), Complex::new(0.0, 0.0)] 
        ]
        
    }
    fn num_qubits(&self) -> usize {2}
    fn name(&self) -> &'static str { "CY" }
}

#[derive(Clone, Debug)]
pub struct CZ;
impl Gate for CZ{
    fn matrix(&self) -> Array2<Complex<f64>>{

        ndarray::array![
           [Complex::new(1.0, 0.0), Complex::new(0.0, 0.0), Complex::new(0.0,0.0), Complex::new(0.0,0.0)],
           [Complex::new(0.0, 0.0), Complex::new(1.0, 0.0), Complex::new(0.0, 0.0), Complex::new(0.0, 0.0)], 
           [Complex::new(0.0, 0.0), Complex::new(0.0, 0.0), Complex::new(1.0, 0.0), Complex::new(0.0, 0.0)], 
           [Complex::new(0.0, 0.0), Complex::new(0.0, 0.0), Complex::new(0.0, 0.0), Complex::new(-1.0, 0.0)] 
        ]
        
    }
    fn num_qubits(&self) -> usize {2}
    fn name(&self) -> &'static str { "CZ" }
}

#[derive(Clone, Debug)]
pub struct RX{
    pub theta: f64
}
impl Gate for RX{
    fn matrix(&self) -> Array2<Complex<f64>>{
        ndarray::array![
           [Complex::new((self.theta / 2.0).cos(), 0.0), Complex::new(0.0, -(self.theta / 2.0).sin())],
           [Complex::new(0.0, -(self.theta / 2.0).sin()), Complex::new((self.theta / 2.0).cos(), 0.0)] 
        ]
    }
    fn num_qubits(&self) -> usize {1}
    fn name(&self) -> &'static str { "RX" }
}

#[derive(Clone, Debug)]
pub struct RY{
    pub theta: f64
}
impl Gate for RY{
    fn matrix(&self) -> Array2<Complex<f64>>{
        ndarray::array![
           [Complex::new((self.theta / 2.0).cos(), 0.0), Complex::new(-(self.theta / 2.0).sin(), 0.0)],
           [Complex::new((self.theta / 2.0).sin(), 0.0), Complex::new((self.theta / 2.0).cos(), 0.0)] 
        ]
    }
    fn num_qubits(&self) -> usize {1}
    fn name(&self) -> &'static str { "RY" }
}

#[derive(Clone, Debug)]
pub struct RZ{
    pub theta: f64
}
impl Gate for RZ{
    fn matrix(&self) -> Array2<Complex<f64>>{
        ndarray::array![
           [Complex::new((self.theta / 2.0).cos(), -(self.theta / 2.0).sin()), Complex::new(0.0, 0.0)],
           [Complex::new(0.0, 0.0), Complex::new((self.theta / 2.0).cos(), (self.theta / 2.0).sin())] 
        ]
    }
    fn num_qubits(&self) -> usize {1}
    fn name(&self) -> &'static str { "RZ" }
}

#[derive(Clone, Debug)]
pub struct CRX{
    pub theta: f64
}
impl Gate for CRX{
    fn matrix(&self) -> Array2<Complex<f64>>{
        
        ndarray::array![
           [Complex::new(1.0, 0.0), Complex::new(0.0, 0.0), Complex::new(0.0,0.0), Complex::new(0.0,0.0)],
           [Complex::new(0.0, 0.0), Complex::new(1.0, 0.0), Complex::new(0.0, 0.0), Complex::new(0.0, 0.0)], 
           [Complex::new(0.0, 0.0), Complex::new(0.0, 0.0), Complex::new((self.theta / 2.0).cos(), 0.0), Complex::new(0.0, -(self.theta / 2.0).sin())], 
           [Complex::new(0.0, 0.0), Complex::new(0.0, 0.0), Complex::new(0.0, -(self.theta / 2.0).sin()), Complex::new((self.theta / 2.0).cos(), 0.0)] 
        ]

    }
    fn num_qubits(&self) -> usize {2}
    fn name(&self) -> &'static str { "CRX" }
}

#[derive(Clone, Debug)]
pub struct CRY{
    pub theta: f64
}
impl Gate for CRY{
    fn matrix(&self) -> Array2<Complex<f64>>{
        
        ndarray::array![
           [Complex::new(1.0, 0.0), Complex::new(0.0, 0.0), Complex::new(0.0,0.0), Complex::new(0.0,0.0)],
           [Complex::new(0.0, 0.0), Complex::new(1.0, 0.0), Complex::new(0.0, 0.0), Complex::new(0.0, 0.0)], 
           [Complex::new(0.0, 0.0), Complex::new(0.0, 0.0), Complex::new((self.theta / 2.0).cos(), 0.0), Complex::new(-(self.theta / 2.0).sin(), 0.0)], 
           [Complex::new(0.0, 0.0), Complex::new(0.0, 0.0), Complex::new((self.theta / 2.0).sin(), 0.0), Complex::new((self.theta / 2.0).cos(), 0.0)] 
        ]

    }
    fn num_qubits(&self) -> usize {2}
    fn name(&self) -> &'static str { "CRY" }
}

#[derive(Clone, Debug)]
pub struct CRZ{
    pub theta: f64
}
impl Gate for CRZ{
    fn matrix(&self) -> Array2<Complex<f64>>{
        
        ndarray::array![
           [Complex::new(1.0, 0.0), Complex::new(0.0, 0.0), Complex::new(0.0,0.0), Complex::new(0.0,0.0)],
           [Complex::new(0.0, 0.0), Complex::new(1.0, 0.0), Complex::new(0.0, 0.0), Complex::new(0.0, 0.0)], 
           [Complex::new(0.0, 0.0), Complex::new(0.0, 0.0), Complex::new((self.theta / 2.0).cos(), -(self.theta / 2.0).sin()), Complex::new(0.0, 0.0)], 
           [Complex::new(0.0, 0.0), Complex::new(0.0, 0.0), Complex::new(0.0, 0.0), Complex::new((self.theta / 2.0).cos(), (self.theta / 2.0).sin())] 
        ]

    }
    fn num_qubits(&self) -> usize {2}
    fn name(&self) -> &'static str { "CRZ" }
}