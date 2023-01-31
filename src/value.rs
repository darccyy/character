use leptos::*;

use crate::attr::Attr;

/// Holds `RwSignal` for attribute type, set closure, and title of attribute
pub struct Value<T>
where
    T: 'static + Attr,
{
    pub signal: RwSignal<T>,
    pub set: Box<dyn Fn(leptos::web_sys::MouseEvent) -> ()>,
    pub title: &'static str,
}

// Render only signal for owned or borrowed value
impl<T> IntoView for &Value<T>
where
    T: Attr + Clone,
{
    fn into_view(self, cx: Scope) -> View {
        self.signal.into_view(cx)
    }
}
