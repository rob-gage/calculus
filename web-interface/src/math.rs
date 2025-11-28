// Copyright Rob Gage 2025

// Copyright Rob Gage 2025

use crate::katex_render;
use leptos::{
    html::Div,
    prelude::*
};
use wasm_bindgen::prelude::*;
use web_sys::Element;

#[component]
pub fn Math(
    latex: Signal<String>
) -> impl IntoView {

    let node = NodeRef::<Div>::new();

    Effect::new(move |_| {
        if let Some(div) = node.get() {
            let element: Element = div.unchecked_into();
            katex_render(&latex.get(), element);
        }
    });

    view! {
        <div class="math"><div node_ref=node></div></div>
    }
}