#![allow(non_snake_case)]
use common::{
    ChangeEvent, ClientListEvent, Column, Event, MsgEvent, Row, CHANGE, CLIENT_LIST, MSG,
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

    // TODO: implement MSG_EVENT

    let cloned_send = send.clone();
    create_effect(move |_| {
        let change = data_change.get();
        logging::log!("in derived data signal, data: {:?}", change);

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
            logging::log!("in derived signal, msg: {msg}");
            if let Ok(evt) = serde_json::from_str::<Event>(&msg) {
                logging::log!("in derived signal parsed: {:?}", evt);
                if evt.t == CLIENT_LIST {
                    // if let Err(e) = serde_json::from_value::<ClientListEvent>(evt.data) {
                    //     logging::log!("err: {e}");
                    // }
                    logging::log!("in derived signal client list: {}", evt.t);
                    if let Ok(cl) = serde_json::from_value::<ClientListEvent>(evt.data) {
                        logging::log!("in derived signal list list: {:?}", cl);
                        set_clients.update(|c| {
                            *c = cl
                                .clients
                                .into_iter()
                                .map(|c| c.name)
                                .collect::<Vec<String>>()
                        });
                    }
                } else if evt.t == MSG {
                    if let Err(e) = serde_json::from_value::<MsgEvent>(evt.data.clone()) {
                        logging::log!("err: {e}");
                    }
                    logging::log!("in derived signal msg: {}", evt.t);
                    if let Ok(m) = serde_json::from_value::<MsgEvent>(evt.data) {
                        logging::log!("in derived signal msg: {:?}", m);
                        set_data.update(|d| *d = m.data);
                        logging::log!("data updated!");
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
