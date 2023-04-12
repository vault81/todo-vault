use std::{cell::RefCell, rc::Rc, sync::Arc};

use entity::{
    chrono,
    todos::{self, Model},
    uuid,
};
use js_sys::Function;
use leptos::{ev::MouseEvent, *};
use leptos_dom::*;
use leptos_router::*;
use wasm_bindgen::{prelude::*, JsCast};
use web_sys::{Document, DomRect, Element, Event, EventTarget, Window};

use super::add_to_local_storage;
use crate::{components::*, functions::*, utils::*};

#[component]
fn TodoRow(
    cx: Scope,
    todo: todos::Model,
    trash_todo: Action<TrashTodo, Result<(), ServerFnError>>,
    edit_todo: MultiAction<EditTodo, Result<(), ServerFnError>>,
) -> impl IntoView {
    let edit_todo_fields = vec![
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
            value: todo.title.clone(),
            required: true,
        },
        FormField {
            id: "description".to_string(),
            label: Some("Description".to_string()),
            input_type: FormFieldInputType::TextArea,
            placeholder: "".to_string(),
            value: todo.description.clone().unwrap_or("".to_string()),
            required: false,
        },
        FormField {
            id: "due_date".to_string(),
            label: Some("Due Date".to_string()),
            input_type: FormFieldInputType::Date,
            placeholder: "".to_string(),
            value: todo
                .due_date
                .map(|dd| dd.format("%Y-%m-%d").to_string())
                .unwrap_or_else(|| "".to_string()),
            required: false,
        },
    ];

    view! { cx,
        <TableRow class="grid grid-cols-12 items-baseline rounded border md:table-row md:my-0 md:rounded-none md:border-b grid-rows-auto min-w-[20rem]">
            <TableCell class="order-1 col-start-1 row-span-3 row-start-1 justify-self-center">
                <div class="flex items-center">
                    <input
                        type="checkbox"
                        class="w-4 h-4 text-orange-600 bg-gray-100 rounded border-gray-300 dark:bg-gray-700 dark:border-gray-600 dark:ring-offset-gray-800 focus:ring-2 focus:ring-orange-500 dark:focus:ring-orange-600 dark:focus:ring-offset-gray-800"
                    />
                </div>
            </TableCell>
            <TableCell class="order-2 col-span-8 col-start-2 row-start-1 p-4 min-w-0 text-lg font-medium text-gray-900 md:text-base dark:text-white truncate">
                {todo.title.clone()}
            </TableCell>
            <TableCell class="overflow-y-auto overflow-x-hidden order-4 col-span-8 col-start-2 row-start-2 p-4 min-w-0 min-h-0 max-h-64 whitespace-pre text-ellipsis md:truncate">
                {todo.description.clone()}
            </TableCell>
            <TableCell class="order-5 col-span-8 col-start-2 row-start-3 p-4">
                <div class="inline md:hidden">"Due Date: "</div>
                {todo
                    .due_date
                    .map(|dd| dd.format("%d.%m.%Y").to_string())
                    .unwrap_or_else(|| "".to_string())}
            </TableCell>
            <TableCell class="grid order-3 grid-cols-1 col-span-2 col-end-12 grid-rows-2 row-span-3 row-start-1 gap-6 justify-items-center justify-self-end self-center p-4 md:flex-row md:grid-cols-2 md:grid-rows-1 md:grid-rows-none md:px-2">
                <FormDrawerButton
                    action=edit_todo
                    title="Edit Todo".to_string()
                    icon=Svg::FileEdit
                    fields=edit_todo_fields
                />
                <ActionForm action=trash_todo>
                    <input type="hidden" name="id" value=move || todo.id.to_string()/>
                    <Button b_type="submit">
                        <div class="w-5 h-5">{Svg::Trash2}</div>
                    </Button>
                </ActionForm>
            </TableCell>
        </TableRow>
    }
}

#[component]
fn TodoList(cx: Scope, list_id: uuid::Uuid) -> impl IntoView {
    let create_todo = create_server_multi_action::<AddTodo>(cx);
    let edit_todo = create_server_multi_action::<EditTodo>(cx);
    let trash_todo = create_server_action::<TrashTodo>(cx);

    let list_todos_resource = create_resource(
        cx,
        move || {
            (
                create_todo.version().get(),
                edit_todo.version().get(),
                trash_todo.version().get(),
                list_id,
            )
        },
        move |_| async move { list_todos(cx, list_id).await.unwrap_or_default() },
    );

    let list_resource = create_resource(
        cx,
        move || (list_id,),
        move |_| async move { find_list(cx, list_id).await },
    );

    // let list_todos_fn = move || list_todos_resource.read(cx);

    let add_todo_fields = vec![
        FormField {
            id: "list_id".to_string(),
            label: None,
            input_type: FormFieldInputType::Hidden,
            placeholder: "".to_string(),
            value: list_id.to_string(),
            required: true,
        },
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
    ];

    let column_headers = vec![
        ColumnHeader {
            id:    "checkbox".to_string(),
            label: "".to_string(),
            width: Some(4),
        },
        ColumnHeader {
            id:    "title".to_string(),
            label: "Title".to_string(),
            width: None,
        },
        ColumnHeader {
            id:    "description".to_string(),
            label: "Description".to_string(),
            width: None,
        },
        ColumnHeader {
            id:    "due_date".to_string(),
            label: "Due Date".to_string(),
            width: None,
        },
        ColumnHeader {
            id:    "action".to_string(),
            label: "Action".to_string(),
            width: Some(48),
        },
    ];

    view! { cx,
        <>
            <Transition fallback=move || {
                view! { cx, <h1 class="bg-red-700">"Loading..."</h1> }
            }>
                <h1 class="mb-4 text-2xl font-semibold text-gray-900 dark:text-white">
                    {list_resource
                        .read(cx)
                        .map(|list| list.expect("no list").title.clone())
                        .unwrap_or_else(|| "".to_string())}
                </h1>
            </Transition>
            <div class="overflow-x-auto relative border-0 border-gray-200 shadow-md md:rounded-lg md:border dark:border-gray-700">
                <div class="flex justify-between items-center p-2">
                    <FormDrawerButton
                        action=create_todo
                        title="Add Todo".to_string()
                        icon=Svg::FilePlus
                        fields=add_todo_fields
                    />
                    <label for="table-search" class="sr-only">
                        "Search"
                    </label>
                    <div class="relative">
                        <div class="absolute left-0 top-2 items-center pl-3 pointer-events-none">
                            <div class="w-5 h-5">{Svg::Search}</div>
                        </div>
                        <input
                            type="text"
                            id="table-search"
                            class="block p-2 pl-10 w-full text-sm text-gray-900 bg-gray-50 rounded-lg border border-gray-300 dark:placeholder-gray-400 dark:text-white dark:bg-gray-700 dark:border-gray-600 focus:border-orange-500 focus:ring-orange-500 min-w-[7em]"
                            placeholder="Search"
                        />
                    </div>
                </div>
                <Transition fallback=move || {
                    view! { cx, <tr class="bg-red-700">"Loading..."</tr> }
                }>
                    <Table column_headers=column_headers.clone()>
                        <For
                            each=move || list_todos_resource.read(cx).unwrap_or(vec![])
                            key=|todo| todo.calc_hash()
                            view=move |cx, todo: todos::Model| {
                                view! { cx, <TodoRow todo=todo trash_todo=trash_todo edit_todo=edit_todo/> }
                            }
                        />
                    </Table>
                </Transition>
            </div>
        </>
    }
}

#[component]
pub fn TodoListPage(cx: Scope) -> impl IntoView {
    let params = use_params_map(cx);
    let list_id = move || {
        params.with(|params| {
            let str =
                params.get("list_id").cloned().unwrap_or_default().clone();

            uuid::Uuid::parse_str(&str).unwrap_or_default()
        })
    };

    create_effect(cx, move |_| {
        let list_id = list_id();
        add_to_local_storage(list_id);
    });

    view! { cx,
        <MainPage>
            <TodoList list_id=list_id()/>
        </MainPage>
    }
}
