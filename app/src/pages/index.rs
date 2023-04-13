use leptos::*;
use leptos_router::*;

use crate::components::*;

#[component]
pub fn IndexPage(cx: Scope) -> impl IntoView {
    view! { cx,
        <MainPage>
            <Outlet/>
        </MainPage>
    }
}
