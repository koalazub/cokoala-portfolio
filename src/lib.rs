use components::navbar::NavBar;
use leptos::*;
use leptos_meta::*;
use leptos_router::*;

// Modules
mod components;
mod pages;

// Top-Level pages
use crate::pages::about::About;
use crate::pages::home::Home;
use crate::pages::not_found::NotFound;

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();

    view! {
        <Html lang="en" dir="ltr" attr:data-theme="light"/>

        <Title text="CoKoala"/>

        <Meta charset="UTF-8"/>
        <Meta name="viewport" content="width=device-width, initial-scale=1.0"/>

        <Router>
            <NavBar />
            <Routes>
                <Route path="/" view=Home/>
                <Route path="/about" view=About/>
                <Route path="/*" view=NotFound/>
            </Routes>
        </Router>
    }
}
