#![allow(non_snake_case)]
use leptos::*;
use rust_crdt_example::App;

fn main() {
    println!("Hello, client!");
    mount_to_body(|| view! { <App />})
}
