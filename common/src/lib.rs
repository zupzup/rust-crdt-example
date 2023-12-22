use serde::{Deserialize, Serialize};
use serde_json::Value;

pub const INIT: &str = "INIT";
pub const MSG: &str = "MSG";
pub const CLIENT_LIST: &str = "CLIENT_LIST";

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
    pub t: String,
    pub data: Value,
}
