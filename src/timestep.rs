use ndarray::linalg::kron;
use num_complex::Complex;
use crate::gateinstance::GateInstance;
use ndarray::{Array2};
// use crate::utils::print_matrix;

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
            // println!("Compiling gate {} on targets {:?}", g.gate.name(), g.targets);

            // build full matrix for each gate instance
            let full_unitary = gate_instance_matrix(g, num_qubits);
            // print_matrix(&g.gate.matrix(), "Gate matrix");
            // print_matrix(&full_unitary, "Expanded to full matrix");
            
            compiled = full_unitary.dot(&compiled);
            // print_matrix(&compiled,  "Compiled matrix");

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
    if num > 1 {
        full = permute_gate(full, &_g.targets, num)
    }
    

    
    full
}


// permuting gate using bit masking -- before, i used eisensum but this is more efficient
// i believe 
pub fn permute_gate(
    matrix: Array2<Complex<f64>>,
    targets: &[usize],
    gate_size: usize,
) -> Array2<Complex<f64>> {
    let dim = matrix.shape()[0];

    assert_eq!(matrix.shape()[1], dim);
    // make sure it's 2^n otherwise smth is wrong lolz
    assert!(dim.is_power_of_two());
    let n = (dim as f64).log2() as usize;

    let start = *targets.iter().min().unwrap();
    // if targets are already contiguous at "start", nothing to do
    let mut contiguous = true;
    for (i, &t) in targets.iter().enumerate() {
        if t != start + i {
            contiguous = false;
            break;
        }
    }
    if contiguous {
        return matrix;
    }

    // precomute a boolean mask for target positions (original qubit indices)
    let mut is_target = vec![false; n];
    for &t in targets {
        is_target[t] = true; // makes them 1 for bit masking
    }

    // build list of non-target original positions in order
    let non_target_positions: Vec<usize> = (0..n).filter(|p| !is_target[*p]).collect();

    // destination positions for gate bits: start..start+gate_size
    let dest_non_target_positions: Vec<usize> = (0..n)
        .filter(|p| (*p < start) || (*p >= start + gate_size))
        .collect();

    // prepare output matrix zeros
    let mut permuted = Array2::<Complex<f64>>::zeros((dim, dim));

    // helper: get bit of index for original qubit pos (MSB=qubit 0)
    let get_bit = |idx: usize, qubit_pos: usize| -> usize {
        // bit position from LSB side is (n - 1 - qubit_pos)
        (idx >> (n - 1 - qubit_pos)) & 1
    };

    // helper: convert bits (MSB-first vec) to index
    let bits_to_index = |bits: &Vec<usize>| -> usize {
        let mut acc = 0usize;
        for &b in bits.iter() {
            acc = (acc << 1) | (b & 1);
        }
        acc
    };

    // for each old row/col index, compute new index
    for old_row in 0..dim {
        // original bits of row (MSB-first)
        let mut orig_bits_row = vec![0usize; n];
        for p in 0..n {
            orig_bits_row[p] = get_bit(old_row, p);
        }

        // compute new bits by placing target bits into [start .. start+gate_size),
        // and filling remaining slots with non-target original bits (in order).
        let mut new_bits_row = vec![0usize; n];

        // place gate bits
        for (r, &t) in targets.iter().enumerate() {
            new_bits_row[start + r] = orig_bits_row[t];
        }

        // fill remaining positions
        let mut src_iter = non_target_positions.iter();
        for &dst in dest_non_target_positions.iter() {
            let &src_pos = src_iter.next().unwrap();
            new_bits_row[dst] = orig_bits_row[src_pos];
        }

        let new_row = bits_to_index(&new_bits_row);

        // same for columns (they use same bit mapping)
        for old_col in 0..dim {
            let mut orig_bits_col = vec![0usize; n];
            for p in 0..n {
                orig_bits_col[p] = get_bit(old_col, p);
            }
            let mut new_bits_col = vec![0usize; n];
            for (r, &t) in targets.iter().enumerate() {
                new_bits_col[start + r] = orig_bits_col[t];
            }
            let mut src_iter_c = non_target_positions.iter();
            for &dst in dest_non_target_positions.iter() {
                let &src_pos = src_iter_c.next().unwrap();
                new_bits_col[dst] = orig_bits_col[src_pos];
            }
            let new_col = bits_to_index(&new_bits_col);

            permuted[[new_row, new_col]] = matrix[[old_row, old_col]];
        }
    }

    permuted
}
