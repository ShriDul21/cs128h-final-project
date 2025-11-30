pub mod circuit;
pub mod gateinstance;
pub mod gates;
pub mod timestep;
pub mod utils;

use crate::circuit::Circuit;
use crate::gateinstance::GateInstance;
use crate::gates::{H,CNOT};
use num_complex::Complex;
use crate::utils::{print_matrix, print_statevector};

use gloo::console;
use js_sys::Date;
use yew::{html, Component, Context, Html};

// Yew either operates with functional components or struct components.
// Here is an example of a functional component. We will probably stick to struct components
// use yew::prelude::*;
// #[function_component(App)]
// fn app() -> Html {
//     html! {
//         <h1 class="container">{ "Quantum Rust" }</h1>
//     }
// }

// Here are struct components which implement a counter based on Yew's example code
// Refer to https://yew.rs/docs/advanced-topics/struct-components/lifecycle for more details. 

// We define a component struct to hold its state
pub struct ExampleComponent {
    value: i64, // This will store the counter value
}

// Define the possible messages which can be sent to the component to update its state
pub enum Msg {
    Increment,
    Decrement,
}

// Implement the Component trait for the ExampleComponent struct, which has 3 key methods
impl Component for ExampleComponent {
    // Config messages based on previous enum
    type Message = Msg;
    // Props are passed down to children components, nothing here since this is the parent (and only) component
    type Properties = ();

    // Constructs the initial state of the component
    fn create(_ctx: &Context<Self>) -> Self {
        Self { value: 0 }
    }

    // Called when a message is sent to the component, bulk of the logic goes here
    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Increment => {
                self.value += 1;
                console::log!("plus one"); // Will output a string to the browser console
                true // Return true to cause the displayed change to update
            }
            Msg::Decrement => {
                self.value -= 1;
                console::log!("minus one");
                true
            }
        }
    }

    // Called to render the component's HTML
    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            // Only one root elemenet allowed, can use empty tag <> </> as a wrapper if needed
            // Styles are handled in external CSS file
            <div>
                <h1>{ "Quantum Rust" }</h1>
                <div class="panel">
                    // A button to send the Increment message
                    <button class="button" onclick={ctx.link().callback(|_| Msg::Increment)}>
                        { "+1" }
                    </button>

                    // A button to send the Decrement message
                    <button class="button" onclick={ctx.link().callback(|_| Msg::Decrement)}>
                        { "-1" }
                    </button>

                    // A button to send two Increment messages
                    <button class="button" onclick={ctx.link().batch_callback(|_| vec![Msg::Increment, Msg::Increment])}>
                        { "+2" }
                    </button>

                </div>

                // Display the current value of the counter
                <p class="counter">
                    { self.value }
                </p>

                // Display the current date and time the page was rendered
                <p class="footer">
                    { "Rendered: " }
                    { String::from(Date::new_0().to_string()) }
                </p>
            </div>
        }
    }
}


fn main() {
    // Use cargo run to test backend, otherwise you won't get output when hosting website
    let mut c = Circuit::new(2);

    let gates = vec![
        GateInstance::new(0, vec![0], Box::new(H)),
        GateInstance::new(1, vec![0,1], Box::new(CNOT)),
        GateInstance::new(2,vec![1], Box::new(H)),
    ];

    let final_unitary = c.compute(gates);
    print_matrix(&final_unitary, "final unitary");

    //|00> state
    let state_vector = ndarray::array![
           [Complex::new(1.0, 0.0), Complex::new(0.0, 0.0), Complex::new(0.0, 0.0), Complex::new(0.0,0.0),
          ] 
    ];

    print_statevector(&state_vector.dot(&final_unitary));
    
    // Use trunk serve --open to host the website
    yew::Renderer::<ExampleComponent>::new().render();
}
