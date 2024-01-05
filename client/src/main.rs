#![allow(non_snake_case)]
use leptos::*;
use rust_crdt_example::App;

fn main() {
    mount_to_body(|| view! { <App />})
}
