use leptos::*;
use leptos_router::A;

use crate::components::menu_butts::Button;

#[component]
pub fn NavBar() -> impl IntoView {
    let nav_styling = "py-2 px-2 font-medium text-gray-500 rounded hover:bg-fuchsia-500 hover:text-white transition duration-300";

    view! {
        <nav class="bg-white shadow-lg">
            <div class="max-w-6xl mx-auto px-4">
                <div class="flex justify-between items-center">
                    <A href="/" class="items-center py-4 px-2">
                        <span class="font-semibold text-gray-500 text-lg">"CoKoala"</span>
                    </A>
                        <Button
                            text="Home"
                            route="/"
                            class=nav_styling
                        />
                        <Button
                            text="About"
                            route="/about"
                            class=nav_styling
                        />
                </div>
            </div>
        </nav>
    }
}
