// Copyright Rob Gage 2025

mod graph;
use graph::Graph;

use leptos::{
    prelude::*,
    mount::mount_to_body,
};

#[component]
pub fn App() -> impl IntoView {
    view! {

<div style="display: flex; flex-direction: column; align-items: center;">
    <div>"Hello world!"</div>
    <div>
        <Graph/>
    </div>
</div>

    }
}

#[wasm_bindgen::prelude::wasm_bindgen(start)]
pub fn start() {
    console_error_panic_hook::set_once();
    mount_to_body(App);
}
