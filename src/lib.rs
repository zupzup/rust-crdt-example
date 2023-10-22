use leptos::*;

#[component]
pub fn App() -> impl IntoView {
    let (count, set_count) = create_signal(0);
    let double_count = move || count.get() * 2;
    set_count.set(0);
    let html = "<p>hi!</p>";
    view! {
        <button class:red=move || count.get() % 2 == 0 on:click=move |_| { set_count.update(|n| *n = *n+1);}>
            "Click Me: "
            {count}
        </button>
        <progress max="50" value=double_count />
        <div inner_html=html/>
    }
}
