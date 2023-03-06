#![warn(clippy::pedantic)]
#![forbid(unsafe_code)]
use app::*;
use leptos::*;
use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen]
pub fn hydrate() {
    console_error_panic_hook::set_once();

    // Add this line:
    tracing_wasm::set_as_global_default();

    leptos::mount_to_body(move |cx| {
        view! { cx, <App/> }
    });
}
