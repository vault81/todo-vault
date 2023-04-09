use entity::{
    chrono,
    todos::{self, Model},
    uuid,
};
use leptos::{ev::MouseEvent, *};
use leptos_dom::*;
use leptos_router::*;

use crate::{components::*, functions::*};

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
    edit_todo: Action<EditTodo, Result<(), ServerFnError>>,
) -> impl IntoView {
    view! {
        cx,
        <tr class="grid grid-cols-12 items-baseline my-4 bg-white rounded border md:table-row md:my-0 md:rounded-none md:border-b dark:bg-gray-800 dark:border-gray-700 hover:bg-gray-50 grid-rows-auto bak-grid-rows-3 min-w-[20rem] dark:hover:bg-gray-600">
            <td class="order-1 col-start-1 row-span-3 row-start-1 justify-self-center p-4">
                <div class="flex items-center">
                    <input type="checkbox" class="w-4 h-4 text-orange-600 bg-gray-100 rounded border-gray-300 dark:bg-gray-700 dark:border-gray-600 dark:ring-offset-gray-800 focus:ring-2 focus:ring-orange-500 dark:focus:ring-orange-600 dark:focus:ring-offset-gray-800" />
                </div>
            </td>
            <td class="order-2 col-span-8 col-start-2 row-start-1 p-4 min-w-0 text-lg font-medium text-gray-900 md:text-base dark:text-white truncate">
                {todo.title.clone()}
            </td>
            <td class="overflow-auto order-4 col-span-8 col-start-2 row-start-2 p-4 min-w-0 min-h-0 max-h-64 whitespace-pre text-ellipsis md:truncate">
                {todo.description.clone()}
            </td>
            <td class="order-5 col-span-8 col-start-2 row-start-3 p-4">
                <div class="inline md:hidden">
                    "Due Date: "
                </div>
                {
                    todo.due_date
                        .map(|dd| dd.format("%d.%m.%Y").to_string())
                        .unwrap_or_else(|| "".to_string())
                }
            </td>
            <td class="grid order-3 grid-cols-1 col-span-2 col-end-12 row-span-3 row-start-1 gap-6 justify-items-center justify-self-end self-center p-4 md:flex-row md:grid-cols-2 md:grid-rows-1 md:grid-rows-none md:px-2 asdasdgrid-rows-2">
                <FormDrawerButton
                    action={edit_todo}
                    title="Edit Todo".to_string()
                    icon={Svg::FileEdit}
                    fields=vec![
                        FormField {
                            id: "id".to_string(),
                            label: None,
                            input_type: FormFieldInputType::Hidden,
                            placeholder: "".to_string(),
                            value: todo.id.to_string(),
                            required: true,
                        },
                        FormField {
                            id: "title".to_string(),
                            label: Some("Title".to_string()),
                            input_type: FormFieldInputType::Text,
                            placeholder: "Do things".to_string(),
                            value: todo.title,
                            required: true,
                        },
                        FormField {
                            id: "description".to_string(),
                            label: Some("Description".to_string()),
                            input_type: FormFieldInputType::TextArea,
                            placeholder: "".to_string(),
                            value: todo.description.unwrap_or("".to_string()),
                            required: false,
                        },
                        FormField {
                            id: "due_date".to_string(),
                            label: Some("Due Date".to_string()),
                            input_type: FormFieldInputType::Date,
                            placeholder: "".to_string(),
                            value: todo.due_date.map(|dd| dd.format("%Y-%m-%d").to_string()).unwrap_or_else(|| "".to_string()),
                            required: false,
                        },
                    ]
                />
                <ActionForm action=trash_todo>
                    <input type="hidden" name="id" value={move || todo.id.to_string()} />
                    <Button b_type="submit">
                        <div class="w-5 h-5">
                            {Svg::Trash2}
                        </div>
                    </Button>
                </ActionForm>
            </td>
        </tr>
    }
}

#[component]
pub fn Table(
    cx: Scope,
    children: Children,
    column_headers: Vec<String>,
) -> impl IntoView {
    view! {
        cx,
        <table class="w-full text-left text-gray-500 table-fixed dark:text-gray-400">
            <thead class="hidden text-gray-700 uppercase bg-gray-50 md:table-header-group dark:text-gray-400 dark:bg-gray-700">
                <tr>
                    <th scope="col" class="p-3 w-8">
                    </th>
                    {column_headers.iter().map(|header| view! {cx,
                        <th scope="col" class="py-3 px-6">{header}</th>
                    }).collect::<Vec<_>>()}
                </tr>
            </thead>
            <tbody>
                {children(cx)}
            </tbody>
        </table>
    }
}

#[component]
fn Todos(cx: Scope) -> impl IntoView {
    let create_todo = create_server_action::<AddTodo>(cx);
    let edit_todo = create_server_action::<EditTodo>(cx);
    let trash_todo = create_server_action::<TrashTodo>(cx);

    let list_todos_resource = create_resource(
        cx,
        move || {
            (
                create_todo.version().get(),
                edit_todo.version().get(),
                trash_todo.version().get(),
            )
        },
        move |_| async move { list_todos(cx).await.unwrap_or_default() },
    );
    // let list_todos_fn = move || list_todos_resource.read(cx);

    view! {
        cx,
        <div class="overflow-x-auto relative border-0 border-gray-200 shadow-md md:rounded-lg md:border dark:border-gray-700">
            <div class="flex justify-between items-center px-2 pt-2 pb-4">
                <FormDrawerButton
                    action={create_todo}
                    title="Add Todo".to_string()
                    icon=Svg::FilePlus
                    fields=vec![
                        FormField {
                            id: "title".to_string(),
                            label: Some("Title".to_string()),
                            input_type: FormFieldInputType::Text,
                            placeholder: "Do things".to_string(),
                            value: "".to_string(),
                            required: true,
                        },
                        FormField {
                            id: "description".to_string(),
                            label: Some("Description".to_string()),
                            input_type: FormFieldInputType::TextArea,
                            placeholder: "".to_string(),
                            value: "".to_string(),
                            required: false,
                        },
                        FormField {
                            id: "due_date".to_string(),
                            label: Some("Due Date".to_string()),
                            input_type: FormFieldInputType::Date,
                            placeholder: "".to_string(),
                            value: "".to_string(),
                            required: false,
                        },
                    ]
                />
                <label for="table-search" class="sr-only">"Search"</label>
                <div class="relative">
                    <div class="absolute left-0 top-2 items-center pl-3 pointer-events-none">
                        <div class="w-5 h-5">
                            {Svg::Search}
                        </div>
                    </div>
                    <input type="text" id="table-search" class="block p-2 pl-10 w-full text-sm text-gray-900 bg-gray-50 rounded-lg border border-gray-300 dark:placeholder-gray-400 dark:text-white dark:bg-gray-700 dark:border-gray-600 focus:border-orange-500 focus:ring-orange-500 min-w-[7em]" placeholder="Search" />
                </div>
            </div>
            <Transition
                fallback={move || view! {cx,<tr class="bg-red-700">"Loading..."</tr>}}
            >
                <Table
                    column_headers= vec!["Title".to_string(), "Description".to_string(), "Due Date".to_string(), "Action".to_string()]
                >
                    <For
                        each={move || list_todos_resource.read(cx).unwrap_or(vec![])}
                        key={|todo| todo.calc_hash()}
                        view=move |cx, todo: todos::Model| {view! {cx,
                            <TodoRow todo={todo} trash_todo={trash_todo} edit_todo={edit_todo} />
                    }} />
                </Table>
            </Transition>
        </div>
    }
}

#[component]
pub fn TodosPage(cx: Scope) -> impl IntoView {
    view! {
        cx,
        <div class="flex overflow-y-auto flex-col h-screen bg-gray-100 dark:bg-gray-900">
            <Navbar/>
            <div class="container flex mx-auto">
                <div class="p-4 my-4 w-64 min-w-full bg-white rounded-lg border border-gray-200 shadow-md dark:bg-gray-800 dark:border-gray-700">
                    <Todos/>
                </div>
            </div>
            <Footer/>
        </div>
    }
}
