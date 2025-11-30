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

    let formula = Signal::derive(
        move || parse_expression(formula_string.get().trim()).ok().map(|e| e.reduce())
    );
    let derivative_formula = Signal::derive(
        move || formula.get().map(|expression| expression.differentiate(&"x".to_string()).reduce())
    );

    let latex = Signal::derive(move || formula.get()
        .map(|f| format!("{}", f)).unwrap_or("".to_string()));
    let derived_latex = Signal::derive(move || derivative_formula.get()
        .map(|f| format!("{}", f)).unwrap_or("".to_string()));
    
    view! {
        <div id="container">
            <div id="header">
                <h1>{r"Differentiation Engine"}</h1>
                <div style="
                    width: 100%;
                    display: flex;
                    flex-direction: row;
                    justify-content: space-between;
                ">
                    <span>Created by Rob Gage</span>
                    <a href="https://whitedogwood.net">White Dogwood</a>
                    <a href="https://whitedogwood.net/projects">Other Projects</a>
                    <a href="https://github.com/rob-gage/calculus">Source</a>
                </div>
            </div>
            <div id="content">
                <div style="display: flex; flex-direction: row; gap: 16px; font-size: large;">
                    <label>f(x) = </label>
                    <input
                        type="text"
                        bind:value=formula_string
                        placeholder="exp(x) * x^2"
                    />
                </div>
                <div style="
                    width: 80%;
                    display: flex;
                    flex-direction: row;
                    gap: 10%;
                    font-size:
                    large;"
                >
                    <div style="margin: 2.5%; width: 45%; height: 200px">
                        <span style="position: absolute; margin: 16px;">f(x) = </span>
                        <Math latex=latex  />
                    </div>
                    <div style="margin: 2.5%; width: 45%; height: 200px; ">
                        <span style="position: absolute; margin: 16px;">{r"f'(x) = "}</span>
                        <Math latex=derived_latex />
                    </div>
                </div>
            </div>
            <div id="graph">
                <Graph
                    formula=formula
                    derivative_formula=derivative_formula
                />
            </div>
            <div id="footer">
                <span>{r"Copyright © Rob Gage"}</span>
            </div>
        </div>
    }
}

#[wasm_bindgen::prelude::wasm_bindgen(start)]
pub fn start() {
    console_error_panic_hook::set_once();
    mount_to_body(App);
}
