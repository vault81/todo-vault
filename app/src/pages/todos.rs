use leptos::*;

use crate::{components::*, functions::*};

#[component]
fn Trashcan(cx: Scope) -> impl IntoView {
    view! {
        cx,
        <svg class="w-4 h-4" aria-hidden="true" fill="none" stroke="currentColor" stroke-width="1.5" viewBox="0 0 24 24" xmlns="http://www.w3.org/2000/svg">
            <path d="M14.74 9l-.346 9m-4.788 0L9.26 9m9.968-3.21c.342.052.682.107 1.022.166m-1.022-.165L18.16 19.673a2.25 2.25 0 01-2.244 2.077H8.084a2.25 2.25 0 01-2.244-2.077L4.772 5.79m14.456 0a48.108 48.108 0 00-3.478-.397m-12 .562c.34-.059.68-.114 1.022-.165m0 0a48.11 48.11 0 013.478-.397m7.5 0v-.916c0-1.18-.91-2.164-2.09-2.201a51.964 51.964 0 00-3.32 0c-1.18.037-2.09 1.022-2.09 2.201v.916m7.5 0a48.667 48.667 0 00-7.5 0" stroke-linecap="round" stroke-linejoin="round"></path>
        </svg>
    }
}

#[component]
fn Table(cx: Scope) -> impl IntoView {
    view!(cx,
        <div class="overflow-x-auto relative shadow-md sm:rounded-lg">
            <table class="w-full text-sm text-left text-gray-500 dark:text-gray-400">
                <thead class="text-xs text-gray-700 uppercase bg-gray-50 dark:text-gray-400 dark:bg-gray-700">
                    <tr>
                        <th scope="col" class="py-3 px-6">
                            "Todo"
                        </th>
                        <th scope="col" class="py-3 px-6">
                            "Done"
                        </th>
                        <th>
                        </th>
                        <th scope="col" class="py-3 px-6">
                            <Button b_type="submit" >
                                "+"
                            </Button>
                        </th>
                    </tr>
                </thead>
                <tbody>
                    <tr class="bg-white border-b dark:bg-gray-800 dark:border-gray-700 hover:bg-gray-50 dark:hover:bg-gray-600">
                        <th scope="row" class="py-4 px-6 font-medium text-gray-900 whitespace-nowrap dark:text-white">
                            "Example 1"
                        </th>
                        <td class="py-4 px-6">
                            "✓"
                        </td>
                        <td class="py-4 px-6 text-right">
                            <a href="#" class="font-medium text-blue-600 dark:text-blue-500 hover:underline">"Edit"</a>
                        </td>
                        <td class="py-4 px-6 text-right">
                            <a href="#" class="text-blue-600 dark:text-blue-500 hover:underline">
                                <Trashcan/>
                            </a>
                        </td>
                    </tr>
                    <tr class="bg-white border-b dark:bg-gray-800 dark:border-gray-700 hover:bg-gray-50 dark:hover:bg-gray-600">
                        <th scope="row" class="py-4 px-6 font-medium text-gray-900 whitespace-nowrap dark:text-white">
                            "Example 2"
                        </th>
                        <td class="py-4 px-6">
                            "⛌"
                        </td>
                        <td class="py-4 px-6 text-right">
                            <a href="#" class="font-medium text-blue-600 dark:text-blue-500 hover:underline">"Edit"</a>
                        </td>
                        <td class="py-4 px-6 text-right">
                            <a href="#" class="text-blue-600 dark:text-blue-500 hover:underline">
                                <Trashcan/>
                            </a>
                        </td>
                    </tr>
                </tbody>
            </table>
        </div>
    )
}

#[component]
pub fn TodosPage(cx: Scope) -> impl IntoView {
    view! { cx,
        <div class="flex flex-col h-screen bg-gray-100 dark:bg-gray-900">
            <Navbar/>
            <div class="container flex overflow-y-auto flex-col items-center mx-auto">
                <div id="root">
                    <p>
                        <div class="p-4 my-4 mx-4 bg-white rounded-lg border border-gray-200 shadow-md sm:p-6 md:p-8 dark:bg-gray-800 dark:border-gray-700">
                            <Table />
                        </div>
                    </p>
                </div>
            </div>
            <Footer/>
        </div>
    }
}
