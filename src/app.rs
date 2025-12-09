use yew::{html, Component, Context, Html, TargetCast, DragEvent, Callback}; // Added Callback
use gloo::console;

use crate::circuit::Circuit;
use crate::gateinstance::GateInstance;
use crate::gates::{H, CNOT, CRX, CRY, CRZ, Z, X, Y, RX, RY, RZ, CCZ};
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
	AddCRX(f64, usize, usize),
	AddCRY(f64, usize, usize),
	AddCRZ(f64, usize, usize),
	AddCCZ(usize, usize, usize),
    Run,
    AddGateAt(String, usize, usize), // (Gate Name, Qubit Index, Time Step)
	SetControl1(String),
	SetControl2(String),
	SetTarget(String),

	SetRotationAngle(String),
    RemoveGateAt(usize, usize), // (Qubit Index, Time Step)
}

pub struct App {
    qubits: usize,
    gates: Vec<GateInstance>,
    result_state: Option<Vec<Complex<f64>>>,
	control1: usize,
	control2: usize,
	target: usize,
	rotation_angle: f64,
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
					Box::new(crate::gates::X),
				));
				true
			}

			Msg::AddY(target) => {
				self.gates.push(GateInstance::new(
					self.gates.len(),
					vec![target],
					Box::new(crate::gates::Y),
				));
				true
			}
			Msg::AddZ(target) => {
				self.gates.push(GateInstance::new(
					self.gates.len(),
					vec![target],
					Box::new(crate::gates::Z),
				));
				true
			}
			Msg::AddRX(angle, target) => {
			self.gates.push(GateInstance::new(
				self.gates.len(),
				vec![target],
				Box::new(crate::gates::RX{theta: angle})),
			);
			true
			}
			Msg::AddRY(angle, target) => {
				self.gates.push(GateInstance::new(
					self.gates.len(),
					vec![target],
					Box::new(crate::gates::RY{theta: angle}),
				));
				true
			}
			Msg::AddRZ(angle, target) => {
				self.gates.push(GateInstance::new(
					self.gates.len(),
					vec![target],
					Box::new(crate::gates::RZ{theta: angle}),		
				));
				true
			}	
            Msg::AddCNOT(control, target) => {
                self.gates.push(GateInstance::new(
                    self.gates.len(),
                    vec![control, target],
                    Box::new(CNOT),
                ));
                true
            }
			Msg::AddCCZ(control1, control2, target) => {
				self.gates.push(GateInstance::new(
					self.gates.len(),
					vec![control1, control2, target],
					Box::new(crate::gates::CCZ),
				));
				true
			}
			Msg::AddCRX(rotation,	control1, target) => {
				self.gates.push(GateInstance::new(
					self.gates.len(),
					vec![control1, target],
					Box::new(crate::gates::CRX{theta: rotation}),
				));
				true
			}
			Msg::AddCRY(rotation,	control1, target) => {
				self.gates.push(GateInstance::new(
					self.gates.len(),
					vec![control1, target],
					Box::new(crate::gates::CRY{theta: rotation}),	
				));
				true
			}
			Msg::AddCRZ(rotation,	control1, target) => {
				self.gates.push(GateInstance::new(
					self.gates.len(),
					vec![control1, target],
					Box::new(crate::gates::CRZ{theta: rotation}),
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
                // For simplicity, H takes 1 qubit. CNOT takes 2.
                let mut targets = vec![qubit];
                if name == "CNOT" || name == "CRX" || name == "CRY" || name == "CRZ" {
                    if qubit + 1 < self.qubits {
                        targets.push(qubit + 1);
                    } else if qubit > 0 {
                        targets.push(qubit - 1);
                    } else {
                        // Can't place CNOT on single qubit or edge case
                        return false; 
                    }
                }
				if name == "CCZ" {
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

                // Remove checks
                self.gates.retain(|g| {
                    if g.time != time { return true; }
                    // If times match, check overlapping targets
                    for t in &g.targets {
                        if targets.contains(t) { return false; }
                    }
                    true
                });

                let gate_obj: Box<dyn crate::gates::Gate> = match name.as_str() {
                    "H" => Box::new(H),
					"X" => Box::new(X),
					"Y" => Box::new(Y),
					"Z" => Box::new(Z),
					"RX" => Box::new(RX{theta: self.rotation_angle}),
					"RY" => Box::new(RY{theta: self.rotation_angle}),
					"RZ" => Box::new(RZ{theta: self.rotation_angle}),
                    "CNOT" => Box::new(CNOT),
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
        }
    }
	

    fn view(&self, ctx: &Context<Self>) -> Html {

        let link = ctx.link();
		let control1 = self.control1;
		let control2 = self.control2;
		let target = self.target;
		let rotation_angle = self.rotation_angle;

        html! {
            <div class="app">
                <h1>{ "Quantum Rust Simulator" }</h1>

                // ---- Qubit Count ----
                <div class="panel">
                    <label>{ "Qubits: " }</label>
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

				// ---- Control 1 ----
                <div class="panel">
                    <label>{ "Control 1: " }</label>
                    <input
                        type="number"
                        min="1"
						max={(self.qubits).to_string()}
                        value={self.control1.to_string()}
                        oninput={link.callback(|e: web_sys::InputEvent| {
                            let input: web_sys::HtmlInputElement = e.target_unchecked_into();
                            Msg::SetControl1(input.value())
                        })}
                    />
                </div>

				// ---- Control 2 ----
                <div class="panel">
                    <label>{ "Control 2 (For Toffoli): " }</label>
                    <input
                        type="number"
                        min="1"
						max={(self.qubits).to_string()}
                        value={self.control2.to_string()}
                        oninput={link.callback(|e: web_sys::InputEvent| {
                            let input: web_sys::HtmlInputElement = e.target_unchecked_into();
                            Msg::SetControl2(input.value())
                        })}
                    />
                </div>

				// ---- Target ----
                <div class="panel">
                    <label>{ "Target: " }</label>
                    <input
                        type="number"
                        min="1"
						max={(self.qubits).to_string()}
                        value={self.target.to_string()}
                        oninput={link.callback(|e: web_sys::InputEvent| {
                            let input: web_sys::HtmlInputElement = e.target_unchecked_into();
                            Msg::SetTarget(input.value())
                        })}
                    />
                </div>

				// ---- Target ----
                <div class="panel">
                    <label>{ "Rotation Angle: " }</label>
                    <input
                        type="number"
                        min="0"
                        value={self.rotation_angle.to_string()}
                        oninput={link.callback(|e: web_sys::InputEvent| {
                            let input: web_sys::HtmlInputElement = e.target_unchecked_into();
                            Msg::SetRotationAngle(input.value())
                        })}
                    />
                </div>

                // ---- Gate Toolbox (Drag & Drop) ----
                <div class="panel">
                    <h3>{ "Gate Toolbox" }</h3>
                    <p class="instruction-text">{ "Drag gates onto the circuit below:" }</p>
                    <div class="toolbox">
						 <div 
							class="toolbox-item" 
							draggable="true" 
							ondragstart={Callback::from(|e: DragEvent| {
								e.data_transfer().unwrap().set_data("application/x-gate", "X").unwrap();
							})}
							onclick={link.callback(move |_| Msg::AddX(target))}  
                        >
							{ "X" }
                        </div>
						<div 
							class="toolbox-item" 
							draggable="true" 
							ondragstart={Callback::from(|e: DragEvent| {
								e.data_transfer().unwrap().set_data("application/x-gate", "Y").unwrap();
							})}
							onclick={link.callback(move |_| Msg::AddY(target))}  
						>	
							{ "Y" }
						</div>
						<div 
							class="toolbox-item" 
							draggable="true" 
							ondragstart={Callback::from(|e: DragEvent| {
								e.data_transfer().unwrap().set_data("application/x-gate", "Z").unwrap();
							})}
							onclick={link.callback(move |_| Msg::AddZ(target))}  
						>	
							{ "Z" }
						</div>

                        <div 
                            class="toolbox-item" 
                            draggable="true" 
                            ondragstart={Callback::from(|e: DragEvent| {
                                e.data_transfer().unwrap().set_data("application/x-gate", "H").unwrap();
                            })}
							onclick={link.callback(move |_| Msg::AddH(target))}  // Add H gate to qubit 0 on click
                        >
                            { "H" }
                        </div>
						<div 
							class="toolbox-item" 
							draggable="true" 
							ondragstart={Callback::from(|e: DragEvent| {
								e.data_transfer().unwrap().set_data("application/x-gate", "RX").unwrap();
							})}
							onclick={link.callback(move |_| Msg::AddRX(rotation_angle, target))}  // Add RX gate to qubit 0 on click
						>	
							{ "RX" }
						</div>
						<div 
							class="toolbox-item" 
							draggable="true" 
							ondragstart={Callback::from(|e: DragEvent| {
								e.data_transfer().unwrap().set_data("application/x-gate", "RY").unwrap();
							})}
							onclick={link.callback(move |_| Msg::AddRY(rotation_angle, target))}  // Add RY gate to qubit 0 on click
						>	
							{ "RY" }
						</div>
						<div 
							class="toolbox-item" 
							draggable="true" 
							ondragstart={Callback::from(|e: DragEvent| {
								e.data_transfer().unwrap().set_data("application/x-gate", "RZ").unwrap();
							})}
							onclick={link.callback(move |_| Msg::AddRZ(rotation_angle, target))}  // Add RZ gate to qubit 0 on click
						>	
							{ "RZ" }
						</div>
						
                        <div 
                            class="toolbox-item" 
                            draggable="true" 
                            ondragstart={Callback::from(|e: DragEvent| {
                                e.data_transfer().unwrap().set_data("application/x-gate", "CNOT").unwrap();
                            })}
							onclick={link.callback(move |_| Msg::AddCNOT(control1, target))}  // Add CNOT gate to qubit 0 on click
                        >
                            { "CNOT" }
                        </div>
						<div 
							class="toolbox-item" 
							draggable="true" 
							ondragstart={Callback::from(|e: DragEvent| {
								e.data_transfer().unwrap().set_data("application/x-gate", "CRX").unwrap();
							})}
							onclick={link.callback(move |_| Msg::AddCRX(rotation_angle, control1, target))}  // Add CRX gate to qubits on click
						>	
							{ "CRX" }
						</div>
						<div 
							class="toolbox-item" 
							draggable="true" 
							ondragstart={Callback::from(|e: DragEvent| {
								e.data_transfer().unwrap().set_data("application/x-gate", "CRY").unwrap();
							})}
							onclick={link.callback(move |_| Msg::AddCRY(rotation_angle, control1, target))}  // Add CRY gate to qubits on click
						>	
							{ "CRY" }
						</div>
						<div 
							class="toolbox-item" 
							draggable="true" 
							ondragstart={Callback::from(|e: DragEvent| {
								e.data_transfer().unwrap().set_data("application/x-gate", "CRZ").unwrap();
							})}
							onclick={link.callback(move |_| Msg::AddCRZ(rotation_angle, control1, target))}  // Add CRZ gate to qubits on click
						>	
							{ "CRZ" }
						</div>
						
						<div 
							class="toolbox-item" 
							draggable="true" 
							ondragstart={Callback::from(|e: DragEvent| {
								e.data_transfer().unwrap().set_data("application/x-gate", "CCZ").unwrap();
							})}
							onclick={link.callback(move |_| Msg::AddCCZ(control1, control2, target))}  // Add CCZ gate to qubits on click
						>	
							{ "CCZ" }
                    </div>
                </div>

              				// ---- Circuit Preview ----
				<div class="panel">
				    <h3>{ "Circuit Preview" }</h3>
				    <div class="circuit-container-scroll">
				        { render_circuit(
                            &build_timeline(self.qubits, &self.gates, 25), // Show at least 25 steps
                            link.callback(|(q, t, g)| Msg::AddGateAt(g, q, t)),
                            link.callback(|(q, t)| Msg::RemoveGateAt(q, t))
                        ) }
				    </div>
				</div>
                
                // ---- Run ----
                <button class="run" onclick={link.callback(|_| Msg::Run)}>
                    { "Run Circuit" }
                </button>

                // ---- Result ----
                <div class="result">
                    <h3>{ "Output Statevector" }</h3>
                    {
                        if let Some(ref state) = self.result_state {
                                html! {
                                    <div>
                                        <div class="histogram-section">
                                            <h3>{ "Probability Distribution" }</h3>
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
                                                <span class="col-basis">{ "Basis State" }</span>
                                                <span class="col-amp">{ "Amplitude" }</span>
                                                <span class="col-prob">{ "Probability" }</span>
                                            </div>
                                            {
                                                state.iter().enumerate().filter(|(_, c)| c.norm_sqr() > 0.00001).map(|(i, c)| {
                                                    let prob = c.norm_sqr();
                                                    let pct = prob * 100.0;
                                                    let bin = format!("{:0width$b}", i, width = self.qubits); 
                                                    // Handle complex formatting nicely
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
                            html! { <p>{ "No results yet." }</p> }
                        }
                    }
                </div>
            </div>
			</div>
        }
		
    }
}
