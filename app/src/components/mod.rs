mod svg;

use leptos::*;
use leptos_dom::*;
use leptos_router::*;
pub use svg::*;

#[component]
pub fn Typography(
    cx: Scope,
    children: Children,
    #[prop(optional, into)] class: String,
) -> impl IntoView {
    view! { cx, <p class=format!("prose dark:prose-invert {class}")>{children(cx)}</p> }
}
