use leptos::*;

// buttons styled with tailwind css
#[component]
pub fn Button(
    cx: Scope,
    children: Children,
    #[prop(optional, into)] b_type: String,
) -> impl IntoView {
    view! {
        cx,
        <button
            type={b_type}
            class="py-2.5 px-5 mr-2 mb-2 text-sm font-medium text-gray-900 bg-white rounded-lg border border-gray-200 dark:text-gray-400 dark:bg-gray-800 dark:border-gray-600 hover:text-blue-700 hover:bg-gray-100 focus:z-10 focus:ring-4 focus:ring-gray-200 focus:outline-none dark:focus:ring-gray-700 dark:hover:text-white dark:hover:bg-gray-700"
        >
            {children(cx)}
        </button>
    }
}
