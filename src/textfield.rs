use leptos::{ev::SubmitEvent, html::Input, *};

pub fn TextField() -> impl IntoView {
    view! {
        <div class="text-field">
            <textarea cols=200 rows=30 />
        </div>
    }
}
