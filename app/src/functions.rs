#[cfg(feature = "ssr")]
use std::sync::{
    atomic::{AtomicI32, Ordering},
    Arc,
};

#[cfg(feature = "ssr")]
use broadcaster::BroadcastChannel;
use entity::{
    chrono::NaiveDate,
    lists,
    prelude::*,
    sea_orm::{
        entity::ActiveModelTrait,
        query::QueryOrder,
        ColumnTrait,
        EntityTrait,
        ModelTrait,
        QueryFilter,
        Select,
    },
    todos,
    uuid,
};
use leptos::*;
use serde::{Deserialize, Serialize};
use thiserror::Error;

#[cfg(feature = "ssr")]
pub fn register_server_functions() -> Result<(), ServerFnError> {
    GetServerCount::register()?;
    AdjustServerCount::register()?;
    ClearServerCount::register()?;
    AddList::register()?;
    FindList::register()?;
    AddTodo::register()?;
    ListTodos::register()?;
    TrashTodo::register()?;
    EditTodo::register()?;
    ToggleTodo::register()?;
    Ok(())
}

#[cfg(feature = "ssr")]
static COUNT: AtomicI32 = AtomicI32::new(0);

#[cfg(feature = "ssr")]
lazy_static::lazy_static! {
    pub static ref COUNT_CHANNEL: BroadcastChannel<i32> = BroadcastChannel::new();
}

#[server(GetServerCount, "/api")]
pub async fn get_server_count() -> Result<i32, ServerFnError> {
    tracing::debug!("get server count");
    Ok(COUNT.load(Ordering::Relaxed))
}

#[server(AdjustServerCount, "/api")]
pub async fn adjust_server_count(delta: i32) -> Result<i32, ServerFnError> {
    let new = COUNT.load(Ordering::Relaxed) + delta;
    COUNT.store(new, Ordering::Relaxed);
    _ = COUNT_CHANNEL.send(&new).await;
    Ok(new)
}

#[server(ClearServerCount, "/api")]
pub async fn clear_server_count() -> Result<i32, ServerFnError> {
    tracing::debug!("clear server count");
    COUNT.store(0, Ordering::Relaxed);
    _ = COUNT_CHANNEL.send(&0).await;
    Ok(0)
}

#[cfg(feature = "ssr")]
pub fn db(cx: Scope) -> Result<Arc<entity::db::Db>, ServerFnError> {
    use_context::<Arc<entity::db::Db>>(cx)
        .ok_or("Pool missing.")
        .map_err(|e| ServerFnError::ServerError(e.to_string()))
}

#[server(AddList, "/api")]
pub async fn add_list(
    cx: Scope,
    title: String,
) -> Result<lists::Model, ServerFnError> {
    let db = db(cx)?;

    let list = lists::ActiveModel::new(title)
        .insert(db.conn())
        .await
        .map_err(|e| {
            let str = format!("{e}");
            ServerFnError::ServerError(str)
        })?;

    Ok(list)
}

#[server(FindList, "/api")]
pub async fn find_list(
    cx: Scope,
    list_id: uuid::Uuid,
) -> Result<lists::Model, ServerFnError> {
    let db = db(cx)?;

    let list = lists::Entity::find_by_id(list_id)
        .one(db.conn())
        .await
        .map_err(|e| ServerFnError::ServerError(format!("{e}")))?;

    if list.is_none() {
        return Err(ServerFnError::ServerError("No list found".to_string()));
    }

    Ok(list.unwrap())
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Order {
    Asc,
    Desc,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TodoOrderBy {
    Title(Order),
    DueDate(Order),
}

#[server(ListTodos, "/api")]
pub async fn list_todos(
    cx: Scope,
    list_id: uuid::Uuid,
    search: Option<String>,
    order_by: Option<TodoOrderBy>,
) -> Result<Vec<todos::Model>, ServerFnError> {
    let db = db(cx)?;

    let list = lists::Entity::find_by_id(list_id)
        .one(db.conn())
        .await
        .map_err(|e| ServerFnError::ServerError(format!("{e}")))?;

    if list.is_none() {
        return Err(ServerFnError::ServerError("No list found".to_string()));
    }

    let todos = list.unwrap().find_related(todos::Entity);

    let todos = if let Some(search) = search {
        let filter = entity::sea_orm::Condition::any()
            .add(todos::Column::Title.contains(&search))
            .add(todos::Column::Description.contains(&search));

        todos.filter(filter)
    } else {
        todos
    };

    let todos = todos
        .order_by_asc(todos::Column::CreatedAt)
        .all(db.conn())
        .await
        .map_err(|err| {
            tracing::error!("Failed to list todos: {}", err);
            ServerFnError::ServerError("No todos found".to_string())
        })?;

    Ok(todos)
}

#[server(AddTodo, "/api")]
pub async fn add_todo(
    cx: Scope,
    list_id: uuid::Uuid,
    title: String,
    description: Option<String>,
    due_date: Option<String>,
) -> Result<(), ServerFnError> {
    let db = db(cx)?;

    let due_date = due_date
        .and_then(|str| if str.is_empty() { None } else { Some(str) })
        .map(|string| {
            let naive_date = NaiveDate::parse_from_str(&string, "%Y-%m-%d")
                .map_err(|op| ServerFnError::ServerError(format!("{}", op)))?;
            Ok(naive_date)
        })
        .transpose()?;

    todos::ActiveModel::new(list_id, title, description, due_date)
        .insert(db.conn())
        .await
        .map_err(|e| {
            let str = format!("{e}");
            ServerFnError::ServerError(str)
        })?;

    Ok(())
}

#[server(TrashTodo, "/api")]
pub async fn trash_todo(
    cx: Scope,
    id: uuid::Uuid,
) -> Result<(), ServerFnError> {
    let db = db(cx)?;

    let todo = todos::Entity::find_by_id(id)
        .one(db.conn())
        .await
        .map_err(|_| ServerFnError::ServerError("No todo found".to_string()))?
        .expect("should be unreachable #160");

    todo.delete(db.conn()).await.map_err(|_| {
        ServerFnError::ServerError("No todo deleted".to_string())
    })?;
    Ok(())
}

#[server(EditTodo, "/api")]
pub async fn edit_todo(
    cx: Scope,
    id: uuid::Uuid,
    title: String,
    description: Option<String>,
    due_date: Option<String>,
) -> Result<(), ServerFnError> {
    let db = db(cx)?;

    let due_date = due_date
        .and_then(|str| if str.is_empty() { None } else { Some(str) })
        .map(|string| {
            let naive_date = NaiveDate::parse_from_str(&string, "%Y-%m-%d")
                .map_err(|op| ServerFnError::ServerError(format!("{}", op)))?;
            Ok(naive_date)
        })
        .transpose()?;

    let mut updated: todos::ActiveModel = todos::Entity::find_by_id(id)
        .one(db.conn())
        .await
        .map_err(|_| ServerFnError::ServerError("No todo found".to_string()))?
        .expect("should be unreachable #183")
        .into();

    updated.title = entity::sea_orm::Set(title);
    updated.description = entity::sea_orm::Set(description);
    updated.due_date = entity::sea_orm::Set(due_date);

    updated.update(db.conn()).await.map_err(|_| {
        ServerFnError::ServerError("No to-do updated".to_string())
    })?;

    Ok(())
}

#[server(ToggleTodo, "/api")]
pub async fn toggle_todo(
    cx: Scope,
    id: uuid::Uuid,
) -> Result<(), ServerFnError> {
    let db = db(cx)?;
    let mut updated: todos::ActiveModel = todos::Entity::find_by_id(id)
        .one(db.conn())
        .await
        .map_err(|_| ServerFnError::ServerError("No to-do found".to_string()))?
        .unwrap()
        .into();

    updated.done = entity::sea_orm::Set(!updated.done.unwrap());

    updated.update(db.conn()).await.map_err(|_| {
        ServerFnError::ServerError("No to-do updated".to_string())
    })?;

    Ok(())
}
