mod app;
mod components;
mod hooks;
mod services;

use crate::app::App;

fn main() {
    services::create_http_clients();
    wasm_logger::init(wasm_logger::Config::default());
    yew::Renderer::<App>::new().render();
}
