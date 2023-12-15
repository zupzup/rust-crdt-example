#![allow(non_snake_case)]
// use leptos::{ev::SubmitEvent, html::Input, *};
use leptos::*;

#[component]
pub fn Connect() -> impl IntoView {
    view! {
        <div class="connect">
            <div class="connect-name">
                <span>Name</span>
                <span><input type="text" name="name" /></span>
                <span><button>connect</button></span>
            </div>
        </div>
    }
}
