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

    let (scale, set_scale) = signal(0.5);


    let canvas_reference = NodeRef::<Canvas>::new();

    // Redraw whenever limits change
    Effect::new(move || {

        // canvas
        let Some (canvas) = canvas_reference.get() else { panic!() };
        let backend: CanvasBackend = CanvasBackend::with_canvas_object(canvas)
            .expect("Failed to create `CanvasBackend`");
        let root = backend.into_drawing_area();
        root.fill(&WHITE).unwrap();

        let scale_minimum: f64 = 0.01;
        let scale_maximum: f64 = 10000.0;
        let scale: f64 = scale_minimum * (scale_maximum / scale_minimum).powf(scale.get());
        let minimum_x: f64 = -scale;
        let maximum_x: f64 = scale;
        let minimum_y: f64 = -scale;
        let maximum_y: f64 = scale;

        // graph
        let mut chart = ChartBuilder::on(&root)
            .margin(20)
            .x_label_area_size(40)
            .y_label_area_size(40)
            .build_cartesian_2d(
                minimum_x..maximum_x,
                minimum_y..maximum_y,
            )
            .unwrap();
        chart.configure_mesh()
            .x_labels(10)
            .y_labels(10)
            .axis_style(&BLACK.mix(0.0))   // invisible stroke
            .draw().unwrap();

        if let (Some (a), Some (b)) = (formula.get(), derivative_formula.get()) {
            let increment: f64 = (maximum_x - minimum_x) / LINE_VERTEX_COUNT as f64;
            let mut x_values: Vec<f64> = Vec::with_capacity(LINE_VERTEX_COUNT);
            for i in 0..LINE_VERTEX_COUNT {
                x_values.push(minimum_x + (i as f64 * increment))
            }
            let a_segments: Vec<Vec<(f64, f64)>> = segments(&a, &x_values, minimum_y, maximum_y);
            let b_segments: Vec<Vec<(f64, f64)>> = segments(&b, &x_values, minimum_y, maximum_y);
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

        chart
            .draw_series(LineSeries::new(
                [(minimum_x, minimum_y), (maximum_x, minimum_y), (maximum_x, maximum_y),
                    (minimum_x, maximum_y), (minimum_x, minimum_y)].into_iter()
                    .map(|(x, y)| (x, y)),
                &BLACK,
            ))
            .unwrap();

        root.present().unwrap();
    });

    view! {

        <div class="graph-container" style="
            width: 100%;
            display: flex;
            flex-direction: column;
            align-items: center
        ">
            <input
                type="range"
                min="0" max="1" step="0.0001"
                style="width: 100%;"
                prop:value=scale.get()
                on:input=move |event|
                    set_scale.set(event_target_value(&event).parse().unwrap_or(scale.get()))
            />
            <canvas
                node_ref=canvas_reference
                width="800"
                height="800"
                style="max-width: 600px; height: auto"
            ></canvas>
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
        if y.is_nan() || y > maximum_y  || y < minimum_y {
            if !segment.is_empty() {
                segments.push(segment);
                segment = Vec::new();
            }
        } else { segment.push((x, y)); }
    }
    if segment.len() != 0 { segments.push(segment); }
    segments
}