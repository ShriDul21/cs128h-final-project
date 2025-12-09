use crate::gateinstance::GateInstance;
use yew::{html, Html, Callback, DragEvent, TargetCast};
use gloo::console;


#[derive(Clone)]

pub enum Cell {
    Empty,
    Gate(usize, String),
    Control(usize),
    Target(usize),
}


pub type Timeline = Vec<Vec<Cell>>; 
// Outer vec = qubits
// Inner vec = timeline positions for each gate index

pub fn build_timeline(qubits: usize, gates: &Vec<GateInstance>, min_steps: usize) -> Timeline {
    let max_time = gates.iter().map(|g| g.time).max().unwrap_or(0);
    // Ensure we have enough space, at least min_steps, and enough to cover the max_time
    let steps = std::cmp::max(max_time + 1, min_steps); 
    
    let mut timeline = vec![vec![Cell::Empty; steps]; qubits];

    for (gate_idx, gate) in gates.iter().enumerate() {
        let t = gate.time;
        if t >= steps { continue; } // Should not happen given logic above

        let name = gate.gate.name(); 

        match name {
            "H" => {
                let q = gate.targets[0];
				timeline[q][t] = Cell::Gate(gate_idx, "H".into());
			}
			"X" => {
				let q = gate.targets[0];
				timeline[q][t] = Cell::Gate(gate_idx, "X".into());

			}
            "Y" => {
                let q = gate.targets[0];
                timeline[q][t] = Cell::Gate(gate_idx, "Y".into());

            }
			"Z" => {
				let q = gate.targets[0];
				timeline[q][t] = Cell::Gate(gate_idx, "Z".into());
			}
            "CNOT" => {
                let control = gate.targets[0];
                let target = gate.targets[1];

				if control >= qubits || target >= qubits || control == target {
					continue;
				}


                timeline[control][t] = Cell::Control(gate_idx);
                timeline[target][t] = Cell::Gate(gate_idx, "CNOT".to_owned());
            }
			"CZ" => {
				let control = gate.targets[0];
				let target = gate.targets[1];

				if control >= qubits || target >= qubits || control == target {
					continue;
				}

				timeline[control][t] = Cell::Control(gate_idx);
				timeline[target][t] = Cell::Gate(gate_idx, "CZ".to_owned());
			}
			"CY" => {
				let control = gate.targets[0];
				let target = gate.targets[1];
				if control >= qubits || target >= qubits || control == target {
					continue;
				}	
				timeline[control][t] = Cell::Control(gate_idx);
				timeline[target][t] = Cell::Gate(gate_idx, "CY".to_owned());
			}
            "CRX" => {
                let control_1 = gate.targets[0];
                let target = gate.targets[1];

				if control_1 >= qubits || target >= qubits || control_1 == target {
					continue;
				}

                timeline[control_1][t] = Cell::Control(gate_idx);
                timeline[target][t] = Cell::Gate(gate_idx, "CRX".to_owned());
            }
            "CRY" => {
                let control = gate.targets[0];
                let target = gate.targets[1];

				if control >= qubits || target >= qubits || control == target {
					continue;
				}

                timeline[control][t] = Cell::Control(gate_idx);
                timeline[target][t] = Cell::Gate(gate_idx, "CRY".to_owned());
            }
            "CRZ" => {
                let control = gate.targets[0];
                let target = gate.targets[1];

				if control >= qubits || target >= qubits || control == target {
					continue;
				}

                timeline[control][t] = Cell::Control(gate_idx);
                timeline[target][t] = Cell::Gate(gate_idx, "CRZ".to_owned());
            }
            "CCZ" => {
                let control_1 = gate.targets[0];
                let control_2 = gate.targets[1];
                let target = gate.targets[2];

				if control_1 >= qubits || control_2 >= qubits || target >= qubits ||
				   control_1 == control_2 || control_1 == target || control_2 == target {
					continue;
				}

                timeline[control_1][t] = Cell::Control(gate_idx);
                timeline[control_2][t] = Cell::Control(gate_idx);
                timeline[target][t] = Cell::Gate(gate_idx, "CCZ".to_owned());
            }
            "RX" => {
                let q = gate.targets[0];
				timeline[q][t] = Cell::Gate(gate_idx, "RX".into());
            }
			"RY" => {
				let q = gate.targets[0];
				timeline[q][t] = Cell::Gate(gate_idx, "RY".into());
            }
            "RZ" => {
                let q = gate.targets[0];
				timeline[q][t] = Cell::Gate(gate_idx, "RZ".into());
            }
            other => {
                if let Some(&q) = gate.targets.first() {
                     if q < qubits {
                         timeline[q][t] = Cell::Gate(gate_idx, other.into());
                     }
                }
            }
        }
    }

    timeline
}



pub fn render_circuit(
    timeline: &Timeline, 
    gates: &Vec<GateInstance>,
    active_gate_idx: Option<usize>,
    on_drop: Callback<(usize, usize, String)>, 
    on_move: Callback<(usize, isize, usize)>,
    on_delete: Callback<usize>, 
    on_select: Callback<usize>,
    on_update_angle: Callback<String>,
    on_update_control: Callback<(usize, String)>,
    on_update_target: Callback<(usize, String)>
) -> Html {
    html! {
        <div class="circuit">
            {
                for timeline.iter().enumerate().map(|(q, row)| {
                    let on_drop = on_drop.clone();
                    let on_move = on_move.clone();
                    let on_select = on_select.clone();
                    let on_delete = on_delete.clone();
                    let on_update_angle = on_update_angle.clone();
                    let on_update_control = on_update_control.clone();
                    let on_update_target = on_update_target.clone();
                    html! {
                        <div class="wire-row" key={q}>

                            <span class="q-label">{ format!("q{}:", q) }</span>

                            {
                                for row.iter().enumerate().map(|(t, cell)| {
                                    let on_drop = on_drop.clone();
                                    let on_move = on_move.clone();
                                    let on_select = on_select.clone();
                                    
                                    let ondragover = Callback::from(|e: DragEvent| {
                                        e.prevent_default();
                                    });

                                    let ondrop = Callback::from(move |e: DragEvent| {
                                        e.prevent_default();
                                        if let Some(dt) = e.data_transfer() {
                                            if let Ok(move_idx_str) = dt.get_data("application/x-gate-move-idx") {
                                                if !move_idx_str.is_empty() {
                                                    if let Ok(idx) = move_idx_str.parse::<usize>() {
                                                        if let Ok(src_q_str) = dt.get_data("application/x-gate-move-q") {
                                                            if let Ok(src_q) = src_q_str.parse::<usize>() {
                                                                let delta = (q as isize) - (src_q as isize);
                                                                on_move.emit((idx, delta, t));
                                                                return;
                                                            }
                                                        }
                                                    }
                                                }
                                            }

                                            if let Ok(gate_type) = dt.get_data("application/x-gate") {
                                                on_drop.emit((q, t, gate_type));
                                            }
                                        }
                                    });



                                    let on_update_angle = on_update_angle.clone();
                                    let on_update_control = on_update_control.clone();
                                    let on_update_target = on_update_target.clone();
                                    let on_delete_idx = on_delete.clone();

                                    match cell {
                                        Cell::Empty => html! { 
                                            <div class="cell empty" {ondragover} {ondrop} key={t}></div> 
                                        },
                                        Cell::Gate(idx, name) => {
                                             let on_select = on_select.clone();
                                             let idx = *idx;
                                             let name = name.clone(); 
                                             let name_for_drag = name.clone(); 
                                             let is_selected = active_gate_idx == Some(idx);
                                             
                                             let popup = if is_selected {
                                                 if let Some(gate_inst) = gates.get(idx) {
                                                     let is_rotation = ["RX", "RY", "RZ", "CRX", "CRY", "CRZ"].contains(&name.as_str());
                                                     let is_multi = gate_inst.targets.len() > 1;
                                                     let num_qubits = timeline.len(); 
                                                     let row_len = row.len();
                                                     let popup_class = if t == 0 {
                                                         "gate-popup left-aligned"
                                                     } else if t >= row_len - 3 {
                                                         "gate-popup right-aligned"
                                                     } else {
                                                         "gate-popup"
                                                     };

                                                     html! {
                                                         <div class={popup_class} onclick={Callback::from(|e: yew::MouseEvent| e.stop_propagation())}> 
                                                             <div class="popup-header">
                                                                 <h4>{ format!("Edit {}", name) }</h4>
                                                             </div>

                                                             {
                                                                 if is_rotation {
                                                                     html! {
                                                                         <div class="popup-prop">
                                                                             <label>{ "Angle" }</label>
                                                                             <input type="number" step="0.1" 
                                                                                key="angle-input"
                                                                                value={
                                                                                    "" 
                                                                                }
                                                                                oninput={on_update_angle.reform(|e: web_sys::InputEvent| {
                                                                                    let input: web_sys::HtmlInputElement = e.target_unchecked_into();
                                                                                    input.value()
                                                                                })}
                                                                             />
                                                                             <div class="angle-presets">
                                                                                 <button class="angle-btn" onclick={on_update_angle.reform(|_| "3.14159".to_string())}>{ "π" }</button>
                                                                                 <button class="angle-btn" onclick={on_update_angle.reform(|_| "1.5708".to_string())}>{ "π/2" }</button>
                                                                                 <button class="angle-btn" onclick={on_update_angle.reform(|_| "0.7854".to_string())}>{ "π/4" }</button>
                                                                             </div>
                                                                         </div>
                                                                     }
                                                                 } else { html!{} }
                                                             }

                                                             {
                                                                 if is_multi {
                                                                     html! {
                                                                         <div class="popup-prop">
                                                                             <label>{ "Qubits" }</label>
                                                                             {
                                                                                 gate_inst.targets.iter().enumerate().map(|(i, t)| {
                                                                                      let is_target = i == gate_inst.targets.len() - 1;
                                                                                      let lbl = if is_target { "Target" } else { "Ctrl" };
                                                                                      let on_update_target = on_update_target.clone();
                                                                                      let on_update_control = on_update_control.clone();
                                                                                      
                                                                                      html! {
                                                                                          <div style="display: flex; gap: 5px; margin-bottom: 5px;" key={i}>
                                                                                              <span style="font-size: 0.8em; color: #888; width: 30px;">{ lbl }</span>
                                                                                              <input type="number" min="0" max={(num_qubits -1).to_string()} value={t.to_string()} 
                                                                                                oninput={Callback::from(move |e: web_sys::InputEvent| {
                                                                                                    let input: web_sys::HtmlInputElement = e.target_unchecked_into();
                                                                                                    let val = input.value();
                                                                                                    if is_target {
                                                                                                        on_update_target.emit((i, val));
                                                                                                    } else {
                                                                                                        on_update_control.emit((i, val));
                                                                                                    }
                                                                                                })}
                                                                                              />
                                                                                          </div>
                                                                                      }
                                                                                 }).collect::<Html>()
                                                                             }
                                                                         </div>
                                                                     }
                                                                 } else { 
                                                                      html! {
                                                                         <div class="popup-prop">
                                                                             <label>{ "Target Qubit" }</label>
                                                                             <input type="number" min="0" max={(num_qubits -1).to_string()} value={gate_inst.targets[0].to_string()} 
                                                                                key="target-single"
                                                                                oninput={on_update_target.reform(|e: web_sys::InputEvent| {
                                                                                    let input: web_sys::HtmlInputElement = e.target_unchecked_into();
                                                                                    (0, input.value())
                                                                                })}
                                                                             />
                                                                         </div>
                                                                     }
                                                                 }
                                                             }

                                                             <div class="popup-actions">
                                                                 <button class="popup-delete-btn" type="button" onclick={Callback::from(move |e: yew::MouseEvent| {
                                                                     e.stop_propagation();
                                                                     console::log!("Delete button clicked for idx:", idx);
                                                                     on_delete_idx.emit(idx);
                                                                 })}>
                                                                    { "Delete" }
                                                                 </button>
                                                             </div>
                                                         </div>
                                                     }
                                                 } else { html!{} }
                                             } else { html!{} };
                                             
                                             let cell_style = if is_selected { "z-index: 1001;" } else { "" };

                                             html! { 
                                                <div class="cell gate" style={cell_style} 
                                                    {ondragover} {ondrop} 
                                                    onclick={Callback::from(move |e: yew::MouseEvent| { e.stop_propagation(); on_select.emit(idx); })} 
                                                    key={t}
                                                    draggable="true"
                                                    ondragstart={Callback::from(move |e: DragEvent| {
                                                        if let Some(dt) = e.data_transfer() {
                                                            dt.set_data("application/x-gate-move-idx", &idx.to_string()).unwrap();
                                                            dt.set_data("application/x-gate-move-q", &q.to_string()).unwrap();
                                                            dt.set_data("application/x-gate", &name_for_drag).unwrap(); 
                                                        }
                                                    })}
                                                >
                                                    <span class="gate-content">{ name }</span>
                                                    { popup }
                                                </div> 
                                            }
                                        },
                                        Cell::Control(idx) => {
                                             let on_select = on_select.clone();
                                             let idx = *idx;
                                             html! { 
                                                <div class="cell control" {ondragover} {ondrop} 
                                                    onclick={Callback::from(move |e: yew::MouseEvent| { e.stop_propagation(); on_select.emit(idx); })} 
                                                    key={t}
                                                    draggable="true"
                                                    ondragstart={Callback::from(move |e: DragEvent| {
                                                        if let Some(dt) = e.data_transfer() {
                                                            dt.set_data("application/x-gate-move-idx", &idx.to_string()).unwrap();
                                                            dt.set_data("application/x-gate-move-q", &q.to_string()).unwrap();
                                                        }
                                                    })}
                                                >
                                                    <span class="gate-content">{ "●" }</span>
                                                </div> 
                                             }
                                        },
                                        Cell::Target(idx) => {
                                             let on_select = on_select.clone();
                                             let idx = *idx;
                                             html! { 
                                                <div class="cell target" {ondragover} {ondrop} 
                                                    onclick={Callback::from(move |e: yew::MouseEvent| { e.stop_propagation(); on_select.emit(idx); })} 
                                                    key={t}
                                                    draggable="true"
                                                     ondragstart={Callback::from(move |e: DragEvent| {
                                                        if let Some(dt) = e.data_transfer() {
                                                            dt.set_data("application/x-gate-move-idx", &idx.to_string()).unwrap();
                                                            dt.set_data("application/x-gate-move-q", &q.to_string()).unwrap();
                                                        }
                                                    })}
                                                >
                                                    <span class="gate-content">{ "⊕" }</span>
                                                </div> 
                                             }
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

