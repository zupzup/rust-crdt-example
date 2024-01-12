#![allow(non_snake_case)]
use common::{ClientListEvent, Column, Event, GridEvent, Row, CLIENT_LIST, GRID};
use leptos::{ev::SubmitEvent, html::Input, *};
use leptos_use::{use_websocket, UseWebsocketReturn};
use rand::prelude::*;

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
    let (data, set_data) = create_signal(init_data());
    let (name, set_name) = create_signal(String::default());

    let cloned_send = send.clone();
    // effect for handling the sending of local updates to other clients via the websocket server
    create_effect(move |_| {
        if let Some(change) = data_change.get() {
            set_data_change.update(|dc| *dc = None);
            set_data.update(|d| {
                let old = &d[change.row].columns[change.column];
                let new = Column {
                    idx: old.idx,
                    peer: name.get(),
                    value: change.value,
                    timestamp: old.timestamp + 1,
                };
                d[change.row].columns[change.column] = new;
            });
            let d = data.get();
            logging::log!("data: {:?}", d);

            let data_event = serde_json::to_value(GridEvent {
                data: d,
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

    // effect for handling incoming websocket messages
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
                        // simple last-write-wins CRDT merge logic
                        set_data.update(|d| {
                            for i in 0..d.len() {
                                for j in 0..d[0].columns.len() {
                                    let local = &d[i].columns[j];
                                    let remote = &m.data[i].columns[j];

                                    if local.timestamp > remote.timestamp {
                                        continue; // local version is newer - nothing to update
                                    }

                                    if local.timestamp == remote.timestamp && random() {
                                        continue; // timestamps are the same, use one at random
                                    }

                                    // overwrite local with remote
                                    d[i].columns[j] = m.data[i].columns[j].clone();
                                }
                            }
                        });
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
                                      set_data_change.update(|dc| *dc = Some(ChangeEvent { row: row.idx, column: col.idx, value: event_target_value(&ev) }));
                                  }
                                  prop:value=move || data.get()[row.idx].columns[col.idx].value.clone()/>
                              }/>
                     </div>
                }/>
            </div>
        </div>
    }
}

fn init_column(idx: usize) -> Column {
    Column {
        idx,
        value: String::default(),
        timestamp: 0,
        peer: String::default(),
    }
}

pub fn init_data() -> Vec<Row> {
    vec![
        Row {
            idx: 0,
            columns: vec![init_column(0), init_column(1), init_column(2)],
        },
        Row {
            idx: 1,
            columns: vec![init_column(0), init_column(1), init_column(2)],
        },
        Row {
            idx: 2,
            columns: vec![init_column(0), init_column(1), init_column(2)],
        },
    ]
}
