use entity::{todos::Entity as Todo, uuid};
use leptos::*;
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
    view! {
        cx,
        <div class=format!("m-4 shadow-xl card card-compact {class}")>
            <div class="card-body">
                {children(cx)}
            </div>
        </div>
    }
}

#[component]
fn ListCard<S, O>(
    cx: Scope,
    id: uuid::Uuid,
    delete_action: MultiAction<S, Result<O, ServerFnError>>,
) -> impl IntoView
where
    S: Clone + ServerFn + leptos::Serializable,
    O: Clone + Serializable + 'static,
{
    view! {
        cx,
        <Card class="card-bordered">
            {format!("todo: {id}")}
            <MultiActionForm action=delete_action>
                <input type="hidden" name="list_id" value={id.to_string()} />
                <button class="btn btn-ghost btn-square btn-sm">
                    <span class="sr-only">"Delete"</span>
                    <div class="w-6 h-6">
                        {Svg::Trash2}
                    </div>
                </button>
            </MultiActionForm>
        </Card>
    }
}

#[component]
pub fn IndexPage(cx: Scope) -> impl IntoView {
    let add_list_action =
        create_multi_action(cx, move |add_list_p: &AddList| {
            let cx = cx.clone();
            let add_list_p = add_list_p.clone();
            async move {
                let list = add_list(cx, add_list_p.title).await?;
                add_to_local_storage(list.id);
                Ok::<(), ServerFnError>(())
            }
        })
        .using_server_fn::<AddList>();

    let delete_list_action =
        create_multi_action(cx.clone(), move |delete_list_p: &DeleteList| {
            let cx = cx.clone();
            let delete_list_p = delete_list_p.clone();
            async move {
                remove_from_local_storage(delete_list_p.list_id);
                delete_list(cx, delete_list_p.list_id).await?;
                Ok::<(), ServerFnError>(())
            }
        })
        .using_server_fn::<DeleteList>();

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

    view! { cx,
            <div class="overflow-y-auto min-h-screen">
                <For
                    each=move || my_lists.read(cx).unwrap_or(vec![])
                    key=|list| list.id
                    view=move |cx, list: entity::lists::Model| {
                        view! { cx,
                            <ListCard id={list.id} delete_action={delete_list_action}/>
                        }
                }
                />
                <MultiActionForm action=add_list_action>
                        <input type="hidden" name="title" value="mytitle" />
                        <button
                            type="submit"
                            class="mx-4 shadow-xl btn btn-primary"
                        >
                            "Add List"
                        </button>
                </MultiActionForm>
            </div>
    }
}
