pub mod circuit;
pub mod gateinstance;
pub mod gates;
pub mod timestep;
pub mod utils;
pub mod app; 
pub mod circuitvisualizer;

use app::App;

fn main() {
    yew::Renderer::<App>::new().render();
}
