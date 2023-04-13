use leptos::*;
use leptos_dom::*;
use leptos_router::*;

#[component]
fn DropdownButton(cx: Scope) -> impl IntoView {
    let (dropdown_open, set_dropdown_open) = create_signal(cx, false);
    let toggle_modal = move || {
        set_dropdown_open(!dropdown_open());
    };

    view! { cx,
        <div>
            <button
                id="dropdownRadioButton"
                on:click=move |_| toggle_modal()
                data-dropdown-toggle="dropdownRadio"
                class="inline-flex items-center py-2.5 px-3 text-sm font-medium text-gray-500 bg-white rounded-lg border border-gray-300 dark:text-white dark:bg-gray-800 dark:border-gray-600 hover:bg-gray-100 focus:ring-4 focus:ring-gray-200 focus:outline-none dark:hover:bg-gray-700 dark:hover:border-gray-600 dark:focus:ring-gray-700"
                type="button"
            >
                <svg
                    class="mr-2 w-4 h-4 text-gray-400"
                    aria-hidden="true"
                    fill="currentColor"
                    viewBox="0 0 20 20"
                    xmlns="http://www.w3.org/2000/svg"
                >
                    <path
                        fill-rule="evenodd"
                        d="M10 18a8 8 0 100-16 8 8 0 000 16zm1-12a1 1 0 10-2 0v4a1 1 0 00.293.707l2.828 2.829a1 1 0 101.415-1.415L11 9.586V6z"
                        clip-rule="evenodd"
                    ></path>
                </svg>
                "Last 30 days"
                <svg
                    class="m-auto w-3 h-3"
                    aria-hidden="true"
                    fill="none"
                    stroke="currentColor"
                    viewBox="0 0 24 24"
                    xmlns="http://www.w3.org/2000/svg"
                >
                    <path
                        stroke-linecap="round"
                        stroke-linejoin="round"
                        stroke-width="2"
                        d="M19 9l-7 7-7-7"
                    ></path>
                </svg>
            </button>
            <div
                id="dropdownRadio"
                class="absolute left-0 top-10 z-10 m-0 w-48 bg-white rounded-lg divide-y divide-gray-100 shadow dark:bg-gray-700 dark:divide-gray-600"
                data-popper-reference-hidden=""
                data-popper-escaped=""
                data-popper-placement="top"
                class:hidden=move || !dropdown_open()
            >
                <ul
                    class="p-3 space-y-1 text-sm text-gray-700 dark:text-gray-200"
                    aria-labelledby="dropdownRadioButton"
                >
                    <li>
                        <div class="flex items-center p-2 rounded hover:bg-gray-100 dark:hover:bg-gray-600">
                            <input
                                id="filter-radio-example-1"
                                type="radio"
                                value=""
                                name="filter-radio"
                                class="w-4 h-4 text-orange-600 bg-gray-100 border-gray-300 dark:bg-gray-700 dark:border-gray-600 dark:ring-offset-gray-800 focus:ring-2 focus:ring-orange-500 dark:focus:ring-orange-600 dark:focus:ring-offset-gray-800"
                            />
                            <label
                                for="filter-radio-example-1"
                                class="ml-2 w-full text-sm font-medium text-gray-900 rounded dark:text-gray-300"
                            >
                                "Last day"
                            </label>
                        </div>
                    </li>
                    <li>
                        <div class="flex items-center p-2 rounded hover:bg-gray-100 dark:hover:bg-gray-600">
                            <input
                                checked=""
                                id="filter-radio-example-2"
                                type="radio"
                                value=""
                                name="filter-radio"
                                class="w-4 h-4 text-orange-600 bg-gray-100 border-gray-300 dark:bg-gray-700 dark:border-gray-600 dark:ring-offset-gray-800 focus:ring-2 focus:ring-orange-500 dark:focus:ring-orange-600 dark:focus:ring-offset-gray-800"
                            />
                            <label
                                for="filter-radio-example-2"
                                class="ml-2 w-full text-sm font-medium text-gray-900 rounded dark:text-gray-300"
                            >
                                "Last 7 days"
                            </label>
                        </div>
                    </li>
                    <li>
                        <div class="flex items-center p-2 rounded hover:bg-gray-100 dark:hover:bg-gray-600">
                            <input
                                id="filter-radio-example-3"
                                type="radio"
                                value=""
                                name="filter-radio"
                                class="w-4 h-4 text-orange-600 bg-gray-100 border-gray-300 dark:bg-gray-700 dark:border-gray-600 dark:ring-offset-gray-800 focus:ring-2 focus:ring-orange-500 dark:focus:ring-orange-600 dark:focus:ring-offset-gray-800"
                            />
                            <label
                                for="filter-radio-example-3"
                                class="ml-2 w-full text-sm font-medium text-gray-900 rounded dark:text-gray-300"
                            >
                                "Last 30 days"
                            </label>
                        </div>
                    </li>
                    <li>
                        <div class="flex items-center p-2 rounded hover:bg-gray-100 dark:hover:bg-gray-600">
                            <input
                                id="filter-radio-example-4"
                                type="radio"
                                value=""
                                name="filter-radio"
                                class="w-4 h-4 text-orange-600 bg-gray-100 border-gray-300 dark:bg-gray-700 dark:border-gray-600 dark:ring-offset-gray-800 focus:ring-2 focus:ring-orange-500 dark:focus:ring-orange-600 dark:focus:ring-offset-gray-800"
                            />
                            <label
                                for="filter-radio-example-4"
                                class="ml-2 w-full text-sm font-medium text-gray-900 rounded dark:text-gray-300"
                            >
                                "Last month"
                            </label>
                        </div>
                    </li>
                    <li>
                        <div class="flex items-center p-2 rounded hover:bg-gray-100 dark:hover:bg-gray-600">
                            <input
                                id="filter-radio-example-5"
                                type="radio"
                                value=""
                                name="filter-radio"
                                class="w-4 h-4 text-orange-600 bg-gray-100 border-gray-300 dark:bg-gray-700 dark:border-gray-600 dark:ring-offset-gray-800 focus:ring-2 focus:ring-orange-500 dark:focus:ring-orange-600 dark:focus:ring-offset-gray-800"
                            />
                            <label
                                for="filter-radio-example-5"
                                class="ml-2 w-full text-sm font-medium text-gray-900 rounded dark:text-gray-300"
                            >
                                "Last year"
                            </label>
                        </div>
                    </li>
                </ul>
            </div>
        </div>
    }
}
