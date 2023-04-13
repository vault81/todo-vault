use std::{
    path::Path,
    sync::{Arc, OnceLock},
};

use leptos::*;
use leptos_dom::*;
use leptos_router::*;

use super::*;
use crate::{components::*, functions::*};

#[derive(Clone, Debug)]
struct NavItem {
    path:   String,
    name:   String,
    active: bool,
}

impl NavItem {
    fn new(path: String, name: String, active: bool) -> Self {
        Self {
            path,
            name,
            active,
        }
    }

    fn href(&self) -> String {
        if !self.path.starts_with('/') {
            let mut href = self.path.clone();
            href.insert(0, '/');
            href
        } else {
            self.path.clone()
        }
    }
}

impl IntoView for NavItem {
    fn into_view(self, cx: Scope) -> View {
        if self.active {
            view! { cx,
                <A
                    href=self.href()
                    class="block py-2 pr-4 pl-3 text-white bg-orange-500 rounded md:p-0 md:text-orange-500 md:bg-transparent dark:text-white"
                >
                    {self.name}
                </A>
            }.into_view(cx)
        } else {
            view! { cx,
                <A
                    href=self.href()
                    class="block py-2 pr-4 pl-3 text-gray-700 rounded md:p-0 md:border-0 dark:text-gray-400 hover:bg-gray-100 md:hover:bg-transparent md:hover:text-blue-700 md:dark:hover:text-white md:dark:hover:bg-transparent dark:hover:bg-gray-700 dark:hover:text-white"
                >
                    {self.name}
                </A>
            }.into_view(cx)
        }.into_view(cx)
    }
}

#[component]
fn Navbar(cx: Scope) -> impl IntoView {
    let route_cx = use_route(cx);

    let items = move || {
        vec![
            NavItem {
                path:   "".to_string(),
                name:   "Home".to_string(),
                active: route_cx.path() == "",
            },
            NavItem {
                path:   "/todo".to_string(),
                name:   "My To-Dos".to_string(),
                active: route_cx.path().starts_with("/todo"),
            },
        ]
    };
    let (menu_open, set_menu_open) = create_signal(cx, false);
    let toggle_menu = move || set_menu_open.update(|curr| *curr = !*curr);

    view! { cx,
        <nav class="py-2.5 px-2 bg-white rounded border-gray-200 sm:px-4 dark:bg-gray-900">
            <div class="container flex flex-wrap justify-between items-center mx-auto">
                <A href="https://vault81.de/" class="flex items-center">
                    <div class="mr-3 h-8 w-8 rounded bg-[#d19522] p-[0.0675rem]" alt="81">
                        {Svg::Logo}
                    </div>
                    <span class="self-center text-xl font-semibold whitespace-nowrap dark:text-white">
                        "TodoVault"
                    </span>
                </A>
                <button
                    type="button"
                    on:click=move |_| toggle_menu()
                    class="inline-flex items-center p-2 ml-3 text-sm text-gray-500 rounded-lg md:hidden dark:text-gray-400 hover:bg-gray-100 focus:ring-2 focus:ring-gray-200 focus:outline-none dark:hover:bg-gray-700 dark:focus:ring-gray-600"
                >
                    <span class="sr-only">"Open main menu"</span>
                    <div class="w-6 h-6">{Svg::Hamburger}</div>
                </button>
                <div
                    class="w-full md:block md:w-auto"
                    class:hidden=move || !menu_open()
                    id="navbar-default"
                >
                    <ul class="flex flex-col p-4 mt-4 bg-gray-50 rounded-lg border border-gray-100 md:flex-row md:mt-0 md:space-x-8 md:text-sm md:font-medium md:bg-white md:border-0 dark:bg-gray-800 dark:border-gray-700 md:dark:bg-gray-900">
                        {items}
                    </ul>
                </div>
            </div>
        </nav>
    }
}

#[component]
fn Footer(cx: Scope) -> impl IntoView {
    view! { cx,
        <footer class="sticky left-0 z-20 p-4 w-full bg-white border-t border-gray-200 shadow dark:bg-gray-800 dark:border-gray-600 top-[100vh]">
            <div class="container flex justify-between items-center mx-auto">
                <span class="text-sm text-gray-500 sm:text-center dark:text-gray-400">
                    <div class="inline-block -scale-x-100">"©"</div>
                    " 2023 "
                    <a href="https://vault81.de/" class="hover:underline">
                        "Vault81"
                    </a>
                </span>
                <ul class="flex flex-wrap items-center text-sm text-gray-500 sm:mt-0 dark:text-gray-400">
                    <li>
                        <a
                            href="https://github.com/vault81/todo-vault"
                            class="mr-4 md:mr-6 hover:underline"
                        >
                            "Repo"
                        </a>
                    </li>
                    <li>
                        <a href="about" class="mr-4 md:mr-6 hover:underline">
                            "About"
                        </a>
                    </li>
                </ul>
            </div>
        </footer>
    }
}

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
