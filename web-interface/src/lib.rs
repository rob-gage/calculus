// Copyright Rob Gage 2025

mod function_list;
mod graph;

use graph::Graph;

use engine::Expression;
use leptos::{
    prelude::*,
    mount::mount_to_body,
};
use leptos::tachys::html::style::IntoStyle;
use syntax::parse_expression;

#[component]
pub fn App() -> impl IntoView {

    let formula_string = RwSignal::new("x".to_string());
    let formula = move || parse_expression(formula_string.get().trim()).ok();
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
                    bind:value=formula_string
                    placeholder="Enter formula"
                    style="padding: 0.5rem; min-width: 300px;"
                />
            </div>
            <div style="display: flex; flex-direction: row;">
                <div style="margin-right: 20px;"><span>{ move ||
                    if let Some (formula) = formula() { format!("{}", formula) }
                    else { formula_string.get() }
                }</span></div>
                <div style="margin: auto"><span>{ move ||
                    if let Some (formula) = derived_formula() { format!("{}", formula) }
                    else { "missing".to_string() }
                }</span></div>
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
