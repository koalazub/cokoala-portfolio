use leptos::*;
use leptos_router::use_navigate;

#[component]
pub fn Button(
    #[prop(into)] text: String,
    #[prop(into)] route: String,
    #[prop(into, optional)] class: Option<String>,
) -> impl IntoView {
    let navigate = use_navigate();
    let class = format!("inline-block {}", class.unwrap_or_else(|| "".into()));

    view! {
        <button
        class=class
        on:click= move |_| { navigate(&route, Default::default()); }
        >
        {text}
        </button>
    }
}
