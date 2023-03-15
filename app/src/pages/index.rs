use leptos::*;

use crate::components::*;

#[component]
pub fn IndexPage(cx: Scope) -> impl IntoView {
    view! { cx,
        <div class="flex flex-col h-screen bg-gray-100 dark:bg-gray-900">
            <Navbar/>
            <div class="container flex mx-auto">
                <div class="overflow-y-auto p-4 my-4 w-64 min-w-full bg-white rounded-lg border border-gray-200 shadow-md dark:bg-gray-800 dark:border-gray-700">
                    <article class="format lg:format-lg dark:invert">
                        <h1>
                            "TodoVault"
                        </h1>
                        <p>
                            "Todo Vault is a free & open source Web-Application to securly store your ToDos and colloborate on them."
                        </p>
                        <h4>
                            "The basic feature set planned for the first alpha release is:"
                        </h4>
                        <ul>
                            <li>"Create, Read Update & Delete Todos"</li>
                            <li>"Sharing of todos via generated link and optional password"</li>
                            <li>"Client-side En- & Decryption of Lists, with Zero-Knowledge of ToDo content by the server."</li>
                            <li>"Basic Filter & Search functionality" </li>
                            <li>"Compatible with common desktop and mobile viewport sizes" </li>
                        </ul>
                        <h4>
                            "Nice to haves which will probably come later"
                        </h4>
                        <ul>
                            <li>"Multiple Languages" </li>
                            <li>"Real-time updates"</li>
                            <li>"Accounts (NO idenityfing info needed on sign-up)"</li>
                        </ul>
                        <h4>
                            "Anti-features which are never going to be implemented."
                        </h4>
                        <ul>
                            <li>"Tracking"</li>
                            <li>"Email Sign-Up"</li>
                            <li>"Bundle Size >2MB (gzipped)"</li>
                        </ul>
                    </article>
                </div>
            </div>
            <Footer/>
        </div>
    }
}
