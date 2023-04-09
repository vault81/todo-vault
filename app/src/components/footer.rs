use leptos::*;

#[component]
pub fn Footer(cx: Scope) -> impl IntoView {
    view! { cx,
        <footer class="sticky left-0 z-20 p-4 w-full bg-white border-t border-gray-200 shadow dark:bg-gray-800 dark:border-gray-600 top-[100vh]">
            <div class="container flex justify-between items-center mx-auto">
                <span class="text-sm text-gray-500 sm:text-center dark:text-gray-400">
                    "🄯 2023 " <a href="https://vault81.de/" class="hover:underline">
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
