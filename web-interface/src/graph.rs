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
        canvas.set_width(800);
        canvas.set_height(800);
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


        if let (Some (a), Some (b)) = (formula.get(), derivative_formula.get()) {
            let (bottom, top): (f64, f64) = (minimum_y.get(), maximum_y.get());
            let a_segments: Vec<Vec<(f64, f64)>> = segments(&a, &x_values, bottom, top);
            let b_segments: Vec<Vec<(f64, f64)>> = segments(&b, &x_values, bottom, top);
            for segment in a_segments {
                chart
                    .draw_series(LineSeries::new(
                        segment.into_iter()
                            .map(|(x, y)| (x, y)),
                        &BLUE,
                    ))
                    .unwrap();
            }
            for segment in b_segments {
                chart
                    .draw_series(LineSeries::new(
                        segment.into_iter()
                            .map(|(x, y)| (x, y)),
                        &GREEN,
                    ))
                    .unwrap();
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

/// Computes the line segments of a formula from provided x values
fn segments(
    formula: &Expression<String>,
    x_values: &[f64],
    minimum_y: f64,
    maximum_y: f64,
) -> Vec<Vec<(f64, f64)>> {
    let mut segments: Vec<Vec<(f64, f64)>> = Vec::new();
    let mut segment: Vec<(f64, f64)> = Vec::new();
    let Ok (y_values) = formula.evaluate(&"x".to_string(), &x_values) else { return vec![] };
    for (&x, y) in x_values.into_iter().zip(y_values.into_iter()) {
        if y.is_nan() || y.is_infinite(){
            if !segment.is_empty() {
                segments.push(segment);
                segment = Vec::new();
            }
        } else if (y < minimum_y  || y > maximum_y) && !segment.is_empty() {
            segment.push((x, y));
            segments.push(segment);
            segment = Vec::new();
        } else { segment.push((x, y)); }
    }
    if segment.len() != 0 { segments.push(segment); }
    segments
}