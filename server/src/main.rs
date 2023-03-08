#![warn(clippy::pedantic)]
#![forbid(unsafe_code)]
mod fileserv;
use std::sync::Arc;

use app::{functions::register_server_functions, *};
use axum::{
    body::Body,
    extract::{Extension, Path},
    response::{IntoResponse, Response},
    routing::{get, post},
    Router,
};
use entity::db::{Db, DbConfig};
use fileserv::file_and_error_handler;
use http::{header, HeaderMap, Request};
use leptos::*;
use leptos_axum::{
    generate_route_list,
    handle_server_fns_with_context,
    LeptosRoutes,
};
use tower_http::{
    compression::CompressionLayer,
    sensitive_headers::SetSensitiveRequestHeadersLayer,
    trace::{DefaultMakeSpan, DefaultOnRequest, DefaultOnResponse, TraceLayer},
};
use tracing::Level;

async fn server_fn_handler(
    Extension(db): Extension<Arc<Db>>,
    path: Path<String>,
    headers: HeaderMap,
    request: Request<Body>,
) -> impl IntoResponse {
    tracing::info!("serverfn: {:?}", path);
    tracing::info!("db: {:?}", db);

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
    tracing_subscriber::fmt().with_max_level(Level::INFO).init();

    register_server_functions().expect("Failed to register server functions");

    // Setting get_configuration(None) means we'll be using cargo-leptos's env values
    let conf = get_configuration(None).await.unwrap();
    let leptos_options = conf.leptos_options;
    let addr = leptos_options.site_addr;
    let routes = generate_route_list(|cx| view! { cx, <App/> }).await;
    let db = Db::connect(&DbConfig::default()).await.unwrap();
    db.run_migrations().await.unwrap();

    // build our application with a route
    let app = Router::new()
        .route("/api/*fn_name", post(server_fn_handler))
        .leptos_routes_with_handler(routes, get(leptos_routes_handler))
        .fallback(file_and_error_handler)
        .layer(Extension(Arc::new(leptos_options)))
        .layer(Extension(Arc::new(db)))
        .layer(CompressionLayer::new())
        .layer(SetSensitiveRequestHeadersLayer::new(vec![
            header::AUTHORIZATION,
            header::COOKIE,
        ]))
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(DefaultMakeSpan::new().include_headers(true))
                .on_request(DefaultOnRequest::new().level(Level::INFO))
                .on_response(DefaultOnResponse::new().level(Level::INFO)),
        );

    // run our app with hyper
    // `axum::Server` is a re-export of `hyper::Server`
    tracing::info!("listening on http://{}", &addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
