use std::fmt::Display;

use leptos::IntoView;

pub trait Attr: Display + IntoView {
    const TITLE: &'static str;

    fn set() -> Self;
}
