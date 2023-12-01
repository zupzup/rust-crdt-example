#[tokio::main]
async fn main() {
    println!("Hello, Server!");
    run().await;
}

pub async fn run() {
    // let app = Router::new().route("ws", get(ws_handler));

    // let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
    //     .await
    //     .unwrap();
    // // tracing::debug!("listening on {}", listener.local_addr().unwrap());
    // axum::serve(
    //     listener,
    //     app.into_make_service_with_connect_info::<SocketAddr>(),
    // )
    // .await
    // .unwrap();
}
