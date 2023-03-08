#![allow(dead_code)]
#![allow(unused_imports)]
#![warn(clippy::pedantic)]
#![forbid(unsafe_code)]
pub mod error_template;

mod components;
pub mod functions;
mod pages;

use leptos::*;
use leptos_meta::*;
use leptos_router::*;

use crate::{error_template::*, pages::*};

#[component]
pub fn App(cx: Scope) -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context(cx);

    view! {
        cx,
        <Stylesheet id="leptos" href="/pkg/todo-vault.css"/>

        <Title text="Welcome to Leptos"/>

        <Router>
            <ErrorBoundary fallback=|cx, errors| view!{cx, <ErrorTemplate errors=errors/>}>
            <Routes>
                <Route path="" view=|cx| view! { cx, <IndexPage/> }/>
                <Route path="/todo" view=|cx| view! { cx, <TodosPage/> }/>
                <Route path="/counter" ssr=SsrMode::Async view=|cx| view! { cx, <CounterPage /> } />
            </Routes>
            </ErrorBoundary>
        </Router>
    }
}
