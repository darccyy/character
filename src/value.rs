use leptos::*;

use crate::attr::Attr;

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
    T: Clone + Attr,
{
    fn into_view(self, cx: Scope) -> View {
        self.signal.into_view(cx)
    }
}
