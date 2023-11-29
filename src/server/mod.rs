use axum::{response::IntoResponse, routing::get, Router};
use std::net::SocketAddr;

pub async fn run() {
    let app = Router::new().route("ws", get(ws_handler));

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();
    // tracing::debug!("listening on {}", listener.local_addr().unwrap());
    axum::serve(
        listener,
        app.into_make_service_with_connect_info::<SocketAddr>(),
    )
    .await
    .unwrap();
}

async fn ws_handler() -> impl IntoResponse {}
