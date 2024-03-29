use entity::uuid;
use leptos::*;
use leptos_dom::*;
use leptos_router::*;

use crate::{components::*, functions::*, utils};

const STORAGE_KEY: &str = "my_todo_lists";

fn retrieve_from_local_storage() -> Vec<uuid::Uuid> {
    let storage = window().local_storage();

    if let Ok(Some(storage)) = storage {
        let list_ids_opt = storage
            .get_item(STORAGE_KEY)
            .ok()
            .flatten()
            .and_then(|lists| {
                let lists: Vec<uuid::Uuid> =
                    serde_json::from_str(&lists).ok()?;
                Some(lists)
            });
        if let Some(list_ids) = list_ids_opt {
            list_ids
        } else {
            vec![]
        }
    } else {
        vec![]
    }
}

pub fn add_to_local_storage(list_id: uuid::Uuid) {
    let storage = window().local_storage();

    if let Ok(Some(storage)) = storage {
        let mut list_ids = retrieve_from_local_storage();
        if list_ids.contains(&list_id) {
            return;
        }
        list_ids.push(list_id);
        let list_ids = serde_json::to_string(&list_ids).unwrap();
        storage.set_item(STORAGE_KEY, &list_ids).unwrap();
    }
}

pub fn remove_from_local_storage(list_id: uuid::Uuid) {
    let storage = window().local_storage();

    if let Ok(Some(storage)) = storage {
        let mut list_ids = retrieve_from_local_storage();
        if !list_ids.contains(&list_id) {
            return;
        }
        list_ids.retain(|id| id != &list_id);
        let list_ids = serde_json::to_string(&list_ids).unwrap();
        storage.set_item(STORAGE_KEY, &list_ids).unwrap();
    }
}

#[component]
pub fn MyTodoListsPage(cx: Scope) -> impl IntoView {
    tracing::info!("MyTodoListsPage");

    let delete_list_action =
        create_multi_action(cx.clone(), move |delete_list_p: &DeleteList| {
            let cx = cx.clone();
            let delete_list_p = delete_list_p.clone();
            async move {
                remove_from_local_storage(delete_list_p.list_id);
                delete_list(cx, delete_list_p.list_id).await?;
                Ok(())
            }
        })
        .using_server_fn::<DeleteList>();

    let add_list_action =
        create_multi_action(cx, move |add_list_p: &AddList| {
            let cx = cx.clone();
            let add_list_p = add_list_p.clone();
            async move {
                let list = add_list(cx, add_list_p.title).await?;
                add_to_local_storage(list.id);
                Ok(())
            }
        })
        .using_server_fn::<AddList>();

    let add_list_fields = vec![FormField {
        id: "title".to_string(),
        label: Some("Title".to_string()),
        input_type: FormFieldInputType::Text,
        placeholder: "My To-Do List".to_string(),
        value: "".to_string(),
        required: true,
    }];

    let my_lists = create_local_resource(
        cx,
        move || {
            (
                add_list_action.version().get(),
                delete_list_action.version().get(),
            )
        },
        move |_| {
            let cx = cx.clone();
            async move {
                let list_ids = retrieve_from_local_storage();
                let lists = list_ids
                    .into_iter()
                    .map(|list_id| find_list(cx.clone(), list_id))
                    .collect::<Vec<_>>();
                let lists = futures::future::join_all(lists).await;
                let lists: Vec<entity::lists::Model> =
                    lists.into_iter().filter_map(Result::ok).collect();
                lists
            }
        },
    );

    let column_headers = vec![
        ColumnHeader {
            id:     "title".to_string(),
            label:  "Title".to_string(),
            width:  None,
            center: false,
        },
        ColumnHeader {
            id:     "actions".to_string(),
            label:  "".to_string(),
            width:  Some(24),
            center: false,
        },
    ];

    let no_lists_row = move || {
        let lists = my_lists.read(cx);
        if lists.is_none() || lists.unwrap_or(vec![]).is_empty() {
            view! { cx,
                <TableRow>
                    <TableCell colspan=2>
                        <div class="flex justify-center items-center">
                            <div class="flex text-gray-500 dark:text-gray-400">
                                <div class="w-6 h-6">{Svg::AlertCircle}</div>
                                <span class="ml-2">
                                    "No lists found. "
                                    "Click the button on the top left of this panel to add a list."
                                </span>
                            </div>
                        </div>
                    </TableCell>
                </TableRow>
            }
            .into_view(cx)
        } else {
            view! { cx, <></> }.into_view(cx)
        }
    };

    view! { cx,
        <MainPage>
            <h1 class="mb-4 text-2xl font-semibold text-gray-900 dark:text-white">
                "My To-Do Lists"
            </h1>
            <div class="overflow-x-auto relative rounded-lg border border-gray-200 shadow-md dark:border-gray-700">
                <div class="flex justify-between items-center p-2 border-b border-gray-200 md:border-none dark:border-gray-700">
                    <FormDrawerButton
                        action=add_list_action
                        title="Add List".to_string()
                        icon=Svg::FilePlus
                        fields=add_list_fields
                    />
                </div>
                <Table column_headers=column_headers>
                    <Transition fallback=move || {
                        view! { cx, <></> }
                    }>{no_lists_row()}</Transition>
                    <For
                        each=move || my_lists.read(cx).unwrap_or(vec![])
                        key=|list| list.id
                        view=move |cx, list: entity::lists::Model| {
                            view! { cx,
                                <TableRow class="border-b">
                                    <TableCell on:click=move |_| {
                                        utils::set_href(format!("/todo/{}", list.id));
                                    }>
                                        <span onclick="event.cancelBubble = true;">{list.title}</span>
                                    </TableCell>
                                    <TableCell class="w-24">
                                        <MultiActionForm action=delete_list_action>
                                            <input type="hidden" name="list_id" value=move || list.id.to_string()/>
                                            <Button class="border-none" b_type="submit">
                                                <div class="w-5 h-5">{Svg::Trash2}</div>
                                            </Button>
                                        </MultiActionForm>
                                    </TableCell>
                                </TableRow>
                            }
                        }
                    />
                </Table>
            </div>
        </MainPage>
    }
}
