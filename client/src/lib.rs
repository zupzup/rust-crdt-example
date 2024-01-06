#![allow(non_snake_case)]
use common::{ClientListEvent, Column, Event, GridEvent, Row, CLIENT_LIST, GRID};
use leptos::{ev::SubmitEvent, html::Input, *};
use leptos_use::{use_websocket, UseWebsocketReturn};

#[derive(Debug, Clone)]
pub struct ChangeEvent {
    pub row: usize,
    pub column: usize,
    pub value: String,
}

#[component]
pub fn App() -> impl IntoView {
    let UseWebsocketReturn { message, send, .. } = use_websocket("ws://localhost:3000/");

    let (clients, set_clients) = create_signal(vec![]);
    let (data_change, set_data_change) = create_signal::<Option<ChangeEvent>>(None);
    let (data, set_data) = create_signal(init_data()); // TODO: move init_data here, add sender and timestamp to data
    let (name, set_name) = create_signal(String::default());

    let cloned_send = send.clone();
    create_effect(move |_| {
        if let Some(change) = data_change.get() {
            set_data_change.update(|dc| *dc = None);
            set_data.update(|d| d[change.row].columns[change.column].value = change.value);
            let d = data.get();
            // TODO: update local peer and increase timestamp

            let data_event = serde_json::to_value(GridEvent {
                data: d,
                timestamp: 1,
                sender: name.get(),
            })
            .expect("can serialize change event");
            let serialized = serde_json::to_string(&Event {
                t: GRID.to_owned(),
                data: data_event,
            })
            .expect("can be serialized");
            cloned_send(&serialized);
        }
    });

    create_effect(move |_| {
        let m = message.get();

        if let Some(msg) = m.clone() {
            if let Ok(evt) = serde_json::from_str::<Event>(&msg) {
                if evt.t == CLIENT_LIST {
                    if let Ok(cl) = serde_json::from_value::<ClientListEvent>(evt.data) {
                        set_clients.update(|c| {
                            *c = cl
                                .clients
                                .into_iter()
                                .map(|c| c.name)
                                .collect::<Vec<String>>()
                        });
                    }
                } else if evt.t == GRID {
                    if let Ok(m) = serde_json::from_value::<GridEvent>(evt.data) {
                        // TODO: implement CRDT logic
                        // check remote timestamp and local timestamp
                        // if remote timestamp is bigger than local, overwrite value, timestamp and
                        // peer, otherwise discard
                        // if timestamp is the same, use one at random
                        set_data.update(|d| *d = m.data);
                    }
                }
            }
        }
        m
    });

    view! {
        <div class="app">
            <div class="container">
                <span class="hidden">{move || data_change.get().is_some()}</span>
                <Connect send={send} set_name={set_name}/>
                <Clients clients={clients}/>
                <Grid data={data} set_data_change={set_data_change}/>
            </div>
        </div>
    }
}

#[component]
pub fn Connect<F>(send: F, set_name: WriteSignal<String>) -> impl IntoView
where
    F: Fn(&str) + Clone + 'static,
{
    let (connected, set_connected) = create_signal(false);
    let name_input: NodeRef<Input> = create_node_ref();

    let submit_handler = move |ev: SubmitEvent| {
        ev.prevent_default();

        let name = name_input.get().expect("input exists").value();
        send(&format!(
            r#"{{"t": "INIT","data": {{ "name": "{}" }} }}"#,
            name,
        ));
        set_connected.update(|c| *c = true);
        set_name.update(|n| *n = name);
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

#[component]
pub fn Clients(clients: ReadSignal<Vec<String>>) -> impl IntoView {
    view! {
        <div class="clients">
            <span>Clients</span>
            <ul class="clients-list">
                <For
                    each=move || clients.get()
                    key=|state| state.clone()
                    children=|child| view! { <li>{child}</li>}
                />
            </ul>
        </div>
    }
}

#[component]
fn Grid(
    data: ReadSignal<Vec<Row>>,
    set_data_change: WriteSignal<Option<ChangeEvent>>,
) -> impl IntoView {
    view! {
        <div class="grid-container">
            <div class="grid">
                <For each=move || data.get()
                 key=|r| r.idx
                 children=move |row| view! {
                     <div class="row">
                         <For each=move || row.columns.clone()
                              key=move |c| format!("{}{}", row.idx, c.idx)
                              children=move |col| view! {
                                  <input type="text" on:input=move |ev| {
                                      let val = event_target_value(&ev);
                                      set_data_change.update(|dc| *dc = Some(ChangeEvent { row: row.idx, column: col.idx, value: val.clone() }));
                                      logging::log!("update event");
                                  }
                                  prop:value=move || data.get()[row.idx].columns[col.idx].value.clone()/>
                              }/>
                     </div>
                }/>
            </div>
        </div>
    }
}

pub fn init_data() -> Vec<Row> {
    vec![
        Row {
            idx: 0,
            columns: vec![
                Column {
                    idx: 0,
                    value: String::from(""),
                },
                Column {
                    idx: 1,
                    value: String::from(""),
                },
                Column {
                    idx: 2,
                    value: String::from(""),
                },
            ],
        },
        Row {
            idx: 1,
            columns: vec![
                Column {
                    idx: 0,
                    value: String::from(""),
                },
                Column {
                    idx: 1,
                    value: String::from(""),
                },
                Column {
                    idx: 2,
                    value: String::from(""),
                },
            ],
        },
        Row {
            idx: 2,
            columns: vec![
                Column {
                    idx: 0,
                    value: String::from(""),
                },
                Column {
                    idx: 1,
                    value: String::from(""),
                },
                Column {
                    idx: 2,
                    value: String::from(""),
                },
            ],
        },
    ]
}
