// Copyright Rob Gage 2025

use engine::Expression;
use leptos::{
    html::Canvas,
    prelude::*,
};
use plotters::{
    prelude::*,
    style::ShapeStyle,
};
use plotters_canvas::CanvasBackend;
use web_sys::HtmlCanvasElement;

const LINE_VERTEX_COUNT: usize = 500;

#[component]
pub fn Graph(
    formula: Signal<Option<Expression<String>>>,
    derivative_formula: Signal<Option<Expression<String>>>,
) -> impl IntoView {
    // reactive graph limits
    let (minimum_x, set_minimum_x) = signal(-10.0);
    let (maximum_x, set_maximum_x) = signal(10.0);
    let (minimum_y, set_minimum_y) = signal(-10.0);
    let (maximum_y, set_maximum_y) = signal(10.0);

    let canvas_reference = NodeRef::<Canvas>::new();

    // Redraw whenever limits change
    Effect::new(move || {
        // canvas
        let Some (canvas) = canvas_reference.get() else { panic!() };
        let backend: CanvasBackend = CanvasBackend::with_canvas_object(canvas)
            .expect("Failed to create `CanvasBackend`");
        let root = backend.into_drawing_area();
        root.fill(&WHITE).unwrap();

        // graph
        let mut chart = ChartBuilder::on(&root)
            .margin(20)
            .x_label_area_size(40)
            .y_label_area_size(40)
            .build_cartesian_2d(
                minimum_x.get()..maximum_x.get(),
                minimum_y.get()..maximum_y.get()
            )
            .unwrap();
        chart.configure_mesh()
            .x_labels(10)
            .y_labels(10)
            .axis_style(&BLACK.mix(0.0))   // invisible stroke
            .draw().unwrap();

        let start: f64 = minimum_x.get();
        let increment: f64 = (maximum_x.get() - start) / LINE_VERTEX_COUNT as f64;
        let mut x_values: Vec<f64> = Vec::with_capacity(LINE_VERTEX_COUNT);
        for i in 0..LINE_VERTEX_COUNT {
            x_values.push(start + (i as f64 * increment))
        }

        if let (Some (blue), Some (red)) = (formula.get(), derivative_formula.get()) {
            if let Ok (y_values) = blue.evaluate(&"x".to_string(), &x_values) {
                chart
                    .draw_series(LineSeries::new(
                        x_values.iter()
                            .zip(y_values.iter())
                            .map(|(x, y)| (*x, *y)),
                            // .filter(|(x, y)|
                            //     x >= &minimum_x.get() && x <= &maximum_x.get()
                            //     && y >= &minimum_y.get() && y <= &maximum_y.get()
                            // ),
                        &BLUE,
                    ))
                    .unwrap()
                    .label("f(x)")
                    .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &BLUE));
            }
            if let Ok (derivative_y_values) = red.evaluate(&"x".to_string(), &x_values) {
                    chart
                        .draw_series(LineSeries::new(
                            x_values.iter()
                                .zip(derivative_y_values.iter())
                                .map(|(x, y)| (*x, *y)),
                                // .filter(|(x, y)|
                                //     x >= &minimum_x.get() && x <= &maximum_x.get()
                                //         && y >= &minimum_y.get() && y <= &maximum_y.get()
                                // ),
                            &RED,
                        ))

                        .unwrap()
                        .label("f(x)'")
                        .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &BLUE));
            }
        }

        root.present().unwrap();
    });

    view! {

<div class="graph-container" style="width: 100%;">
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