use leptos::*;
use leptos_dom::*;
use leptos_router::*;

use crate::{components::*, functions::*};

#[component]
fn MainCard(cx: Scope, children: Children) -> impl IntoView {
    view! { cx,
        <div class="container flex mx-auto animate-intro-2">
            <div class="p-4 my-4 w-64 min-w-full bg-white rounded-lg border border-gray-200 shadow-md dark:bg-gray-800 dark:border-gray-700">
                {children(cx)}
            </div>
        </div>
    }
}

#[component]
pub fn MainPage(cx: Scope, children: Children) -> impl IntoView {
    // let (loaded, set_loaded) = create_signal(cx, || false);
    view! { cx,
        <div class="bg-gray-100 dark:bg-gray-900">
            <div class="flex overflow-y-auto flex-col min-h-screen animate-intro-1">
                <Navbar/>
                <MainCard>{children(cx)}</MainCard>
                <Footer/>
            </div>
        </div>
    }
}
