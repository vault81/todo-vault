use entity::{
    chrono,
    todos::{self, Model},
    uuid,
};
use leptos::*;
use leptos_dom::*;
use leptos_router::*;

use crate::{components::*, functions::*};

#[component]
fn FormDrawer(
    cx: Scope,
    create_todo_action: Action<AddTodo, Result<(), ServerFnError>>,
) -> impl IntoView {
    let (drawer_open, set_drawer_open) = create_signal(cx, false);
    let open_drawer = move || {
        set_drawer_open(true);
    };
    let close_drawer = move || {
        set_drawer_open(false);
    };

    view! {
        cx,

        // <!-- drawer init and show -->
        <Button on:click=move |_| open_drawer()>
            <div class="w-5 h-5">
                {Svg::FilePlus}
                <span class="sr-only">"Add Todo"</span>
            </div>
        </Button>

        // <!-- drawer component -->
        <div id="drawer-form"  class="overflow-y-auto fixed top-0 left-0 z-40 p-4 w-80 h-screen bg-white transition-transform -translate-x-full dark:bg-gray-800" class:transform-none={move || drawer_open()} tabindex="-1" aria-labelledby="drawer-form-label">
            <h5 id="drawer-form-label" class="inline-flex items-center mb-6 text-base font-semibold text-gray-500 uppercase dark:text-gray-400"><svg class="mr-2 w-5 h-5" aria-hidden="true" fill="currentColor" viewBox="0 0 20 20" xmlns="http://www.w3.org/2000/svg"><path fill-rule="evenodd" d="M18 10a8 8 0 11-16 0 8 8 0 0116 0zm-7-4a1 1 0 11-2 0 1 1 0 012 0zM9 9a1 1 0 000 2v3a1 1 0 001 1h1a1 1 0 100-2v-3a1 1 0 00-1-1H9z" clip-rule="evenodd"></path></svg>"New Todo"</h5>
            <button type="button" on:click=move |_| close_drawer() data-drawer-hide="drawer-form" aria-controls="drawer-form" class="inline-flex absolute top-2.5 right-2.5 items-center p-1.5 text-sm text-gray-400 bg-transparent rounded-lg hover:text-gray-900 hover:bg-gray-200 dark:hover:bg-gray-600 dark:hover:text-white" >
                <svg aria-hidden="true" class="w-5 h-5" fill="currentColor" viewBox="0 0 20 20" xmlns="http://www.w3.org/2000/svg"><path fill-rule="evenodd" d="M4.293 4.293a1 1 0 011.414 0L10 8.586l4.293-4.293a1 1 0 111.414 1.414L11.414 10l4.293 4.293a1 1 0 01-1.414 1.414L10 11.414l-4.293 4.293a1 1 0 01-1.414-1.414L8.586 10 4.293 5.707a1 1 0 010-1.414z" clip-rule="evenodd"></path></svg>
                <span class="sr-only">"Close menu"</span>
            </button>
            <div class="mb-6">
                <ActionForm action=create_todo_action class="mb-6">
                    <div class="mb-6">
                        <label for="title" class="block mb-2 text-sm font-medium text-gray-900 dark:text-white">"Title"</label>
                        <input id="title" type="text" name="title" class="block p-2.5 w-full text-sm text-gray-900 bg-gray-50 rounded-lg border border-gray-300 dark:placeholder-gray-400 dark:text-white dark:bg-gray-700 dark:border-gray-600 focus:border-blue-500 focus:ring-blue-500 dark:focus:ring-blue-500 dark:focus:border-blue-500" placeholder="Do Dishes" required />
                    </div>
                    <div class="mb-6">
                        <label for="description" class="block mb-2 text-sm font-medium text-gray-900 dark:text-white">"Description"</label>
                        <textarea id="description" name="text" rows="4" class="block p-2.5 w-full text-sm text-gray-900 bg-gray-50 rounded-lg border border-gray-300 dark:placeholder-gray-400 dark:text-white dark:bg-gray-700 dark:border-gray-600 focus:border-blue-500 focus:ring-blue-500 dark:focus:ring-blue-500 dark:focus:border-blue-500" placeholder="Write todo description..."></textarea>
                    </div>
                    <div class="relative mb-6">
                        <label for="due_date" class="block mb-2 text-sm font-medium text-gray-900 dark:text-white">"Description"</label>
                        <input type="date" id="due_date" name="due_date" class="block p-2.5 w-full text-sm text-gray-900 bg-gray-50 rounded-lg border border-gray-300 dark:placeholder-gray-400 dark:text-white dark:bg-gray-700 dark:border-gray-600 focus:border-blue-500 focus:ring-blue-500 datepicker-input dark:focus:ring-blue-500 dark:focus:border-blue-500" min="2000-01-01" max="2999-99-99"/>
                    </div>
                    <button type="submit" class="flex justify-center items-center py-2.5 px-5 mr-2 mb-2 w-full text-sm font-medium text-white bg-blue-700 rounded-lg dark:bg-blue-600 hover:bg-blue-800 focus:ring-4 focus:ring-blue-300 focus:outline-none dark:hover:bg-blue-700 dark:focus:ring-blue-800"><svg class="mr-2 w-5 h-5" aria-hidden="true" fill="currentColor" viewBox="0 0 20 20" xmlns="http://www.w3.org/2000/svg"><path fill-rule="evenodd" d="M6 2a1 1 0 00-1 1v1H4a2 2 0 00-2 2v10a2 2 0 002 2h12a2 2 0 002-2V6a2 2 0 00-2-2h-1V3a1 1 0 10-2 0v1H7V3a1 1 0 00-1-1zm0 5a1 1 0 000 2h8a1 1 0 100-2H6z" clip-rule="evenodd"></path></svg>"Create event"</button>
                </ActionForm>
            </div>
        </div>
    }
}

#[component]
fn DropdownButton(cx: Scope) -> impl IntoView {
    let (dropdown_open, set_dropdown_open) = create_signal(cx, false);
    let toggle_modal = move || {
        set_dropdown_open(!dropdown_open());
    };

    view! {
        cx,
        <div>
            <button id="dropdownRadioButton" on:click=move |_| toggle_modal() data-dropdown-toggle="dropdownRadio" class="inline-flex items-center py-2.5 px-3 text-sm font-medium text-gray-500 bg-white rounded-lg border border-gray-300 dark:text-white dark:bg-gray-800 dark:border-gray-600 hover:bg-gray-100 focus:ring-4 focus:ring-gray-200 focus:outline-none dark:hover:bg-gray-700 dark:hover:border-gray-600 dark:focus:ring-gray-700" type="button">
                <svg class="mr-2 w-4 h-4 text-gray-400" aria-hidden="true" fill="currentColor" viewBox="0 0 20 20" xmlns="http://www.w3.org/2000/svg"><path fill-rule="evenodd" d="M10 18a8 8 0 100-16 8 8 0 000 16zm1-12a1 1 0 10-2 0v4a1 1 0 00.293.707l2.828 2.829a1 1 0 101.415-1.415L11 9.586V6z" clip-rule="evenodd"></path></svg>
                "Last 30 days"
                <svg class="m-auto w-3 h-3" aria-hidden="true" fill="none" stroke="currentColor" viewBox="0 0 24 24" xmlns="http://www.w3.org/2000/svg"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19 9l-7 7-7-7"></path></svg>
            </button>
            // <!-- Dropdown menu -->
            <div
                id="dropdownRadio"
                class="absolute left-0 top-10 z-10 m-0 w-48 bg-white rounded-lg divide-y divide-gray-100 shadow dark:bg-gray-700 dark:divide-gray-600"
                data-popper-reference-hidden="" data-popper-escaped="" data-popper-placement="top"
                class:hidden= move || !dropdown_open()
            >
                <ul class="p-3 space-y-1 text-sm text-gray-700 dark:text-gray-200" aria-labelledby="dropdownRadioButton">
                    <li>
                        <div class="flex items-center p-2 rounded hover:bg-gray-100 dark:hover:bg-gray-600">
                            <input id="filter-radio-example-1" type="radio" value="" name="filter-radio" class="w-4 h-4 text-orange-600 bg-gray-100 border-gray-300 dark:bg-gray-700 dark:border-gray-600 dark:ring-offset-gray-800 focus:ring-2 focus:ring-orange-500 dark:focus:ring-orange-600 dark:focus:ring-offset-gray-800" />
                            <label for="filter-radio-example-1" class="ml-2 w-full text-sm font-medium text-gray-900 rounded dark:text-gray-300">"Last day"</label>
                        </div>
                    </li>
                    <li>
                        <div class="flex items-center p-2 rounded hover:bg-gray-100 dark:hover:bg-gray-600">
                            <input checked="" id="filter-radio-example-2" type="radio" value="" name="filter-radio" class="w-4 h-4 text-orange-600 bg-gray-100 border-gray-300 dark:bg-gray-700 dark:border-gray-600 dark:ring-offset-gray-800 focus:ring-2 focus:ring-orange-500 dark:focus:ring-orange-600 dark:focus:ring-offset-gray-800" />
                            <label for="filter-radio-example-2" class="ml-2 w-full text-sm font-medium text-gray-900 rounded dark:text-gray-300">"Last 7 days"</label>
                        </div>
                    </li>
                    <li>
                        <div class="flex items-center p-2 rounded hover:bg-gray-100 dark:hover:bg-gray-600">
                            <input id="filter-radio-example-3" type="radio" value="" name="filter-radio" class="w-4 h-4 text-orange-600 bg-gray-100 border-gray-300 dark:bg-gray-700 dark:border-gray-600 dark:ring-offset-gray-800 focus:ring-2 focus:ring-orange-500 dark:focus:ring-orange-600 dark:focus:ring-offset-gray-800" />
                            <label for="filter-radio-example-3" class="ml-2 w-full text-sm font-medium text-gray-900 rounded dark:text-gray-300">"Last 30 days"</label>
                        </div>
                    </li>
                    <li>
                        <div class="flex items-center p-2 rounded hover:bg-gray-100 dark:hover:bg-gray-600">
                            <input id="filter-radio-example-4" type="radio" value="" name="filter-radio" class="w-4 h-4 text-orange-600 bg-gray-100 border-gray-300 dark:bg-gray-700 dark:border-gray-600 dark:ring-offset-gray-800 focus:ring-2 focus:ring-orange-500 dark:focus:ring-orange-600 dark:focus:ring-offset-gray-800" />
                            <label for="filter-radio-example-4" class="ml-2 w-full text-sm font-medium text-gray-900 rounded dark:text-gray-300">"Last month"</label>
                        </div>
                    </li>
                    <li>
                        <div class="flex items-center p-2 rounded hover:bg-gray-100 dark:hover:bg-gray-600">
                            <input id="filter-radio-example-5" type="radio" value="" name="filter-radio" class="w-4 h-4 text-orange-600 bg-gray-100 border-gray-300 dark:bg-gray-700 dark:border-gray-600 dark:ring-offset-gray-800 focus:ring-2 focus:ring-orange-500 dark:focus:ring-orange-600 dark:focus:ring-offset-gray-800" />
                            <label for="filter-radio-example-5" class="ml-2 w-full text-sm font-medium text-gray-900 rounded dark:text-gray-300">"Last year"</label>
                        </div>
                    </li>
                </ul>
            </div>
        </div>
    }
}

#[component]
fn TodoRow(
    cx: Scope,
    todo: todos::Model,
    trash_todo: Action<TrashTodo, Result<(), ServerFnError>>,
) -> impl IntoView {
    view! {
        cx,
        <tr class="font-normal bg-white border-b dark:bg-gray-800 dark:border-gray-700 hover:bg-gray-50 dark:hover:bg-gray-600">
            <td class="p-4 w-4">
                <div class="flex items-center">
                    <input id="checkbox-table-search-1" type="checkbox" class="w-4 h-4 text-orange-600 bg-gray-100 rounded border-gray-300 dark:bg-gray-700 dark:border-gray-600 dark:ring-offset-gray-800 focus:ring-2 focus:ring-orange-500 dark:focus:ring-orange-600 dark:focus:ring-offset-gray-800" />
                    <label for="checkbox-table-search-1" class="sr-only">"checkbox"</label>
                </div>
            </td>
            <th scope="row" class="py-4 px-6 font-medium text-gray-900 whitespace-nowrap dark:text-white">
                {todo.title.clone()}
            </th>
            <td class="py-4 px-6">
                {todo.text.clone()}
            </td>
            <td class="py-4 px-4">
                {
                    todo.due_date
                        .clone()
                        .map(|dd| dd.format("%d.%m.%Y").to_string())
                        .unwrap_or_else(|| "".to_string())
                }
            </td>
            <td class="flex py-4 px-6">
                <ActionForm action=trash_todo>
                    <input type="hidden" name="id" value={move || todo.id.to_string()} />
                    <button type="submit" class="grid items-center p-2.5 text-sm text-gray-900 dark:text-gray-400 hover:text-orange-500">
                        <div class="w-5 h-5">
                            {Svg::FileEdit}
                        </div>
                    </button>
                </ActionForm>
                <ActionForm action=trash_todo>
                    <input type="hidden" name="id" value={move || todo.id.to_string()} />
                    <button type="submit" class="grid items-center p-2.5 text-sm text-gray-900 dark:text-gray-400 hover:text-orange-500">
                        <div class="w-5 h-5">
                            {Svg::Trash2}
                        </div>
                    </button>
                </ActionForm>
            </td>
        </tr>
    }
}

#[component]
fn Todos(cx: Scope) -> impl IntoView {
    let create_todo_action = create_server_action::<AddTodo>(cx);
    let trash_todo = create_server_action::<TrashTodo>(cx);

    let list_todos_resource = create_local_resource(
        cx,
        move || {
            (
                trash_todo.version().get(),
                create_todo_action.version().get(),
            )
        },
        move |_| async move { list_todos(cx).await.unwrap_or_default() },
    );
    let list_todos_fn = move || list_todos_resource.read(cx);

    view! {
        cx,
        <div class="overflow-x-auto relative border border-gray-200 shadow-md sm:rounded-lg dark:border-gray-700">
            <div class="flex justify-between items-center px-2 pt-2 pb-4">
                <FormDrawer create_todo_action={create_todo_action}/>
                <label for="table-search" class="sr-only">"Search"</label>
                <div class="relative">
                    <div class="absolute left-0 top-2 items-center pl-3 pointer-events-none">
                        <svg class="w-5 h-5 text-gray-500 dark:text-gray-400" aria-hidden="true" fill="currentColor" viewBox="0 0 20 20" xmlns="http://www.w3.org/2000/svg"><path fill-rule="evenodd" d="M8 4a4 4 0 100 8 4 4 0 000-8zM2 8a6 6 0 1110.89 3.476l4.817 4.817a1 1 0 01-1.414 1.414l-4.816-4.816A6 6 0 012 8z" clip-rule="evenodd"></path></svg>
                    </div>
                    <input type="text" id="table-search" class="block p-2 pl-10 w-full text-sm text-gray-900 bg-gray-50 rounded-lg border border-gray-300 dark:placeholder-gray-400 dark:text-white dark:bg-gray-700 dark:border-gray-600 focus:border-orange-500 focus:ring-orange-500 min-w-[7em]" placeholder="Search" />
                </div>
            </div>
            <table class="w-full text-sm text-left text-gray-500 dark:text-gray-400">
                <thead class="text-xs text-gray-700 uppercase bg-gray-50 dark:text-gray-400 dark:bg-gray-700">
                    <tr>
                        <th scope="col" class="p-4">
                            <div class="flex items-center">
                                <input id="checkbox-all-search" type="checkbox" class="w-4 h-4 text-orange-600 bg-gray-100 rounded border-gray-300 dark:bg-gray-700 dark:border-gray-600 dark:ring-offset-gray-800 focus:ring-2 focus:ring-orange-500 dark:focus:ring-orange-600 dark:focus:ring-offset-gray-800" />
                                <label for="checkbox-all-search" class="sr-only">"checkbox"</label>
                            </div>
                        </th>
                        <th scope="col" class="py-3 px-6">
                            "Title"
                        </th>
                        <th scope="col" class="py-3 px-6">
                            "Description"
                        </th>
                        <th scope="col" class="py-3 px-4">
                            "Due Date"
                        </th>
                        <th scope="col" class="py-3 px-6">
                            "Action"
                        </th>
                    </tr>
                </thead>
                <tbody>
                    <For
                        each={move || list_todos_fn().unwrap_or_default()}
                        key={|todo| todo.id.to_string()}
                        view=move |cx, todo: todos::Model| {view! {cx,
                            <TodoRow todo={todo} trash_todo={trash_todo} />
                    }} />
                    <div></div>
                </tbody>
            </table>
        </div>
    }
}

#[component]
pub fn TodosPage(cx: Scope) -> impl IntoView {
    view! {
        cx,
        <div class="flex flex-col min-h-screen bg-gray-100 dark:bg-gray-900">
            <Navbar/>
            <div class="container flex mx-auto">
                <div class="overflow-y-auto p-4 my-4 w-64 min-w-full bg-white rounded-lg border border-gray-200 shadow-md dark:bg-gray-800 dark:border-gray-700">
                    <Todos />
                </div>
            </div>
            <Footer/>
        </div>
    }
}
