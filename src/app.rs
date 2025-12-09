use yew::{html, Component, Context, Html, TargetCast, DragEvent, Callback}; // Added Callback
use gloo::console;

use crate::circuit::Circuit;
use crate::gateinstance::GateInstance;
use crate::gates::{H, CNOT};
use num_complex::Complex;
use crate::circuitvisualizer::{build_timeline, render_circuit};

#[derive(Clone)]
pub enum Msg {
    SetQubits(String),
    AddH(usize),
    AddCNOT(usize, usize),
    Run,
    AddGateAt(String, usize, usize), // (Gate Name, Qubit Index, Time Step)
}

pub struct App {
    qubits: usize,
    gates: Vec<GateInstance>,
    result_state: Option<Vec<Complex<f64>>>,
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            qubits: 2,
            gates: vec![],
            result_state: None,
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

            Msg::AddH(target) => {
				//view_gate_panel(&self, ctx);
                self.gates.push(GateInstance::new(
                    self.gates.len(),
                    vec![target],
                    Box::new(H),
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
                if name == "CNOT" {
                    if qubit + 1 < self.qubits {
                        targets.push(qubit + 1);
                    } else if qubit > 0 {
                        targets.push(qubit - 1);
                    } else {
                        // Can't place CNOT on single qubit or edge case
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
                    "CNOT" => Box::new(CNOT),
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

                // ---- Gate Toolbox (Drag & Drop) ----
                <div class="panel">
                    <h3>{ "Gate Toolbox" }</h3>
                    <p class="instruction-text">{ "Drag gates onto the circuit below:" }</p>
                    <div class="toolbox">
                        <div 
                            class="toolbox-item" 
                            draggable="true" 
                            ondragstart={Callback::from(|e: DragEvent| {
                                e.data_transfer().unwrap().set_data("application/x-gate", "H").unwrap();
                            })}
                        >
                            { "H" }
                        </div>
                        <div 
                            class="toolbox-item" 
                            draggable="true" 
                            ondragstart={Callback::from(|e: DragEvent| {
                                e.data_transfer().unwrap().set_data("application/x-gate", "CNOT").unwrap();
                            })}
                        >
                            { "CNOT" }
                        </div>
                    </div>
                </div>

				<div class="panel">
				    <h3>{ "Circuit Preview" }</h3>
				    <div class="circuit-container-scroll">
				        { render_circuit(
                            &build_timeline(self.qubits, &self.gates, 25), // Show at least 25 steps
                            link.callback(|(q, t, g)| Msg::AddGateAt(g, q, t))
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
        }
    }
}
