use std::fmt::Display;

use leptos::IntoView;

/// Character attribute
///
/// Eg. First name or Age
pub trait Attr: Display + IntoView {
    /// Name of attribute
    const TITLE: &'static str;

    /// Set with random value
    fn set() -> Self;
}
