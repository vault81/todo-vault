use entity::{lists, todos::Entity as Todo, uuid};
use leptos::*;
use leptos_icons::*;
use leptos_router::*;

use crate::{components::*, functions::*};

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
fn Card(
    cx: Scope,
    children: Children,
    #[prop(optional, into)] class: String,
) -> impl IntoView {
    view! { cx,
        <div class=format!("m-4 shadow-xl card card-compact {class}")>
            <div class="card-body">{children(cx)}</div>
        </div>
    }
}

#[component]
fn Spinner<F>(
    cx: Scope,
    hidden: F,
    #[prop(optional, into)] class: String,
) -> impl IntoView
where
    F: Fn() -> bool + 'static,
{
    view! { cx,
        <span
            class=format!("loading loading-spinner loading-xs {class}")
            class:hidden=hidden
        ></span>
    }
}

#[component]
fn ListCard(
    cx: Scope,
    id: uuid::Uuid,
    my_lists: Resource<(), Vec<lists::Model>>, // size = 16 (0x10), align = 0x8
) -> impl IntoView {
    let delete_list_action =
        create_multi_action(cx, move |delete_list_p: &DeleteList| {
            let delete_list_p = delete_list_p.clone();
            async move {
                tracing::info!("delete list: {:?}", delete_list_p.list_id);
                // remove from resource
                my_lists.update(|lists| {
                    if let Some(lists) = lists {
                        lists.retain(|list| list.id != delete_list_p.list_id);
                    }
                });
                delete_list(cx, delete_list_p.list_id).await?;
                remove_from_local_storage(delete_list_p.list_id);
                Ok::<(), ServerFnError>(())
            }
        })
        .using_server_fn::<DeleteList>();
    let delete_list_pending = move || {
        let submissions = delete_list_action.submissions().get();

        let pending = submissions.iter().any(|s| s.pending().get());
        tracing::info!("delete_list_pending: {}", pending);
        pending
    };

    // let edit_list_action =
    //     create_multi_action(cx.clone(), move |edit_list_p: &EditList| {
    //         let cx = cx.clone();
    //         let edit_list_p = edit_list_p.clone();
    //         async move {
    //             edit_list(cx, edit_list_p.list_id, edit_list_p.title).await?;
    //             Ok::<(), ServerFnError>(())
    //         }
    //     });

    view! { cx,
        <Card class="card-bordered">
            {format!("todo: {id}")} <MultiActionForm action=delete_list_action>
                <input type="hidden" name="list_id" value=id.to_string()/>
                <button class="btn btn-ghost btn-square btn-sm">
                    <span class="sr-only">"Delete"</span>
                    <Spinner class="w-6 h-6" hidden=move || !delete_list_pending()/>
                    <div class:hidden=delete_list_pending>
                        <Icon class="w-6 h-6" icon=icon!(OcTrashLg)/>
                    </div>
                </button>
            </MultiActionForm>
        </Card>
    }
}

struct MyLists {
    lists: Vec<uuid::Uuid>,
}

#[component]
pub fn IndexPage(cx: Scope) -> impl IntoView {
    let my_lists = create_local_resource(
        cx,
        move || (),
        move |_| async move {
            let list_ids = retrieve_from_local_storage();
            let lists = list_ids
                .into_iter()
                .map(|list_id| find_list(cx, list_id))
                .collect::<Vec<_>>();

            // TODO: handle errors better (for deleted lists this is fine,
            // but for other errors we should probably show an error message)
            futures::future::join_all(lists)
                .await
                .into_iter()
                .filter_map(Result::ok)
                .collect()
        },
    );

    let add_list_action =
        create_multi_action(cx, move |add_list_p: &AddList| {
            let add_list_p = add_list_p.clone();
            async move {
                let list = add_list(cx, add_list_p.title).await?;
                add_to_local_storage(list.id);
                my_lists.update(|lists: &mut Option<Vec<lists::Model>>| {
                    if let Some(lists) = lists {
                        lists.push(list);
                    }
                });
                tracing::info!("new_lists: {:?}", my_lists);
                Ok::<(), ServerFnError>(())
            }
        })
        .using_server_fn::<AddList>();

    let add_list_pending = move || {
        let submissions = add_list_action.submissions().get();

        let pending = submissions.iter().any(|s| s.pending().get());

        tracing::info!("add_list_pending: {}", pending);
        pending
    };

    view! { cx,
        <div class="overflow-y-auto min-h-screen">
            <For
                each=move || my_lists.read(cx).unwrap_or(vec![])
                key=|list| list.id
                view=move |cx, list: entity::lists::Model| {
                    view! { cx, <ListCard id=list.id my_lists=my_lists/> }
                }
            />
            <MultiActionForm action=add_list_action>
                <input type="hidden" name="title" value="mytitle"/>
                <button type="submit" class="mx-4 shadow-xl btn btn-primary">
                    <Spinner class="w-6 h-6" hidden=move || !add_list_pending()/>
                    <div class:hidden=add_list_pending>
                        <Icon class="w-6 h-6" icon=icon!(OcFileAddedSm)/>
                    </div>
                    <span>"Add List"</span>
                </button>
            </MultiActionForm>
        </div>
    }
}
