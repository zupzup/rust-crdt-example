use serde::{Deserialize, Serialize};
use serde_json::Value;

pub const INIT: &str = "INIT";
pub const GRID: &str = "GRID";
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
pub struct GridEvent {
    pub data: Vec<Row>,
    pub sender: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Row {
    pub idx: usize,
    pub columns: Vec<Column>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Column {
    pub peer: String,
    pub timestamp: usize,
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
