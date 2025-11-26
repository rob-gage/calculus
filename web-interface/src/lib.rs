// Copyright Rob Gage 2025

mod function_list;
mod graph;

use graph::Graph;

use engine::Expression;
use leptos::{
    prelude::*,
    mount::mount_to_body,
};
use syntax::parse_expression;

#[component]
pub fn App() -> impl IntoView {

    let (formula_string, set_formula_string) = signal("".to_string());
    let formula = move || parse_expression(&formula_string.get()).ok();
    let derived_formula = move || formula()
        .map(|expression| expression.differentiate(&"x".to_string()));

    
    view! {
        <div style="display: flex; flex-direction: column; align-items: center;">
            <div>
                <h1>{r"Rob's Differentiation Engine"}</h1>
            </div>
            <div>
                <input
                    type="text"
                    on:input:target=move |event| set_formula_string.set(event.target().value())
                    placeholder="Enter formula"
                    style="padding: 0.5rem; min-width: 300px;"
                />
            </div>
            <div style="display: flex; flex-direction: row;">
                <div style="margin: auto;"></div>
                <div style="margin: auto"></div>
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
