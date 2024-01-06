use common::{Client, ClientListEvent, Event, GridEvent, InitEvent, CLIENT_LIST, GRID, INIT};
use futures_util::{SinkExt, StreamExt};
use log::{info, warn};
use std::collections::HashMap;
use std::sync::Arc;
use std::{env, io::Error};
use tokio::net::{TcpListener, TcpStream};
use tokio::sync::{
    mpsc::{unbounded_channel, UnboundedSender},
    RwLock,
};
use tokio_tungstenite::tungstenite::Message;

type Clients = Arc<RwLock<HashMap<String, WsClient>>>;

#[derive(Debug, Clone)]
pub struct WsClient {
    pub name: String,
    pub sender: UnboundedSender<String>,
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    let clients: Clients = Arc::new(RwLock::new(HashMap::new()));
    let _ = env_logger::try_init();
    let addr = env::args()
        .nth(1)
        .unwrap_or_else(|| "127.0.0.1:3000".to_string());

    let try_socket = TcpListener::bind(&addr).await;
    let listener = try_socket.expect("ws socket failed");
    info!("WS Listening on: {addr}");

    while let Ok((stream, _)) = listener.accept().await {
        tokio::spawn(accept_connection(stream, clients.clone()));
    }

    Ok(())
}

async fn handle_init(
    ev: &InitEvent,
    clients: Clients,
    sender: UnboundedSender<String>,
    client_id: Arc<RwLock<Option<String>>>,
) {
    let name = ev.name.to_owned();
    *client_id.write().await = Some(name.clone());
    clients.as_ref().write().await.insert(
        name.clone(),
        WsClient {
            name: ev.name.to_owned(),
            sender: sender.clone(),
        },
    );

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
        let _ = client.1.sender.send(serialized.clone());
    });
}

async fn handle_close(
    clients: Clients,
    client_id: Arc<RwLock<Option<String>>>,
    addr: std::net::SocketAddr,
) {
    if let Some(ref ci) = *client_id.read().await {
        clients.as_ref().write().await.remove(ci);
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
            let _ = client.1.sender.send(serialized.clone());
        });
        info!("disconnected: {:?} at {addr}", ci);
    }
}

async fn handle_change(ev: &GridEvent, clients: Clients) {
    let d = ev.data.clone();

    let client_msg_event = Event {
        t: GRID.to_string(),
        data: serde_json::to_value(GridEvent {
            data: d.clone(),
            sender: ev.sender.clone(),
            timestamp: ev.timestamp,
        })
        .expect("can serialize GRID event"),
    };

    let serialized =
        serde_json::to_string(&client_msg_event).expect("can serialize client GRID event");

    clients.read().await.iter().for_each(|client| {
        if client.0 != &ev.sender {
            let _ = client.1.sender.send(serialized.clone());
        }
    })
}

async fn accept_connection(stream: TcpStream, clients: Clients) {
    let addr = stream
        .peer_addr()
        .expect("connected streams should have a peer address");
    let ws_stream = tokio_tungstenite::accept_async(stream)
        .await
        .expect("Error during the websocket handshake occurred");

    info!("new ws connection: {addr}");

    let (mut sender, mut receiver) = ws_stream.split();
    let (tx, mut rx) = unbounded_channel::<String>();
    let client_id: Arc<RwLock<Option<String>>> = Arc::new(RwLock::new(None));

    loop {
        tokio::select! {
            msg = receiver.next() => {
                match msg {
                    Some(msg) => {
                        let msg = msg.expect("msg is there");
                        if msg.is_text() {
                            if let Ok(evt) = serde_json::from_str::<Event>(msg.to_text().expect("msg is text")) {
                                match evt.t.as_str() {
                                    INIT => {
                                        if let Ok(event) = serde_json::from_value::<InitEvent>(evt.data) {
                                            handle_init(&event, clients.clone(), tx.clone(), client_id.clone()).await;
                                        }
                                    },
                                    GRID => {
                                        if let Ok(event) = serde_json::from_value::<GridEvent>(evt.data) {
                                            handle_change(&event, clients.clone()).await;
                                        }
                                    }
                                    event_type => {
                                        warn!("unknown event: {event_type}");
                                    }
                                }
                            }
                        } else if msg.is_close() {
                            handle_close(clients.clone(), client_id.clone(), addr).await;
                            break;
                        }
                    }
                    None => break,
                }
            },
            Some(ev) = rx.recv() => {
                sender.send(Message::Text(ev.to_owned())).await.expect("msg was sent");
            },
        }
    }
}
