use leptos::*;

use crate::attr::Attr;

pub struct Value<T>
where
    T: 'static + Attr,
{
    pub signal: RwSignal<T>,
    pub random: Box<dyn Fn(leptos::web_sys::MouseEvent) -> ()>,
    pub title: &'static str,
}

// Render element with button to reset
impl<T> IntoView for Value<T>
where
    T: IntoView + Clone + Attr,
{
    fn into_view(self, cx: Scope) -> View {
        view! { cx,
            <p>
                <button on:click= self.random > "Random" </button>
                " " { self.title }
                " - " { self.signal }
            </p>
        }
        .into_view(cx)
    }
}

// Render only signal for borrowed value
impl<T> IntoView for &Value<T>
where
    T: IntoView + Clone + Attr,
{
    fn into_view(self, cx: Scope) -> View {
        self.signal.into_view(cx)
    }
}
