use std::sync::{Arc, Mutex};

use serde::{Serialize, Deserialize};
use sqlite::Connection;



pub type Db = Arc<Mutex<Connection>>;

#[derive(Debug, Deserialize, Serialize)]
pub struct Todo {
    pub task: String
}

#[derive(Debug, Deserialize, Serialize)]
pub enum Status {
    PENDING,
    DONE,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct TodoResponse {
    id: u64,
    task: String,
    status: Status,
}