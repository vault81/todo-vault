#[cfg(feature = "ssr")]
use std::sync::{
    atomic::{AtomicI32, Ordering},
    Arc,
};

#[cfg(feature = "ssr")]
use broadcaster::BroadcastChannel;
use leptos::*;

#[cfg(feature = "ssr")]
pub fn register_server_functions() -> Result<(), ServerFnError> {
    _ = GetServerCount::register()?;
    _ = AdjustServerCount::register()?;
    _ = ClearServerCount::register()?;
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

#[cfg(feature = "ssr")]
pub fn db(cx: Scope) -> Result<Arc<entity::db::Db>, ServerFnError> {
    Ok(use_context::<Arc<entity::db::Db>>(cx)
        .ok_or("Pool missing.")
        .map_err(|e| ServerFnError::ServerError(e.to_string()))?)
}

#[server(AdjustServerCount, "/api")]
pub async fn adjust_server_count(
    cx: Scope,
    delta: i32,
    msg: String,
) -> Result<i32, ServerFnError> {
    tracing::debug!("adjust server count");
    tracing::debug!("delta: {}", delta);
    tracing::debug!("msg: {}", msg);
    let db = db(cx)?;
    let new = COUNT.load(Ordering::Relaxed) + delta;
    COUNT.store(new, Ordering::Relaxed);
    _ = COUNT_CHANNEL.send(&new).await;
    println!("message = {msg:?}");
    Ok(new)
}

#[server(ClearServerCount, "/api")]
pub async fn clear_server_count() -> Result<i32, ServerFnError> {
    tracing::debug!("clear server count");
    COUNT.store(0, Ordering::Relaxed);
    _ = COUNT_CHANNEL.send(&0).await;
    Ok(0)
}
