use leptos::*;
use leptos_router::*;

#[component]
pub fn Navbar(cx: Scope) -> impl IntoView {
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
                            <A href="/todo" class="block p-0 text-gray-700 rounded dark:text-gray-400 hover:text-blue-700 dark:hover:text-white">"Todo"</A>
                        </li>
                        <li>
                            <A href="/counter" class="block p-0 text-gray-700 rounded dark:text-gray-400 hover:text-blue-700 dark:hover:text-white">"Counter"</A>
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
