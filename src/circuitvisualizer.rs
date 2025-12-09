use crate::gateinstance::GateInstance;
use yew::{html, Html};

#[derive(Clone)]
pub enum Cell {
    Empty,
    Gate(String),
    Control,
    Target,
}


pub type Timeline = Vec<Vec<Cell>>; 
// Outer vec = qubits
// Inner vec = timeline positions for each gate index

pub fn build_timeline(qubits: usize, gates: &Vec<GateInstance>) -> Timeline {
    let mut timeline = vec![vec![Cell::Empty; gates.len()]; qubits];

    for (i, gate) in gates.iter().enumerate() {
        let name = gate.gate.name(); // You likely have something similar

        match name {
            "H" => {
                let q = gate.targets[0];
                timeline[q][i] = Cell::Gate("H".into());
            }
            "CNOT" => {
                let control = gate.targets[0];
                let target = gate.targets[1];

                timeline[control][i] = Cell::Control;
                timeline[target][i] = Cell::Gate("CNOT".to_owned());
            }
            "Y" => {
                let q = gate.targets[0];
                timeline[q][i] = Cell::Gate("Y".into());

            }
            "CY" => {
                let control = gate.targets[0];
                let target = gate.targets[1];

                timeline[control][i] = Cell::Control;
                timeline[target][i] = Cell::Gate("CY".to_owned());
            }
            "CZ" => {
                let control = gate.targets[0];
                let target = gate.targets[1];

                timeline[control][i] = Cell::Control;
                timeline[target][i] = Cell::Gate("CZ".to_owned());
            }
            "CCZ" => {
                let control_1 = gate.targets[0];
                let control_2 = gate.targets[1];
                let target = gate.targets[2];

                timeline[control_1][i] = Cell::Control;
                timeline[control_2][i] = Cell::Control;
                timeline[target][i] = Cell::Gate("Z".to_owned());
            }
            other => {
                let q = gate.targets[0];
                timeline[q][i] = Cell::Gate(other.into());
            }
        }
    }

    timeline
}

pub fn render_circuit(timeline: &Timeline) -> Html {
    html! {
        <div class="circuit">
            {
                for timeline.iter().enumerate().map(|(q, row)| {
                    html! {
                        <div class="wire-row">

                            <span class="q-label">{ format!("q{}:", q) }</span>

                            {
                                for row.iter().map(|cell| match cell {
                                    Cell::Empty => html! { <div class="cell empty"></div> },
                                    Cell::Gate(name) => html! { <div class="cell gate">{ name }</div> },
                                    Cell::Control => html! { <div class="cell control">{ "●" }</div> },
                                    Cell::Target => html! { <div class="cell target">{ "⊕" }</div> },
                                })
                            }

                        </div>
                    }
                })
            }
        </div>
    }
}

