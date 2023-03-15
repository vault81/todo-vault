#![allow(dead_code)]
#![allow(unused_imports)]
#![forbid(unsafe_code)]
#![feature(once_cell)]
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

        <Title text="TodoVault"/>

        <Router>
            <ErrorBoundary fallback=|cx, errors| view!{cx, <ErrorTemplate errors=errors/>}>
            <Routes>
                <Route path="/" view=move |cx| view! { cx, <IndexPage/> }/>
                <Route path="/todo" ssr=SsrMode::Async view=move |cx| view! { cx, <TodosPage /> }/>
                <Route path="/counter" ssr=SsrMode::Async view=move |cx| view! { cx, <CounterPage /> } />
                <Route path="/about" view=move |cx| view! { cx, <AboutPage /> } />
            </Routes>
            </ErrorBoundary>
        </Router>
    }
}
