// Copyright Rob Gage 2025

mod function_list;
mod graph;
mod math;

use graph::Graph;
use math::Math;

use engine::Expression;
use leptos::{
    prelude::*,
    mount::mount_to_body,
};
use wasm_bindgen::prelude::wasm_bindgen;
use syntax::parse_expression;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = katex, js_name = render)]
    fn katex_render(src: &str, element: web_sys::Element);
}

#[component]
pub fn App() -> impl IntoView {

    let formula_string = RwSignal::new("x".to_string());
    let formula = move || parse_expression(formula_string.get().trim()).ok().map(|e| e.reduce());
    let derived_formula = move || formula()
        .map(|expression| expression.differentiate(&"x".to_string()).reduce());

    let latex = Signal::derive(move || formula()
        .map(|f| format!("{}", f)).unwrap_or("".to_string()));
    let derived_latex = Signal::derive(move || derived_formula()
        .map(|f| format!("{}", f)).unwrap_or("".to_string()));
    
    view! {
        <div style="
            margin: auto;
            max-width: 800px;
            display: flex;
            flex-direction: column;
            align-items: center;
        ">
            <div>
                <h1>{r"Rob's Differentiation Engine"}</h1>
            </div>
            <div>
                <input
                    type="text"
                    bind:value=formula_string
                    placeholder="(10 + exp(x))/x^2"
                    style="padding: 0.5rem; min-width: 300px;"
                />
            </div>
            <div style="width: 100%; display: flex; flex-direction: row;">
                <div style="width: 50%;">
                    <Math latex=latex  />
                </div>
                <div style="width: 50%;">
                    <Math latex=derived_latex  />
                </div>
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
