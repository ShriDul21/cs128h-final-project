pub mod circuit;
pub mod gateinstance;
pub mod gates;
pub mod timestep;
pub mod utils;
pub mod app; 
pub mod circuitvisualizer;
use app::App;

// Use cargo run to test backend features
// Use trunk open --serve to run the frontend
fn main() {
    yew::Renderer::<App>::new().render();
}
