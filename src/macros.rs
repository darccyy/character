#[macro_export]
macro_rules! create_attributes {
    ( $cx: expr, $( $x: ty ),* ) => {{
        let list = (
            $( {
                let signal = create_rw_signal($cx, <$x>::random());
                Value {
                    signal,
                    random: box move |_| signal.set(<$x>::random()),
                    title: <$x>::TITLE,
                }
            }, )*
        );

        let random_all = move |_: leptos::web_sys::MouseEvent| {
            $( ${ignore(x)}
                list.${index()}.signal.set(<$x>::random());
            )*
        };

        (random_all, list)
    }};
}

#[macro_export]
macro_rules! derive_display {
    ($t: ty) => {
        impl std::fmt::Display for $t {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "{}", self.0)
            }
        }

        crate::derive_into_view!($t);
    };
}

#[macro_export]
macro_rules! derive_into_view {
    ($t: ty) => {
        impl leptos::IntoView for $t {
            fn into_view(self, cx: leptos::Scope) -> leptos::View {
                self.to_string().into_view(cx)
            }
        }
    };
}
