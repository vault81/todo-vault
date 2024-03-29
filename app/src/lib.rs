#![allow(dead_code)]
#![allow(unused_imports)]
#![forbid(unsafe_code)]
pub mod error_template;

mod components;
mod functions;
mod pages;
mod utils;

#[cfg(feature = "ssr")]
pub use functions::register_server_functions;
use leptos::*;
use leptos_meta::*;
use leptos_router::*;

use crate::{components::*, error_template::*, pages::*};

#[component]
pub fn App(cx: Scope) -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context(cx);

    view! { cx,
        <Stylesheet id="leptos" href="/pkg/todo-vault.css"/>
        <Title text="TodoVault"/>
        <Router>
            <ErrorBoundary fallback=|cx, errors| {
                view! { cx, <ErrorTemplate errors=errors/> }
            }>
                <Routes>
                    <Route
                        path="/"
                        view=move |cx| {
                            view! { cx, <AboutPage/> }
                        }
                    />
                    <Route
                        path="/todo"
                        view=move |cx| {
                            view! { cx, <MyTodoListsPage/> }
                        }
                    />
                    <Route
                        path="/todo/:list_id"
                        view=move |cx| {
                            view! { cx, <TodoListPage/> }
                        }
                    />
                    <Route
                        path="/about"
                        view=move |cx| {
                            view! { cx, <AboutPage/> }
                        }
                    />
                </Routes>
            </ErrorBoundary>
        </Router>
    }
}
