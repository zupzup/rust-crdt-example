#![allow(non_snake_case)]
use common::{
    ChangeEvent, ClientListEvent, Column, Event, GridEvent, Row, CHANGE, CLIENT_LIST, GRID,
};
use leptos::*;
use leptos_use::{use_websocket, UseWebsocketReturn};

mod clients;
mod connect;
mod grid;

#[component]
pub fn App() -> impl IntoView {
    let UseWebsocketReturn { message, send, .. } = use_websocket("ws://localhost:3000/");
    let (clients, set_clients) = create_signal(vec![]);
    let (data_change, set_data_change) = create_signal::<Option<ChangeEvent>>(None);
    let (data, set_data) = create_signal(vec![
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
    ]);

    let cloned_send = send.clone();
    create_effect(move |_| {
        let change = data_change.get();
        let change_event = serde_json::to_value(change).expect("can serialize change event");
        let serialized = serde_json::to_string(&Event {
            t: CHANGE.to_owned(),
            data: change_event,
        })
        .expect("can be serialized");
        cloned_send(&serialized);
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
                <connect::Connect send=send />
                <clients::Clients clients={clients}/>
                <grid::Grid data={data} set_data_change={set_data_change}/>
            </div>
        </div>
    }
}
