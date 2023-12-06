use futures_util::{SinkExt, StreamExt};
use log::{info, warn};
use serde::Deserialize;
use std::collections::HashMap;
use std::sync::Arc;
use std::time::Duration;
use std::{env, io::Error};
use tokio::net::{TcpListener, TcpStream};
use tokio::sync::RwLock;
use tokio_tungstenite::tungstenite::Message;

type Clients = Arc<RwLock<HashMap<String, Client>>>;

#[derive(Debug, Clone)]
pub struct Client {
    pub name: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct InitEvent {
    pub name: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct MsgEvent {
    pub text: String,
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

async fn handle_init(ev: &InitEvent) {}

async fn handle_msg(ev: &MsgEvent) {}

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
    let mut interval = tokio::time::interval(Duration::from_millis(1000));

    loop {
        tokio::select! {
            msg = receiver.next() => {
                match msg {
                    Some(msg) => {
                        let msg = msg.expect("msg is there");
                        if msg.is_text() ||msg.is_binary() {
                            sender.send(msg.clone()).await.expect("can be sent");
                            let txt = msg.to_text().expect("msg is text");
                            if let Ok(event) = serde_json::from_str::<InitEvent>(txt) {
                                handle_init(&event).await;
                            } else if let Ok(event) = serde_json::from_str::<MsgEvent>(txt) {
                                handle_msg(&event).await;
                            } else {
                                warn!("unknown event: {txt}");
                            }
                        } else if msg.is_close() {
                            break;
                        }
                    }
                    None => break,
                }
            }
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
