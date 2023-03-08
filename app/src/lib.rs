#![warn(clippy::pedantic)]
#![forbid(unsafe_code)]
mod counters;
pub mod error_template;

pub mod functions;

use leptos::*;
use leptos_meta::*;
use leptos_router::*;

use crate::{
    counters::{Counters, CountersProps},
    error_template::*,
};

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
                <Route path="/" view=|cx| view! { cx, <HomePage/> }/>
                <Route path="/other" view=|cx| view! { cx, <OtherPage/> }/>
                <Route path="/counter" view=|cx| view! { cx, <CounterPage /> } />
            </Routes>
            </ErrorBoundary>
        </Router>
    }
}

#[component]
fn Button(cx: Scope) -> impl IntoView {
    view! {
        cx,
        <button class="py-1 px-1 text-sm font-medium text-white bg-blue-700 rounded-lg dark:bg-blue-600 hover:bg-blue-800 focus:ring-4 focus:ring-blue-300 focus:outline-none dark:hover:bg-blue-700 dark:focus:ring-blue-800">
            "Add"
        </button>
    }
}

#[component]
fn Trashcan(cx: Scope) -> impl IntoView {
    view! {
        cx,
        <svg class="w-4 h-4" aria-hidden="true" fill="none" stroke="currentColor" stroke-width="1.5" viewBox="0 0 24 24" xmlns="http://www.w3.org/2000/svg">
            <path d="M14.74 9l-.346 9m-4.788 0L9.26 9m9.968-3.21c.342.052.682.107 1.022.166m-1.022-.165L18.16 19.673a2.25 2.25 0 01-2.244 2.077H8.084a2.25 2.25 0 01-2.244-2.077L4.772 5.79m14.456 0a48.108 48.108 0 00-3.478-.397m-12 .562c.34-.059.68-.114 1.022-.165m0 0a48.11 48.11 0 013.478-.397m7.5 0v-.916c0-1.18-.91-2.164-2.09-2.201a51.964 51.964 0 00-3.32 0c-1.18.037-2.09 1.022-2.09 2.201v.916m7.5 0a48.667 48.667 0 00-7.5 0" stroke-linecap="round" stroke-linejoin="round"></path>
        </svg>
    }
}

#[component]
fn Table(cx: Scope) -> impl IntoView {
    view!(cx,
        <div class="overflow-x-auto relative shadow-md sm:rounded-lg">
            <table class="w-full text-sm text-left text-gray-500 dark:text-gray-400">
                <thead class="text-xs text-gray-700 uppercase bg-gray-50 dark:text-gray-400 dark:bg-gray-700">
                    <tr>
                        <th scope="col" class="py-3 px-6">
                            "Todo"
                        </th>
                        <th scope="col" class="py-3 px-6">
                            "Done"
                        </th>
                        <th>
                        </th>
                        <th scope="col" class="py-3 px-6">
                            <Button/>
                        </th>
                    </tr>
                </thead>
                <tbody>
                    <tr class="bg-white border-b dark:bg-gray-800 dark:border-gray-700 hover:bg-gray-50 dark:hover:bg-gray-600">
                        <th scope="row" class="py-4 px-6 font-medium text-gray-900 whitespace-nowrap dark:text-white">
                            "Example 1"
                        </th>
                        <td class="py-4 px-6">
                            "✓"
                        </td>
                        <td class="py-4 px-6 text-right">
                            <a href="#" class="font-medium text-blue-600 dark:text-blue-500 hover:underline">"Edit"</a>
                        </td>
                        <td class="py-4 px-6 text-right">
                            <a href="#" class="text-blue-600 dark:text-blue-500 hover:underline">
                                <Trashcan/>
                            </a>
                        </td>
                    </tr>
                    <tr class="bg-white border-b dark:bg-gray-800 dark:border-gray-700 hover:bg-gray-50 dark:hover:bg-gray-600">
                        <th scope="row" class="py-4 px-6 font-medium text-gray-900 whitespace-nowrap dark:text-white">
                            "Example 2"
                        </th>
                        <td class="py-4 px-6">
                            "⛌"
                        </td>
                        <td class="py-4 px-6 text-right">
                            <a href="#" class="font-medium text-blue-600 dark:text-blue-500 hover:underline">"Edit"</a>
                        </td>
                        <td class="py-4 px-6 text-right">
                            <a href="#" class="text-blue-600 dark:text-blue-500 hover:underline">
                                <Trashcan/>
                            </a>
                        </td>
                    </tr>
                </tbody>
            </table>
        </div>
    )
}

#[component]
fn CounterPage(cx: Scope) -> impl IntoView {
    view! { cx,
        <div class="flex flex-col h-screen bg-gray-100 dark:bg-gray-900">
            <Navbar/>
            <div class="container flex overflow-y-auto flex-col items-center mx-auto">
                <div id="root">
                    <p>
                        <div class="p-4 my-4 mx-4 bg-white rounded-lg border border-gray-200 shadow-md sm:p-6 md:p-8 dark:bg-gray-800 dark:border-gray-700">
                            <Counters/>
                        </div>
                    </p>
                </div>
            </div>
            <Footer/>
        </div>
    }
}

#[component]
fn OtherPage(cx: Scope) -> impl IntoView {
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

#[component]
fn HomePage(cx: Scope) -> impl IntoView {
    view! { cx,
        <div class="flex flex-col h-screen bg-gray-100 dark:bg-gray-900">
            <Navbar/>
            <div class="container flex overflow-y-auto flex-col items-center mx-auto">
                <div id="root">
                    <p>
                        <div class="p-4 my-4 mx-4 bg-white rounded-lg border border-gray-200 shadow-md sm:p-6 md:p-8 dark:bg-gray-800 dark:border-gray-700">
                            <Table />
                        </div>
                    </p>
                </div>
            </div>
            <Footer/>
        </div>
    }
}

#[component]
fn Navbar(cx: Scope) -> impl IntoView {
    view!(cx,
        <nav class="py-2.5 px-2.5 bg-white rounded border-t border-gray-200 dark:bg-gray-800 dark:border-gray-600">
            <div class="container flex flex-wrap justify-between items-center mx-auto">
                <a href="https://vault81.de/" class="flex items-center">
                    <img src="/logo.svg" class="mr-3 h-8 w-8 rounded bg-[#d19522] p-[0.0675rem]" alt="81"/>
                    <span class="self-center text-xl font-semibold whitespace-nowrap dark:text-white">"TodoVault"</span>
                </a>
                <div class="" id="navbar-default">
                    <ul class="flex flex-row p-1 space-x-2 text-sm font-medium rounded-lg md:space-x-8">
                        <li>
                            <A href="/" class="block p-0 text-gray-700 rounded dark:text-gray-400 hover:text-blue-700 dark:hover:text-white">"Home"</A>
                        </li>
                        <li>
                            <A href="/other" class="block p-0 text-gray-700 rounded dark:text-gray-400 hover:text-blue-700 dark:hover:text-white">"Other"</A>
                        </li>
                        <li>
                            <A href="/counter" class="block p-0 text-gray-700 rounded dark:text-gray-400 hover:text-blue-700 dark:hover:text-white">"Counters"</A>
                        </li>
                    </ul>
                </div>
                <div class="flex order-2">
                    <a href="/login" type="button" class="py-2.5 px-5 text-sm font-medium text-center text-white bg-blue-700 rounded-lg md:mr-0 dark:bg-blue-600 hover:bg-blue-800 focus:ring-4 focus:ring-blue-300 focus:outline-none dark:hover:bg-blue-700 dark:focus:ring-blue-800">
                        "Example3"
                    </a>
                </div>
            </div>
        </nav>
    )
}

#[component]
fn Footer(cx: Scope) -> impl IntoView {
    view!(cx,
        <footer class="sticky left-0 z-20 p-4 w-full bg-white border-t border-gray-200 shadow md:p-6 dark:bg-gray-800 dark:border-gray-600 top-[100vh]">
            <div class="container flex justify-between items-center mx-auto">
                <span class="text-sm text-gray-500 sm:text-center dark:text-gray-400">
                    "🄯 2023 "
                    <a href="https://vault81.de/" class="hover:underline">"Vault81"</a>
                </span>
                <ul class="flex flex-wrap items-center text-sm text-gray-500 sm:mt-0 dark:text-gray-400">
                    <li><a href="https://github.com/vault81/auth_vault_api" class="mr-4 md:mr-6 hover:underline">"Repo"</a></li>
                    <li><a href="about" class="mr-4 md:mr-6 hover:underline">"About"</a></li>
                </ul>
            </div>
        </footer>
    )
}
