// Copyright Rob Gage 2025

mod function_list;
mod graph;

use function_list::FunctionList;
use graph::Graph;

use engine::Expression;
use leptos::{
    prelude::*,
    mount::mount_to_body,
};

#[component]
pub fn App() -> impl IntoView {
    
    let (functions, set_functions) = signal(vec![]);
    
    view! {
        <div style="display: flex; flex-direction: column; align-items: center;">
            <div>
                <h1>{r"Rob's Differentiation Engine"}</h1>
            </div>
            <div>
                <FunctionList
                    functions=functions
                    set_functions=set_functions
                />
            </div>
            <div>
                <Graph />
            </div>
        </div>
    }
}

#[wasm_bindgen::prelude::wasm_bindgen(start)]
pub fn start() {
    console_error_panic_hook::set_once();
    mount_to_body(App);
}
