#![feature(macro_metavar_expr, box_syntax)]

mod attr;
mod macros;
mod types;
mod value;

use leptos::*;

use crate::{attr::Attr, types::*, value::Value};

#[component]
pub fn App(cx: Scope) -> impl IntoView {
    let (random_all, (age, first, last)): _ = create_attributes!(cx, Age, First, Last);

    view! { cx,
        <button on:click= random_all > "Random All" </button>

        <p>
            {&first} " " {&last} ", " {&age}
        </p>

        { first }
        { last }
        { age }
    }
}
