// Copyright Rob Gage 2025

use engine::Expression;
use leptos::prelude::*;

#[component]
pub fn FunctionList(
    functions: ReadSignal<Vec<(Expression, Expression)>>,
    set_functions: WriteSignal<Vec<(Expression, Expression)>>,
) -> impl IntoView {

    view! {

        <ul>
            {move || {
                functions.get().iter().enumerate().map(|(i, _)| {
                    view! {
                        <li>
                            "test "
                            <button on:click=move |_| {
                                // Remove item i
                                set_functions.update(|vec| {
                                    vec.remove(i);
                                });
                            }>
                                "Remove"
                            </button>
                        </li>
                    }
                }).collect::<Vec<_>>()
            }}
        </ul>

        <button on:click=move |_| {
            set_functions.update(|vec| {
                vec.push((
                    Expression::Variable(0),
                    Expression::Variable(1)
                ));
            });
        }>+</button>

    }

}