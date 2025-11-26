// Copyright Rob Gage 2025

// Copyright Rob Gage 2025

use crate::katex_render;
use leptos::{
    html::Span,
    prelude::*
};
use wasm_bindgen::prelude::*;
use web_sys::Element;

#[component]
pub fn Math(
    latex: Signal<String>
) -> impl IntoView {

    let node = NodeRef::<Span>::new();

    Effect::new(move |_| {
        if let Some(span) = node.get() {
            let element: Element = span.unchecked_into();
            katex_render(&latex.get(), element);
        }
    });

    view! {
        <div class="math">
            <span node_ref=node></span>
        </div>
    }
}