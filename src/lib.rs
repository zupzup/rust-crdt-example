use leptos::{ev::SubmitEvent, html::Input, *};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone)]
struct DatabaseEntry {
    key: String,
    value: RwSignal<i32>,
}

// Important: https://leptos-rs.github.io/leptos/reactivity/working_with_signals.html (with macro!)
// Split up Component and Logic for complex components for testing

#[component]
pub fn App() -> impl IntoView {
    let (cinp_val, set_cinp_val) = create_signal("initial value".to_string());
    let (ucinp_val, set_ucinp_val) = create_signal("uncontrolled initial value".to_string());
    let uncontrolled_input: NodeRef<Input> = create_node_ref();

    provide_context(set_cinp_val);

    let (value, _set_value) = create_signal(10);
    let is_odd = move || value.get() & 1 == 1;

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

    create_effect(move |_| {
        logging::log!("Value: {}", count.get());
    });
    // DON'T update state in effects - use derived signals, or memos for that
    // Effects are only for side-effects

    let on_submit = move |ev: SubmitEvent| {
        ev.prevent_default();

        let val = uncontrolled_input.get().expect("input exists").value();

        set_ucinp_val.update(|v| *v = val);
    };

    let values = vec![0, 1, 2];

    let html = "<p>hi!</p>";
    let msg = move || if is_odd() { "Odd" } else { "Even" };
    view! {
        <p>
            {msg}
        </p>
            <Show when=move || { value.get() > 5 } fallback=|| view! {<p>"Small"</p>}> // use this
                                                                                       // for
                                                                                       // expensive
                                                                                       // rerender
                                                                                       // logic
                <p>"Big"</p>
            </Show>
        <br />
        <input type="text" on:input=move |ev| {set_cinp_val.update(|v| *v = event_target_value(&ev));} prop:value={cinp_val}/> -> input value is: {cinp_val}
        <br/>
        <form on:submit=on_submit>
            <input type="text" value=ucinp_val node_ref=uncontrolled_input />
            <input type="submit" value="send" />
        </form>
        Value of uncontrolled input: {ucinp_val}
        <br/>
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
        <br/>
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
        <br/>
        <NumericInput />
        <br/>
        <br/>
        <PeopleComponent />
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

#[component]
pub fn NumericInput() -> impl IntoView {
    let (value, set_value) = create_signal(Ok(0));
    let _setter = use_context::<WriteSignal<String>>().expect("context is there"); // use from a
                                                                                   // parent
                                                                                   // component,
                                                                                   // without
                                                                                   // having to
                                                                                   // pass it
                                                                                   // through the
                                                                                   // whole tree

    let on_input = move |ev| set_value.set(event_target_value(&ev).parse::<i32>());
    view! {
        <h1>"Error Handling"</h1>
        <label>
            "Type a number: (or not!) "
            <input type="number" on:input=on_input/>
            <ErrorBoundary
            fallback=|errors| view! {
                <div class="error"><p>"Not a number!"</p>
                    <ul>
                        {move || errors.get().into_iter().map(|(_, e)| view! { <li>{e.to_string()}</li>}).collect_view()}
                    </ul>
                </div>
            }>
            <p>"You entered " <strong>{value}</strong></p>
            </ErrorBoundary>
        </label>
    }
}

#[component]
pub fn PeopleComponent() -> impl IntoView {
    let people = create_resource(|| (), |_| async move { fetch_data().await });

    view! {
        <div>
        {move || match people.get() {
            Some(Some(ppl)) => view! { <p>{format!("name: {}, height: {}, hair color: {}", ppl.name, ppl.height, ppl.hair_color)}</p> }.into_view(),
            None => view! { <p>"Loading1"</p> }.into_view(),
            Some(None) => view! { <p>"Loading2"</p> }.into_view()
        }}
        </div>
    }
}

#[derive(Serialize, Deserialize, Clone)]
struct People {
    name: String,
    height: String,
    hair_color: String,
}

const SWAPI_URL: &str = "https://swapi.dev/api/people/1";

// API
async fn fetch_data() -> Option<People> {
    let json = gloo_net::http::Request::get(SWAPI_URL)
        .send()
        .await
        .ok()?
        .text()
        .await
        .ok()?;
    logging::log!("json: {}", json);
    People::de(&json).ok()
}
