// Copyright Rob Gage 2025

use leptos::{
    html::Canvas,
    prelude::*,
};
use plotters::prelude::*;
use plotters_canvas::CanvasBackend;
use web_sys::HtmlCanvasElement;

#[component]
pub fn Graph() -> impl IntoView {
    // reactive graph limits
    let (minimum_x, set_minimum_x) = signal(-10.0);
    let (maximum_x, set_maximum_x) = signal(10.0);
    let (minimum_y, set_minimum_y) = signal(-10.0);
    let (maximum_y, set_maximum_y) = signal(10.0);

    let canvas_reference = NodeRef::<Canvas>::new();

    // Redraw whenever limits change
    Effect::new(move || {
        println!("test");
        web_sys::console::log_1(&"Canvas found, effect running!".into());
        // canvas
        let Some (canvas) = canvas_reference.get() else { panic!() };
        let backend: CanvasBackend = CanvasBackend::with_canvas_object(canvas)
            .expect("Failed to create `CanvasBackend`");
        let root = backend.into_drawing_area();
        root.fill(&BLACK).unwrap();

        // graph
        let mut chart = ChartBuilder::on(&root)
            .margin(10)
            .x_label_area_size(40)
            .y_label_area_size(40)
            .build_cartesian_2d(
                minimum_x.get()..maximum_x.get(),
                minimum_y.get()..minimum_y.get()
            )
            .unwrap();
        chart.configure_mesh().draw().unwrap();

        root.present().unwrap();
    });

    view! {

<div class="graph-container">
    <canvas
        node_ref=canvas_reference
        width="500"
        height="500"
        style="border: 1px black solid;"
    ></canvas>
    <div>
        <h4>Horizontal axis</h4>
        <label>"Minimum: "</label>
        <input type="number"
            prop:value=minimum_x.get()
            on:input=move |e|set_minimum_x.set(
                event_target_value(&e).parse().unwrap_or(minimum_x.get())
            )
        />
        <label>"Maximum: "</label>
        <input type="number"
            prop:value=maximum_x.get()
            on:input=move |e| set_maximum_x.set(
                event_target_value(&e).parse().unwrap_or(maximum_x.get())
        )
        />
    </div>

    <div>
        <h4>Vertical axis</h4>
        <label>"Minimum: "</label>
        <input type="number"
            prop:value=minimum_y.get()
            on:input=move |e| set_minimum_y.set(
                event_target_value(&e).parse().unwrap_or(minimum_y.get())
            )
        />
        <label>"Maximum: "</label>
        <input type="number"
            prop:value=maximum_y.get()
            on:input=move |e| set_maximum_y.set(
                event_target_value(&e).parse().unwrap_or(maximum_y.get())
            )
        />
    </div>
</div>

    }
}