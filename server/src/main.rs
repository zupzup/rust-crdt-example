use futures_util::{SinkExt, StreamExt};
use log::{info, warn};
use serde::{Deserialize, Serialize};
use serde_json::Value;
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

const INIT: &str = "INIT";
const MSG: &str = "MSG";
const CLIENT_LIST: &str = "CLIENT_LIST";

#[derive(Debug, Clone)]
pub struct WsClient {
    pub name: String,
    pub sender: UnboundedSender<(String, String)>,
}

//TODO: use https://docs.rs/automerge/0.5.5/automerge/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Client {
    pub name: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct InitEvent {
    pub name: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct MsgEvent {
    pub text: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ClientListEvent {
    pub clients: Vec<Client>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Event {
    t: String,
    data: Value,
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    let clients: Clients = Arc::new(RwLock::new(HashMap::new()));

    let _ = env_logger::try_init();
    let addr = env::args()
        .nth(1)
        .unwrap_or_else(|| "127.0.0.1:3000".to_string());

    // Create the event loop and TCP listener we'll accept connections on.
    let try_socket = TcpListener::bind(&addr).await;
    let listener = try_socket.expect("Failed to bind");
    info!("Listening on: {addr}");

    while let Ok((stream, _)) = listener.accept().await {
        tokio::spawn(accept_connection(stream, clients.clone()));
    }

    Ok(())
}

async fn handle_init(ev: &InitEvent, clients: Clients, sender: UnboundedSender<(String, String)>) {
    clients.as_ref().write().await.insert(
        ev.name.to_owned(),
        WsClient {
            name: ev.name.to_owned(),
            sender: sender.clone(),
        },
    );
    info!("added {}", ev.name);
    let cl = ClientListEvent {
        clients: clients
            .read()
            .await
            .clone()
            .into_values()
            .map(|c| Client { name: c.name })
            .collect(),
    };
    let ser_list = serde_json::to_value(&cl).expect("can serialize cleints list");
    let clients_list_event = Event {
        t: CLIENT_LIST.to_string(),
        data: ser_list,
    };
    let serialized =
        serde_json::to_string(&clients_list_event).expect("can serialize client list event");
    clients.read().await.iter().for_each(|client| {
        info!("sending client list to {}", client.1.name);
        let _ = client
            .1
            .sender
            .send((client.1.name.to_owned(), serialized.clone()));
    });
    info!("new client list: {:?}", clients);
}

async fn handle_msg(ev: &MsgEvent, clients: Clients, sender: UnboundedSender<(String, String)>) {
    clients.read().await.iter().for_each(|client| {
        info!("sending to {}", client.1.name);
        let client_msg_event = Event {
            t: MSG.to_string(),
            data: serde_json::to_value(MsgEvent {
                text: ev.text.clone(),
            })
            .expect("can serialize msg event"),
        };
        let serialized =
            serde_json::to_string(&client_msg_event).expect("can serialized client msg event");
        let _ = sender.send((client.1.name.to_owned(), serialized.clone()));
    })
}

async fn accept_connection(stream: TcpStream, clients: Clients) {
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
                        if msg.is_text() ||msg.is_binary() {
                            let txt = msg.to_text().expect("msg is text");
                            if let Ok(evt) = serde_json::from_str::<Event>(txt) {
                                match evt.t.as_str() {
                                    INIT => {
                                        if let Ok(event) = serde_json::from_value::<InitEvent>(evt.data) {
                                            handle_init(&event, clients.clone(), tx.clone()).await;
                                        }
                                    },
                                    MSG => {
                                        if let Ok(event) = serde_json::from_value::<MsgEvent>(evt.data) {
                                            handle_msg(&event, clients.clone(), tx.clone()).await;
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

    // read.try_filter(|msg| {
    //     msg.to_text().and_then(|txt: &str| {
    //         if let Ok(event) = serde_json::from_str::<InitEvent>(txt) {
    //             handle_init(&event).await;
    //         } else if let Ok(event) = serde_json::from_str::<MsgEvent>(txt) {
    //             handle_init(&event).await;
    //         } else {
    //             warn!("unknown event: {txt}");
    //         }
    //     });
    // match serde_json::from_str(msg.to_text()) {
    //     Ok(init: InitEvent) => {
    // // TODO: for init - add new client to the client map
    //     }
    //     Ok(init: MsgEvent) => {
    // // TODO: for msg, handle incoming message (CRDT)
    //     }
    //     _ => {
    //         warn!("unknown event: {msg}");
    //     }
    // }
    // TODO: create two payloads - init and msg
    // info!("msg: {msg}, binary: {}", msg.is_binary());
    // future::ready(msg.is_text() || msg.is_binary())
    // })
    // .forward(write)
    // .await
    // .expect("Failed to forward messages")
}
