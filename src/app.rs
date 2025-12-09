use yew::{html, Component, Context, Html, TargetCast};
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
}

pub struct App {
    qubits: usize,
    gates: Vec<GateInstance>,
    result_state: Option<String>,
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

                self.result_state = Some(format!("{:?}", out));

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

                // ---- Add Gates ----
                <div class="panel">
                    <h3>{ "Add Gates" }</h3>

                    // Add H
                    <button onclick={link.callback(|_| Msg::AddH(0))}>
                        { "H on qubit 0" }
                    </button>

                    // Add CNOT
                    <button onclick={link.callback(|_| Msg::AddCNOT(0,1))}>
                        { "CNOT 0 → 1" }
                    </button>
                </div>

				<div class="panel">
				    <h3>{ "Circuit Preview" }</h3>
				    { render_circuit(&build_timeline(self.qubits, &self.gates)) }
				</div>

                // ---- Run ----
                <button class="run" onclick={link.callback(|_| Msg::Run)}>
                    { "Run Circuit" }
                </button>

                // ---- Result ----
                <div class="result">
                    <h3>{ "Output Statevector" }</h3>
                    {
                        if let Some(ref s) = self.result_state {
                            html! { <pre>{ s }</pre> }
                        } else {
                            html! { <p>{ "No results yet." }</p> }
                        }
                    }
                </div>
            </div>
        }
    }
}
