use leptos::prelude::*;

#[allow(non_snake_case)]
#[component]
fn App() -> impl IntoView {
    view! {
        <main>
            <h1>"Alrighty we're here"</h1>
            </main>
    }
}

fn main() {
    mount_to_body(App);
}
