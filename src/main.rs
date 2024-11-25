use leptos::prelude::*;

#[component]
fn App() -> impl IntoView {
    view! {
        <div class="min-h-screen bg-gray-100">
            <nav class="bg-white shadow-lg">
                <div class="max-w-6xl mx-auto px-4">
                    <div class="flex justify-between items-center h-16">
                        <h1 class="text-2xl font-bold">CoKoala</h1>
                        <div class="flex space-x-4">
                            <button class="px-4 py-2 bg-blue-500 text-white rounded hover:bg-blue-600">
                                "Blog"
                            </button>
                            <button class="px-4 py-2 bg-green-500 text-white rounded hover:bg-green-600">
                                "Resume"
                            </button>
                        </div>
                    </div>
                </div>
            </nav>
            <main class="container mx-auto mt-8 px-4">
                <h1 class="text-3xl font-bold">"Alrighty we're not here"</h1>
            </main>
        </div>
    }
}

fn main() {
    _ = console_log::init_with_level(log::Level::Debug);
    console_error_panic_hook::set_once();

    mount_to_body(App);
}
