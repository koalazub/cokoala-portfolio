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
    _ = console_log::init_with_level(log::Level::Debug);
    console_error_panic_hook::set_once();

    mount_to_body(App);
}
