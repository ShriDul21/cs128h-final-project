use yew::{html, Component, Context, Html, TargetCast, DragEvent, Callback}; // Added Callback
use gloo::console;

use crate::circuit::Circuit;
use crate::gateinstance::GateInstance;
use crate::gates::{H, CY, CZ, CNOT, CRX, CRY, CRZ, Z, X, Y, RX, RY, RZ, CCZ};
use num_complex::Complex;
use crate::circuitvisualizer::{build_timeline, render_circuit};

#[derive(Clone)]
pub enum Msg {
    SetQubits(String),
	AddX(usize),
	AddY(usize),
	AddZ(usize),
    AddH(usize),
	AddRX(f64, usize),
	AddRY(f64, usize),
	AddRZ(f64, usize),
    AddCNOT(usize, usize),
	AddCZ(usize, usize),
	AddCY(usize, usize),
	AddCRX(f64, usize, usize),
	AddCRY(f64, usize, usize),
	AddCRZ(f64, usize, usize),
	AddCCZ(usize, usize, usize),
    Run,
    AddGateAt(String, usize, usize), // (gate name, qubit index, time step)
	SetControl1(String),
	SetControl2(String),
	SetTarget(String),

	SetRotationAngle(String), 
    RemoveGateAt(usize, usize), // (qubit index, time step)
    
    // selection + editing
    SelectGate(usize),
    DeselectGate,
    DeleteSelectedGate,
    DeleteGate(usize), // Explicit deletion by index
    MoveGate(usize, isize, usize), // (idx, delta_rows, new_time)
    UpdateSelectedGateAngle(String),
    UpdateSelectedControl(usize, String), // (index in targets, new_qubit_val)
    UpdateSelectedTarget(usize, String),  // (index in targets, new_qubit_val)
}

pub struct App {
    qubits: usize,
    gates: Vec<GateInstance>,
    result_state: Option<Vec<Complex<f64>>>,
	control1: usize,
	control2: usize,
	target: usize,
	rotation_angle: f64,
    selected_gate: Option<usize>,
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            qubits: 2,
            gates: vec![],
            result_state: None,
			control1: 0,
			control2: 0,
			target: 0,
			rotation_angle: 0.0,
            selected_gate: None,
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::SetQubits(v) => {
                if let Ok(q) = v.parse::<usize>() {
                    self.qubits = q;
                }
                true
            }

            Msg::SetControl1(v) => {
                if let Ok(c1) = v.parse::<usize>() {
                    self.control1 = c1;
                }
                true
            }

            Msg::SetControl2(v) => {
                if let Ok(c2) = v.parse::<usize>() {
                    self.control2 = c2;
                }
                true
            }

            Msg::SetTarget(v) => {
                if let Ok(t) = v.parse::<usize>() {
                    self.target = t;
                }
                true
            }
			Msg::SetRotationAngle(v) => {
				if let Ok(angle) = v.parse::<f64>() {
					self.rotation_angle = angle;
				}
				true
			}

            Msg::AddH(target) => {
				//view_gate_panel(&self, ctx);
                self.gates.push(GateInstance::new(
                    self.gates.len(),
                    vec![target],
                    Box::new(H),
                ));
                true
            }

			Msg::AddX(target) => {
				self.gates.push(GateInstance::new(
					self.gates.len(),
					vec![target],
					Box::new(X),
				));
				true
			}

			Msg::AddY(target) => {
				self.gates.push(GateInstance::new(
					self.gates.len(),
					vec![target],
					Box::new(Y),
				));
				true
			}
			Msg::AddZ(target) => {
				self.gates.push(GateInstance::new(
					self.gates.len(),
					vec![target],
					Box::new(Z),
				));
				true
			}
			Msg::AddRX(angle, target) => {
			self.gates.push(GateInstance::new(
				self.gates.len(),
				vec![target],
				Box::new(RX{theta: angle})),
			);
			true
			}
			Msg::AddRY(angle, target) => {
				self.gates.push(GateInstance::new(
					self.gates.len(),
					vec![target],
					Box::new(RY{theta: angle}),
				));
				true
			}
			Msg::AddRZ(angle, target) => {
				self.gates.push(GateInstance::new(
					self.gates.len(),
					vec![target],
					Box::new(RZ{theta: angle}),		
				));
				true
			}	
            Msg::AddCNOT(control, target) => {
				if control == target {
					false;
				}
                self.gates.push(GateInstance::new(
                    self.gates.len(),
                    vec![control, target],
                    Box::new(CNOT),
                ));
                true
            }
			Msg::AddCZ(control, target) => {	
				if control == target {
					false;
				}
				self.gates.push(GateInstance::new(
					self.gates.len(),
					vec![control, target],
					Box::new(CZ),
				));
				true
			}
			Msg::AddCY(control, target) => {
				if control == target {
					false;
				}	
				self.gates.push(GateInstance::new(
					self.gates.len(),
					vec![control, target],
					Box::new(CY),
				));
				true
			}
			Msg::AddCCZ(control1, control2, target) => {
				if control1 == target || control2 == target || control1 == control2 {
					false;
				}
				self.gates.push(GateInstance::new(
					self.gates.len(),
					vec![control1, control2, target],
					Box::new(CCZ),
				));
				true
			}
			Msg::AddCRX(rotation,	control1, target) => {
				if control1 == target{
					false;
				}
				self.gates.push(GateInstance::new(
					self.gates.len(),
					vec![control1, target],
					Box::new(CRX{theta: rotation}),
				));
				true
			}
			Msg::AddCRY(rotation,	control1, target) => {
				if control1 == target {
					false;
				}
				self.gates.push(GateInstance::new(
					self.gates.len(),
					vec![control1, target],
					Box::new(CRY{theta: rotation}),	
				));
				true
			}
			Msg::AddCRZ(rotation,	control1, target) => {
				if control1 == target {
					false;
				}
				self.gates.push(GateInstance::new(
					self.gates.len(),
					vec![control1, target],
					Box::new(CRZ{theta: rotation}),
				));
				true
			}

            
            Msg::RemoveGateAt(qubit, time) => {
                // Remove the gate found at (qubit, time)
                self.gates.retain(|g| {
                    if g.time == time && g.targets.contains(&qubit) {
                        return false;
                    }
                    true
                });
                self.selected_gate = None;
                self.result_state = None; // Reset results
                true
            }

            Msg::Run => {
                console::log!("Running quantum circuit…");

                let mut circuit = Circuit::new(self.qubits);
                let final_u = circuit.compute(self.gates.clone());

                // Initial |0...0> state vector
                let dim = 1 << self.qubits;
                let mut state = vec![Complex::new(0.0, 0.0); dim];
                state[0] = Complex::new(1.0, 0.0); // |0...0>

                let input = ndarray::Array2::from_shape_vec((1, dim), state).unwrap();
                let out = input.dot(&final_u);

                self.result_state = Some(out.iter().cloned().collect());

                true
            }

            Msg::AddGateAt(name, qubit, time) => {
                // Remove existing gates at this time that overlap with the new gate's target
                let mut targets = vec![qubit];
                if name == "CNOT" || name == "CRX" || name == "CRY" || name == "CRZ" || name == "CZ"  || name == "CY" {
                    if qubit + 1 < self.qubits {
                        targets.push(qubit + 1);
                    } else if qubit > 0 {
                        targets.push(qubit - 1);
                    } else {
                        // Can't place CNOT on single qubit or edge case
                        return false; 
                    }
                }
				else if name == "CCZ" {
					if qubit + 2 < self.qubits {
						targets.push(qubit + 1);
						targets.push(qubit + 2);
					} else if qubit > 1 {
						targets.push(qubit - 1);
						targets.push(qubit - 2);
					} else {
						// Can't place CCZ on less than 3 qubits or edge case
						return false; 
					}
				}
				else {}

                // Remove checks
                self.gates.retain(|g| {
                    if g.time != time { return true; }
                    // If times match, check overlapping targets
                    for t in &g.targets {
                        if targets.contains(t) { return false; }
                    }
                    true
                });
                self.selected_gate = None;

                let gate_obj: Box<dyn crate::gates::Gate> = match name.as_str() {
                    "H" => Box::new(H),
					"X" => Box::new(X),
					"Y" => Box::new(Y),
					"Z" => Box::new(Z),
					"RX" => Box::new(RX{theta: self.rotation_angle}),
					"RY" => Box::new(RY{theta: self.rotation_angle}),
					"RZ" => Box::new(RZ{theta: self.rotation_angle}),
                    "CNOT" => Box::new(CNOT),
					"CZ" => Box::new(CZ),
					"CY" => Box::new(CY),
					"CRX" => Box::new(CRX{theta: self.rotation_angle}),
					"CRY" => Box::new(CRY{theta: self.rotation_angle}),
					"CRZ" => Box::new(CRZ{theta: self.rotation_angle}),
                    "CCZ" => Box::new(CCZ),
					_ => return false,
                };

                self.gates.push(GateInstance::new(time, targets, gate_obj));
                
                // Clear state on change
                self.result_state = None; 
                true
            }


            Msg::SelectGate(idx) => {
                if idx < self.gates.len() {
                    self.selected_gate = Some(idx);
                }
                true
            }
            Msg::DeselectGate => {
                self.selected_gate = None;
                true
            }
            Msg::DeleteSelectedGate => {
                if let Some(idx) = self.selected_gate {
                    if idx < self.gates.len() {
                        self.gates.remove(idx);
                        self.selected_gate = None;
                        self.result_state = None;
                    }
                }
                true
            }
            Msg::DeleteGate(idx) => {
                 gloo::console::log!(format!("DeleteGate: idx={}, gates_len={}", idx, self.gates.len()));
                 if idx < self.gates.len() {
                    self.gates.remove(idx);
                    // If deleted gate was selected, deselect
                    if self.selected_gate == Some(idx) {
                        self.selected_gate = None;
                    } else if let Some(selected) = self.selected_gate {
                        // adjust index if needed
                        if idx < selected {
                            self.selected_gate = Some(selected - 1);
                        }
                    }
                    self.result_state = None;
                 } else {
                     gloo::console::error!("DeleteGate index out of bounds");
                 }
                true
            }
            Msg::MoveGate(idx, delta, new_time) => {
                 if idx < self.gates.len() {
                    let mut gate = self.gates[idx].clone();
                    
                    // Validate new targets
                    let mut valid = true;
                    let mut new_targets = vec![];
                    for &t in &gate.targets {
                        let new_t = t as isize + delta;
                        if new_t < 0 || new_t >= self.qubits as isize {
                            valid = false;
                            break;
                        }
                        new_targets.push(new_t as usize);
                    }

                    if valid {
                        // Remove old gate
                        self.gates.remove(idx);
                        
                        // Clean up collisions at new spot
                        let time = new_time;
                         self.gates.retain(|g| {
                            if g.time != time { return true; }
                            for t in &g.targets {
                                if new_targets.contains(t) { return false; }
                            }
                            true
                        });

                        gate.targets = new_targets;
                        gate.time = time;
                        self.gates.push(gate);
                        
                         self.selected_gate = None;
                         self.result_state = None;
                    }
                 }
                true
            }
            Msg::UpdateSelectedGateAngle(val) => {
                if let (Some(idx), Ok(angle)) = (self.selected_gate, val.parse::<f64>()) {
                     if let Some(gate_inst) = self.gates.get_mut(idx) {
                         let name = gate_inst.gate.name();
                         let new_gate: Box<dyn crate::gates::Gate> = match name {
                             "RX" => Box::new(RX{theta: angle}),
                             "RY" => Box::new(RY{theta: angle}),
                             "RZ" => Box::new(RZ{theta: angle}),
                             "CRX" => Box::new(CRX{theta: angle}),
                             "CRY" => Box::new(CRY{theta: angle}),
                             "CRZ" => Box::new(CRZ{theta: angle}),
                             _ => return false, // Not a rotation gate
                         };
                         gate_inst.gate = new_gate;
                         self.result_state = None;
                     }
                }
                true
            }
            Msg::UpdateSelectedControl(ctrl_idx, val) => {
                 if let (Some(idx), Ok(q)) = (self.selected_gate, val.parse::<usize>()) {
                    if let Some(gate_inst) = self.gates.get_mut(idx) {
                        if ctrl_idx < gate_inst.targets.len() {
                             // Check if q is used by any OTHER target/control
                             let already_used = gate_inst.targets.iter().enumerate()
                                 .any(|(i, &existing)| i != ctrl_idx && existing == q);
                             
                             if !already_used {
                                gate_inst.targets[ctrl_idx] = q;
                                self.result_state = None;
                             }
                        }
                    }
                 }
                 true
            }
            Msg::UpdateSelectedTarget(tgt_idx, val) => {
                 if let (Some(idx), Ok(q)) = (self.selected_gate, val.parse::<usize>()) {
                    if let Some(gate_inst) = self.gates.get_mut(idx) {
                        if tgt_idx < gate_inst.targets.len() {

                            let already_used = gate_inst.targets.iter().enumerate()
                                 .any(|(i, &existing)| i != tgt_idx && existing == q);
                             
                             if !already_used {
                                gate_inst.targets[tgt_idx] = q;
                                self.result_state = None;
                             }
                        }
                    }
                 }
                 true
            }
        }
    }
	

    fn view(&self, ctx: &Context<Self>) -> Html {

        let link = ctx.link();
		let control1 = self.control1;
		let control2 = self.control2;
		let target = self.target;
		let rotation_angle = self.rotation_angle;

        html! {
            <div class="app main-layout">
                {
                    if self.selected_gate.is_some() {
                        html! { <div class="popup-backdrop" onclick={link.callback(|_| Msg::DeselectGate)}></div> }
                    } else { html!{} }
                }
                <aside class="sidebar">
                    <h1>{ "Quantum Rust" }</h1> 

                    // ---- Input Controls Group ----
                    <div class="sidebar-section">
                        <h3>{ "Configuration" }</h3>
                        <div class="control-group">
                            <label>{ "Qubits" }</label>
                            <input
                                type="number"
                                min="1"
                                value={self.qubits.to_string()}
                                oninput={link.callback(|e: web_sys::InputEvent| {
                                    let input: web_sys::HtmlInputElement = e.target_unchecked_into();
                                    Msg::SetQubits(input.value())
                                })}
                            />
                        </div>

                        <div class="control-group">
                            <label>{ "Control 1" }</label>
                            <input type="number" min="0" max={(self.qubits).to_string()} value={self.control1.to_string()}
                                oninput={link.callback(|e: web_sys::InputEvent| {
                                    let input: web_sys::HtmlInputElement = e.target_unchecked_into();
                                    Msg::SetControl1(input.value())
                                })}
                            />
                        </div>
                        <div class="control-group">
                            <label>{ "Control 2" }</label>
                            <input type="number" min="0" max={(self.qubits).to_string()} value={self.control2.to_string()}
                                oninput={link.callback(|e: web_sys::InputEvent| {
                                    let input: web_sys::HtmlInputElement = e.target_unchecked_into();
                                    Msg::SetControl2(input.value())
                                })}
                            />
                        </div>
                        <div class="control-group">
                            <label>{ "Target" }</label>
                            <input type="number" min="0" max={(self.qubits).to_string()} value={self.target.to_string()}
                                oninput={link.callback(|e: web_sys::InputEvent| {
                                    let input: web_sys::HtmlInputElement = e.target_unchecked_into();
                                    Msg::SetTarget(input.value())
                                })}
                            />
                        </div>
                        <div class="control-group">
                            <label>{ "Angle" }</label>
                            <input type="number" min="0" value={self.rotation_angle.to_string()}
                                oninput={link.callback(|e: web_sys::InputEvent| {
                                    let input: web_sys::HtmlInputElement = e.target_unchecked_into();
                                    Msg::SetRotationAngle(input.value())
                                })}
                            />
                        </div>
                    </div>

                    // ---- Gate Toolbox ----
                    <div class="sidebar-section">
                        <h3>{ "Gates" }</h3>
                        <p class="instruction-text">{ "Drag to circuit or use inputs above + click to add." }</p>
                        <div class="toolbox">
                            // single qubit gates
                            <div class="toolbox-item" draggable="true" 
                                ondragstart={Callback::from(|e: DragEvent| { e.data_transfer().unwrap().set_data("application/x-gate", "X").unwrap(); })}
                                onclick={link.callback(move |_| Msg::AddX(target))}>{ "X" }</div>
                            <div class="toolbox-item" draggable="true" 
                                ondragstart={Callback::from(|e: DragEvent| { e.data_transfer().unwrap().set_data("application/x-gate", "Y").unwrap(); })}
                                onclick={link.callback(move |_| Msg::AddY(target))}>{ "Y" }</div>
                            <div class="toolbox-item" draggable="true" 
                                ondragstart={Callback::from(|e: DragEvent| { e.data_transfer().unwrap().set_data("application/x-gate", "Z").unwrap(); })}
                                onclick={link.callback(move |_| Msg::AddZ(target))}>{ "Z" }</div>
                            <div class="toolbox-item" draggable="true" 
                                ondragstart={Callback::from(|e: DragEvent| { e.data_transfer().unwrap().set_data("application/x-gate", "H").unwrap(); })}
                                onclick={link.callback(move |_| Msg::AddH(target))}>{ "H" }</div>
                            
                            // rotation gates
                            <div class="toolbox-item" draggable="true" 
                                ondragstart={Callback::from(|e: DragEvent| { e.data_transfer().unwrap().set_data("application/x-gate", "RX").unwrap(); })}
                                onclick={link.callback(move |_| Msg::AddRX(rotation_angle, target))}>{ "RX" }</div>
                            <div class="toolbox-item" draggable="true" 
                                ondragstart={Callback::from(|e: DragEvent| { e.data_transfer().unwrap().set_data("application/x-gate", "RY").unwrap(); })}
                                onclick={link.callback(move |_| Msg::AddRY(rotation_angle, target))}>{ "RY" }</div>
                            <div class="toolbox-item" draggable="true" 
                                ondragstart={Callback::from(|e: DragEvent| { e.data_transfer().unwrap().set_data("application/x-gate", "RZ").unwrap(); })}
                                onclick={link.callback(move |_| Msg::AddRZ(rotation_angle, target))}>{ "RZ" }</div>

                            // multi qubit gartes
                            <div class="toolbox-item" draggable="true" 
                                ondragstart={Callback::from(|e: DragEvent| { e.data_transfer().unwrap().set_data("application/x-gate", "CNOT").unwrap(); })}
                                onclick={link.callback(move |_| Msg::AddCNOT(control1, target))}>{ "CX" }</div>
                                <div class="toolbox-item" draggable="true" 
                                ondragstart={Callback::from(|e: DragEvent| { e.data_transfer().unwrap().set_data("application/x-gate", "CY").unwrap(); })}
                                onclick={link.callback(move |_| Msg::AddCY(control1, target))}>{ "CY" }</div>
                                <div class="toolbox-item" draggable="true" 
                                    ondragstart={Callback::from(|e: DragEvent| { e.data_transfer().unwrap().set_data("application/x-gate", "CZ").unwrap(); })}
                                    onclick={link.callback(move |_| Msg::AddCZ(control1, target))}>{ "CZ" }</div>
                            
                            // controlledrotation gates
                             <div class="toolbox-item" draggable="true" 
                                ondragstart={Callback::from(|e: DragEvent| { e.data_transfer().unwrap().set_data("application/x-gate", "CRX").unwrap(); })}
                                onclick={link.callback(move |_| Msg::AddCRX(rotation_angle, control1, target))}>{ "CRX" }</div>
                             <div class="toolbox-item" draggable="true" 
                                ondragstart={Callback::from(|e: DragEvent| { e.data_transfer().unwrap().set_data("application/x-gate", "CRY").unwrap(); })}
                                onclick={link.callback(move |_| Msg::AddCRY(rotation_angle, control1, target))}>{ "CRY" }</div>
                             <div class="toolbox-item" draggable="true" 
                                ondragstart={Callback::from(|e: DragEvent| { e.data_transfer().unwrap().set_data("application/x-gate", "CRZ").unwrap(); })}
                                onclick={link.callback(move |_| Msg::AddCRZ(rotation_angle, control1, target))}>{ "CRZ" }</div>

                            <div class="toolbox-item" draggable="true" 
                                ondragstart={Callback::from(|e: DragEvent| { e.data_transfer().unwrap().set_data("application/x-gate", "CCZ").unwrap(); })}
                                onclick={link.callback(move |_| Msg::AddCCZ(control1, control2, target))}>{ "CCZ" }</div>
                        </div>
                    </div>
                </aside>

                <main class="workspace">
                    // ---- Circuit Area ----
                    <div class="circuit-area">
                        <div class="circuit-header">
                            <h3>{ "Composer" }</h3>
                             <button class="run" onclick={link.callback(|_| Msg::Run)}>
                                { "Run Circuit" }
                            </button>
                        </div>
                        <div class="circuit-container-scroll">
                            { render_circuit(
                                &build_timeline(self.qubits, &self.gates, 25), 
                                &self.gates,
                                self.selected_gate,
                                link.callback(|(q, t, g)| Msg::AddGateAt(g, q, t)),
                                link.callback(move |(idx, d, t)| Msg::MoveGate(idx, d, t)),
                                link.callback(move |idx| Msg::DeleteGate(idx)), 
                                link.callback(|idx| Msg::SelectGate(idx)),
                                link.callback(|val| Msg::UpdateSelectedGateAngle(val)),
                                link.callback(|(idx, val)| Msg::UpdateSelectedControl(idx, val)),
                                link.callback(|(idx, val)| Msg::UpdateSelectedTarget(idx, val)),
                            ) }
                        </div>
                    </div>

                    // ---- Results Bottom Panel ----
                    <div class="results-area">
                        <h3>{ "Results" }</h3>
                        {
                            if let Some(ref state) = self.result_state {
                                html! {
                                    <div class="results-layout">
                                        <div class="histogram-section">
                                            <h4>{ "Probability Distribution" }</h4>
                                            <div class="histogram-container">
                                                {
                                                    state.iter().enumerate()
                                                        .filter(|(_, c)| c.norm_sqr() > 0.0001)
                                                        .map(|(i, c)| {
                                                            let prob = c.norm_sqr();
                                                            let pct = prob * 100.0;
                                                            let bin = format!("{:0width$b}", i, width = self.qubits);
                                                            html! {
                                                                <div class="histogram-bar-group">
                                                                    <div class="histogram-chart-area">
                                                                        <div class="histogram-value">{ format!("{:.1}%", pct) }</div>
                                                                        <div class="histogram-bar" style={format!("--h: {:.2}%", pct)}></div>
                                                                    </div>
                                                                    <div class="histogram-label">{ format!("|{}>", bin) }</div>
                                                                </div>
                                                            }
                                                        }).collect::<Html>()
                                                }
                                            </div>
                                        </div>

                                        <div class="statevector-container">
                                            <div class="state-header">
                                                <span class="col-basis">{ "State" }</span>
                                                <span class="col-amp">{ "Amplitude" }</span>
                                                <span class="col-prob">{ "Probability" }</span>
                                            </div>
                                            {
                                                state.iter().enumerate().filter(|(_, c)| c.norm_sqr() > 0.00001).map(|(i, c)| {
                                                    let prob = c.norm_sqr();
                                                    let pct = prob * 100.0;
                                                    let bin = format!("{:0width$b}", i, width = self.qubits); 
                                                    let sign = if c.im >= 0.0 { "+" } else { "-" };
                                                    let amp_str = format!("{:.3} {} {:.3}i", c.re, sign, c.im.abs());

                                                    html! {
                                                        <div class="state-row">
                                                            <span class="col-basis">{ format!("|{}>", bin) }</span>
                                                            <span class="col-amp">{ amp_str }</span>
                                                            <div class="col-prob">
                                                                <div class="prob-bar-bg">
                                                                    <div class="prob-bar-fill" style={format!("width: {:.2}%", pct)}></div>
                                                                </div>
                                                                <span class="prob-text">{ format!("{:.2}%", pct) }</span>
                                                            </div>
                                                        </div>
                                                    }
                                                }).collect::<Html>()
                                            }
                                        </div>
                                    </div>
                                }
                            } else {
                                // Empty state placeholder
                                html! { 
                                    <div class="empty-results">
                                        <div class="placeholder-graph">
                                            <div class="placeholder-bar" style="height: 20%"></div>
                                            <div class="placeholder-bar" style="height: 50%"></div>
                                            <div class="placeholder-bar" style="height: 30%"></div>
                                        </div>
                                        <p>{ "Run the circuit to see distribution and statevector." }</p>
                                    </div> 
                                }
                            }
                        }
                    </div>
                </main>


            
            </div>
        }
		
    }
}
