#![allow(non_snake_case)]
use leptos::{ev::SubmitEvent, html::Input, *};

#[component]
pub fn Connect<F>(send: F) -> impl IntoView
where
    F: Fn(&str) + Clone + 'static,
{
    let (connected, set_connected) = create_signal(false);
    let name_input: NodeRef<Input> = create_node_ref();

    let submit_handler = move |ev: SubmitEvent| {
        ev.prevent_default();

        let name = name_input.get().expect("input exists").value();
        send(&format!(
            r#"{{"t": "INIT", "data": {{ "name": "{}" }} }}"#,
            name
        ));
        set_connected.update(|c| *c = true);
    };

    view! {
        <div class="connect">
            <div class="connect-name">
                <form on:submit=submit_handler>
                    <span>Name</span>
                    <span><input type="text" name="name" node_ref=name_input disabled=move|| connected.get()/></span>
                    <span><input type="submit" disabled=move || connected.get() value="connect"/></span>
                </form>
            </div>
        </div>
    }
}
