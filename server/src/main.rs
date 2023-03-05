mod db;
mod fileserv;
use std::sync::Arc;

use app::*;
use axum::{
    body::Body,
    extract::{Extension, Path},
    response::{IntoResponse, Response},
    routing::{get, post},
    Router,
};
use db::Db;
use fileserv::file_and_error_handler;
use http::{HeaderMap, Request};
use leptos::*;
use leptos_axum::{
    generate_route_list,
    handle_server_fns_with_context,
    LeptosRoutes,
};

use crate::db::DbConfig;

async fn server_fn_handler(
    Extension(db): Extension<Arc<Db>>,
    path: Path<String>,
    headers: HeaderMap,
    request: Request<Body>,
) -> impl IntoResponse {
    log!("{:?}", path);

    handle_server_fns_with_context(
        path,
        headers,
        move |cx| {
            provide_context(cx, db.clone());
        },
        request,
    )
    .await
}

async fn leptos_routes_handler(
    Extension(db): Extension<Arc<Db>>,
    _path: Path<String>,
    Extension(options): Extension<Arc<LeptosOptions>>,
    request: Request<Body>,
) -> Response {
    let handler = leptos_axum::render_app_to_stream_with_context(
        (*options).clone(),
        move |cx| {
            provide_context(cx, db.clone());
        },
        |cx| view! { cx, <App/> },
    );
    handler(request).await.into_response()
}

#[tokio::main]
async fn main() {
    simple_logger::init_with_level(log::Level::Debug)
        .expect("couldn't initialize logging");

    // Setting get_configuration(None) means we'll be using cargo-leptos's env values
    // For deployment these variables are:
    // <https://github.com/leptos-rs/todo-vault#executing-a-server-on-a-remote-machine-without-the-toolchain>
    // Alternately a file can be specified such as Some("Cargo.toml")
    // The file would need to be included with the executable when moved to deployment
    let conf = get_configuration(None).await.unwrap();
    let leptos_options = conf.leptos_options;
    let addr = leptos_options.site_addr;
    let routes = generate_route_list(|cx| view! { cx, <App/> }).await;
    let db = Db::connect(&DbConfig::default()).await.unwrap();
    db.run_migrations().await.unwrap();

    // build our application with a route
    let app = Router::new()
        .route("/api/*fn_name", post(server_fn_handler))
        .route("/special/:id", get(leptos_routes_handler))
        .leptos_routes(
            leptos_options.clone(),
            routes,
            |cx| view! { cx, <App/> },
        )
        .fallback(file_and_error_handler)
        .layer(Extension(Arc::new(leptos_options)))
        .layer(Extension(Arc::new(db)));

    // run our app with hyper
    // `axum::Server` is a re-export of `hyper::Server`
    log!("listening on http://{}", &addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
