use leptos::*;

#[derive(Debug, Clone)]
struct DatabaseEntry {
    key: String,
    value: RwSignal<i32>,
}

#[component]
pub fn App() -> impl IntoView {
    let (count, set_count) = create_signal(0);
    let double_count = move || count.get() * 2;
    set_count.set(0);

    let (data, _set_data) = create_signal(vec![
        DatabaseEntry {
            key: "foo".to_string(),
            value: create_rw_signal(10),
        },
        DatabaseEntry {
            key: "bar".to_string(),
            value: create_rw_signal(20),
        },
        DatabaseEntry {
            key: "baz".to_string(),
            value: create_rw_signal(15),
        },
    ]);

    let values = vec![0, 1, 2];

    let html = "<p>hi!</p>";
    view! {
        <button on:click=move |_| {
            data.with(|data| {
                for row in data {
                    row.value.update(|value| *value *= 2);
                }
            });
            logging::log!("{:?}", data.get());
        }>
            "Update Values"
        </button>

        <button class:red=move || count.get() % 2 == 0 on:click=move |_| { set_count.update(|n| *n = *n+1);}>
            "Click Me: "
            {count}
        </button>
        <Progress max=50 progress=count />
        <Progress progress=double_count />
        <div inner_html=html/>
        <ul>
            {values.into_iter().map(|num| view! {
              <li>
                  <button on:click=move |_| set_count.update(|n| *n += num)>
                      {num}
                  </button>
              </li>
            }).collect_view()}
        </ul>
        <For
            each=move || data.get()
            key=|state| state.key.clone()
            let:child
        >
            <p>{child.value}</p>
        </For>
    }
}

// Show progress towards a goal
#[component]
pub fn Progress(
    // maximum value
    #[prop(default = 10)] max: u16,
    // amount of progress that should be displayed
    #[prop(into)] progress: Signal<i32>,
) -> impl IntoView {
    view! {
        <progress max=max value=progress />
    }
}
