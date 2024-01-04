use common::{
    ChangeEvent, Client, ClientListEvent, Column, Event, GridEvent, InitEvent, Row, CHANGE,
    CLIENT_LIST, GRID, INIT,
};
use futures_util::{SinkExt, StreamExt};
use log::{info, warn};
use std::collections::HashMap;
use std::sync::Arc;
use std::time::Duration;
use std::{env, io::Error};
use tokio::net::{TcpListener, TcpStream};
use tokio::sync::{
    mpsc::{unbounded_channel, UnboundedSender},
    RwLock,
};
use tokio_tungstenite::tungstenite::Message;

type Clients = Arc<RwLock<HashMap<String, WsClient>>>;

type Data = Arc<RwLock<Vec<Row>>>;

#[derive(Debug, Clone)]
pub struct WsClient {
    pub name: String,
    pub sender: UnboundedSender<(String, String)>,
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    let clients: Clients = Arc::new(RwLock::new(HashMap::new()));
    // TODO: init_data method
    let data = Arc::new(RwLock::new(vec![
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
    ]));

    let _ = env_logger::try_init();
    let addr = env::args()
        .nth(1)
        .unwrap_or_else(|| "127.0.0.1:3000".to_string());

    // Create the event loop and TCP listener we'll accept connections on.
    let try_socket = TcpListener::bind(&addr).await;
    let listener = try_socket.expect("Failed to bind");
    info!("Listening on: {addr}");

    while let Ok((stream, _)) = listener.accept().await {
        tokio::spawn(accept_connection(stream, clients.clone(), data.clone()));
    }

    Ok(())
}

async fn handle_init(
    ev: &InitEvent,
    clients: Clients,
    data: Data,
    sender: UnboundedSender<(String, String)>,
) {
    clients.as_ref().write().await.insert(
        ev.name.to_owned(),
        WsClient {
            name: ev.name.to_owned(),
            sender: sender.clone(),
        },
    );

    info!("added {}", ev.name);

    let serialized_data = serde_json::to_string(&Event {
        t: GRID.to_string(),
        data: serde_json::to_value(GridEvent {
            data: data.read().await.clone(),
        })
        .expect("can serialize data"),
    })
    .expect("can serialize data event");

    let serialized = serde_json::to_string(&Event {
        t: CLIENT_LIST.to_string(),
        data: serde_json::to_value(ClientListEvent {
            clients: clients
                .read()
                .await
                .clone()
                .into_values()
                .map(|c| Client { name: c.name })
                .collect(),
        })
        .expect("can serialize clients list"),
    })
    .expect("can serialize client list event");
    clients.read().await.iter().for_each(|client| {
        info!("sending client list to {}", client.1.name);
        let _ = client
            .1
            .sender
            .send((client.1.name.to_owned(), serialized.clone()));
        let _ = client
            .1
            .sender
            .send((client.1.name.to_owned(), serialized_data.clone()));
    });
    // info!("new client list: {:?}", clients);
}

async fn handle_change(ev: &ChangeEvent, clients: Clients, data: Data) {
    info!("handling change {:?}", ev);
    // change the data
    data.write().await[ev.row].columns[ev.column].value = ev.value.clone();

    let updated = data.read_owned().await;

    clients.read().await.iter().for_each(|client| {
        info!("sending to {}", client.1.name);
        let client_msg_event = Event {
            t: GRID.to_string(),
            data: serde_json::to_value(GridEvent {
                data: updated.clone(),
            })
            .expect("can serialize GRID event"),
        };
        let serialized =
            serde_json::to_string(&client_msg_event).expect("can serialized client GRID event");
        let _ = client
            .1
            .sender
            .send((client.1.name.to_owned(), serialized.clone()));
    })
}

async fn accept_connection(stream: TcpStream, clients: Clients, data: Data) {
    let addr = stream
        .peer_addr()
        .expect("connected streams should have a peer address");
    info!("Peer address: {addr}");

    let ws_stream = tokio_tungstenite::accept_async(stream)
        .await
        .expect("Error during the websocket handshake occurred");

    info!("New WebSocket connection: {addr}");

    let (mut sender, mut receiver) = ws_stream.split();
    let mut interval = tokio::time::interval(Duration::from_millis(5000));

    let (tx, mut rx) = unbounded_channel::<(String, String)>();

    loop {
        tokio::select! {
            msg = receiver.next() => {
                match msg {
                    Some(msg) => {
                        let msg = msg.expect("msg is there");
                        if msg.is_text() {
                            let txt = msg.to_text().expect("msg is text");
                            if let Ok(evt) = serde_json::from_str::<Event>(txt) {
                                match evt.t.as_str() {
                                    INIT => {
                                        if let Ok(event) = serde_json::from_value::<InitEvent>(evt.data) {
                                            handle_init(&event, clients.clone(), data.clone(), tx.clone()).await;
                                        }
                                    },
                                    CHANGE => {
                                        if let Ok(event) = serde_json::from_value::<ChangeEvent>(evt.data) {
                                            handle_change(&event, clients.clone(), data.clone()).await;
                                        }
                                    }
                                    event_type => {
                                        warn!("unknown event: {event_type}");
                                    }
                                }
                            } else {
                                warn!("unknown event: {txt}");
                            }
                        } else if msg.is_close() {
                            break;
                        }
                    }
                    None => break,
                }
            },
            Some(ev) = rx.recv() => {
                let msg = Message::Text(ev.1.to_owned());
                info!("sending msg: {} for addr {addr}", msg.clone());
                sender.send(msg).await.expect(
                    "msg was sent");
            },
            _ = interval.tick() => {
                sender.send(Message::Text("tick".to_owned())).await.expect("can be sent");
            }
        }
    }
}
