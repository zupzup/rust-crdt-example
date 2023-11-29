use leptos::*;

#[cfg(feature = "server")]
mod server;

#[cfg(feature = "server")]
#[tokio::main]
async fn main() {
    println!("Hello, Server!");
    server::run().await
}

#[cfg(not(feature = "server"))]
fn main() {
    println!("Hello, client!");
    mount_to_body(|| view! { <App />})
}
