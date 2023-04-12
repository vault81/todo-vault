use leptos::*;

use crate::components::*;

#[component]
pub fn AboutPage(cx: Scope) -> impl IntoView {
    view! { cx,
        <MainPage>
            <article class="format lg:format-lg dark:invert">
                <h1>"TodoVault"</h1>
                <p>
                    "Todo Vault is a free & open source Web-Application to securly store your ToDos and colloborate on them."
                </p>
                <h4>"The basic feature set planned for the first alpha release is:"</h4>
                <ul>
                    <li>"Create, Read Update & Delete Todos"</li>
                    <li>"Sharing of todos via generated link and optional password"</li>
                    <li>
                        "Client-side En- & Decryption of Lists, with Zero-Knowledge of ToDo content by the server."
                    </li>
                    <li>"Basic Filter & Search functionality"</li>
                    <li>"Compatible with common desktop and mobile viewport sizes"</li>
                </ul>
                <h4>"Nice to haves which will probably come later"</h4>
                <ul>
                    <li>"Multiple Languages"</li>
                    <li>"Real-time updates"</li>
                    <li>"Accounts (NO idenityfing info needed on sign-up)"</li>
                </ul>
                <h4>"Anti-features which are never going to be implemented."</h4>
                <ul>
                    <li>"Tracking"</li>
                    <li>"Email Sign-Up"</li>
                    <li>"Bundle Size >2MB (gzipped)"</li>
                </ul>
            </article>
        </MainPage>
    }
}
