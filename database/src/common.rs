use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub enum Action {
    Get { key: String },
    Set { key: String, value: String },
    Remove { key: String },
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Reply {
    Get(Result<Option<String>, String>),
    Set(Result<(), String>),
    Remove(Result<(), String>),
}
