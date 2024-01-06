use serde::{Deserialize, Serialize};
use serde_json::Value;

pub const INIT: &str = "INIT";
pub const GRID: &str = "GRID";
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
pub struct GridEvent {
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

pub fn init_data() -> Vec<Row> {
    vec![
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
    ]
}
