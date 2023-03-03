use leptos::*;
use leptos_meta::*;
use leptos_router::*;

#[component]
pub fn App(cx: Scope) -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context(cx);

    view! {
        cx,

        <Stylesheet id="leptos" href="/pkg/todo-vault.css"/>

        <Title text="Welcome to Leptos"/>

        <Router>
            <Routes>
                <Route path="" view=|cx| view! { cx, <HomePage/> }/>
            </Routes>
        </Router>
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
                        <div class="p-4 my-4 mx-4 max-w-sm bg-white rounded-lg border border-gray-200 shadow-md sm:p-6 md:p-8 dark:bg-gray-800 dark:border-gray-700">
                            <div class="format dark:format-invert">
                                "About."
                            </div>
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
                    <img src="logo.svg" class="mr-3 h-8 w-8 rounded bg-[#d19522] p-[0.0675rem]" alt="81"/>
                    <span class="self-center text-xl font-semibold whitespace-nowrap dark:text-white">"AuthVault"</span>
                </a>
                <div class="" id="navbar-default">
                    <ul class="flex flex-row p-1 space-x-2 text-sm font-medium rounded-lg md:space-x-8">
                        <li>
                            <a href="" class="block p-0 text-gray-700 rounded dark:text-gray-400 hover:text-blue-700 dark:hover:text-white">"Home" </a>
                        </li>
                        <li>
                            <a href="/counter" class="block p-0 text-gray-700 rounded dark:text-gray-400 hover:text-blue-700 dark:hover:text-white">"Counter"</a>
                        </li>
                    </ul>
                </div>
                <div class="flex order-2">
                    <a href="/login" type="button" class="py-2.5 px-5 text-sm font-medium text-center text-white bg-blue-700 rounded-lg md:mr-0 dark:bg-blue-600 hover:bg-blue-800 focus:ring-4 focus:ring-blue-300 focus:outline-none dark:hover:bg-blue-700 dark:focus:ring-blue-800">
                        "Login"
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
                    "© 2023 "
                    <a href="https://vault81.de/" class="hover:underline">"Vault81"</a>
                    " All Rights Reserved."
                </span>
                <ul class="flex flex-wrap items-center text-sm text-gray-500 sm:mt-0 dark:text-gray-400">
                    <li><a href="https://github.com/vault81/auth_vault_api" class="mr-4 md:mr-6 hover:underline">"Repo"</a></li>
                    <li><a href="about" class="mr-4 md:mr-6 hover:underline">"About"</a></li>
                </ul>
            </div>
        </footer>
    )
}
