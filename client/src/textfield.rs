#![allow(non_snake_case)]
// use leptos::{ev::SubmitEvent, html::Input, *};
use leptos::*;

#[component]
pub fn TextField() -> impl IntoView {
    view! {
        <div class="text-field-container">
            <Header />
            <div class="text-field" contentEditable>
                <div><strong>Hello</strong> World!</div>
                <div>Line 2 <i>hey</i></div>
            </div>
        </div>
    }
}

#[component]
fn Header() -> impl IntoView {
    view! {
        <div class="text-field-header">
            <span class="btn"><strong>B</strong></span>
            <span class="btn"><strong>I</strong></span>
            <span class="btn"><strong>U</strong></span>
        </div>
    }
}
