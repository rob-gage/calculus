// Copyright Rob Gage 2025

use leptos::{
    prelude::*,
    mount::mount_to_body,
};

#[component]
pub fn App() -> impl IntoView {
    view! {
        <div>"Hello world!"</div>
    }
}

#[wasm_bindgen::prelude::wasm_bindgen(start)]
pub fn start() {
    console_error_panic_hook::set_once();
    mount_to_body(|| view! { <App/> });
}
