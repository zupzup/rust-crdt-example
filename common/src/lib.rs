use serde::{Deserialize, Serialize};
use serde_json::Value;

pub const INIT: &str = "INIT";
pub const MSG: &str = "MSG";
pub const CHANGE: &str = "CHANGE";
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
pub struct ChangeEvent {
    pub row: usize,
    pub column: usize,
    pub value: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct MsgEvent {
    pub data: Vec<Row>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Row {
    pub idx: usize,
    pub columns: Vec<Column>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Column {
    pub idx: usize,
    pub value: String,
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
