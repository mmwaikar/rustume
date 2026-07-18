mod app;

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::wasm_bindgen;

#[cfg(target_arch = "wasm32")]
use leptos::mount_to_body;

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen(start)]
pub fn main() {
    mount_to_body(app::App);
}

#[cfg(not(target_arch = "wasm32"))]
pub fn main() {
    println!("rustume is a client-side Leptos app. Build the wasm target to run it in the browser.");
}
