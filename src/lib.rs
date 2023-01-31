#![feature(macro_metavar_expr, box_syntax)]

mod attr;
mod macros;
mod types;
mod value;

use leptos::*;

use crate::{attr::Attr, types::*, value::Value};

#[component]
pub fn App(cx: Scope) -> impl IntoView {
    // Create attributes and `set_all` closure
    let (set_all, (age, first, last, sex)): _ = create_attributes!(cx, Age, First, Last, Sex);

    view! { cx,
        <button on:click= set_all > "Random All" </button>

        // Summary
        <p>
            {&first} " " {&last} ", " {&age} " " {&sex}
        </p>

        // Each attribute, with set button
        { view(cx, first) }
        { view(cx, last) }
        { view(cx, age) }
        { view(cx, sex) }
    }
}

// Render element for attribute, with set button
fn view<T>(cx: Scope, value: Value<T>) -> impl IntoView
where
    T: Attr + Clone,
{
    view! { cx,
        <p>
            <button on:click= value.set > "Random" </button>
            " " { value.title }
            " - " { value.signal }
        </p>
    }
}
