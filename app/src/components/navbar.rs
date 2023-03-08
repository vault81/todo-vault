use std::{
    path::Path,
    sync::{Arc, OnceLock},
};

use leptos::*;
use leptos_router::*;

use super::*;

#[derive(Clone, Debug)]
struct NavItem {
    path: String,
    name: String,
}

impl NavItem {
    fn new(path: String, name: String) -> Self {
        Self {
            path,
            name,
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

#[derive(Clone, Debug)]
struct NavItems(Vec<NavItem>);

impl IntoView for NavItem {
    fn into_view(self, cx: Scope) -> View {
        let route_cx = use_route(cx);
        if route_cx.path() == self.path {
            view!{
                cx,
                <A
                    href={self.href()}
                    class="block py-2 pr-4 pl-3 text-white bg-orange-500 rounded md:p-0 md:text-orange-500 md:bg-transparent dark:text-white"
                >
                    {self.name}
                </A>
            }.into_view(cx)
        } else {
            view!{cx,
                <A
                    href={self.href()}
                    class="block py-2 pr-4 pl-3 text-gray-700 rounded md:p-0 md:border-0 dark:text-gray-400 hover:bg-gray-100 md:hover:bg-transparent md:hover:text-blue-700 md:dark:hover:text-white md:dark:hover:bg-transparent dark:hover:bg-gray-700 dark:hover:text-white"
                >
                    {self.name}
                </A>
            }.into_view(cx)
        }.into_view(cx)
    }
}

impl IntoView for NavItems {
    fn into_view(self, cx: Scope) -> View {
        view! {cx,
            <For
                each={move || self.clone().0}
                key={|nav_item| nav_item.href()}
                view=move |cx, item: NavItem| {view! {cx,
                <li>
                    {item}
                </li>
            }} />
        }
        .into_view(cx)
    }
}

#[component]
pub fn Navbar(cx: Scope) -> impl IntoView {
    let get_nav_items = move || {
        let items = vec![
            NavItem {
                path: "".to_string(),
                name: "Home".to_string(),
            },
            NavItem {
                path: "/todo".to_string(),
                name: "Todo".to_string(),
            },
            NavItem {
                path: "/counter".to_string(),
                name: "Counter".to_string(),
            },
        ];
        NavItems(items)
    };
    let (menu_open, set_menu_open) = create_signal(cx, false);
    let toggle_menu = move || set_menu_open.update(|curr| *curr = !*curr);

    view! {cx,
        <nav class="py-2.5 px-2 bg-white rounded border-gray-200 sm:px-4 dark:bg-gray-900">
            <div class="container flex flex-wrap justify-between items-center mx-auto">
                <A href="https://vault81.de/" class="flex items-center">
                    <div class="mr-3 h-8 w-8 rounded bg-[#d19522] p-[0.0675rem]" alt="81">
                        {Svg::Logo}
                    </div>
                    <span class="self-center text-xl font-semibold whitespace-nowrap dark:text-white">"TodoVault"</span>
                </A>
                <button type="button" on:click=move |_| toggle_menu() class="inline-flex items-center p-2 ml-3 text-sm text-gray-500 rounded-lg md:hidden dark:text-gray-400 hover:bg-gray-100 focus:ring-2 focus:ring-gray-200 focus:outline-none dark:hover:bg-gray-700 dark:focus:ring-gray-600">
                    <span class="sr-only">"Open main menu"</span>
                    <div class="w-6 h-6" >
                        {Svg::Hamburger}
                    </div>
                </button>
                <div class="w-full md:block md:w-auto" class:hidden=move || !menu_open() id="navbar-default">
                    <ul class="flex flex-col p-4 mt-4 bg-gray-50 rounded-lg border border-gray-100 md:flex-row md:mt-0 md:space-x-8 md:text-sm md:font-medium md:bg-white md:border-0 dark:bg-gray-800 dark:border-gray-700 md:dark:bg-gray-900">
                        {get_nav_items}
                    </ul>
                </div>
            </div>
        </nav>
    }
}
