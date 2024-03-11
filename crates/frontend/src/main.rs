mod app;
mod components;
mod hooks;
mod services;

use crate::app::App;

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::Renderer::<App>::new().render();
}
