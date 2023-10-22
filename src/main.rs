use leptos::*;
use rust_crdt_example::App;

fn main() {
    println!("Hello, world!");
    mount_to_body(|| view! { <App />})
}
