use leptos::*;

use crate::components::*;

#[component]
pub fn AboutPage(cx: Scope) -> impl IntoView {
    view! { cx,
        <div class="flex overflow-y-auto flex-col h-screen bg-gray-100 dark:bg-gray-900">
            <Navbar/>
            <div class="container flex flex-col items-center mx-auto">
                <div id="root">
                    <p>
                    </p>
                </div>
            </div>
            <Footer/>
        </div>
    }
}
