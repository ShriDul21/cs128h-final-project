use crate::gateinstance::GateInstance;
use yew::{html, Html, Callback, DragEvent};


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

pub fn build_timeline(qubits: usize, gates: &Vec<GateInstance>, min_steps: usize) -> Timeline {
    let max_time = gates.iter().map(|g| g.time).max().unwrap_or(0);
    // Ensure we have enough space, at least min_steps, and enough to cover the max_time
    let steps = std::cmp::max(max_time + 1, min_steps); 
    
    let mut timeline = vec![vec![Cell::Empty; steps]; qubits];

    for gate in gates {
        let t = gate.time;
        if t >= steps { continue; } // Should not happen given logic above

        let name = gate.gate.name(); 

        match name {
            "H" => {
                let q = gate.targets[0];
                if q < qubits {
                    timeline[q][t] = Cell::Gate("H".into());
                }
            }
            "CNOT" => {
                let control = gate.targets[0];
                let target = gate.targets[1];

                timeline[control][t] = Cell::Control;
                timeline[target][t] = Cell::Gate("CNOT".to_owned());
            }
            "Y" => {
                let q = gate.targets[0];
                timeline[q][t] = Cell::Gate("Y".into());

            }
            "CY" => {
                let control = gate.targets[0];
                let target = gate.targets[1];

                timeline[control][t] = Cell::Control;
                timeline[target][t] = Cell::Gate("CY".to_owned());
            }
            "CZ" => {
                let control = gate.targets[0];
                let target = gate.targets[1];

                timeline[control][t] = Cell::Control;
                timeline[target][t] = Cell::Gate("CZ".to_owned());
            }
            "CCZ" => {
                let control_1 = gate.targets[0];
                let control_2 = gate.targets[1];
                let target = gate.targets[2];

                timeline[control_1][t] = Cell::Control;
                timeline[control_2][t] = Cell::Control;
                timeline[target][t] = Cell::Gate("Z".to_owned());
            }
            other => {
                if let Some(&q) = gate.targets.first() {
                     if q < qubits {
                         timeline[q][t] = Cell::Gate(other.into());
                     }
                }
            }
        }
    }

    timeline
}


// ... (Timeline definition)

pub fn render_circuit(timeline: &Timeline, on_drop: Callback<(usize, usize, String)>) -> Html {
    html! {
        <div class="circuit">
            {
                for timeline.iter().enumerate().map(|(q, row)| {
                    let on_drop = on_drop.clone();
                    html! {
                        <div class="wire-row">

                            <span class="q-label">{ format!("q{}:", q) }</span>

                            {
                                for row.iter().enumerate().map(|(t, cell)| {
                                    let on_drop = on_drop.clone();
                                    
                                    let ondragover = Callback::from(|e: DragEvent| {
                                        e.prevent_default();
                                    });

                                    let ondrop = Callback::from(move |e: DragEvent| {
                                        e.prevent_default();
                                        if let Some(dt) = e.data_transfer() {
                                            if let Ok(gate_type) = dt.get_data("application/x-gate") {
                                                on_drop.emit((q, t, gate_type));
                                            }
                                        }
                                    });

                                    match cell {
                                        Cell::Empty => html! { 
                                            <div class="cell empty" {ondragover} {ondrop}></div> 
                                        },
                                        Cell::Gate(name) => html! { 
                                            <div class="cell gate" {ondragover} {ondrop}>{ name }</div> 
                                        },
                                        Cell::Control => html! { 
                                            <div class="cell control" {ondragover} {ondrop}>{ "●" }</div> 
                                        },
                                        Cell::Target => html! { 
                                            <div class="cell target" {ondragover} {ondrop}>{ "⊕" }</div> 
                                        },
                                    }
                                })
                            }

                        </div>
                    }
                })
            }
        </div>
    }
}

