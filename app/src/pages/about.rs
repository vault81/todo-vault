use leptos::*;

use crate::components::*;

#[component]
pub fn AboutPage(cx: Scope) -> impl IntoView {
    view! { cx,
        <MainPage>
            <article class="max-w-none format format-orange lg:format-lg dark:invert">
                <h2>"TodoVault: Your Privacy-Focused To-Do List Solution"</h2>
                <p>
                    "TodoVault is a free and open-source web application designed to manage your to-do lists and facilitate collaboration with others. Our goal is to offer an easy-to-use, self-hosted alternative to popular to-do list applications like Google Keep and Microsoft To-Do, while prioritizing privacy and security."
                </p>
                <p>
                    "Though our current feature set is limited, we are working diligently to develop a comprehensive to-do list solution that meets the needs of individuals and teams alike."
                </p>
                <h4>" Key Features of Our Current Minimum Viable Product (MVP) Include:"</h4>
                <ul>
                    <li>"Support for multiple to-do lists"</li>
                    <li>"Create, read, update, and delete (CRUD) capabilities for tasks"</li>
                    <li>" To-do list sharing via link"</li>
                    <li>"Basic search functionality"</li>
                    <li>"Compatibility with common desktop and mobile viewport sizes"</li>
                </ul>
                <h4>"Exciting Additions on Our Development Roadmap:"</h4>
                <ul>
                    <li>"Optional passwords with server-side validation"</li>
                    <li>"Support for multiple languages"</li>
                    <li>"Real-time updates for seamless collaboration"</li>
                    <li>"Optional passwords with client-side end-to-end (E2E) encryption"</li>
                </ul>
                <h4>"We Are Committed to Avoiding the Following \"Anti-Features\":"</h4>
                <ul>
                    <li>"User tracking"</li>
                    <li>"Email sign-up requirements"</li>
                    <li>"Bundle size exceeding 2MB (gzipped)"</li>
                </ul>
            </article>
        </MainPage>
    }
}
