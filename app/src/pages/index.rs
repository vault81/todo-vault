use leptos::*;

use crate::components::*;

#[component]
pub fn IndexPage(cx: Scope) -> impl IntoView {
    view! { cx,
        <div class="flex flex-col h-screen bg-gray-100 dark:bg-gray-900">
            <Navbar/>
            <div class="container flex overflow-y-auto flex-col items-center mx-auto">
                <div id="root">
                    <p>
                        <div class="p-4 my-4 mx-4 bg-white rounded-lg border border-gray-200 shadow-md sm:p-6 md:p-8 dark:bg-gray-800 dark:border-gray-700">
                            <h2 class="text-lg font-semibold text-gray-700 dark:text-gray-200"> "Welcome to Leptos" </h2>
                            <p class="mt-2 text-gray-600 dark:text-gray-400"> "Leptos is a simple todo app written in Rust and compiled to WebAssembly." </p>
                        </div>
                    </p>
                </div>
            </div>
            <Footer/>
        </div>
    }
}
