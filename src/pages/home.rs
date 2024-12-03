use crate::components::counter_btn::Button;
use leptos::*;

/// Default Home Page
#[component]
pub fn Home() -> impl IntoView {
    view! {
        <ErrorBoundary fallback=|errors| {
            view! {
                <h1>"Uh oh! Something went wrong!"</h1>

                <p>"Errors: "</p>
                // Render a list of errors as strings - good for development purposes
                <ul>
                    {move || {
                        errors
                            .get()
                            .into_iter()
                            .map(|(_, e)| view! { <li>{e.to_string()}</li> })
                            .collect_view()
                    }}

                </ul>
            }
        }>

            <div class="container">

                <picture>
                    <source
                        srcset="https://avatars.githubusercontent.com/u/7111524?v=4"
                        media="(prefers-color-scheme: dark)"
                    />
                    <img
                        src="https://avatars.githubusercontent.com/u/7111524?v=4"
                        alt="Leptos Logo"
                        height="200"
                        width="400"
                    />
                </picture>

                <h1>"Hello? Is this thing on?"</h1>

                <div class="buttons">
                    <Button/>
                    <Button increment=5/>
                </div>

            </div>
        </ErrorBoundary>
    }
}
